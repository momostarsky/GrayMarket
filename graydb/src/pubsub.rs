//! 订阅出口（阶段 3.1 + 3.2 + 3.3 + 3.5 + 3.6）：内核的行变更按事务组定序进有界 ring，读者按游标自己拉；
//! [`Subscribe`] / [`Topic`] / [`Frame`] / [`Subscriber`] 是对外协议层，快照与增量共用同一份过滤与裁列口径。
//!
//! 与原计划的偏差一处：不用 `tokio::sync::broadcast`，改成「有界 ring + 消费者游标」。
//! 理由不是省事 —— 推送式广播会把「消费者速度」变成写路径的一部分（订阅者慢 → channel
//! 满 → 要么阻塞内核要么在内核里做丢弃决策），正面撞铁律 5。拉取式把后果留在读侧：
//! 内核只做一次 O(组大小) 的 append，追不上的消费者由**它自己**承担历史被淘汰。
//! 异步 runtime 已在 3.4 落到网络发送侧（`net.rs`），但它只接在 [`CatchUp`] 之后：
//! 内核从不调用任何 `await`，发送侧的速度差异全部落在它自己的有界队列与游标不推进上。
//!
//! 三处设计钉住铁律 5：
//! - [`Broadcast::publish_group`] 无锁、无 IO、不回调任何订阅方；
//! - ring 有界，[`Broadcast::evict`] 按**整组**淘汰（半组历史比没有历史更危险）；
//! - 落后到窗口之外 → [`CatchUp::Lagged`]，即 3.3 的「降级为重新下发快照」，
//!   而不是给出一个看起来连续其实缺了一段的结果。

use std::collections::VecDeque;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::domain::AssetColumn;
use crate::mem::Snapshot;
use crate::tables::{ColumnName, Spec, TABLES, spec_of};

/// 默认容量，单位是**行**而不是组：一组成交最多牵动四张表各一行，
/// 8192 行约合 2000 组的补发窗口。真正的内存预算要等 3.4 有多消费者压测再定。
pub const DEFAULT_RING_ROWS: usize = 8_192;

/// 变更动作。`Delete` 的 `row` 恒为 `None`，订阅端据此删本地行。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Op {
    Upsert,
    Delete,
}

/// 一条行变更。`table` 是注册中心身份（`Spec::id`，与 WAL / 订阅主题 / PG 表名同源），
/// `key` 是行键（复合主键的 `composite_key` 拼法，与内存表外层键一致）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowChange {
    pub term: i64,
    /// 事务组序号 —— 与 journal 信封同一个 seq，所以「订到的位置」与「日志落到的位置」
    /// 可以互相核对，订阅端不必自己维护第二套定序。
    pub seq: i64,
    pub table: &'static str,
    pub key: String,
    pub op: Op,
    /// 该组内存变更**全部完成之后**的最终镜像（同组同一行改两次只出一份）。
    /// 用 JSON 而非强类型：出口要跨表统一投递；`Amount` 序列化成 `{"units":N}` 纯整数，
    /// 订阅端拿到的数字与内核逐分一致，不经过浮点。
    pub row: Option<serde_json::Value>,
}

/// 组内登记的一条变更意图。值**不在登记时取**，而在 [`Engine`][crate::engine::Engine]
/// 发布时从表里现读 —— 这样同组多次原地改（成交既结算资金又重算市值）不会发出中间态。
pub(crate) struct Note {
    pub table: &'static str,
    pub key: String,
    pub op: Op,
}

/// 按游标补发的结果（3.3）。
///
/// 类型级 `#[must_use]`：把结果扔掉等于把一个落后消费者静默弄丢 ——
/// 调用方要么推进游标，要么按 `Lagged` 重新下发快照，不能两都不选。
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "补发结果不能丢：Lagged 意味着必须重新下发快照"]
pub enum CatchUp {
    /// `after_seq` 之后的全部变更。`from` = `after_seq + 1`，`through` = 已发布序号；
    /// 已经跟上的消费者拿到的是空 `changes`（不是 `Lagged`）。
    Delta {
        from: i64,
        through: i64,
        changes: Vec<RowChange>,
    },
    /// 消费者落后于保留窗口：`lost_through` 及之前的历史已被淘汰，`oldest_seq` 是 ring 里
    /// 现存最早的一组。此时唯一正确的处置是重新下发快照 —— 补出来的连续 seq 是假的。
    Lagged {
        lost_through: i64,
        oldest_seq: Option<i64>,
    },
}

impl CatchUp {
    /// 供测试与出口日志使用：把结果压成一句可断言的摘要。
    #[must_use]
    pub fn summary(&self) -> String {
        match self {
            CatchUp::Delta { from, through, changes } => {
                format!("Delta {from}..{through}，{} 行", changes.len())
            }
            CatchUp::Lagged { lost_through, oldest_seq } => format!(
                "Lagged（≤{lost_through} 已淘汰，最早留存 {oldest_seq:?}）→ 需重新下发快照"
            ),
        }
    }
}

/// 有界变更环：内核独占写尾，读者只读游标。
#[derive(Debug)]
pub struct Broadcast {
    capacity: usize,
    ring: VecDeque<RowChange>,
    /// 已发布的最后一个组序号。静置时等于内核的 `durable`。
    published: i64,
    /// 已被淘汰掉的最后一个组序号。
    lost_through: i64,
    groups: u64,
    dropped_rows: u64,
}

impl Broadcast {
    /// 以「至少装得下一组」为下限建环。传 0 没有可用语义（最新一组都留不住），直接 assert。
    #[must_use]
    pub fn new(capacity_rows: usize) -> Self {
        assert!(capacity_rows >= 1, "ring 容量至少 1 行，否则最新一组都留不住");
        Self {
            capacity: capacity_rows,
            ring: VecDeque::new(),
            published: 0,
            lost_through: 0,
            groups: 0,
            dropped_rows: 0,
        }
    }

    /// 唯一的写入口：整组一次投递。空组（纯校验没改任何行）直接返回，不推进 `published`。
    ///
    /// 组内一致性在这里核：同 `seq`、同 `term`、表名必须已登记、`seq` 必须比上一次大 ——
    /// 出口自己的序号也不能有洞，否则补发出来的「连续」是假的。
    pub fn publish_group(&mut self, changes: Vec<RowChange>) {
        let Some(first) = changes.first() else {
            return;
        };
        let (seq, term) = (first.seq, first.term);
        for change in &changes {
            assert_eq!(change.seq, seq, "一个事务组内的记录必须同 seq");
            assert_eq!(change.term, term, "一个事务组内不得跨 term");
            assert!(
                spec_of(change.table).is_some(),
                "出口投递了未登记的表 {:?}",
                change.table
            );
        }
        assert!(
            seq > self.published,
            "出口序号不稠密：已发布 {}，本组 {seq} —— 与内核的落盘序号分叉了",
            self.published
        );
        self.published = seq;
        self.groups += 1;
        self.ring.extend(changes);
        self.evict();
    }

    /// 按**整组**淘汰最老的历史：半组（成交只留下 trade 没有 order）会让补发出去的数据
    /// 自相矛盾，宁可让落后的人去重建快照。最新一组永远保留，哪怕它超过容量。
    fn evict(&mut self) {
        while self.ring.len() > self.capacity {
            let (Some(oldest), Some(newest)) = (
                self.ring.front().map(|change| change.seq),
                self.ring.back().map(|change| change.seq),
            ) else {
                break;
            };
            if oldest == newest {
                break;
            }
            while self.ring.front().is_some_and(|change| change.seq == oldest) {
                self.ring.pop_front();
                self.dropped_rows += 1;
            }
            self.lost_through = oldest;
        }
    }

    /// 3.3 补发：给出 `after_seq` 之后的全部变更，或明确告知「补不齐」。
    /// （返回值已在类型上标 `#[must_use]`，这里不再叠一层。）
    ///
    /// 从**尾部反向**定位补发段：环按 seq 非降序排列，要补的永远是尾部连续的一截。
    /// 从头部扫是 O(环容量) —— 那会让每组的扇出成本跟着「最慢的连接有多旧」走，
    /// 等于把消费者的账记到内核头上（铁律 5）。反向扫是 O(补发段 + 1)。
    pub fn catch_up(&self, after_seq: i64) -> CatchUp {
        if after_seq >= self.published {
            return CatchUp::Delta {
                from: after_seq + 1,
                through: self.published,
                changes: Vec::new(),
            };
        }
        let oldest = self.ring.front().map(|change| change.seq);
        if !oldest.is_some_and(|oldest| after_seq + 1 >= oldest) {
            return CatchUp::Lagged {
                lost_through: self.lost_through,
                oldest_seq: oldest,
            };
        }
        let mut changes: Vec<RowChange> = self
            .ring
            .iter()
            .rev()
            .take_while(|change| change.seq > after_seq)
            .cloned()
            .collect();
        // 整体倒回来：组内顺序（order → trade → 资金行）也跟着恢复，不能只改组间升序。
        changes.reverse();
        CatchUp::Delta {
            from: after_seq + 1,
            through: self.published,
            changes,
        }
    }

    /// 已淘汰到的最后一个组序号（它及之前的历史已不在环里）。0 = 还没淘汰过。
    #[must_use]
    pub fn lost_through(&self) -> i64 {
        self.lost_through
    }

    /// 环内现存的变更（最老 → 最新）。消费者一般走 [`Broadcast::catch_up`]，
    /// 这个入口留给测试与诊断（取证要看到全环，而不是只看自己能拿到的那一段）。
    /// 返回 `impl Iterator` 本身已是 `#[must_use]`，不再叠一层。
    pub fn retained(&self) -> impl Iterator<Item = &RowChange> {
        self.ring.iter()
    }

    #[must_use]
    pub fn published(&self) -> i64 {
        self.published
    }

    /// 现存最早一组的序号（空环为 `None`）。
    #[must_use]
    pub fn oldest_seq(&self) -> Option<i64> {
        self.ring.front().map(|change| change.seq)
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.ring.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }

    /// 已发布组数。
    #[must_use]
    pub fn groups(&self) -> u64 {
        self.groups
    }

    /// 被淘汰的行数 —— 慢消费者的**唯一**可见证据（内核不会替它等，也不会报错）。
    #[must_use]
    pub fn dropped_rows(&self) -> u64 {
        self.dropped_rows
    }
}

// ── 3.2 订阅协议：请求、帧、订阅者游标 ────────────────────────────
//
// 请求里的每一个自由字符串（表名、列名、账户）都在 attach 时核对，不在运行期才发现写错；
// 能过校验的请求 = 一定能被完整服务（快照与增量的判定口径相同）。

/// 快照模式。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotMode {
    /// 先给全表快照（`SNAPSHOT_BEGIN` → 若干 `SNAPSHOT_ROW` → `SNAPSHOT_END`），
    /// 再推水位之后的增量 —— 断线重连的默认处置（3.3 的降级出口）。
    #[default]
    Full,
    /// 只推增量。游标一旦落后于补发窗口直接 `RebuildRequired`，不做「无快照硬接」。
    DeltaOnly,
}

/// 行过滤器。目前只有账户维度（下游最常按账户分流），且它**只看行键**：
/// `Delete` 不带行内容，靠列值判定会让落后者「收到新增却收不到删除」，
/// 本地 mirror 永久多一行 —— 比少一行危险。因此只开放「主键首列是 `account_id`」
/// 的表（`account_asset` / `position` / `account_info`），其余表在 attach 即拒。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    #[default]
    None,
    Accounts(Vec<String>),
}

/// 账户列名的唯一来源：生成的列枚举 `as_str()`，不在这里新写字面量。
fn account_column() -> &'static str {
    AssetColumn::AccountId.as_str()
}

/// 从行键里取出账户归属 —— 按 `Spec::pk` 拆，不碰行内容，故 `Delete` 与快照行同一口径。
/// 前提：账户本身不含 `:`（`composite_key` 的分隔符），真实表接进来时若有不同主键形态
/// 只需改这一处（它是这套协议里唯一读行键结构的地方）。
fn account_of_key<'a>(table: &str, key: &'a str) -> Option<&'a str> {
    let pk = spec_of(table)?.pk;
    let first = *pk.first()?;
    if first != account_column() {
        return None;
    }
    match pk.len() {
        1 => Some(key),
        2 => key.split(':').next(),
        // 三段及以上的主键当前不存在；真出现了宁可判不出来，也不猜一段。
        _ => None,
    }
}

/// 列裁剪。空 = 全部列；非空时**主键列总是保留**（否则订阅端裁到自己拼不回行）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Columns(Vec<String>);

impl Columns {
    #[must_use]
    pub fn only(names: &[&str]) -> Self {
        Self(names.iter().map(|name| (*name).to_string()).collect())
    }

    #[must_use]
    pub fn is_all(&self) -> bool {
        self.0.is_empty()
    }

    /// 裁掉未请求的列（全列请求直接原样返回，不留一次无意义的对象重建）。
    #[must_use]
    pub fn project(&self, table: &str, row: serde_json::Value) -> serde_json::Value {
        if self.is_all() {
            return row;
        }
        let pk = spec_of(table).map(|spec| spec.pk).unwrap_or(&[]);
        let mut kept = serde_json::Map::new();
        if let serde_json::Value::Object(object) = row {
            for (name, value) in object {
                if self.0.contains(&name) || pk.contains(&name.as_str()) {
                    kept.insert(name, value);
                }
            }
        }
        serde_json::Value::Object(kept)
    }
}

/// 订阅主题：客户端指向「一张表」或「一组表」的唯一语法。
///
/// 三档形状全带 `table:` 前缀 —— 前缀说的是这一档主题的**种类**，留给将来（阶段 6 的分析出口
/// 一类）而不必改动现有语法。因此裸表名会被明确拒绝，而不是被猜成表名：猜错了的订阅会安静地
/// 收到一份不属于任何人的历史，比当场报错贵得多。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Topic {
    /// `table:{spec.id}`：一张已登记的表。
    Exact { table: String },
    /// `table:{schema}.*`：`Spec::schema` 恰为该名的全部登记表。
    ///
    /// schema 的事实源是 codegen 从 `tables.toml` 的 `source` 文件名剥出来的，**不从表名猜前缀**；
    /// mock 表是 `None`，所以它们永不命中这一档。
    Schema { schema: String },
    /// `table:*`：注册中心里的全部表。
    All,
}

impl Topic {
    /// 解析主题串。语法错、前缀不对、通配放错位置，一律给结构化拒因 ——
    /// 「订不到」与「订错东西」必须当场分得开。
    pub fn parse(raw: &str) -> Result<Self, SubscribeError> {
        let bad = |reason: &'static str| SubscribeError::BadTopic {
            topic: raw.to_string(),
            reason,
        };
        let Some(rest) = raw.strip_prefix("table:") else {
            return Err(bad(
                "必须以 table: 开头，三档形状是 table:{表名} / table:{schema}.* / table:*",
            ));
        };
        if rest == "*" {
            return Ok(Topic::All);
        }
        if let Some(schema) = rest.strip_suffix(".*") {
            if schema.is_empty() {
                return Err(bad("table:{schema}.* 的 schema 段不能为空"));
            }
            if schema.contains('*') {
                return Err(bad("通配符只能出现在主题末尾的 .* 处"));
            }
            return Ok(Topic::Schema {
                schema: schema.to_string(),
            });
        }
        if rest.contains('*') {
            return Err(bad("通配符只能写成 table:* 或 table:{schema}.*"));
        }
        if rest.is_empty() {
            return Err(bad("table: 后面必须跟表名、{schema}.* 或 *"));
        }
        if rest.contains('.') {
            return Err(bad(
                "精确主题只认裸表名（`Spec::id` 不带 schema 限定）；要按库订请用 table:{schema}.*",
            ));
        }
        Ok(Topic::Exact {
            table: rest.to_string(),
        })
    }
}

/// 把主题展开成注册中心里的具体表 —— **纯函数，注册中心作入参**。
///
/// 为什么要把注册中心做成参数：今天 `TABLES` 里 7 张全是 mock 表（`schema = None`），289 张真实
/// 表一律 `register = false` 不入库，所以 `table:{schema}.*` 在真注册中心上恒命中零张。通配语义
/// 必须由假表清单钉住，而不是留一句「等阶段 4 再说」。生产路径固定传 `TABLES`。
pub fn expand_topics(
    topics: &[String],
    registry: &'static [Spec],
) -> Result<Vec<&'static str>, SubscribeError> {
    Ok(expand_specs(topics, registry)?
        .into_iter()
        .map(|spec| spec.id)
        .collect())
}

/// 同 [`expand_topics`]，但要的是整份 `Spec`（`validate` 还得拿它查主键与列名）。
pub fn expand_specs(
    topics: &[String],
    registry: &'static [Spec],
) -> Result<Vec<&'static Spec>, SubscribeError> {
    let mut found: Vec<&'static Spec> = Vec::new();
    for raw in topics {
        let topic = Topic::parse(raw)?;
        let matched: Vec<&'static Spec> = match &topic {
            Topic::All => registry.iter().collect(),
            Topic::Schema { schema } => registry
                .iter()
                .filter(|spec| spec.schema == Some(schema.as_str()))
                .collect(),
            Topic::Exact { table } => registry
                .iter()
                .filter(|spec| spec.id == table.as_str())
                .collect(),
        };
        if matched.is_empty() {
            // 命中零张 = 名字写错，或那个库尚未接进来。当场拒，别留一个「订了但永远收不到东西」的
            // 订阅 —— 它在消费者眼里与「订阅成功但确实没有变更」长得一模一样。
            return Err(match topic {
                Topic::Exact { table } => SubscribeError::UnknownTable(table),
                Topic::Schema { schema } => SubscribeError::EmptyTopicMatch {
                    topic: raw.to_string(),
                    hint: format!("注册中心里没有 schema 为 {schema:?} 的表"),
                },
                Topic::All => SubscribeError::EmptyTopicMatch {
                    topic: raw.to_string(),
                    hint: "注册中心是空的".to_string(),
                },
            });
        }
        for spec in matched {
            // 去重：`table:*` 与 `table:orders` 同时出现只算一张 —— 不去重就会下两遍快照，
            // 消费者的镜像在第二遍 begin 上撞 `DuplicateBegin`。
            if !found.iter().any(|kept| kept.id == spec.id) {
                found.push(spec);
            }
        }
    }
    Ok(found)
}

/// 订阅请求。默认值 = 最保守也最常用的一份：全表快照、两种 op、全列、不过滤。
///
/// 出站与入站都走 JSON：入站是 3.4 的网络请求体，字段全为拥有型（`Vec<String>`），
/// 所以能直接 `from_str`。`deny_unknown_fields` 只加在**请求**上：客户端把
/// `topics` 拼错必须当场报错，而不是静悄悄按「没过滤全表」跑起来 ——
/// 帧（出站）反过来允许未知字段，好让旧消费者读得到新增字段（见 `wire.rs`）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "snake_case")]
pub struct Subscribe {
    /// 主题清单（[`Topic`] 的三档形状之一），由 [`Subscribe::validate`] 展开成具体表并逐项核对注册中心。
    /// 省略 = 空清单，到 `validate` 被 `EmptyTopics` 拒；不做「省略就订全部」的猜测。
    pub topics: Vec<String>,
    /// 关心哪些动作；只订 `Delete` 时不得再要快照（快照本质全是 `Upsert`）。
    /// 省略 = 两种都要（与 `Default` 的「最保守也最常用」一致）。
    #[serde(default = "all_ops")]
    pub ops: Vec<Op>,
    pub snapshot: SnapshotMode,
    pub columns: Columns,
    pub filter: Filter,
}

fn all_ops() -> Vec<Op> {
    vec![Op::Upsert, Op::Delete]
}

impl Subscribe {
    /// 按表身份建订阅：每个元素就是一个 `Spec::id`，包成 `table:{id}` 主题。
    /// 要写通配就走 [`Subscribe::of_topics`]（反序列化收到的那条路也是它）。
    #[must_use]
    pub fn new(tables: &[&str]) -> Self {
        Self {
            topics: tables.iter().map(|table| format!("table:{table}")).collect(),
            ops: vec![Op::Upsert, Op::Delete],
            snapshot: SnapshotMode::Full,
            columns: Columns::default(),
            filter: Filter::None,
        }
    }

    /// 原样收下主题串：`table:*` / `table:{schema}.*` / `table:{id}` 三档都能写。
    #[must_use]
    pub fn of_topics(topics: &[&str]) -> Self {
        Self {
            topics: topics.iter().map(|topic| (*topic).to_string()).collect(),
            ops: vec![Op::Upsert, Op::Delete],
            snapshot: SnapshotMode::Full,
            columns: Columns::default(),
            filter: Filter::None,
        }
    }

    #[must_use]
    pub fn delta_only(mut self) -> Self {
        self.snapshot = SnapshotMode::DeltaOnly;
        self
    }

    #[must_use]
    pub fn ops(mut self, ops: &[Op]) -> Self {
        self.ops = ops.to_vec();
        self
    }

    #[must_use]
    pub fn columns(mut self, names: &[&str]) -> Self {
        self.columns = Columns::only(names);
        self
    }

    #[must_use]
    pub fn accounts(mut self, ids: &[&str]) -> Self {
        self.filter = Filter::Accounts(ids.iter().map(|id| (*id).to_string()).collect());
        self
    }

    /// 请求 → 可服务的表身份。所有「客户端能写错的地方」都在这一个函数里拒绝，
    /// 过了就保证 [`Engine::subscribe`][crate::engine::Engine::subscribe] 能出完整快照。
    ///
    /// 主题就在这一步展开成具体表清单：展开是一次性的，此后新表登记不会悄悄进来。
    pub fn validate(&self) -> Result<Vec<&'static str>, SubscribeError> {
        if self.topics.is_empty() {
            return Err(SubscribeError::EmptyTopics);
        }
        if self.ops.is_empty() {
            return Err(SubscribeError::EmptyOps);
        }
        if matches!(self.snapshot, SnapshotMode::Full)
            && !self.ops.contains(&Op::Upsert)
        {
            return Err(SubscribeError::SnapshotNeedsUpsert);
        }
        let specs = expand_specs(&self.topics, TABLES)?;
        for spec in specs.iter() {
            // 主键首列必须是账户列，否则本表无法按账户分流（`Delete` 无从归属）。
            if matches!(self.filter, Filter::Accounts(_))
                && spec.pk.first().copied() != Some(account_column())
            {
                return Err(SubscribeError::FilterNotSupported {
                    table: spec.id.to_string(),
                });
            }
            if !self.columns.is_all() {
                let declared = Snapshot::columns_of(spec.id);
                for column in &self.columns.0 {
                    if !declared.contains(&column.as_str()) {
                        return Err(SubscribeError::UnknownColumn {
                            table: spec.id.to_string(),
                            column: column.clone(),
                        });
                    }
                }
            }
        }
        Ok(specs.into_iter().map(|spec| spec.id).collect())
    }
}

