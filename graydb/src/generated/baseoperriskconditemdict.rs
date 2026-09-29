//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_risk_conditemdict`（codegen：字段与列名源自 DDL，`BaseoperRiskConditemdictColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperRiskConditemdict {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub risk_cond_item_no: i32,
    pub risk_cond_item_content: String,
    pub remark_info: String,
    pub time_stamp: i64,
}

/// `tb_baseoper_risk_conditemdict` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperRiskConditemdictColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    RiskCondItemNo,
    RiskCondItemContent,
    RemarkInfo,
    TimeStamp,
}

impl ColumnName for BaseoperRiskConditemdictColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::RiskCondItemNo, Self::RiskCondItemContent, Self::RemarkInfo, Self::TimeStamp];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::RiskCondItemNo => "risk_cond_item_no",
            Self::RiskCondItemContent => "risk_cond_item_content",
            Self::RemarkInfo => "remark_info",
            Self::TimeStamp => "time_stamp",
        }
    }
}

impl DataTable for BaseoperRiskConditemdict {
    const ID: &'static str = "tb_baseoper_risk_conditemdict";
    type Column = BaseoperRiskConditemdictColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperRiskConditemdictColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperRiskConditemdict {}
