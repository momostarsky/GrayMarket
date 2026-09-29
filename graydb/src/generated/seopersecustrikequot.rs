//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_secu_strike_quot`（codegen：字段与列名源自 DDL，`SeoperSecuStrikeQuotColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSecuStrikeQuot {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub order_dir: i32,
    pub strike_price: Decimal,
    pub strike_qty: Decimal,
    pub strike_amt: Decimal,
    pub time_stamp: i64,
    pub remark_info: String,
}

/// `tb_seoper_secu_strike_quot` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSecuStrikeQuotColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    SecuCode,
    OrderDir,
    StrikePrice,
    StrikeQty,
    StrikeAmt,
    TimeStamp,
    RemarkInfo,
}

impl ColumnName for SeoperSecuStrikeQuotColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::SecuCode, Self::OrderDir, Self::StrikePrice, Self::StrikeQty, Self::StrikeAmt, Self::TimeStamp, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::OrderDir => "order_dir",
            Self::StrikePrice => "strike_price",
            Self::StrikeQty => "strike_qty",
            Self::StrikeAmt => "strike_amt",
            Self::TimeStamp => "time_stamp",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperSecuStrikeQuot {
    const ID: &'static str = "tb_seoper_secu_strike_quot";
    type Column = SeoperSecuStrikeQuotColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSecuStrikeQuotColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSecuStrikeQuot {}
