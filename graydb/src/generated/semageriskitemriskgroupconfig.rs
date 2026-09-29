//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_item_riskgroup_config`（codegen：字段与列名源自 DDL，`SemageRiskItemRiskgroupConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskItemRiskgroupConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub risk_item_config_no: i32,
    pub risk_item_kind: i32,
    pub workgroup_id: i32,
    pub risk_appr_up: Decimal,
    pub risk_appr_down: Decimal,
    pub remark_info: String,
}

/// `tb_semage_risk_item_riskgroup_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskItemRiskgroupConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    RiskItemConfigNo,
    RiskItemKind,
    WorkgroupId,
    RiskApprUp,
    RiskApprDown,
    RemarkInfo,
}

impl ColumnName for SemageRiskItemRiskgroupConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::RiskItemConfigNo, Self::RiskItemKind, Self::WorkgroupId, Self::RiskApprUp, Self::RiskApprDown, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::RiskItemConfigNo => "risk_item_config_no",
            Self::RiskItemKind => "risk_item_kind",
            Self::WorkgroupId => "workgroup_id",
            Self::RiskApprUp => "risk_appr_up",
            Self::RiskApprDown => "risk_appr_down",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageRiskItemRiskgroupConfig {
    const ID: &'static str = "tb_semage_risk_item_riskgroup_config";
    type Column = SemageRiskItemRiskgroupConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskItemRiskgroupConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskItemRiskgroupConfig {}
