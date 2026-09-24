use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use crate::domain::{Order, Trade};
/// 订单/成交追加写 JSONL —— 将来直接升级为带 term/seq 的二进制段文件。
pub struct Journal {
    writer: BufWriter<std::fs::File>,
}

impl Journal {
    pub fn open(path: &std::path::Path) ->  std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { writer: BufWriter::new(file) })
    }
    pub fn append_order(&mut self, order: &Order) -> std::io::Result<()> {
        serde_json::to_writer(&mut self.writer, order)?;
        self.writer.write_all(b"\n")?;
        Ok(())
    }
    pub fn append_trade(&mut self, trade: &Trade) -> std::io::Result<()> {
        serde_json::to_writer(&mut self.writer, trade)?;
        self.writer.write_all(b"\n")?;
        Ok(())
    }
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}