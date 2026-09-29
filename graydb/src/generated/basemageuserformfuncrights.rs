//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_user_formfuncrights`（codegen：字段与列名源自 DDL，`BasemageUserFormfuncrightsColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageUserFormfuncrights {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub user_no: i32,
    pub co_no: i32,
    pub user_type: i32,
    pub form_func_rights_str: String,
}

/// `tb_basemage_user_formfuncrights` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageUserFormfuncrightsColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    UserNo,
    CoNo,
    UserType,
    FormFuncRightsStr,
}

impl ColumnName for BasemageUserFormfuncrightsColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::UserNo, Self::CoNo, Self::UserType, Self::FormFuncRightsStr];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::UserNo => "user_no",
            Self::CoNo => "co_no",
            Self::UserType => "user_type",
            Self::FormFuncRightsStr => "form_func_rights_str",
        }
    }
}

impl DataTable for BasemageUserFormfuncrights {
    const ID: &'static str = "tb_basemage_user_formfuncrights";
    type Column = BasemageUserFormfuncrightsColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageUserFormfuncrightsColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageUserFormfuncrights {}
