//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_code_item_config`（codegen：字段与列名源自 DDL，`SemageRiskCodeItemConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskCodeItemConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub risk_code_item_no: i32,
    pub risk_code_item_content: String,
    pub risk_code_item_config_type: i32,
    pub risk_code_item_config_str: String,
    pub remark_info: String,
    pub time_stamp: i64,
}

/// `tb_semage_risk_code_item_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskCodeItemConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    RiskCodeItemNo,
    RiskCodeItemContent,
    RiskCodeItemConfigType,
    RiskCodeItemConfigStr,
    RemarkInfo,
    TimeStamp,
}

impl ColumnName for SemageRiskCodeItemConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::RiskCodeItemNo, Self::RiskCodeItemContent, Self::RiskCodeItemConfigType, Self::RiskCodeItemConfigStr, Self::RemarkInfo, Self::TimeStamp];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::RiskCodeItemNo => "risk_code_item_no",
            Self::RiskCodeItemContent => "risk_code_item_content",
            Self::RiskCodeItemConfigType => "risk_code_item_config_type",
            Self::RiskCodeItemConfigStr => "risk_code_item_config_str",
            Self::RemarkInfo => "remark_info",
            Self::TimeStamp => "time_stamp",
        }
    }
}

impl DataTable for SemageRiskCodeItemConfig {
    const ID: &'static str = "tb_semage_risk_code_item_config";
    type Column = SemageRiskCodeItemConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskCodeItemConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskCodeItemConfig {}
