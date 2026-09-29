//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_dictionary_custom`（codegen：字段与列名源自 DDL，`BasemageDictionaryCustomColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageDictionaryCustom {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub dict_no: i32,
    pub dict_name: String,
    pub dict_item_name: String,
    pub dict_item_value: String,
    pub remark_info: String,
}

/// `tb_basemage_dictionary_custom` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageDictionaryCustomColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    DictNo,
    DictName,
    DictItemName,
    DictItemValue,
    RemarkInfo,
}

impl ColumnName for BasemageDictionaryCustomColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::DictNo, Self::DictName, Self::DictItemName, Self::DictItemValue, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::DictNo => "dict_no",
            Self::DictName => "dict_name",
            Self::DictItemName => "dict_item_name",
            Self::DictItemValue => "dict_item_value",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageDictionaryCustom {
    const ID: &'static str = "tb_basemage_dictionary_custom";
    type Column = BasemageDictionaryCustomColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageDictionaryCustomColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageDictionaryCustom {}
