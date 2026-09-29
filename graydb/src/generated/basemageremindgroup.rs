//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_remind_group`（codegen：字段与列名源自 DDL，`BasemageRemindGroupColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageRemindGroup {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub remind_group_no: i32,
    pub remind_group_name: String,
    pub remind_group_type: i32,
    pub remark_info: String,
}

/// `tb_basemage_remind_group` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageRemindGroupColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    RemindGroupNo,
    RemindGroupName,
    RemindGroupType,
    RemarkInfo,
}

impl ColumnName for BasemageRemindGroupColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::RemindGroupNo, Self::RemindGroupName, Self::RemindGroupType, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::RemindGroupNo => "remind_group_no",
            Self::RemindGroupName => "remind_group_name",
            Self::RemindGroupType => "remind_group_type",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageRemindGroup {
    const ID: &'static str = "tb_basemage_remind_group";
    type Column = BasemageRemindGroupColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageRemindGroupColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageRemindGroup {}
