//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_exposure_config`（codegen：字段与列名源自 DDL，`SemageRiskExposureConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskExposureConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub risk_item_config_batch_no: i32,
    pub risk_item_config_no: i32,
    pub risk_item_no: i32,
    pub risk_exposure_str: String,
}

/// `tb_semage_risk_exposure_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskExposureConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    RiskItemConfigBatchNo,
    RiskItemConfigNo,
    RiskItemNo,
    RiskExposureStr,
}

impl ColumnName for SemageRiskExposureConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::RiskItemConfigBatchNo, Self::RiskItemConfigNo, Self::RiskItemNo, Self::RiskExposureStr];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::RiskItemConfigBatchNo => "risk_item_config_batch_no",
            Self::RiskItemConfigNo => "risk_item_config_no",
            Self::RiskItemNo => "risk_item_no",
            Self::RiskExposureStr => "risk_exposure_str",
        }
    }
}

impl DataTable for SemageRiskExposureConfig {
    const ID: &'static str = "tb_semage_risk_exposure_config";
    type Column = SemageRiskExposureConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskExposureConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskExposureConfig {}
