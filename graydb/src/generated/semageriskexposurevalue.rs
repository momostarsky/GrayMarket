//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_exposure_value`（codegen：字段与列名源自 DDL，`SemageRiskExposureValueColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskExposureValue {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub exposure_no: i32,
    pub exposure_level_value: i32,
    pub exposure_value: Decimal,
    pub modi_user_no: i32,
    pub remark_info: String,
}

/// `tb_semage_risk_exposure_value` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskExposureValueColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    ExposureNo,
    ExposureLevelValue,
    ExposureValue,
    ModiUserNo,
    RemarkInfo,
}

impl ColumnName for SemageRiskExposureValueColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::ExposureNo, Self::ExposureLevelValue, Self::ExposureValue, Self::ModiUserNo, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::ExposureNo => "exposure_no",
            Self::ExposureLevelValue => "exposure_level_value",
            Self::ExposureValue => "exposure_value",
            Self::ModiUserNo => "modi_user_no",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageRiskExposureValue {
    const ID: &'static str = "tb_semage_risk_exposure_value";
    type Column = SemageRiskExposureValueColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskExposureValueColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskExposureValue {}
