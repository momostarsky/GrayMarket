//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_ctrl_type_config`（codegen：字段与列名源自 DDL，`BasemageCtrlTypeConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageCtrlTypeConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub ctrl_type_id: i32,
    pub ctrl_type_name: String,
    pub relation_dict_no: i32,
    pub multi_check_flag: i32,
    pub remark_info: String,
}

/// `tb_basemage_ctrl_type_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageCtrlTypeConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CtrlTypeId,
    CtrlTypeName,
    RelationDictNo,
    MultiCheckFlag,
    RemarkInfo,
}

impl ColumnName for BasemageCtrlTypeConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CtrlTypeId, Self::CtrlTypeName, Self::RelationDictNo, Self::MultiCheckFlag, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CtrlTypeId => "ctrl_type_id",
            Self::CtrlTypeName => "ctrl_type_name",
            Self::RelationDictNo => "relation_dict_no",
            Self::MultiCheckFlag => "multi_check_flag",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageCtrlTypeConfig {
    const ID: &'static str = "tb_basemage_ctrl_type_config";
    type Column = BasemageCtrlTypeConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageCtrlTypeConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageCtrlTypeConfig {}
