//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_risk_itemdict`（codegen：字段与列名源自 DDL，`BaseoperRiskItemdictColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperRiskItemdict {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub risk_item_no: i32,
    pub risk_item_type_name: String,
    pub risk_item_type: String,
    pub risk_item_kind_str: String,
    pub risk_item_content: String,
    pub remark_info: String,
    pub risk_item_ctrl_str: String,
    pub risk_item_order_dir_str: String,
    pub comp_dir_str: String,
    pub time_stamp: i64,
}

/// `tb_baseoper_risk_itemdict` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperRiskItemdictColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    RiskItemNo,
    RiskItemTypeName,
    RiskItemType,
    RiskItemKindStr,
    RiskItemContent,
    RemarkInfo,
    RiskItemCtrlStr,
    RiskItemOrderDirStr,
    CompDirStr,
    TimeStamp,
}

impl ColumnName for BaseoperRiskItemdictColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::RiskItemNo, Self::RiskItemTypeName, Self::RiskItemType, Self::RiskItemKindStr, Self::RiskItemContent, Self::RemarkInfo, Self::RiskItemCtrlStr, Self::RiskItemOrderDirStr, Self::CompDirStr, Self::TimeStamp];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::RiskItemNo => "risk_item_no",
            Self::RiskItemTypeName => "risk_item_type_name",
            Self::RiskItemType => "risk_item_type",
            Self::RiskItemKindStr => "risk_item_kind_str",
            Self::RiskItemContent => "risk_item_content",
            Self::RemarkInfo => "remark_info",
            Self::RiskItemCtrlStr => "risk_item_ctrl_str",
            Self::RiskItemOrderDirStr => "risk_item_order_dir_str",
            Self::CompDirStr => "comp_dir_str",
            Self::TimeStamp => "time_stamp",
        }
    }
}

impl DataTable for BaseoperRiskItemdict {
    const ID: &'static str = "tb_baseoper_risk_itemdict";
    type Column = BaseoperRiskItemdictColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperRiskItemdictColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperRiskItemdict {}
