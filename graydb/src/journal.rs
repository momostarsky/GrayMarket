//! journal：`(term, seq)` 定序的实体流水与它的回放读取（阶段 2.1 / 2.4）。
//!
//! 一行一条 [`Record`]：信封给任期与序号，`entry` 给整行内容（记实体，不记 delta）。
//! 落盘以 [`Journal::commit`] 收尾 —— 它 `flush` 后再 `sync_data`，只有这里返回 `Ok`，
//! 内存才允许跟着改：阶段 1.5 那句「先写日志再改内存」的「写」到这一层才算真的写完。
//!
//! 恢复端 [`Journal::read`] 的裁决（宁停不脏）：
//! - 末条没有换行 = 崩溃发生在写入中途 → **丢弃并停止**，交由上层显式处置；
//! - 中间行解析失败 / `seq` 不稠密 / 跨 `term` / 行内序号与信封不符 → 直接 `Err`，
//!   绝不做「猜测式补齐」——猜出来的余额比停机更糟。

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::{Order, Trade};
use crate::tables::{DataTable, spec_of};

/// 一条流水承载的行。
///
/// 序列化成外部标签形式：`{"orders": {…整行…}}`，键名由 `#[serde(rename_all = "snake_case")]`
/// 从变体名导出，而 [`Entry::table_id`] 取的是 `DataTable::ID`（与 `Spec::id` 在加载期双向锚定）。
/// 两者必须相等，由 `table_tag_matches_registry_identity` 测试钉住 —— WAL 里的表名
/// 因此不可能与注册中心、订阅主题、COPY 目标写岔。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Entry {
    Orders(Order),
    Trades(Trade),
}

impl Entry {
    /// 承载表的身份（`Spec::id` 同源，不是新写的字符串）。
    #[must_use]
    pub fn table_id(&self) -> &'static str {
        match self {
            Entry::Orders(_) => Order::ID,
            Entry::Trades(_) => Trade::ID,
        }
    }

    /// 行键：回放靠它判「首见即下单」，也供上层做幂等核对。
    #[must_use]
    pub fn row_key(&self) -> &str {
        match self {
            Entry::Orders(order) => &order.order_id,
            Entry::Trades(trade) => &trade.trade_id,
        }
    }

    /// 行自带的序号 —— 必须等于信封的 `seq`，否则说明写入端拼错了组。
    #[must_use]
    pub fn row_seq(&self) -> i64 {
        match self {
            Entry::Orders(order) => order.seq,
            Entry::Trades(trade) => trade.seq,
        }
    }

    #[must_use]
    pub fn into_order(self) -> Option<Order> {
        match self {
            Entry::Orders(order) => Some(order),
            Entry::Trades(_) => None,
        }
    }

    #[must_use]
    pub fn into_trade(self) -> Option<Trade> {
        match self {
            Entry::Trades(trade) => Some(trade),
            Entry::Orders(_) => None,
        }
    }
}

/// 落盘记录：`(term, seq)` 定序，`entry` 定内容。
///
/// 刻意不 derive `PartialEq`：行类型（`Order`/`Trade`）本身没有结构相等，
/// 对比两份日志的测试都按字段逐项比，不假装存在一种「整条相等」的语义。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    /// 任期。阶段 5.3 才递增，模拟阶段恒为 0；跨 term 的日志拒绝回放。
    pub term: i64,
    /// 事务组序号：一组（trade + order）共用同一个 seq。
    pub seq: i64,
    pub entry: Entry,
}

/// [`Journal::read`] 的结果。
#[derive(Debug)]
pub struct ReplayLog {
    /// 完整且序号稠密的记录。
    pub records: Vec<Record>,
    /// 全批任期。
    pub term: i64,
    /// 末条不完整的原始尾串。`Some` = 崩溃吞掉了最后一组，恢复出来的状态**少一笔**，
    /// 上层必须显式处置（停服人工核对 / 向从库补齐），不能当成正常启动继续撮合。
    pub dropped_tail: Option<String>,
}

