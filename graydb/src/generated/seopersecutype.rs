//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_secu_type`（codegen：字段与列名源自 DDL，`SeoperSecuTypeColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSecuType {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub exch_sub_type: i32,
    pub secu_type: i32,
    pub asset_type: i32,
    pub par_value: Decimal,
    pub type_unit: i32,
    pub report_unit: i32,
    pub min_unit: i32,
    pub max_qty: Decimal,
    pub min_qty: Decimal,
    pub step_price: Decimal,
    pub param_detail_flag: i32,
    pub time_stamp: i64,
    pub t0_flag: i32,
    pub remark_info: String,
}

/// `tb_seoper_secu_type` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSecuTypeColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    ExchSubType,
    SecuType,
    AssetType,
    ParValue,
    TypeUnit,
    ReportUnit,
    MinUnit,
    MaxQty,
    MinQty,
    StepPrice,
    ParamDetailFlag,
    TimeStamp,
    T0Flag,
    RemarkInfo,
}

impl ColumnName for SeoperSecuTypeColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::ExchSubType, Self::SecuType, Self::AssetType, Self::ParValue, Self::TypeUnit, Self::ReportUnit, Self::MinUnit, Self::MaxQty, Self::MinQty, Self::StepPrice, Self::ParamDetailFlag, Self::TimeStamp, Self::T0Flag, Self::RemarkInfo];
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
            Self::SecuType => "secu_type",
            Self::AssetType => "asset_type",
            Self::ParValue => "par_value",
            Self::TypeUnit => "type_unit",
            Self::ReportUnit => "report_unit",
            Self::MinUnit => "min_unit",
            Self::MaxQty => "max_qty",
            Self::MinQty => "min_qty",
            Self::StepPrice => "step_price",
            Self::ParamDetailFlag => "param_detail_flag",
            Self::TimeStamp => "time_stamp",
            Self::T0Flag => "t0_flag",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperSecuType {
    const ID: &'static str = "tb_seoper_secu_type";
    type Column = SeoperSecuTypeColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSecuTypeColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSecuType {}
