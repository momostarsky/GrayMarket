//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_asac_capit_unsettle`（codegen：字段与列名源自 DDL，`SemageAsacCapitUnsettleColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageAsacCapitUnsettle {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub order_date: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub co_no: i32,
    pub buy_amt: Decimal,
    pub sell_amt: Decimal,
    pub settle_date: i32,
    pub settle_time: i32,
    pub busi_type: i32,
}

/// `tb_semage_asac_capit_unsettle` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageAsacCapitUnsettleColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    OrderDate,
    PdNo,
    AsacNo,
    SettleCrncyType,
    ExchCrncyType,
    CoNo,
    BuyAmt,
    SellAmt,
    SettleDate,
    SettleTime,
    BusiType,
}

impl ColumnName for SemageAsacCapitUnsettleColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::OrderDate, Self::PdNo, Self::AsacNo, Self::SettleCrncyType, Self::ExchCrncyType, Self::CoNo, Self::BuyAmt, Self::SellAmt, Self::SettleDate, Self::SettleTime, Self::BusiType];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::OrderDate => "order_date",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::CoNo => "co_no",
            Self::BuyAmt => "buy_amt",
            Self::SellAmt => "sell_amt",
            Self::SettleDate => "settle_date",
            Self::SettleTime => "settle_time",
            Self::BusiType => "busi_type",
        }
    }
}

impl DataTable for SemageAsacCapitUnsettle {
    const ID: &'static str = "tb_semage_asac_capit_unsettle";
    type Column = SemageAsacCapitUnsettleColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageAsacCapitUnsettleColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageAsacCapitUnsettle {}
