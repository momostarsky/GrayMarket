//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_code_item_config_custcode`（codegen：字段与列名源自 DDL，`SemageRiskCodeItemConfigCustcodeColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskCodeItemConfigCustcode {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub risk_code_item_no: i32,
    pub risk_code_item_config_type: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub remark_info: String,
    pub time_stamp: i64,
}

/// `tb_semage_risk_code_item_config_custcode` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskCodeItemConfigCustcodeColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    RiskCodeItemNo,
    RiskCodeItemConfigType,
    ExchNo,
    SecuCode,
    RemarkInfo,
    TimeStamp,
}

impl ColumnName for SemageRiskCodeItemConfigCustcodeColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::RiskCodeItemNo, Self::RiskCodeItemConfigType, Self::ExchNo, Self::SecuCode, Self::RemarkInfo, Self::TimeStamp];
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
            Self::RiskCodeItemConfigType => "risk_code_item_config_type",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::RemarkInfo => "remark_info",
            Self::TimeStamp => "time_stamp",
        }
    }
}

impl DataTable for SemageRiskCodeItemConfigCustcode {
    const ID: &'static str = "tb_semage_risk_code_item_config_custcode";
    type Column = SemageRiskCodeItemConfigCustcodeColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskCodeItemConfigCustcodeColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskCodeItemConfigCustcode {}
