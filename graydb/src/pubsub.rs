//! 订阅出口（阶段 3.1 + 3.3）：内核的行变更按事务组定序进有界 ring，读者按游标自己拉。
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

use serde::{Deserialize, Serialize};

use crate::tables::spec_of;

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
}
