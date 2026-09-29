//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_swap_code_info`（codegen：字段与列名源自 DDL，`SeoperSwapCodeInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSwapCodeInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub exch_no: i32,
    pub futu_code: String,
    pub futu_name: String,
    pub pinyin_short: String,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub futu_type: i32,
    pub asset_type: i32,
    pub type_unit: i32,
    pub report_unit: i32,
    pub min_unit: i32,
    pub max_qty: Decimal,
    pub min_qty: Decimal,
    pub param_detail_flag: i32,
    pub price_up: Decimal,
    pub price_down: Decimal,
    pub step_price: Decimal,
    pub fair_price: Decimal,
    pub remark_info: String,
    pub t0_flag: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub secu_type: i32,
}

/// `tb_seoper_swap_code_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSwapCodeInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    ExchNo,
    FutuCode,
    FutuName,
    PinyinShort,
    SettleCrncyType,
    ExchCrncyType,
    FutuType,
    AssetType,
    TypeUnit,
    ReportUnit,
    MinUnit,
    MaxQty,
    MinQty,
    ParamDetailFlag,
    PriceUp,
    PriceDown,
    StepPrice,
    FairPrice,
    RemarkInfo,
    T0Flag,
    SecuCode,
    SecuName,
    SecuType,
}

impl ColumnName for SeoperSwapCodeInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::ExchNo, Self::FutuCode, Self::FutuName, Self::PinyinShort, Self::SettleCrncyType, Self::ExchCrncyType, Self::FutuType, Self::AssetType, Self::TypeUnit, Self::ReportUnit, Self::MinUnit, Self::MaxQty, Self::MinQty, Self::ParamDetailFlag, Self::PriceUp, Self::PriceDown, Self::StepPrice, Self::FairPrice, Self::RemarkInfo, Self::T0Flag, Self::SecuCode, Self::SecuName, Self::SecuType];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::ExchNo => "exch_no",
            Self::FutuCode => "futu_code",
            Self::FutuName => "futu_name",
            Self::PinyinShort => "pinyin_short",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::FutuType => "futu_type",
            Self::AssetType => "asset_type",
            Self::TypeUnit => "type_unit",
            Self::ReportUnit => "report_unit",
            Self::MinUnit => "min_unit",
            Self::MaxQty => "max_qty",
            Self::MinQty => "min_qty",
            Self::ParamDetailFlag => "param_detail_flag",
            Self::PriceUp => "price_up",
            Self::PriceDown => "price_down",
            Self::StepPrice => "step_price",
            Self::FairPrice => "fair_price",
            Self::RemarkInfo => "remark_info",
            Self::T0Flag => "t0_flag",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::SecuType => "secu_type",
        }
    }
}

impl DataTable for SeoperSwapCodeInfo {
    const ID: &'static str = "tb_seoper_swap_code_info";
    type Column = SeoperSwapCodeInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSwapCodeInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSwapCodeInfo {}
