//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seotcsecu_asac_posi_adjust_jour`（codegen：字段与列名源自 DDL，`SeotcsecuAsacPosiAdjustJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeotcsecuAsacPosiAdjustJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub adjust_jour_no: i64,
    pub jour_flag: i32,
    pub adjust_qty: Decimal,
    pub adjust_amt: Decimal,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub secu_type: i32,
    pub invest_type: i32,
    pub asset_type: i32,
    pub begin_qty: Decimal,
    pub curr_qty: Decimal,
    pub cost_price: Decimal,
    pub cost_amt: Decimal,
    pub unit_nav: Decimal,
    pub posi_market_value: Decimal,
    pub open_date: i32,
    pub remark_info: String,
}

/// `tb_seotcsecu_asac_posi_adjust_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeotcsecuAsacPosiAdjustJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    AdjustJourNo,
    JourFlag,
    AdjustQty,
    AdjustAmt,
    CoNo,
    PdNo,
    AsacNo,
    ExchNo,
    SecuCode,
    SecuName,
    SecuType,
    InvestType,
    AssetType,
    BeginQty,
    CurrQty,
    CostPrice,
    CostAmt,
    UnitNav,
    PosiMarketValue,
    OpenDate,
    RemarkInfo,
}

impl ColumnName for SeotcsecuAsacPosiAdjustJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::AdjustJourNo, Self::JourFlag, Self::AdjustQty, Self::AdjustAmt, Self::CoNo, Self::PdNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::SecuType, Self::InvestType, Self::AssetType, Self::BeginQty, Self::CurrQty, Self::CostPrice, Self::CostAmt, Self::UnitNav, Self::PosiMarketValue, Self::OpenDate, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::AdjustJourNo => "adjust_jour_no",
            Self::JourFlag => "jour_flag",
            Self::AdjustQty => "adjust_qty",
            Self::AdjustAmt => "adjust_amt",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::SecuType => "secu_type",
            Self::InvestType => "invest_type",
            Self::AssetType => "asset_type",
            Self::BeginQty => "begin_qty",
            Self::CurrQty => "curr_qty",
            Self::CostPrice => "cost_price",
            Self::CostAmt => "cost_amt",
            Self::UnitNav => "unit_nav",
            Self::PosiMarketValue => "posi_market_value",
            Self::OpenDate => "open_date",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeotcsecuAsacPosiAdjustJour {
    const ID: &'static str = "tb_seotcsecu_asac_posi_adjust_jour";
    type Column = SeotcsecuAsacPosiAdjustJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeotcsecuAsacPosiAdjustJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeotcsecuAsacPosiAdjustJour {}
