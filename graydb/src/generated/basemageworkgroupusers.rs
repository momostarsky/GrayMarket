//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_workgroup_users`（codegen：字段与列名源自 DDL，`BasemageWorkgroupUsersColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageWorkgroupUsers {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub workgroup_id: i32,
    pub user_no: i32,
    pub effective_date: i32,
    pub expired_date: i32,
    pub instr_rights_str: String,
    pub remark_info: String,
}

/// `tb_basemage_workgroup_users` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageWorkgroupUsersColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    WorkgroupId,
    UserNo,
    EffectiveDate,
    ExpiredDate,
    InstrRightsStr,
    RemarkInfo,
}

impl ColumnName for BasemageWorkgroupUsersColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::WorkgroupId, Self::UserNo, Self::EffectiveDate, Self::ExpiredDate, Self::InstrRightsStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::WorkgroupId => "workgroup_id",
            Self::UserNo => "user_no",
            Self::EffectiveDate => "effective_date",
            Self::ExpiredDate => "expired_date",
            Self::InstrRightsStr => "instr_rights_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageWorkgroupUsers {
    const ID: &'static str = "tb_basemage_workgroup_users";
    type Column = BasemageWorkgroupUsersColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageWorkgroupUsersColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageWorkgroupUsers {}
