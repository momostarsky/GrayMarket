//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_secu_type_stepprice_info`（codegen：字段与列名源自 DDL，`SeoperSecuTypeSteppriceInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSecuTypeSteppriceInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub exch_sub_type: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub secu_type: i32,
    pub price_up: Decimal,
    pub price_down: Decimal,
    pub step_price: Decimal,
    pub remark_info: String,
}

/// `tb_seoper_secu_type_stepprice_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSecuTypeSteppriceInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    ExchSubType,
    SettleCrncyType,
    ExchCrncyType,
    SecuType,
    PriceUp,
    PriceDown,
    StepPrice,
    RemarkInfo,
}

impl ColumnName for SeoperSecuTypeSteppriceInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::ExchSubType, Self::SettleCrncyType, Self::ExchCrncyType, Self::SecuType, Self::PriceUp, Self::PriceDown, Self::StepPrice, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::ExchSubType => "exch_sub_type",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::SecuType => "secu_type",
            Self::PriceUp => "price_up",
            Self::PriceDown => "price_down",
            Self::StepPrice => "step_price",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperSecuTypeSteppriceInfo {
    const ID: &'static str = "tb_seoper_secu_type_stepprice_info";
    type Column = SeoperSecuTypeSteppriceInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSecuTypeSteppriceInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSecuTypeSteppriceInfo {}
