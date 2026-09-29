//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_co_custo_fieldvalue_map`（codegen：字段与列名源自 DDL，`SemageCoCustoFieldvalueMapColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageCoCustoFieldvalueMap {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub custo_id: i32,
    pub custo_field: String,
    pub table_field: String,
    pub custo_field_value: String,
    pub table_field_value: String,
}

/// `tb_semage_co_custo_fieldvalue_map` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageCoCustoFieldvalueMapColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    CustoId,
    CustoField,
    TableField,
    CustoFieldValue,
    TableFieldValue,
}

impl ColumnName for SemageCoCustoFieldvalueMapColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::CustoId, Self::CustoField, Self::TableField, Self::CustoFieldValue, Self::TableFieldValue];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::CustoId => "custo_id",
            Self::CustoField => "custo_field",
            Self::TableField => "table_field",
            Self::CustoFieldValue => "custo_field_value",
            Self::TableFieldValue => "table_field_value",
        }
    }
}

impl DataTable for SemageCoCustoFieldvalueMap {
    const ID: &'static str = "tb_semage_co_custo_fieldvalue_map";
    type Column = SemageCoCustoFieldvalueMapColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageCoCustoFieldvalueMapColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageCoCustoFieldvalueMap {}
