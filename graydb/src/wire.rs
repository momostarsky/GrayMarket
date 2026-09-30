//! 编解码边界（阶段 3.4）：把出站的 [`Frame`] 变成网线上的字节，再把字节变回可用的类型。
//!
//! 为什么非要一层拥有型表名（[`WireFrame`]）而不是直接给 [`Frame`] 加 `Deserialize`：
//! `Frame` 里的 `RowChange.table` 是 `&'static str`（内核侧零分配），serde 对它的
//! `Deserialize` 实现要求 `'de: 'static` —— 也就是输入缓冲区必须活到程序结束。网络读上来的
//! 行属于本次连接，生命周期不可能静态，硬套只会把一个真问题藏起来（要么 `Box::leak` 每行，
//! 要么在类型上撒谎）。所以这里给出一头一尾两个类型，中间是显式转换：
//!
//! - 出站：`Frame` → NDJSON 一行（[`encode_line`]），零拷贝地借用 `&'static str`；
//! - 入站：一行 → `WireFrame`（[`decode_line`]，表名是 `String`）；
//! - 归位：`WireFrame` → `Frame`（[`WireFrame::to_frame`]）——表名经注册中心
//!   [`spec_of`] 核对后拿回 `Spec::id` 那个 `&'static str`，**未登记的表名在这里必错**，
//!   不会造出一个协议类型里没有的身份。
//!
//! 帧格式先定 NDJSON（一行一帧）而不是紧凑字节流：与 journal 的 JSONL 同族，可以直接
//! telnet 上看、与日志逐行对账，零新依赖。换成带长度前缀的字节流是吞吐优化，属明确的
//! 后续项 —— 换的只是 [`encode_line`] / [`decode_line`] 这一对函数，`Mirror` 与发送侧
//! 的队列语义不动。取舍写在这里，是为了下次不必再猜一遍。
//!
//! 帧（出站）允许未知字段：新版本加的字段不该让旧消费者整帧读不出。请求（入站）反过来
//! `deny_unknown_fields`（见 [`Subscribe`](crate::pubsub::Subscribe)）：拼错字段名必须当场报错。

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

use crate::pubsub::{Frame, Op, RowChange, Subscribe};
use crate::tables::spec_of;

/// 一帧的最大字节数。防御的是对端（或中间代理）把半包、超长帧直接喂进解析器 ——
/// 超了就在分配之前拒掉，而不是解出一个 500MB 的 `row` 之后才发现不对。
pub const MAX_LINE_BYTES: usize = 1 << 20;

/// 拥有型的一行变更：与 [`RowChange`] 同字段，只有 `table` 变成 `String`。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WireRowChange {
    pub term: i64,
    pub seq: i64,
    pub table: String,
    pub key: String,
    pub op: Op,
    pub row: Option<serde_json::Value>,
}

impl WireRowChange {
    /// 归位成内核侧的 `RowChange`：表名必须已在注册中心登记，否则拒绝。
    pub fn to_row_change(&self) -> Result<RowChange, WireError> {
        let spec = spec_of(&self.table).ok_or_else(|| WireError::UnknownTable(self.table.clone()))?;
        Ok(RowChange {
            term: self.term,
            seq: self.seq,
            table: spec.id,
            key: self.key.clone(),
            op: self.op,
            row: self.row.clone(),
        })
    }
}

/// 拥有型的一帧，与 [`Frame`] 的 serde 表示逐字段对应（外部标签 + snake_case）。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WireFrame {
    SnapshotBegin { table: String, term: i64, seq: i64 },
    SnapshotRow(WireRowChange),
    SnapshotEnd { table: String, rows: usize },
    Delta { through: i64, changes: Vec<WireRowChange> },
    RebuildRequired { lost_through: i64 },
}

impl WireFrame {
    /// 归位成协议类型（表名过注册中心核对）。测试拿它证明「编码 → 解码 → 归位」是无损往返。
    pub fn to_frame(&self) -> Result<Frame, WireError> {
        Ok(match self {
            WireFrame::SnapshotBegin { table, term, seq } => Frame::SnapshotBegin {
                table: table_id(table)?,
                term: *term,
                seq: *seq,
            },
            WireFrame::SnapshotRow(change) => Frame::SnapshotRow(change.to_row_change()?),
            WireFrame::SnapshotEnd { table, rows } => Frame::SnapshotEnd {
                table: table_id(table)?,
                rows: *rows,
            },
            WireFrame::Delta { through, changes } => Frame::Delta {
                through: *through,
                changes: changes.iter().map(WireRowChange::to_row_change).collect::<Result<_, _>>()?,
            },
            WireFrame::RebuildRequired { lost_through } => Frame::RebuildRequired {
                lost_through: *lost_through,
            },
        })
    }

    /// 本帧推进到的水位（`Delta` 才有）；快照帧与 `RebuildRequired` 返回 `None`。
    #[must_use]
    pub fn through(&self) -> Option<i64> {
        match self {
            WireFrame::Delta { through, .. } => Some(*through),
            _ => None,
        }
    }
}

fn table_id(table: &str) -> Result<&'static str, WireError> {
    spec_of(table).map(|spec| spec.id).ok_or_else(|| WireError::UnknownTable(table.to_string()))
}

/// 编解码这一层的错误。跨进程收到的每一行都可能畸形，所以这里必须是值而不是 panic ——
/// 但对**已经收下并开始应用的帧**，`Mirror` 反过来选择停（见 [`MirrorError`]）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireError {
    /// 空行：换行符被单独读上来了，或对端只写了一半。
    Empty,
    /// 超过 [`MAX_LINE_BYTES`]，在分配与解析之前就拒掉。
    TooLong { bytes: usize, limit: usize },
    /// JSON 本身读不出来（含缺字段）。
    BadJson(String),
    /// 编码结果里出现裸换行 —— 那会一帧劈成两行，属于编码器 bug，不是对端问题。
    EmbeddedNewline,
    /// 表名不在注册中心：协议身份只有 `Spec::id` 这一个来源。
    UnknownTable(String),
}

impl fmt::Display for WireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WireError::Empty => write!(f, "收到空行，一帧都没读到"),
            WireError::TooLong { bytes, limit } => {
                write!(f, "单帧 {bytes} 字节，超过上限 {limit}，在解析前拒掉")
            }
            WireError::BadJson(detail) => write!(f, "帧不是合法 JSON：{detail}"),
            WireError::EmbeddedNewline => write!(f, "编码结果含裸换行，会把一帧劈成两行"),
            WireError::UnknownTable(table) => write!(f, "表 {table:?} 未在注册中心登记，拒绝归位"),
        }
    }
}

impl std::error::Error for WireError {}

/// 一帧编成一行（不含行尾换行）。
pub fn encode_line(frame: &Frame) -> Result<String, WireError> {
    let line = serde_json::to_string(frame).map_err(|err| WireError::BadJson(err.to_string()))?;
    // 兜住「键值里带换行也没关系」这件事：serde 会把字符串内的换行转义成 \n 两个字符，
    // 所以真出现裸换行只可能是编码器 bug；宁可报错也不发出会把帧劈开的字节。
    if line.contains('\n') {
        return Err(WireError::EmbeddedNewline);
    }
    Ok(line)
}

/// 一批帧编成可直接 `write_all` 的一整块 NDJSON（每帧一行，结尾带换行）。
pub fn encode_block(frames: &[Frame]) -> Result<String, WireError> {
    let mut out = String::new();
    for frame in frames {
        out.push_str(&encode_line(frame)?);
        out.push('\n');
    }
    Ok(out)
}

/// 一批帧编成每元素一行的列表（发送侧按批投递，写出去时才拼成块）。
pub fn encode_lines(frames: &[Frame]) -> Result<Vec<String>, WireError> {
    frames.iter().map(encode_line).collect()
}

/// 一行解成 `WireFrame`。先验长度与空行，再交给 JSON 解析器。
pub fn decode_line(line: &str) -> Result<WireFrame, WireError> {
    checked(line)?;
    serde_json::from_str(line.trim()).map_err(|err| WireError::BadJson(err.to_string()))
}

/// 一行解成入站订阅请求。与帧走同一道长度与空行防御。
///
/// 请求比帧多一层严格：`Subscribe` 上的 `deny_unknown_fields` 会把拼错的字段名
/// 直接回给客户端。过滤条件被静默丢弃等于把一个不想收数据的订阅者变成全量订阅者。
pub fn decode_request(line: &str) -> Result<Subscribe, WireError> {
    checked(line)?;
    serde_json::from_str(line.trim()).map_err(|err| WireError::BadJson(err.to_string()))
}

/// 两类入站共用的前置校验：空行与超长都在这一步报，不进解析器。
fn checked(line: &str) -> Result<(), WireError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(WireError::Empty);
    }
    if trimmed.len() > MAX_LINE_BYTES {
        return Err(WireError::TooLong { bytes: trimmed.len(), limit: MAX_LINE_BYTES });
    }
    Ok(())
}

// ── 拒订回话 ───────────────────────────────────────────────
//
// 它不是 [`Frame`] 的一臂：帧描述的是内核状态，而「这个请求我不服务」根本不是状态。
// 但也不能只关连接：那样客户端只剩一个 EOF 可推，拿不到「为什么」。所以走一条
// 握手层的控制行 —— 服务端写完它就收摊，客户端读到它就知道是拒订而不是掉线。

/// 编出一条拒订回话。`reason` 直接用 [`SubscribeError`][crate::pubsub::SubscribeError] 的
/// `Display`：那句人话就是协议内容。原因里带引号与换行都走 JSON 转义，一行一帧的前提不破坏。
pub fn reject_line(reason: &str) -> String {
    let payload = serde_json::json!({ "reject": { "reason": reason } });
    serde_json::to_string(&payload).expect("单字段对象不该编不出")
}

/// 一行是不是拒订回话；是则交出原因文本。
pub fn as_reject(line: &str) -> Option<String> {
    let value = serde_json::from_str::<serde_json::Value>(line).ok()?;
    value
        .get("reject")
        .and_then(|reject| reject.get("reason"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
}

// ── 消费者镜像 ───────────────────────────────────────────────────
//
// 这是「收到帧之后的那半套规矩」：帧序列必须能重建成与内核一致的行集。
// 与内核侧一样持 `BTreeMap`，键序稳定，比对与打印都不依赖到达顺序。

/// 一张表正在进行中的快照。
#[derive(Debug, Clone, Copy)]
struct Pending {
    /// 快照水位：本表每一行的 `seq` 都必须恰等于它，多一分少一分都是协议腐化。
    watermark: i64,
    /// 已收行数，与 `SNAPSHOT_END` 声明的行数核对。
    rows: usize,
}

/// 消费者从帧序列重建出来的行集。
///
/// 判定原则与内核同一路：**宁可停下报错，也不「补一个看起来合理的值」继续跑**。
/// 快照行 seq 与水位不符、END 行数对不上、`Delete` 却带着行、水位倒退 —— 每一条都
/// 让 [`Mirror::apply`] 返回 [`MirrorError`] 并保持原样，调用方据此决定重连还是告警。
/// 静默吸收才是真的危险：本地镜像与内核从此分叉，而两边都觉得自己是对的。
#[derive(Debug, Default)]
pub struct Mirror {
    rows: BTreeMap<String, BTreeMap<String, serde_json::Value>>,
    /// 正在进行中的快照（表 → 水位与已收行数）。
    pending: BTreeMap<String, Pending>,
    /// 本镜像见过的 `term`。跨 `term` 说明服务端换过任，历史不可续用。
    term: Option<i64>,
    /// 已确认的水位：`Delta.through` 必须严格超过它。
    last_through: i64,
    /// 收到 `REBUILD_REQUIRED` 后记下缺口，此后除快照外的帧一律拒收。
    stale: Option<i64>,
    /// 累计应用的行数（含删除），给摘要与测试用。
    applied: u64,
}

impl Mirror {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 按帧序列重建。
    pub fn apply_all(&mut self, frames: &[WireFrame]) -> Result<(), MirrorError> {
        for frame in frames {
            self.apply(frame)?;
        }
        Ok(())
    }

    /// 应用一帧。失败时镜像不被改动（每条写入都在校验通过之后）。
    pub fn apply(&mut self, frame: &WireFrame) -> Result<(), MirrorError> {
        match frame {
            WireFrame::SnapshotBegin { table, term, seq } => self.begin(table, *term, *seq),
            WireFrame::SnapshotRow(change) => self.snapshot_row(change),
            WireFrame::SnapshotEnd { table, rows } => self.end(table, *rows),
            WireFrame::Delta { through, changes } => self.delta(*through, changes),
            WireFrame::RebuildRequired { lost_through } => {
                // 缺口落在自己身上：明说「本地不可信」，并丢掉进行中的半张快照。
                // 已有的行留着（供排障看差集），但除了重新走快照，不接受任何后续帧。
                self.stale = Some(*lost_through);
                self.pending.clear();
                Ok(())
            }
        }
    }

    fn begin(&mut self, table: &str, term: i64, seq: i64) -> Result<(), MirrorError> {
        if let Some(prev) = self.term
            && prev != term
        {
            return Err(MirrorError::TermChanged { prev, got: term });
        }
        if self.pending.contains_key(table) {
            return Err(MirrorError::DuplicateBegin { table: table.to_string() });
        }
        self.term = Some(term);
        // 重新下发快照正是「已判落后」的唯一出路：接过这张表的权威内容，就顺手销掉落后标记。
        self.stale = None;
        // 重新下发快照 = 这张表以服务端为准，旧内容整张作废（哪怕它曾属于别的表集合）。
        self.rows.insert(table.to_string(), BTreeMap::new());
        self.pending.insert(table.to_string(), Pending { watermark: seq, rows: 0 });
        Ok(())
    }

    fn snapshot_row(&mut self, change: &WireRowChange) -> Result<(), MirrorError> {
        let reason = if change.op != Op::Upsert {
            Some("动作不是 upsert")
        } else if change.row.is_none() {
            Some("快照行不带行值")
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(MirrorError::BadSnapshotRow {
                table: change.table.clone(),
                key: change.key.clone(),
                reason,
            });
        }
        // 水位与进行中的快照必须是同一个数：这既证明行没串行到别的批次，
        // 也证明「快照截至哪一组」这件事两端读出来一致。
        let pending = self
            .pending
            .get(&change.table)
            .ok_or_else(|| MirrorError::RowOutsideSnapshot {
                table: change.table.clone(),
                key: change.key.clone(),
            })?;
        if change.seq != pending.watermark {
            return Err(MirrorError::BadSnapshotRow {
                table: change.table.clone(),
                key: change.key.clone(),
                reason: "seq 不等于快照水位",
            });
        }
        let watermark = pending.watermark;
        // 半路来的一条增量不该伪装成快照：它一定带着大于水位的 seq。
        if watermark < self.last_through {
            return Err(MirrorError::OutOfOrder { prev_through: self.last_through, through: watermark });
        }
        let row = change.row.clone().expect("上面已校验过 row 为 Some");
        self.rows
            .entry(change.table.clone())
            .or_default()
            .insert(change.key.clone(), row);
        // 校验全部通过才记账：中途失败时不能留下「行已插入但行数没加」的半截状态。
        if let Some(pending) = self.pending.get_mut(&change.table) {
            pending.rows += 1;
        }
        self.applied += 1;
        Ok(())
    }

    fn end(&mut self, table: &str, declared: usize) -> Result<(), MirrorError> {
        let pending = self
            .pending
            .remove(table)
            .ok_or_else(|| MirrorError::EndWithoutBegin { table: table.to_string() })?;
        if pending.rows != declared {
            // 已经把这张表清空过一次，行数不符就不能让它留在镜像里冒充完整。
            self.rows.remove(table);
            return Err(MirrorError::RowCountMismatch {
                table: table.to_string(),
                expected: declared,
                got: pending.rows,
            });
        }
        // 快照就是「截至这一组的全部内容」：收下它等于确认了这个水位，
        // 否则一条 through 小于水位的旧增量会被当成新批接受，交接不重不漏的前提就空了。
        self.last_through = self.last_through.max(pending.watermark);
        Ok(())
    }

    fn delta(&mut self, through: i64, changes: &[WireRowChange]) -> Result<(), MirrorError> {
        if let Some(lost_through) = self.stale {
            return Err(MirrorError::Stale { lost_through });
        }
        if through <= self.last_through {
            return Err(MirrorError::OutOfOrder { prev_through: self.last_through, through });
        }
        // 整批先验后写：中途一条畸形不该让前面的行已经落库、水位却没推进。
        for change in changes {
            if change.seq <= self.last_through || change.seq > through {
                return Err(MirrorError::SeqOutOfBatch {
                    seq: change.seq,
                    last_through: self.last_through,
                    through,
                });
            }
            let reason = match (change.op, &change.row) {
                (Op::Upsert, None) => Some("upsert 不带行值"),
                (Op::Delete, Some(_)) => Some("delete 带着行值"),
                _ => None,
            };
            if let Some(reason) = reason {
                return Err(MirrorError::BadRowShape {
                    table: change.table.clone(),
                    key: change.key.clone(),
                    reason,
                });
            }
        }
        for change in changes {
            match change.op {
                Op::Upsert => {
                    self.rows
                        .entry(change.table.clone())
                        .or_default()
                        .insert(change.key.clone(), change.row.clone().expect("整批已验过"));
                }
                Op::Delete => {
                    // 删不存在的行不当错误：幂等地接受它（重新 attach 之后本地本就可能没有这行）。
                    if let Some(table) = self.rows.get_mut(&change.table) {
                        table.remove(&change.key);
                    }
                }
            }
            self.applied += 1;
        }
        self.last_through = through;
        Ok(())
    }

    /// 某表的行集（键 → 行）。
    #[must_use]
    pub fn table(&self, table: &str) -> Option<&BTreeMap<String, serde_json::Value>> {
        self.rows.get(table)
    }

    #[must_use]
    pub fn get(&self, table: &str, key: &str) -> Option<&serde_json::Value> {
        self.rows.get(table)?.get(key)
    }

    /// 镜像里所有表名（按名排序，与内核的 `TABLES` 顺序无关）。
    #[must_use]
    pub fn tables(&self) -> Vec<&str> {
        self.rows.keys().map(String::as_str).collect()
    }

    /// 全部表的行数之和。
    #[must_use]
    pub fn rows(&self) -> usize {
        self.rows.values().map(BTreeMap::len).sum()
    }

    #[must_use]
    pub fn is_stale(&self) -> bool {
        self.stale.is_some()
    }

    /// 快照进行中的表（还没收到 `SNAPSHOT_END`）。非空表示帧序列被截断。
    #[must_use]
    pub fn unfinished(&self) -> Vec<&str> {
        self.pending.keys().map(String::as_str).collect()
    }

    #[must_use]
    pub fn last_through(&self) -> i64 {
        self.last_through
    }

    #[must_use]
    pub fn applied(&self) -> u64 {
        self.applied
    }

    /// 一句可打印、可断言的摘要。
    #[must_use]
    pub fn summary(&self) -> String {
        let mut out = format!(
            "{} 表 / {} 行，水位 {}，已应用 {} 行",
            self.rows.len(),
            self.rows(),
            self.last_through,
            self.applied
        );
        if let Some(term) = self.term {
            out.push_str(&format!("，term {term}"));
        }
        if self.stale.is_some() {
            out.push_str("，已判落后");
        }
        if !self.pending.is_empty() {
            out.push_str(&format!("，{} 表快照未完", self.pending.len()));
        }
        out
    }
}

/// 收下帧之后发现的问题：全是「协议不该长这样」，不是「网络抖了一下」。
///
/// 所以这里没有重试语义 —— 出现任何一条，调用方都该把这份副本丢掉重来，
/// 而不是继续喂帧（继续喂只会让分叉越来越大）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirrorError {
    /// 已收到 `REBUILD_REQUIRED`，本地副本不可信，除重新快照外一律拒收。
    Stale { lost_through: i64 },
    /// 服务端换了任：旧 term 的历史与新 term 的不是同一条线。
    TermChanged { prev: i64, got: i64 },
    /// 同一张表的快照没结束又来了个 begin。
    DuplicateBegin { table: String },
    /// `SNAPSHOT_END` 没有对应的 begin。
    EndWithoutBegin { table: String },
    /// 快照行不在任何进行中的快照里（多半是错序或串连接）。
    RowOutsideSnapshot { table: String, key: String },
    /// 快照行本身不对：动作、行值或 seq 与水位不符。
    BadSnapshotRow { table: String, key: String, reason: &'static str },
    /// 声明的行数与实收不符 —— 这张表不完整，留在镜像里就是脏读。
    RowCountMismatch { table: String, expected: usize, got: usize },
    /// 水位没往前走：重复投递或乱序。
    OutOfOrder { prev_through: i64, through: i64 },
    /// 批内某条的 seq 落在 `(last_through, through]` 之外。
    SeqOutOfBatch { seq: i64, last_through: i64, through: i64 },
    /// 增量行的动作与行值不匹配。
    BadRowShape { table: String, key: String, reason: &'static str },
}

impl fmt::Display for MirrorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MirrorError::Stale { lost_through } => {
                write!(f, "本地副本已落后（≤{lost_through} 有缺口），拒绝继续应用，须重新走快照")
            }
            MirrorError::TermChanged { prev, got } => {
                write!(f, "服务端 term 从 {prev} 变成 {got}，旧历史不可续用")
            }
            MirrorError::DuplicateBegin { table } => write!(f, "表 {table} 的快照重复 begin"),
            MirrorError::EndWithoutBegin { table } => write!(f, "表 {table} 收到 end 却没有 begin"),
            MirrorError::RowOutsideSnapshot { table, key } => {
                write!(f, "表 {table} 的行 {key:?} 不在进行中的快照里")
            }
            MirrorError::BadSnapshotRow { table, key, reason } => {
                write!(f, "表 {table} 的行 {key:?} 作为快照行不合规：{reason}")
            }
            MirrorError::RowCountMismatch { table, expected, got } => {
                write!(f, "表 {table} 声明 {expected} 行，实收 {got} 行，整表作废")
            }
            MirrorError::OutOfOrder { prev_through, through } => {
                write!(f, "水位没推进：已到 {prev_through}，本批却写着 {through}")
            }
            MirrorError::SeqOutOfBatch { seq, last_through, through } => write!(
                f,
                "批内 seq {seq} 落在 ({last_through}, {through}] 之外"
            ),
            MirrorError::BadRowShape { table, key, reason } => {
                write!(f, "表 {table} 的行 {key:?} 与动作不匹配：{reason}")
            }
        }
    }
}

impl std::error::Error for MirrorError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pubsub::Subscribe;
    use serde_json::json;

    fn change(seq: i64, table: &'static str, key: &str, op: Op, row: Option<i64>) -> RowChange {
        RowChange {
            term: 1,
            seq,
            table,
            key: key.to_string(),
            op,
            row: row.map(|units| json!({ "quantity": { "units": units } })),
        }
    }

    /// 一帧覆盖五种形态 + 两列，往返一趟够长。
    fn a_frame_sequence() -> Vec<Frame> {
        vec![
            Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
            Frame::SnapshotRow(change(7, "position", "A001:09018", Op::Upsert, Some(800))),
            Frame::SnapshotRow(change(7, "position", "A002:600000", Op::Upsert, Some(2000))),
            Frame::SnapshotEnd { table: "position", rows: 2 },
            Frame::Delta {
                through: 8,
                changes: vec![
                    change(8, "position", "A001:09018", Op::Delete, None),
                    change(8, "account_asset", "A001", Op::Upsert, Some(99279800)),
                ],
            },
            Frame::Delta { through: 9, changes: Vec::new() },
            Frame::RebuildRequired { lost_through: 12 },
        ]
    }

    #[test]
    fn a_round_trip_through_the_wire_returns_the_same_frame() {
        // 往返必须逐字节等价于原帧：编解码边界一旦「差不多就行」，两端就会各自解释。
        for frame in a_frame_sequence() {
            let line = encode_line(&frame).expect("编码不该失败");
            let wire = decode_line(&line).expect("解码不该失败");
            assert_eq!(wire.to_frame().expect("表名都登记过"), frame, "往返改变了帧：{line}");
        }
    }

    #[test]
    fn frames_use_snake_case_external_tags() {
        // 线上形状是协议的一部分：变了就是两端各读各的，所以钉死字面量。
        assert_eq!(
            encode_line(&Frame::Delta { through: 9, changes: Vec::new() }).unwrap(),
            r#"{"delta":{"through":9,"changes":[]}}"#
        );
        assert_eq!(
            encode_line(&Frame::RebuildRequired { lost_through: 2 }).unwrap(),
            r#"{"rebuild_required":{"lost_through":2}}"#
        );
        let begin = encode_line(&Frame::SnapshotBegin { table: "position", term: 1, seq: 7 }).unwrap();
        assert_eq!(begin, r#"{"snapshot_begin":{"table":"position","term":1,"seq":7}}"#);
    }

    #[test]
    fn one_frame_is_exactly_one_line_even_when_a_key_holds_a_newline() {
        // 复合键本身不会有换行，但任何字符串值都可能：转义必须由 serde 做完，
        // 否则一帧劈成两行，下一行读出来是垃圾。
        let nasty = "A001\n09018";
        let frame = Frame::SnapshotRow(change(7, "position", nasty, Op::Upsert, Some(1)));
        let line = encode_line(&frame).expect("带换行的键应被转义而不是拒绝");
        assert!(!line.contains('\n'), "编码结果里不许有裸换行：{line}");
        let wire = decode_line(&line).expect("解码不该失败");
        let WireFrame::SnapshotRow(decoded) = wire else { panic!("帧型变了") };
        assert_eq!(decoded.key, nasty, "换行必须原样回来");

        let block = encode_block(&a_frame_sequence()).unwrap();
        assert!(block.ends_with('\n'), "块结尾该有换行");
        assert_eq!(block.lines().count(), 7, "一帧一行");
    }

    #[test]
    fn unknown_fields_survive_but_missing_ones_do_not() {
        // 前向兼容：旧消费者读到新字段不该整帧失败。
        let with_extra = r#"{"delta":{"through":3,"changes":[],"server_build":"next"}}"#;
        assert_eq!(
            decode_line(with_extra).unwrap(),
            WireFrame::Delta { through: 3, changes: Vec::new() }
        );
        // 反过来，缺字段必须报错：静默按默认值补出一个 through=0 会伪装成合法水位。
        let missing = r#"{"delta":{"changes":[]}}"#;
        assert!(matches!(decode_line(missing), Err(WireError::BadJson(_))));
    }

    #[test]
    fn blank_and_oversized_lines_are_refused_before_parsing() {
        assert_eq!(decode_line(""), Err(WireError::Empty));
        assert_eq!(decode_line("\n"), Err(WireError::Empty));
        assert_eq!(decode_line("   "), Err(WireError::Empty));
        let huge = "x".repeat(MAX_LINE_BYTES + 1);
        assert_eq!(
            decode_line(&huge),
            Err(WireError::TooLong { bytes: MAX_LINE_BYTES + 1, limit: MAX_LINE_BYTES })
        );
    }

    #[test]
    fn an_unregistered_table_name_is_refused_at_reassembly() {
        // 归位这一步顺带守住协议身份：wire 上写 ghost_table 不许变出一个 Frame。
        let line = r#"{"snapshot_begin":{"table":"ghost_table","term":1,"seq":7}}"#;
        assert_eq!(
            decode_line(line).unwrap().to_frame(),
            Err(WireError::UnknownTable("ghost_table".to_string()))
        );
    }

    #[test]
    fn a_mirror_rebuilds_rows_and_watermark_from_frames() {
        let mut mirror = Mirror::new();
        mirror.apply_all(&decoded(a_frame_sequence())).expect("重建不该失败");
        assert_eq!(mirror.tables(), vec!["account_asset", "position"]);
        assert_eq!(
            mirror.get("account_asset", "A001").expect("增量写进来的行该在"),
            &json!({ "quantity": { "units": 99279800 } })
        );
        assert_eq!(
            mirror.get("position", "A002:600000").expect("快照行该在"),
            &json!({ "quantity": { "units": 2000 } })
        );
        assert_eq!(mirror.get("position", "A001:09018"), None, "Delete 该真的删掉本地行");
        assert_eq!(mirror.last_through(), 9, "空批也要推进水位");
        assert_eq!(mirror.rows(), 2);
        assert_eq!(mirror.unfinished(), Vec::<&str>::new(), "快照都该收尾");
        // 这串帧以 rebuild_required 结尾，落后标记必须还在（它是客户端决定重连的依据）。
        assert!(mirror.is_stale());
    }

    fn decoded(frames: Vec<Frame>) -> Vec<WireFrame> {
        frames
            .iter()
            .map(|frame| decode_line(&encode_line(frame).unwrap()).unwrap())
            .collect()
    }

    #[test]
    fn rebuild_required_blocks_further_deltas_until_a_new_snapshot() {
        let mut mirror = Mirror::new();
        mirror.apply_all(&decoded(a_frame_sequence())).unwrap();
        assert!(mirror.is_stale(), "收到 rebuild_required 就该明说本地不可信");

        // 落后之后继续喂增量 = 在一份有洞的副本上叠改动，必须拒。
        let late = decoded(vec![Frame::Delta {
            through: 13,
            changes: vec![change(13, "position", "A003:600000", Op::Upsert, Some(1))],
        }]);
        assert_eq!(
            mirror.apply_all(&late),
            Err(MirrorError::Stale { lost_through: 12 }),
            "缺口的位置要报出来，否则调用方无从判断丢了什么"
        );
        assert_eq!(mirror.get("position", "A003:600000"), None, "拒收就不该留下半条");
        assert_eq!(mirror.last_through(), 9, "游标不能被拒掉的帧推进");

        // 重新走快照是唯一的出路，而它必须能把 stale 清掉。
        let fresh = decoded(vec![
            Frame::SnapshotBegin { table: "position", term: 1, seq: 13 },
            Frame::SnapshotRow(change(13, "position", "A003:600000", Op::Upsert, Some(1))),
            Frame::SnapshotEnd { table: "position", rows: 1 },
            Frame::Delta { through: 14, changes: vec![change(14, "position", "A003:600000", Op::Delete, None)] },
        ]);
        mirror.apply_all(&fresh).expect("重新快照该能续上");
        assert!(!mirror.is_stale());
        assert_eq!(mirror.get("position", "A003:600000"), None);
        assert_eq!(mirror.last_through(), 14);
    }

    #[test]
    fn the_mirror_stops_on_protocol_corruption_instead_of_guessing() {
        // 每一条都单独验一次「停」，因为它们在真实故障里各自对应一种网络/版本事故。
        let cases: Vec<(Vec<Frame>, MirrorError)> = vec![
            (
                vec![
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                ],
                MirrorError::DuplicateBegin { table: "position".to_string() },
            ),
            (
                vec![Frame::SnapshotEnd { table: "position", rows: 0 }],
                MirrorError::EndWithoutBegin { table: "position".to_string() },
            ),
            (
                // 快照行 seq 掉到水位之前：多半是错批次串了进来。
                vec![
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                    Frame::SnapshotRow(change(6, "position", "A001:09018", Op::Upsert, Some(1))),
                ],
                MirrorError::BadSnapshotRow {
                    table: "position".to_string(),
                    key: "A001:09018".to_string(),
                    reason: "seq 不等于快照水位",
                },
            ),
            (
                vec![
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                    Frame::SnapshotRow(change(7, "position", "A001:09018", Op::Delete, None)),
                ],
                MirrorError::BadSnapshotRow {
                    table: "position".to_string(),
                    key: "A001:09018".to_string(),
                    reason: "动作不是 upsert",
                },
            ),
            (
                vec![
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                    Frame::SnapshotRow(change(7, "position", "A001:09018", Op::Upsert, Some(1))),
                    Frame::SnapshotEnd { table: "position", rows: 2 },
                ],
                MirrorError::RowCountMismatch { table: "position".to_string(), expected: 2, got: 1 },
            ),
            (
                // 换任：旧 term 的历史与新 term 不是同一条线，续用会拼出一个从没存在过的状态。
                // 另一张表先正常快照（term 1），再用 term 2 开第二张表。
                vec![
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                    Frame::SnapshotEnd { table: "position", rows: 0 },
                    Frame::SnapshotBegin { table: "account_asset", term: 2, seq: 8 },
                ],
                MirrorError::TermChanged { prev: 1, got: 2 },
            ),
            (
                vec![
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                    Frame::SnapshotRow(change(7, "position", "A001:09018", Op::Upsert, Some(1))),
                    Frame::SnapshotEnd { table: "position", rows: 1 },
                    Frame::Delta { through: 7, changes: Vec::new() },
                ],
                MirrorError::OutOfOrder { prev_through: 7, through: 7 },
            ),
            (
                vec![
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                    Frame::SnapshotRow(change(7, "position", "A001:09018", Op::Upsert, Some(1))),
                    Frame::SnapshotEnd { table: "position", rows: 1 },
                    // 批内 seq 越过 through：这种批一出现，两端对「哪一组算已确认」就没有共识了。
                    Frame::Delta {
                        through: 9,
                        changes: vec![change(11, "position", "A001:09018", Op::Upsert, Some(2))],
                    },
                ],
                MirrorError::SeqOutOfBatch { seq: 11, last_through: 7, through: 9 },
            ),
            (
                vec![
                    Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
                    Frame::SnapshotRow(change(7, "position", "A001:09018", Op::Upsert, Some(1))),
                    Frame::SnapshotEnd { table: "position", rows: 1 },
                    Frame::Delta {
                        through: 8,
                        changes: vec![change(8, "position", "A001:09018", Op::Delete, Some(3))],
                    },
                ],
                MirrorError::BadRowShape {
                    table: "position".to_string(),
                    key: "A001:09018".to_string(),
                    reason: "delete 带着行值",
                },
            ),
        ];

        for (frames, expected) in cases {
            let mut mirror = Mirror::new();
            let err = mirror
                .apply_all(&decoded(frames))
                .expect_err("畸形帧必须报错，不许静默吸收");
            assert_eq!(err, expected, "报错不对味：{err}");
        }
    }

    #[test]
    fn a_rejected_batch_leaves_not_even_part_of_it_applied() {
        // 整批先验后写：第二条畸形时第一条也不许落库，否则水位与行集从此对不上。
        let mut mirror = Mirror::new();
        let snapshot = decoded(vec![
            Frame::SnapshotBegin { table: "position", term: 1, seq: 7 },
            Frame::SnapshotRow(change(7, "position", "A001:09018", Op::Upsert, Some(1))),
            Frame::SnapshotEnd { table: "position", rows: 1 },
        ]);
        mirror.apply_all(&snapshot).expect("先建立一个干净的快照");
        let before = mirror.get("position", "A001:09018").expect("快照行").clone();

        let bad = decoded(vec![Frame::Delta {
            through: 8,
            changes: vec![
                change(8, "position", "A001:09018", Op::Upsert, Some(999)),
                change(8, "position", "A002:600000", Op::Upsert, None),
            ],
        }]);
        assert!(matches!(mirror.apply_all(&bad), Err(MirrorError::BadRowShape { .. })));
        assert_eq!(mirror.get("position", "A001:09018").expect("还是原行"), &before, "整批要么全落要么全不落");
        assert_eq!(mirror.get("position", "A002:600000"), None);
        assert_eq!(mirror.last_through(), 7, "水位没推进才对");
    }

    #[test]
    fn a_truncated_snapshot_is_visible_instead_of_looking_complete() {
        // 连接被掐在快照中间：镜像必须能说「我还欠一张表」，不能报出一张少了行的表当完整。
        let mut mirror = Mirror::new();
        mirror
            .apply_all(&decoded(vec![
                Frame::SnapshotBegin { table: "position", term: 1, seq: 0 },
                Frame::SnapshotRow(change(0, "position", "A001:09018", Op::Upsert, Some(1))),
            ]))
            .unwrap();
        assert_eq!(mirror.unfinished(), vec!["position"]);
        assert_eq!(mirror.summary(), "1 表 / 1 行，水位 0，已应用 1 行，term 1，1 表快照未完");
    }

    #[test]
    fn an_empty_delete_is_idempotent_rather_than_an_error() {
        // 重新 attach 之后本地可能本就没有这行；把它报成错误会让重连永远走不完。
        let mut mirror = Mirror::new();
        mirror
            .apply_all(&decoded(vec![Frame::Delta {
                through: 4,
                changes: vec![change(4, "position", "ghost:key", Op::Delete, None)],
            }]))
            .expect("删不存在的行该被幂等接受");
        assert_eq!(mirror.rows(), 0);
        assert_eq!(mirror.last_through(), 4);
    }

    #[test]
    fn a_rejection_travels_on_the_wire_as_a_readable_reason() {
        // 拒订原因必须是客户端读得到的内容，而不是一个作不得数的 EOF。
        let line = reject_line("表 orders 的主键首列不是账户列");
        assert_eq!(as_reject(&line).as_deref(), Some("表 orders 的主键首列不是账户列"));
        // 转发的一帧不该被误读成拒订，反之也一样。
        let frame = encode_line(&Frame::Delta { through: 3, changes: Vec::new() }).unwrap();
        assert_eq!(as_reject(&frame), None);
        assert!(matches!(decode_line(&line), Err(WireError::BadJson(_))), "拒订行不是帧");
        // 换行与引号都得被转义掉，不然一帧劈成两行。
        assert_eq!(as_reject(&reject_line("他说：\"不\"\n第二行")).as_deref(), Some("他说：\"不\"\n第二行"));
    }

    #[test]
    fn a_request_coming_off_the_wire_keeps_its_rejections() {
        // 入站请求直接解成 Subscribe：校验必须在 attach 之前跑完，网络侧不能有什么特权。
        let spec: Subscribe = serde_json::from_str(r#"{"tables":["position"]}"#).expect("省略字段该走默认");
        assert_eq!(spec.ops, vec![Op::Upsert, Op::Delete], "省略 ops = 两种都要");
        assert!(spec.columns.is_all() && matches!(spec.filter, crate::pubsub::Filter::None));

        // 拼错字段名要当场报错：静悄悄按默认跑，等于把一个过滤条件丢掉。
        let typo = r#"{"tabels":["position"]}"#;
        let err = serde_json::from_str::<Subscribe>(typo).expect_err("未知字段必须拒");
        assert!(err.to_string().contains("tabels"), "报错要点名写错的字段：{err}");
    }
}
