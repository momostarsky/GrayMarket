//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_dictionary_relation`（codegen：字段与列名源自 DDL，`BaseoperDictionaryRelationColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperDictionaryRelation {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub dict_type: i32,
    pub dict_no: i32,
    pub relation_dict_no: i32,
    pub dict_item_value: String,
    pub relation_dict_item_values: String,
    pub remark_info: String,
}

/// `tb_baseoper_dictionary_relation` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperDictionaryRelationColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    DictType,
    DictNo,
    RelationDictNo,
    DictItemValue,
    RelationDictItemValues,
    RemarkInfo,
}

impl ColumnName for BaseoperDictionaryRelationColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::DictType, Self::DictNo, Self::RelationDictNo, Self::DictItemValue, Self::RelationDictItemValues, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::DictType => "dict_type",
            Self::DictNo => "dict_no",
            Self::RelationDictNo => "relation_dict_no",
            Self::DictItemValue => "dict_item_value",
            Self::RelationDictItemValues => "relation_dict_item_values",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BaseoperDictionaryRelation {
    const ID: &'static str = "tb_baseoper_dictionary_relation";
    type Column = BaseoperDictionaryRelationColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperDictionaryRelationColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperDictionaryRelation {}