impl ReplayLog {
    /// 已落盘的最后一个序号（空日志为 0，与新建内核的起点一致）。
    #[must_use]
    pub fn last_seq(&self) -> i64 {
        self.records.last().map_or(0, |record| record.seq)
    }
}

/// 追加写句柄。刻意不暴露公开的 `append` —— 写入必须经 [`GroupWriter`]，
/// 这样「带信封（term/seq）+ 收尾落盘」是结构上唯一可行的写法。
pub struct Journal {
    writer: BufWriter<File>,
    path: PathBuf,
}

impl Journal {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
            path: path.to_path_buf(),
        })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 一组的收尾：推到 OS 再 `sync_data` 落盘。
    ///
    /// 用 `sync_data` 而非 `sync_all`：只需要内容持久，不为 mtime 多付一次元数据写。
    pub(crate) fn commit(&mut self) -> std::io::Result<()> {
        self.writer.flush()?;
        self.writer.get_ref().sync_data()?;
        Ok(())
    }

    /// 整行一次 `write_all`：崩溃只会留下「解析不了的半行」，回放据此判定截断。
    fn append(&mut self, record: &Record) -> std::io::Result<()> {
        let mut line = serde_json::to_vec(record)
            .map_err(|err| std::io::Error::other(format!("journal 序列化失败: {err}")))?;
        line.push(b'\n');
        self.writer.write_all(&line)
    }

    /// 2.4 读回全部可采信的记录（校验见模块头）。
    pub fn read(path: &Path) -> anyhow::Result<ReplayLog> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(ReplayLog {
                    records: Vec::new(),
                    term: 0,
                    dropped_tail: None,
                });
            }
            Err(err) => return Err(anyhow::anyhow!("journal 打不开 {}: {err}", path.display())),
        };
        let mut reader = BufReader::new(file);
        let mut records: Vec<Record> = Vec::new();
        let mut term: Option<i64> = None;
        let mut line = String::new();
        let mut lineno = 0_usize;

        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                break; // 干净收尾
            }
            lineno += 1;
            // 只认换行结尾：没有换行的末条 = 崩溃写在半路（2.4 丢弃并停止）。
            let complete = line.ends_with('\n');
            let payload = line.trim_end_matches(['\n', '\r']);
            if payload.trim().is_empty() {
                continue;
            }
            if !complete {
                return Ok(ReplayLog {
                    records,
                    term: term.unwrap_or(0),
                    dropped_tail: Some(payload.to_string()),
                });
            }
            let record: Record = serde_json::from_str(payload).map_err(|err| {
                anyhow::anyhow!(
                    "journal 第 {lineno} 行损坏（内部记录，不做猜测式补齐）: {err}"
                )
            })?;

            let table_id = record.entry.table_id();
            if spec_of(table_id).is_none() {
                return Err(anyhow::anyhow!(
                    "journal 第 {lineno} 行指向未登记的表 {table_id:?} —— 表身份与注册中心分叉"
                ));
            }
            if record.entry.row_seq() != record.seq {
                return Err(anyhow::anyhow!(
                    "journal 第 {lineno} 行序号与信封不符（信封 {}，行内 {}）",
                    record.seq,
                    record.entry.row_seq()
                ));
            }
            match term {
                None => term = Some(record.term),
                Some(first) if first != record.term => {
                    return Err(anyhow::anyhow!(
                        "journal 第 {lineno} 行跨任期（{first} → {}）—— 日切/接管未处理，拒绝混放",
                        record.term
                    ));
                }
                Some(_) => {}
            }
            // 稠密性按「事务组」而不是按「记录」判定：同组的多条记录共用一个 seq（1.7），
            // 所以合法序列是 1,1,2,3,3,… —— 只允许持平或 +1，跳号就是日志有洞。
            if let Some(last) = records.last()
                && record.seq != last.seq
                && record.seq != last.seq + 1
            {
                return Err(anyhow::anyhow!(
                    "journal 序号不稠密: 上一组 {}，本条 {} —— 日志有洞或乱序，拒绝恢复",
                    last.seq,
                    record.seq
                ));
            }
            records.push(record);
        }

        Ok(ReplayLog {
            records,
            term: term.unwrap_or(0),
            dropped_tail: None,
        })
    }
}

