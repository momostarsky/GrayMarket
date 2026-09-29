//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_asac_trade_posi_settle`（codegen：字段与列名源自 DDL，`SemageAsacTradePosiSettleColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageAsacTradePosiSettle {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub co_no: i32,
    pub secu_type: i32,
    pub invest_type: i32,
    pub asset_type: i32,
    pub buy_amt: Decimal,
    pub sell_amt: Decimal,
    pub buy_qty: Decimal,
    pub sell_qty: Decimal,
}

/// `tb_semage_asac_trade_posi_settle` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageAsacTradePosiSettleColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    PdNo,
    AsacNo,
    ExchNo,
    SecuCode,
    SecuName,
    CoNo,
    SecuType,
    InvestType,
    AssetType,
    BuyAmt,
    SellAmt,
    BuyQty,
    SellQty,
}

impl ColumnName for SemageAsacTradePosiSettleColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::PdNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::CoNo, Self::SecuType, Self::InvestType, Self::AssetType, Self::BuyAmt, Self::SellAmt, Self::BuyQty, Self::SellQty];
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
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::CoNo => "co_no",
            Self::SecuType => "secu_type",
            Self::InvestType => "invest_type",
            Self::AssetType => "asset_type",
            Self::BuyAmt => "buy_amt",
            Self::SellAmt => "sell_amt",
            Self::BuyQty => "buy_qty",
            Self::SellQty => "sell_qty",
        }
    }
}

impl DataTable for SemageAsacTradePosiSettle {
    const ID: &'static str = "tb_semage_asac_trade_posi_settle";
    type Column = SemageAsacTradePosiSettleColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageAsacTradePosiSettleColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageAsacTradePosiSettle {}
