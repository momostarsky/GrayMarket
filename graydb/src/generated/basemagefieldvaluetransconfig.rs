//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_field_value_trans_config`（codegen：字段与列名源自 DDL，`BasemageFieldValueTransConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageFieldValueTransConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub field_config_no: i64,
    pub out_field_value: String,
    pub table_field_value: String,
    pub remark_info: String,
}

/// `tb_basemage_field_value_trans_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageFieldValueTransConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    FieldConfigNo,
    OutFieldValue,
    TableFieldValue,
    RemarkInfo,
}

impl ColumnName for BasemageFieldValueTransConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::FieldConfigNo, Self::OutFieldValue, Self::TableFieldValue, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::FieldConfigNo => "field_config_no",
            Self::OutFieldValue => "out_field_value",
            Self::TableFieldValue => "table_field_value",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageFieldValueTransConfig {
    const ID: &'static str = "tb_basemage_field_value_trans_config";
    type Column = BasemageFieldValueTransConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageFieldValueTransConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageFieldValueTransConfig {}