/// 一个事务组的写入句柄：`term`/`seq` 由内核注入，因此「带信封的写入」是唯一入口。
pub struct GroupWriter<'a> {
    journal: &'a mut Journal,
    term: i64,
    seq: i64,
}

impl GroupWriter<'_> {
    /// 由内核在唯一落盘点构造（字段私有，信封三件套只能从 `Engine::commit_journal` 进来）。
    pub(crate) fn new(journal: &mut Journal, term: i64, seq: i64) -> GroupWriter<'_> {
        GroupWriter { journal, term, seq }
    }

    pub fn order(&mut self, order: &Order) -> std::io::Result<()> {
        self.journal.append(&Record {
            term: self.term,
            seq: self.seq,
            entry: Entry::Orders(order.clone()),
        })
    }

    pub fn trade(&mut self, trade: &Trade) -> std::io::Result<()> {
        self.journal.append(&Record {
            term: self.term,
            seq: self.seq,
            entry: Entry::Trades(trade.clone()),
        })
    }
}

#[cfg(test)]
mod tests {
    use account::amount::{Money, Price, Quantity};

    use super::*;

    /// `Entry` 序列化后的外层键（即 WAL 里落下来的表名）。
    fn entry_keys(entry: &Entry) -> Vec<String> {
        serde_json::to_value(entry)
            .expect("序列化 entry")
            .as_object()
            .expect("entry 应序列化为单键对象")
            .keys()
            .cloned()
            .collect()
    }

    /// 表名（serde 从变体名导出的外部标签）必须等于注册中心的表身份 —— 两处一旦写岔，
    /// 回放会按错的表分流，归档也会进错表。
    #[test]
    fn table_tag_matches_registry_identity() {
        assert_eq!(
            entry_keys(&Entry::Orders(sample_order())),
            vec![Order::ID.to_string()],
            "`Entry::Orders` 的 serde 标签必须等于 `Spec::id`"
        );
        assert_eq!(entry_keys(&Entry::Trades(sample_trade())), vec![Trade::ID.to_string()]);
        assert!(spec_of(Order::ID).is_some() && spec_of(Trade::ID).is_some());
    }

    /// 往返后信封三件套不丢，且行内 seq 与信封一致（读侧的采信前提）。
    #[test]
    fn record_round_trips_with_envelope() {
        let record = Record {
            term: 0,
            seq: 7,
            entry: Entry::Orders(sample_order()),
        };
        let text = serde_json::to_string(&record).expect("编码");
        let back: Record = serde_json::from_str(&text).expect("解码");
        assert_eq!(back.seq, 7);
        assert_eq!(back.term, 0);
        assert_eq!(back.entry.table_id(), "orders");
        assert_eq!(back.entry.row_seq(), back.seq);
        assert_eq!(back.entry.row_key(), "ORD-ROUNDTRIP");
    }

    fn sample_order() -> Order {
        Order {
            order_id: "ORD-ROUNDTRIP".to_string(),
            account_id: "ACC-001".to_string(),
            symbol: "600000".to_string(),
            side: crate::domain::Side::Buy,
            price: Price::from_units(60_600),
            quantity: Quantity::from_units(100),
            filled_qty: Quantity::from_units(0),
            status: crate::domain::OrderStatus::New,
            created_at: "2026-09-29T09:30:00".to_string(),
            seq: 7,
        }
    }

    fn sample_trade() -> Trade {
        Trade {
            trade_id: "TRD-ROUNDTRIP".to_string(),
            order_id: "ORD-ROUNDTRIP".to_string(),
            account_id: "ACC-001".to_string(),
            symbol: "600000".to_string(),
            side: crate::domain::Side::Buy,
            price: Price::from_units(60_600),
            quantity: Quantity::from_units(100),
            amount: Money::from_units(60_600),
            seq: 7,
        }
    }
}
