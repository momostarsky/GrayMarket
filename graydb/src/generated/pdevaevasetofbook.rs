//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdeva_eva_set_of_book`（codegen：字段与列名源自 DDL，`PdevaEvaSetOfBookColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdevaEvaSetOfBook {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub set_of_book_no: i32,
    pub set_of_book_name: String,
    pub remark_info: String,
}

/// `tb_pdeva_eva_set_of_book` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdevaEvaSetOfBookColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    SetOfBookNo,
    SetOfBookName,
    RemarkInfo,
}

impl ColumnName for PdevaEvaSetOfBookColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::SetOfBookNo, Self::SetOfBookName, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::SetOfBookNo => "set_of_book_no",
            Self::SetOfBookName => "set_of_book_name",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for PdevaEvaSetOfBook {
    const ID: &'static str = "tb_pdeva_eva_set_of_book";
    type Column = PdevaEvaSetOfBookColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdevaEvaSetOfBookColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdevaEvaSetOfBook {}
