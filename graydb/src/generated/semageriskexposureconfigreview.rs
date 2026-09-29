//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_exposure_config_review`（codegen：字段与列名源自 DDL，`SemageRiskExposureConfigReviewColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskExposureConfigReview {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub risk_review_jour_no: i64,
    pub risk_item_config_batch_no: i32,
    pub risk_item_config_no: i32,
    pub risk_item_no: i32,
    pub risk_exposure_str: String,
    pub review_status: i32,
    pub valid_flag: i32,
}

/// `tb_semage_risk_exposure_config_review` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskExposureConfigReviewColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    RiskReviewJourNo,
    RiskItemConfigBatchNo,
    RiskItemConfigNo,
    RiskItemNo,
    RiskExposureStr,
    ReviewStatus,
    ValidFlag,
}

impl ColumnName for SemageRiskExposureConfigReviewColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::RiskReviewJourNo, Self::RiskItemConfigBatchNo, Self::RiskItemConfigNo, Self::RiskItemNo, Self::RiskExposureStr, Self::ReviewStatus, Self::ValidFlag];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::RiskReviewJourNo => "risk_review_jour_no",
            Self::RiskItemConfigBatchNo => "risk_item_config_batch_no",
            Self::RiskItemConfigNo => "risk_item_config_no",
            Self::RiskItemNo => "risk_item_no",
            Self::RiskExposureStr => "risk_exposure_str",
            Self::ReviewStatus => "review_status",
            Self::ValidFlag => "valid_flag",
        }
    }
}

impl DataTable for SemageRiskExposureConfigReview {
    const ID: &'static str = "tb_semage_risk_exposure_config_review";
    type Column = SemageRiskExposureConfigReviewColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskExposureConfigReviewColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskExposureConfigReview {}
