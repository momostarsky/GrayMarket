//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdmage_pd_unit_settle_posi_unsettle`（codegen：字段与列名源自 DDL，`PdmagePdUnitSettlePosiUnsettleColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdmagePdUnitSettlePosiUnsettle {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub order_date: i32,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub co_no: i32,
    pub secu_type: i32,
    pub invest_type: i32,
    pub asset_type: i32,
    pub cost_amt: Decimal,
    pub intrst_cost_amt: Decimal,
    pub buy_amt: Decimal,
    pub sell_amt: Decimal,
    pub buy_qty: Decimal,
    pub sell_qty: Decimal,
    pub settle_date: i32,
    pub settle_time: i32,
}

/// `tb_pdmage_pd_unit_settle_posi_unsettle` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdmagePdUnitSettlePosiUnsettleColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    OrderDate,
    PdNo,
    PdUnitNo,
    AsacNo,
    ExchNo,
    SecuCode,
    SecuName,
    CoNo,
    SecuType,
    InvestType,
    AssetType,
    CostAmt,
    IntrstCostAmt,
    BuyAmt,
    SellAmt,
    BuyQty,
    SellQty,
    SettleDate,
    SettleTime,
}

impl ColumnName for PdmagePdUnitSettlePosiUnsettleColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::OrderDate, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::CoNo, Self::SecuType, Self::InvestType, Self::AssetType, Self::CostAmt, Self::IntrstCostAmt, Self::BuyAmt, Self::SellAmt, Self::BuyQty, Self::SellQty, Self::SettleDate, Self::SettleTime];
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
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::CoNo => "co_no",
            Self::SecuType => "secu_type",
            Self::InvestType => "invest_type",
            Self::AssetType => "asset_type",
            Self::CostAmt => "cost_amt",
            Self::IntrstCostAmt => "intrst_cost_amt",
            Self::BuyAmt => "buy_amt",
            Self::SellAmt => "sell_amt",
            Self::BuyQty => "buy_qty",
            Self::SellQty => "sell_qty",
            Self::SettleDate => "settle_date",
            Self::SettleTime => "settle_time",
        }
    }
}

impl DataTable for PdmagePdUnitSettlePosiUnsettle {
    const ID: &'static str = "tb_pdmage_pd_unit_settle_posi_unsettle";
    type Column = PdmagePdUnitSettlePosiUnsettleColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdmagePdUnitSettlePosiUnsettleColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdmagePdUnitSettlePosiUnsettle {}