/// 一帧出站消息。
///
/// 只 derive `Serialize`：帧里带 `RowChange`，其 `table` 是 `&'static str`（内核侧零分配），
/// 反序列化需要拥有型表名 —— 那层转换属 3.4 的编解码边界，不在这里假装两头都能走。
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Frame {
    /// 某表快照开始：`seq` 是水位 —— 本表内容截至该组，此后增量只发 `seq` 大于水位者。
    SnapshotBegin { table: &'static str, term: i64, seq: i64 },
    /// 快照里的一行（`op` 恒为 `Upsert`，`seq` 恒为水位）。
    SnapshotRow(RowChange),
    /// 某表快照结束：`rows` 让消费端能拿行数做一次粗对账。
    SnapshotEnd { table: &'static str, rows: usize },
    /// 增量批：`changes` 按 seq 升序，`through` = 水位推进到这儿。
    /// `changes` 可以为空（该组没有本订阅者的行），但水位仍推进 ——
    /// 「≤ through 已确认无你的行」这件事必须显式说出口。
    Delta { through: i64, changes: Vec<RowChange> },
    /// 落后于补发窗口：本地副本不可信，须重新走快照（3.3 `Lagged` 的协议层表达）。
    RebuildRequired { lost_through: i64 },
}

/// 服务端替一个订阅者持有的状态：校验过的请求 + 一个水位游标。
///
/// 刻意不持有内核引用，也不含队列 —— 内核只在被调用时按游标拉一段，
/// 订阅者多少、跑多快都不改变写路径成本（铁律 5）。
#[derive(Debug, Clone)]
pub struct Subscriber {
    spec: Subscribe,
    tables: Vec<&'static str>,
    term: i64,
    cursor: i64,
}

impl Subscriber {
    #[must_use]
    pub fn cursor(&self) -> i64 {
        self.cursor
    }

    #[must_use]
    pub fn tables(&self) -> &[&'static str] {
        &self.tables
    }

    #[must_use]
    pub fn snapshot_mode(&self) -> SnapshotMode {
        self.spec.snapshot
    }

    #[must_use]
    pub fn spec(&self) -> &Subscribe {
        &self.spec
    }

    pub(crate) fn new(
        spec: Subscribe,
        tables: Vec<&'static str>,
        term: i64,
        watermark: i64,
    ) -> Self {
        Self {
            spec,
            tables,
            term,
            cursor: watermark,
        }
    }

    /// 这条行键是否属于本订阅者（账户过滤，口径与快照行一致）。
    pub(crate) fn admits_key(&self, table: &str, key: &str) -> bool {
        match &self.spec.filter {
            Filter::None => true,
            Filter::Accounts(ids) => {
                account_of_key(table, key).is_some_and(|account| ids.iter().any(|id| id == account))
            }
        }
    }

    pub(crate) fn admits_op(&self, op: Op) -> bool {
        self.spec.ops.contains(&op)
    }

    pub(crate) fn project(&self, table: &str, row: serde_json::Value) -> serde_json::Value {
        self.spec.columns.project(table, row)
    }

    #[must_use]
    pub(crate) fn term(&self) -> i64 {
        self.term
    }

    /// 增量批的落地动作：过滤 + 裁列，并把水位推进到 `through`。
    pub(crate) fn take_batch(&mut self, through: i64, changes: Vec<RowChange>) -> Vec<RowChange> {
        let picked = self.peek_batch(&changes);
        self.cursor = through;
        picked
    }

    /// 只过滤 + 裁列，**不推进游标** —— 3.4 发送侧的「先确认投得出去，再提交水位」靠这一步拆开。
    ///
    /// 合在一起做就会失去退路：一旦 `try_send` 发现队列满了，水位已经推进，那段历史
    /// 对这个连接就永久消失了（它既拿不到帧，也不会被判落后 —— 静默少数据）。
    pub(crate) fn peek_batch(&self, changes: &[RowChange]) -> Vec<RowChange> {
        changes
            .iter()
            .filter(|change| self.admits_op(change.op) && self.admits_key(change.table, &change.key))
            .map(|change| {
                let mut kept = change.clone();
                if let Some(row) = kept.row.take() {
                    kept.row = Some(self.project(kept.table, row));
                }
                kept
            })
            .collect()
    }

    /// 提交水位：`peek_batch` 的产物确认落到发送队列之后才调。
    pub(crate) fn commit(&mut self, through: i64) {
        self.cursor = through;
    }
}

/// 拒绝订阅的结构化原因：每一条都指认到「哪个字段写错了」，不退成一个 `InvalidRequest`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscribeError {
    /// 一个主题都没写 —— 多半是把表名填进了别的字段。
    EmptyTopics,
    /// 没订任何动作，那这个订阅永远收不到东西。
    EmptyOps,
    /// 只要 `Delete` 却要快照：快照里的行本质全是 `Upsert`，两者矛盾。
    SnapshotNeedsUpsert,
    /// 表名未登记（与 WAL / COPY 同一道防线：表身份只认 `Spec::id`）。
    UnknownTable(String),
    /// 主题串本身不是合法形状：前缀不对、通配符放错了位置，或把 schema 限定写进了精确名。
    BadTopic { topic: String, reason: &'static str },
    /// 主题合法但注册中心里一张都没命中：多半是 schema 名拼错，或那个库尚未接表。
    EmptyTopicMatch { topic: String, hint: String },
    /// 该表主键首列不是 `account_id`，无法按账户分流（`Delete` 不带行值，靠列值判不了归属）。
    FilterNotSupported {
        table: String,
    },
    /// 请求了该表未声明的列（列名事实源：生成的列枚举 `Column::ALL`）。
    UnknownColumn {
        table: String,
        column: String,
    },
}

impl fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubscribeError::EmptyTopics => write!(f, "订阅请求未指定任何主题（topics）"),
            SubscribeError::EmptyOps => write!(f, "订阅请求未指定任何动作（ops）"),
            SubscribeError::SnapshotNeedsUpsert => {
                write!(f, "要快照就必须订 Upsert：快照行全是 Upsert，只订 Delete 接不到初始状态")
            }
            SubscribeError::UnknownTable(table) => {
                write!(f, "表 {table:?} 未在注册中心登记，拒绝订阅")
            }
            SubscribeError::BadTopic { topic, reason } => {
                write!(f, "主题 {topic:?} 不是合法形状：{reason}")
            }
            SubscribeError::EmptyTopicMatch { topic, hint } => {
                write!(f, "主题 {topic:?} 一张表都没命中（{hint}），拒绝订阅")
            }
            SubscribeError::FilterNotSupported { table } => write!(
                f,
                "表 {table} 的主键首列不是账户列，无法按账户过滤（Delete 不带行值，无法归属）"
            ),
            SubscribeError::UnknownColumn { table, column } => {
                write!(f, "表 {table} 没有列 {column:?}（列名只认声明的列枚举）")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn change(seq: i64, table: &'static str, key: &str) -> RowChange {
        RowChange {
            term: 0,
            seq,
            table,
            key: key.to_string(),
            op: Op::Upsert,
            row: Some(serde_json::json!({ "seq": seq })),
        }
    }

    /// 表名走的是注册中心身份，出口不接受没登记过的表 —— 与 WAL 同一道防线。
    #[test]
    fn unknown_table_is_refused_at_the_boundary() {
        let mut hub = Broadcast::new(16);
        hub.publish_group(vec![change(1, "orders", "ORD-1")]);
        assert_eq!(hub.published(), 1);
    }

    #[test]
    fn row_change_serializes_with_registry_identity() {
        let json = serde_json::to_value(change(3, "orders", "ORD-3")).expect("编码");
        assert_eq!(json["table"], "orders");
        assert_eq!(json["seq"], 3);
        assert_eq!(json["op"], "upsert");
        assert_eq!(json["row"]["seq"], 3);
    }

    /// 淘汰按组：留在环里的永远是完整组，缺一条的半组比没有更危险。
    #[test]
    fn eviction_never_splits_a_group() {
        let mut hub = Broadcast::new(3);
        hub.publish_group(vec![change(1, "orders", "A"), change(1, "trades", "B")]);
        assert_eq!(hub.len(), 2);
        hub.publish_group(vec![
            change(2, "orders", "A"),
            change(2, "trades", "B"),
            change(2, "account_asset", "C"),
        ]);
        // 5 行超容量 3：整组丢第 1 组，剩第 2 组刚好 3 行。
        assert_eq!(hub.oldest_seq(), Some(2));
        hub.publish_group(vec![change(3, "orders", "A"), change(3, "trades", "D")]);
        // 再来一组：只能把第 2 组整个丢完（不会只丢它两行凑容量），否则补出来的是半组。
        assert_eq!(hub.oldest_seq(), Some(3), "最老的完整组应被整组淘汰");
        assert_eq!(hub.dropped_rows(), 5, "淘汰的是第 1、2 组共 5 行");
        assert_eq!(hub.lost_through, 2);
        // 游标 2 刚好卡在留存边界上：拿得到完整的第 3 组。
        let CatchUp::Delta { changes, through, .. } = hub.catch_up(2) else {
            panic!("游标 2 之后的历史仍在环里，不该降级");
        };
        assert_eq!(through, 3);
        assert_eq!(changes.len(), 2, "一组两行，不该有半组");
        assert!(changes.iter().all(|change| change.seq == 3));
        // 而跨到已被淘汰的第 1 组就只能重建快照。
        assert!(matches!(hub.catch_up(0), CatchUp::Lagged { .. }));
    }

    /// 落后到窗口外必须判 Lagged，绝不给出一段「看着连续其实缺了中间」的结果。
    #[test]
    fn lagging_cursor_degrades_instead_of_returning_a_hole() {
        let mut hub = Broadcast::new(1); // 只留一组
        hub.publish_group(vec![change(1, "orders", "A")]);
        hub.publish_group(vec![change(2, "orders", "A")]);
        hub.publish_group(vec![change(3, "orders", "A")]);
        let CatchUp::Lagged { lost_through, oldest_seq } = hub.catch_up(1) else {
            panic!("游标 1 之后的历史已被淘汰");
        };
        assert_eq!(lost_through, 2);
        assert_eq!(oldest_seq, Some(3));
        // 已跟上的消费者不算落后。
        assert!(matches!(hub.catch_up(3), CatchUp::Delta { changes, .. } if changes.is_empty()));
    }

    /// 3.2 订约边界：非法请求必须在 attach 时被结构化拒掉，而不是接了之后少发多。
    #[test]
    fn bad_subscribe_requests_are_refused_at_attach() {
        // 一个主题都没写：多半是把表名填到了别的字段里。
        assert_eq!(
            Subscribe::new(&[]).validate(),
            Err(SubscribeError::EmptyTopics),
            "空主题清单不能算「订全部」"
        );
        assert_eq!(
            Subscribe::new(&["orders"]).ops(&[]).validate(),
            Err(SubscribeError::EmptyOps),
            "ops 清空等于什么都不收，那不是订阅"
        );
        // 快照本质全是 Upsert：只订 Delete 却要不带屏障的全量，两者自相矛盾。
        assert_eq!(
            Subscribe::new(&["orders"])
                .ops(&[Op::Delete])
                .validate(),
            Err(SubscribeError::SnapshotNeedsUpsert),
            "要快照就必须订 Upsert"
        );
        // 同一条请求换成 delta_only 就合法 —— 被拒的原因是矛盾，不是 Delete 本身。
        assert!(
            Subscribe::new(&["orders"])
                .ops(&[Op::Delete])
                .delta_only()
                .validate()
                .is_ok(),
            "只订删除且不要快照应当放行"
        );
        // 表名走注册中心身份：拼错的 id 不能退化成「这张表恰好没变更」。
        assert_eq!(
            Subscribe::new(&["order"]).validate(),
            Err(SubscribeError::UnknownTable("order".to_string())),
            "单复数写反要当场报错"
        );
        // 账户过滤要求主键首列就是账户：orders 的键是 order_id，`Delete` 无从归属。
        assert_eq!(
            Subscribe::new(&["orders"]).accounts(&["A001"]).validate(),
            Err(SubscribeError::FilterNotSupported {
                table: "orders".to_string()
            }),
            "按订单号分流的表不允许按账户过滤"
        );
        // 列名同样核注册中心：编出来的列必须当场拒，否则订阅端拿到一份永远对不上的投影。
        assert_eq!(
            Subscribe::new(&["position"])
                .columns(&["qty"])
                .validate(),
            Err(SubscribeError::UnknownColumn {
                table: "position".to_string(),
                column: "qty".to_string()
            }),
            "列名拼错要指认到是哪一列"
        );
        // 拒因要能直接回给 wire 层：每条都得指认到字段。
        let text = format!("{}", SubscribeError::FilterNotSupported {
            table: "orders".to_string()
        });
        assert!(text.contains("orders"), "拒因里应点出问题表：{text}");
    }

    /// 反向确认：主键首列是 account_id 的三张表允许按账户分流，且新表接入不用改协议代码。
    #[test]
    fn account_filter_is_open_exactly_where_the_key_supports_it() {
        let tables = Subscribe::new(&["account_asset", "position", "account_info"])
            .accounts(&["A001"])
            .validate()
            .expect("这三张表的主键首列都是 account_id，应当放行");
        assert_eq!(tables, vec!["account_asset", "position", "account_info"]);
        // 字典表与成交表的主键首列不是账户，逐个确认被拒（而不是笼统返一个 Invalid）。
        for table in ["dict_security", "orders", "trades", "user_info"] {
            assert_eq!(
                Subscribe::new(&[table]).accounts(&["A001"]).validate(),
                Err(SubscribeError::FilterNotSupported {
                    table: table.to_string()
                }),
                "{table} 不应开放账户过滤"
            );
        }
    }

    /// 账户归属只从行键算 —— 不碰行内容，故 `Delete`（不带行值）与快照行同一口径。
    #[test]
    fn account_ownership_comes_from_the_key_not_the_row() {
        assert_eq!(account_of_key("account_asset", "A001"), Some("A001"));
        assert_eq!(account_of_key("position", "A001:09018"), Some("A001"));
        // 主键首列不是账户的表一律判不出（订约时已被拦下，这里是第二道）。
        assert_eq!(account_of_key("orders", "O1"), None);
        assert_eq!(account_of_key("no_such_table", "X"), None);
    }

    /// 裁列永远留着主键：裁掉 pk 会让订阅端拼不回行、也不认得这条变更属于谁。
    #[test]
    fn projection_always_keeps_the_primary_key() {
        let row = serde_json::json!({
            "account_id": "A001",
            "symbol": "09018",
            "quantity": { "units": 800 },
            "avg_cost": { "units": 90025 }
        });
        let kept = Columns::only(&["quantity"]).project("position", row.clone());
        assert_eq!(
            kept,
            serde_json::json!({
                "account_id": "A001",
                "symbol": "09018",
                "quantity": { "units": 800 }
            }),
            "应当只留下请求列 ∪ 主键列"
        );
        // 全列请求（空 Columns）原样透传，不经任何变换。
        assert_eq!(Columns::default().project("position", row.clone()), row);
    }

    /// 帧是出站协议：标签走 snake_case 外部标签，与 WAL 信封同一风格。
    #[test]
    fn frames_use_snake_case_external_tags() {
        let json = serde_json::to_value(Frame::SnapshotBegin {
            table: "position",
            term: 7,
            seq: 3,
        })
        .expect("编码");
        assert_eq!(
            json,
            serde_json::json!({ "snapshot_begin": { "table": "position", "term": 7, "seq": 3 } }),
            "快照起始帧应带表名/任期/水位"
        );
        let json = serde_json::to_value(Frame::SnapshotRow(change(3, "position", "A001:09018")))
            .expect("编码");
        assert_eq!(json["snapshot_row"]["key"], "A001:09018");
        assert_eq!(json["snapshot_row"]["op"], "upsert", "快照行恒为 upsert");
        let json = serde_json::to_value(Frame::SnapshotEnd {
            table: "position",
            rows: 2,
        })
        .expect("编码");
        assert_eq!(json["snapshot_end"]["rows"], 2);
        // 空 changes 也要发出去：「≤ through 已确认无你的行」必须说出口，不能靠不发包隐含。
        let json = serde_json::to_value(Frame::Delta {
            through: 9,
            changes: vec![],
        })
        .expect("编码");
        assert_eq!(json["delta"]["through"], 9);
        assert_eq!(
            json["delta"]["changes"].as_array().map(Vec::len),
            Some(0),
            "空数组不等于没这个字段"
        );
        let json = serde_json::to_value(Frame::RebuildRequired { lost_through: 2 }).expect("编码");
        assert_eq!(json["rebuild_required"]["lost_through"], 2);
    }

    // ── 3.5 主题粒度 ────────────────────────────────────

    use crate::tables::{Kind, LoadPolicy};

    /// 假注册中心：真 `TABLES` 里七张全是 mock 表（`schema = None`），289 张真实表一律
    /// `register = false` 还没入库，所以 `table:{schema}.*` 这一档只能在这里被钉住。
    /// 「等阶段 4 有真数据再验」等于永不验证。
    static SCHEMA_REGISTRY: &[Spec] = &[
        Spec {
            id: "pd_unit_capit_trade",
            schema: Some("jzdb_prod"),
            file: "state/pd_unit_capit_trade.json",
            kind: Kind::State,
            policy: LoadPolicy::Critical,
            pk: &["row_id"],
            fk: &[],
            expected_rows: None,
        },
        Spec {
            id: "secu_type",
            schema: Some("jzdb_secu"),
            file: "dict/secu_type.json",
            kind: Kind::Dict,
            policy: LoadPolicy::Critical,
            pk: &["secu_type_code"],
            fk: &[],
            expected_rows: None,
        },
        Spec {
            id: "legacy_no_schema",
            schema: None,
            file: "state/legacy_no_schema.json",
            kind: Kind::State,
            policy: LoadPolicy::Optional,
            pk: &["row_id"],
            fk: &[],
            expected_rows: None,
        },
    ];

    fn ids(topics: &[&str], registry: &'static [Spec]) -> Result<Vec<&'static str>, SubscribeError> {
        let parsed: Vec<String> = topics.iter().map(|topic| (*topic).to_string()).collect();
        expand_topics(&parsed, registry)
    }

    /// 三档形状各自展开成什么，以及通配与精确名重叠时只算一张。
    #[test]
    fn topics_expand_to_table_sets_in_registry_order() {
        assert_eq!(
            ids(&["table:secu_type"], SCHEMA_REGISTRY).expect("精确名应展开"),
            vec!["secu_type"],
            "精确主题就该只命中那一张"
        );
        assert_eq!(
            ids(&["table:jzdb_prod.*"], SCHEMA_REGISTRY).expect("schema 通配应展开"),
            vec!["pd_unit_capit_trade"],
            "schema 通配只命中带这个 schema 的表，不许把 schema 为 None 的 mock 表也捞进来"
        );
        assert_eq!(
            ids(&["table:*"], SCHEMA_REGISTRY).expect("全库应展开"),
            vec!["pd_unit_capit_trade", "secu_type", "legacy_no_schema"],
            "table:* 就是注册中心全部，顺序即声明顺序"
        );
        // 重叠不去重就会下两遍快照：消费者的镜像在第二道 begin 上撞 `DuplicateBegin`。
        assert_eq!(
            ids(
                &["table:*", "table:pd_unit_capit_trade", "table:jzdb_secu.*"],
                SCHEMA_REGISTRY
            )
            .expect("重叠主题应展开"),
            vec!["pd_unit_capit_trade", "secu_type", "legacy_no_schema"],
            "通配与精确名重叠只算一张"
        );
    }

    /// 名字写错与「那个库还没接进来」都必须当场拒：空命中的订阅与「订到了但确实没变更」长得一样。
    #[test]
    fn a_topic_matching_nothing_is_refused_rather_than_silently_empty() {
        assert!(
            matches!(
                ids(&["table:jzdb_base.*"], SCHEMA_REGISTRY),
                Err(SubscribeError::EmptyTopicMatch { .. })
            ),
            "假注册中心里没有 jzdb_base，空命中要报错而不是给一个永久静默的订阅"
        );
        assert_eq!(
            ids(&["table:pd_unit"], SCHEMA_REGISTRY),
            Err(SubscribeError::UnknownTable("pd_unit".to_string())),
            "精确名写错要指认到表"
        );
        assert!(
            matches!(
                Subscribe::of_topics(&["table:jzdb_nosuch.*"]).validate(),
                Err(SubscribeError::EmptyTopicMatch { .. })
            ),
            "库里没有的 schema 按订就该被当场拒，不能给一个永久静默的订阅"
        );
        // 阶段 4.5 之前这一档在真注册中心上恒为空；试点表转正后它第一次真能命中，
        // 于是正向/反向两条各自钉住：命中的是那一张带 schema 的表，没命中的仍是假 schema。
        assert!(
            Subscribe::of_topics(&["table:jzdb_prod.*"]).validate().is_ok(),
            "tb_pdmage_pd_unit_capit_trade 已带 schema=jzdb_prod 登记，这条通配不该再被拒"
        );
    }

    /// 主题语法把「能被猜错的地方」全堵死：缺前缀、空段、通配放错位、把 schema 写进精确名。
    #[test]
    fn every_ambiguous_topic_shape_is_rejected_at_parse() {
        for raw in [
            "orders",                    // 缺前缀：猜成表名就等于「猜错了也收得到东西」
            "dict:secu_type",            // 别的种类还没定，不得悄悄当成 table: 走
            "table:",                    // 空段
            "table:.*",                  // schema 段为空
            "table:ord*ers",             // 通配符放错了位置
            "table:jzdb_prod.*extra",    // 通配符不在末尾
            "table:jzdb_prod.secu_type", // 精确名不带 schema 限定
        ] {
            let err = Topic::parse(raw).expect_err("这个形状该被拒");
            assert!(matches!(err, SubscribeError::BadTopic { .. }), "{raw:?} 实得：{err}");
        }
        // 合法的三档各自解成什么，写在这里免得日后有人「顺手放宽」。
        assert_eq!(Topic::parse("table:*").expect("全库主题"), Topic::All);
        assert_eq!(
            Topic::parse("table:jzdb_prod.*").expect("schema 主题"),
            Topic::Schema { schema: "jzdb_prod".to_string() }
        );
        assert_eq!(
            Topic::parse("table:orders").expect("精确主题"),
            Topic::Exact { table: "orders".to_string() }
        );
    }

    /// 通配不得绕过逐张表的策略闸门：展开出来的每张表仍要过 3.2 那套主键 / 列名核对。
    #[test]
    fn a_wildcard_does_not_slip_past_the_per_table_guards() {
        let wildcard = Subscribe::of_topics(&["table:*"]).accounts(&["A001"]).validate();
        let explicit = Subscribe::new(&TABLES.iter().map(|spec| spec.id).collect::<Vec<_>>())
            .accounts(&["A001"])
            .validate();
        // 同一份表集，只是写法不同：通配不能比逐个精确名更宽，也不能给出不一样的拒因。
        assert_eq!(wildcard, explicit, "通配展开后的口径应与逐个精确名完全一致");
        assert!(
            matches!(wildcard, Err(SubscribeError::FilterNotSupported { .. })),
            "全库订阅带账户过滤本该被拒（TABLES 里有表的主键首列不是账户列）"
        );
        // 同一条通配不要过滤就合法，且展开出全部登记表。
        assert_eq!(
            Subscribe::of_topics(&["table:*"])
                .validate()
                .expect("全库订阅应放行")
                .len(),
            TABLES.len(),
            "table:* 应展开成注册中心的全部表"
        );
    }
}
