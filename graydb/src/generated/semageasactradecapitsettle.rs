//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_asac_trade_capit_settle`（codegen：字段与列名源自 DDL，`SemageAsacTradeCapitSettleColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageAsacTradeCapitSettle {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub co_no: i32,
    pub buy_amt: Decimal,
    pub sell_amt: Decimal,
}

/// `tb_semage_asac_trade_capit_settle` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageAsacTradeCapitSettleColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    PdNo,
    AsacNo,
    SettleCrncyType,
    ExchCrncyType,
    CoNo,
    BuyAmt,
    SellAmt,
}

impl ColumnName for SemageAsacTradeCapitSettleColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::PdNo, Self::AsacNo, Self::SettleCrncyType, Self::ExchCrncyType, Self::CoNo, Self::BuyAmt, Self::SellAmt];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::CoNo => "co_no",
            Self::BuyAmt => "buy_amt",
            Self::SellAmt => "sell_amt",
        }
    }
}

impl DataTable for SemageAsacTradeCapitSettle {
    const ID: &'static str = "tb_semage_asac_trade_capit_settle";
    type Column = SemageAsacTradeCapitSettleColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageAsacTradeCapitSettleColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageAsacTradeCapitSettle {}
