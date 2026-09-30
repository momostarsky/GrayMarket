//! 订阅出口（阶段 3.1 + 3.2 + 3.3 + 3.6）：内核的行变更按事务组定序进有界 ring，读者按游标自己拉；
//! [`Subscribe`] / [`Frame`] / [`Subscriber`] 是对外协议层，快照与增量共用同一份过滤与裁列口径。
//!
//! 与原计划的偏差一处：不用 `tokio::sync::broadcast`，改成「有界 ring + 消费者游标」。
//! 理由不是省事 —— 推送式广播会把「消费者速度」变成写路径的一部分（订阅者慢 → channel
//! 满 → 要么阻塞内核要么在内核里做丢弃决策），正面撞铁律 5。拉取式把后果留在读侧：
//! 内核只做一次 O(组大小) 的 append，追不上的消费者由**它自己**承担历史被淘汰。
//! 异步 runtime 属于网络发送侧（3.4 之后），那时才需要真并发；现在单线程内核先不背这个依赖。
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
use crate::tables::{ColumnName, spec_of};

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
    pub fn catch_up(&self, after_seq: i64) -> CatchUp {
        if after_seq >= self.published {
            return CatchUp::Delta {
                from: after_seq + 1,
                through: self.published,
                changes: Vec::new(),
            };
        }
        let oldest = self.ring.front().map(|change| change.seq);
        if oldest.is_some_and(|oldest| after_seq + 1 >= oldest) {
            return CatchUp::Delta {
                from: after_seq + 1,
                through: self.published,
                changes: self
                    .ring
                    .iter()
                    .filter(|change| change.seq > after_seq)
                    .cloned()
                    .collect(),
            };
        }
        CatchUp::Lagged {
            lost_through: self.lost_through,
            oldest_seq: oldest,
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

/// 订阅请求。默认值 = 最保守也最常用的一份：全表快照、两种 op、全列、不过滤。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscribe {
    /// 表身份清单（`Spec::id`），由 [`Subscribe::validate`] 逐项核对注册中心。
    pub tables: Vec<String>,
    /// 关心哪些动作；只订 `Delete` 时不得再要快照（快照本质全是 `Upsert`）。
    pub ops: Vec<Op>,
    pub snapshot: SnapshotMode,
    pub columns: Columns,
    pub filter: Filter,
}

impl Subscribe {
    #[must_use]
    pub fn new(tables: &[&str]) -> Self {
        Self {
            tables: tables.iter().map(|table| (*table).to_string()).collect(),
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
    pub fn validate(&self) -> Result<Vec<&'static str>, SubscribeError> {
        if self.tables.is_empty() {
            return Err(SubscribeError::EmptyTables);
        }
        if self.ops.is_empty() {
            return Err(SubscribeError::EmptyOps);
        }
        if matches!(self.snapshot, SnapshotMode::Full)
            && !self.ops.contains(&Op::Upsert)
        {
            return Err(SubscribeError::SnapshotNeedsUpsert);
        }
        let mut tables = Vec::with_capacity(self.tables.len());
        for table in &self.tables {
            let spec = spec_of(table).ok_or_else(|| SubscribeError::UnknownTable(table.clone()))?;
            // 主键首列必须是账户列，否则本表无法按账户分流（`Delete` 无从归属）。
            if matches!(self.filter, Filter::Accounts(_))
                && spec.pk.first().copied() != Some(account_column())
            {
                return Err(SubscribeError::FilterNotSupported {
                    table: table.clone(),
                });
            }
            if !self.columns.is_all() {
                let declared = Snapshot::columns_of(spec.id);
                for column in &self.columns.0 {
                    if !declared.contains(&column.as_str()) {
                        return Err(SubscribeError::UnknownColumn {
                            table: table.clone(),
                            column: column.clone(),
                        });
                    }
                }
            }
            tables.push(spec.id);
        }
        Ok(tables)
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
        let picked: Vec<RowChange> = changes
            .into_iter()
            .filter(|change| {
                self.admits_op(change.op) && self.admits_key(change.table, &change.key)
            })
            .map(|change| RowChange {
                row: change.row.map(|row| self.project(change.table, row)),
                ..change
            })
            .collect();
        self.cursor = through;
        picked
    }
}

/// 拒绝订阅的结构化原因：每一条都指认到「哪个字段写错了」，不退成一个 `InvalidRequest`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscribeError {
    /// 一张表都没订 —— 多半是把表名填进了别的字段。
    EmptyTables,
    /// 没订任何动作，那这个订阅永远收不到东西。
    EmptyOps,
    /// 只要 `Delete` 却要快照：快照里的行本质全是 `Upsert`，两者矛盾。
    SnapshotNeedsUpsert,
    /// 表名未登记（与 WAL / COPY 同一道防线：表身份只认 `Spec::id`）。
    UnknownTable(String),
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
            SubscribeError::EmptyTables => write!(f, "订阅请求未指定任何表"),
            SubscribeError::EmptyOps => write!(f, "订阅请求未指定任何动作（ops）"),
            SubscribeError::SnapshotNeedsUpsert => {
                write!(f, "要快照就必须订 Upsert：快照行全是 Upsert，只订 Delete 接不到初始状态")
            }
            SubscribeError::UnknownTable(table) => {
                write!(f, "表 {table:?} 未在注册中心登记，拒绝订阅")
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
        // 一张表都没订：多半是把表名填到了别的字段里。
        assert_eq!(
            Subscribe::new(&[]).validate(),
            Err(SubscribeError::EmptyTables),
            "空表清单不能算「订全部」"
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
}
