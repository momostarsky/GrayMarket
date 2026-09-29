//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_dictionary`（codegen：字段与列名源自 DDL，`BaseoperDictionaryColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperDictionary {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub dict_no: i32,
    pub dict_name: String,
    pub dict_item_name: String,
    pub dict_item_value: String,
    pub time_stamp: i64,
    pub remark_info: String,
}

/// `tb_baseoper_dictionary` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperDictionaryColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    DictNo,
    DictName,
    DictItemName,
    DictItemValue,
    TimeStamp,
    RemarkInfo,
}

impl ColumnName for BaseoperDictionaryColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::DictNo, Self::DictName, Self::DictItemName, Self::DictItemValue, Self::TimeStamp, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::DictNo => "dict_no",
            Self::DictName => "dict_name",
            Self::DictItemName => "dict_item_name",
            Self::DictItemValue => "dict_item_value",
            Self::TimeStamp => "time_stamp",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BaseoperDictionary {
    const ID: &'static str = "tb_baseoper_dictionary";
    type Column = BaseoperDictionaryColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperDictionaryColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperDictionary {}
