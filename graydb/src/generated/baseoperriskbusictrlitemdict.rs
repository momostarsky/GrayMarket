//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_risk_busictrlitemdict`（codegen：字段与列名源自 DDL，`BaseoperRiskBusictrlitemdictColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperRiskBusictrlitemdict {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub risk_item_no: i32,
    pub risk_item_content: String,
    pub remark_info: String,
    pub time_stamp: i64,
}

/// `tb_baseoper_risk_busictrlitemdict` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperRiskBusictrlitemdictColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    RiskItemNo,
    RiskItemContent,
    RemarkInfo,
    TimeStamp,
}

impl ColumnName for BaseoperRiskBusictrlitemdictColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::RiskItemNo, Self::RiskItemContent, Self::RemarkInfo, Self::TimeStamp];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::RiskItemNo => "risk_item_no",
            Self::RiskItemContent => "risk_item_content",
            Self::RemarkInfo => "remark_info",
            Self::TimeStamp => "time_stamp",
        }
    }
}

impl DataTable for BaseoperRiskBusictrlitemdict {
    const ID: &'static str = "tb_baseoper_risk_busictrlitemdict";
    type Column = BaseoperRiskBusictrlitemdictColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperRiskBusictrlitemdictColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperRiskBusictrlitemdict {}
