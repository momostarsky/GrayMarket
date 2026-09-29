//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_cond_item_config`（codegen：字段与列名源自 DDL，`SemageRiskCondItemConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskCondItemConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub risk_cond_item_config_no: i32,
    pub risk_cond_item_config_content: String,
    pub risk_cond_item_no: i32,
    pub risk_cond_item_code: String,
    pub risk_cond_item_value_str: String,
    pub risk_cond_item_start_time: i32,
    pub risk_cond_item_end_time: i32,
    pub risk_cond_item_start_date: i32,
    pub risk_cond_item_end_date: i32,
    pub comp_dir: i32,
    pub rule_flag: i32,
    pub remark_info: String,
    pub time_stamp: i64,
    pub risk_item_config_ctrl_str: String,
}

/// `tb_semage_risk_cond_item_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskCondItemConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    RiskCondItemConfigNo,
    RiskCondItemConfigContent,
    RiskCondItemNo,
    RiskCondItemCode,
    RiskCondItemValueStr,
    RiskCondItemStartTime,
    RiskCondItemEndTime,
    RiskCondItemStartDate,
    RiskCondItemEndDate,
    CompDir,
    RuleFlag,
    RemarkInfo,
    TimeStamp,
    RiskItemConfigCtrlStr,
}

impl ColumnName for SemageRiskCondItemConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::RiskCondItemConfigNo, Self::RiskCondItemConfigContent, Self::RiskCondItemNo, Self::RiskCondItemCode, Self::RiskCondItemValueStr, Self::RiskCondItemStartTime, Self::RiskCondItemEndTime, Self::RiskCondItemStartDate, Self::RiskCondItemEndDate, Self::CompDir, Self::RuleFlag, Self::RemarkInfo, Self::TimeStamp, Self::RiskItemConfigCtrlStr];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::RiskCondItemConfigNo => "risk_cond_item_config_no",
            Self::RiskCondItemConfigContent => "risk_cond_item_config_content",
            Self::RiskCondItemNo => "risk_cond_item_no",
            Self::RiskCondItemCode => "risk_cond_item_code",
            Self::RiskCondItemValueStr => "risk_cond_item_value_str",
            Self::RiskCondItemStartTime => "risk_cond_item_start_time",
            Self::RiskCondItemEndTime => "risk_cond_item_end_time",
            Self::RiskCondItemStartDate => "risk_cond_item_start_date",
            Self::RiskCondItemEndDate => "risk_cond_item_end_date",
            Self::CompDir => "comp_dir",
            Self::RuleFlag => "rule_flag",
            Self::RemarkInfo => "remark_info",
            Self::TimeStamp => "time_stamp",
            Self::RiskItemConfigCtrlStr => "risk_item_config_ctrl_str",
        }
    }
}

impl DataTable for SemageRiskCondItemConfig {
    const ID: &'static str = "tb_semage_risk_cond_item_config";
    type Column = SemageRiskCondItemConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskCondItemConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskCondItemConfig {}
