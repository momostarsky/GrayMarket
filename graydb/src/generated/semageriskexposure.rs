//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_exposure`（codegen：字段与列名源自 DDL，`SemageRiskExposureColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskExposure {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub exposure_no: i32,
    pub exposure_name: String,
    pub exposure_type: i32,
    pub exposure_level: i32,
    pub modi_user_no: i32,
    pub remark_info: String,
}

/// `tb_semage_risk_exposure` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskExposureColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    ExposureNo,
    ExposureName,
    ExposureType,
    ExposureLevel,
    ModiUserNo,
    RemarkInfo,
}

impl ColumnName for SemageRiskExposureColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::ExposureNo, Self::ExposureName, Self::ExposureType, Self::ExposureLevel, Self::ModiUserNo, Self::RemarkInfo];
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
            Self::ExposureName => "exposure_name",
            Self::ExposureType => "exposure_type",
            Self::ExposureLevel => "exposure_level",
            Self::ModiUserNo => "modi_user_no",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageRiskExposure {
    const ID: &'static str = "tb_semage_risk_exposure";
    type Column = SemageRiskExposureColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskExposureColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskExposure {}
