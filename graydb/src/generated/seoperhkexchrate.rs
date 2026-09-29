//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_hk_exch_rate`（codegen：字段与列名源自 DDL，`SeoperHkExchRateColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperHkExchRate {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub exch_no: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub buy_ref_rate: Decimal,
    pub sell_ref_rate: Decimal,
    pub settle_buy_rate: Decimal,
    pub settle_sell_rate: Decimal,
    pub pboc_rate: Decimal,
}

/// `tb_seoper_hk_exch_rate` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperHkExchRateColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    ExchNo,
    SettleCrncyType,
    ExchCrncyType,
    BuyRefRate,
    SellRefRate,
    SettleBuyRate,
    SettleSellRate,
    PbocRate,
}

impl ColumnName for SeoperHkExchRateColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::ExchNo, Self::SettleCrncyType, Self::ExchCrncyType, Self::BuyRefRate, Self::SellRefRate, Self::SettleBuyRate, Self::SettleSellRate, Self::PbocRate];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::ExchNo => "exch_no",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::BuyRefRate => "buy_ref_rate",
            Self::SellRefRate => "sell_ref_rate",
            Self::SettleBuyRate => "settle_buy_rate",
            Self::SettleSellRate => "settle_sell_rate",
            Self::PbocRate => "pboc_rate",
        }
    }
}

impl DataTable for SeoperHkExchRate {
    const ID: &'static str = "tb_seoper_hk_exch_rate";
    type Column = SeoperHkExchRateColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperHkExchRateColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperHkExchRate {}
