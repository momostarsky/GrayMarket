//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_checkcode_info`（codegen：字段与列名源自 DDL，`BasemageCheckcodeInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageCheckcodeInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub phone: String,
    pub check_code: String,
    pub pwd_valid_date: i32,
    pub pwd_valid_time: i32,
    pub remark_info: String,
}

/// `tb_basemage_checkcode_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageCheckcodeInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    Phone,
    CheckCode,
    PwdValidDate,
    PwdValidTime,
    RemarkInfo,
}

impl ColumnName for BasemageCheckcodeInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::Phone, Self::CheckCode, Self::PwdValidDate, Self::PwdValidTime, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::Phone => "phone",
            Self::CheckCode => "check_code",
            Self::PwdValidDate => "pwd_valid_date",
            Self::PwdValidTime => "pwd_valid_time",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageCheckcodeInfo {
    const ID: &'static str = "tb_basemage_checkcode_info";
    type Column = BasemageCheckcodeInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageCheckcodeInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageCheckcodeInfo {}
