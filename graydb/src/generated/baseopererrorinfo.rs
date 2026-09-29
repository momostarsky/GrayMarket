//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_error_info`（codegen：字段与列名源自 DDL，`BaseoperErrorInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperErrorInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub error_no: i32,
    pub error_level: i32,
    pub error_prompt: String,
    pub error_deal: String,
}

/// `tb_baseoper_error_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperErrorInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ErrorNo,
    ErrorLevel,
    ErrorPrompt,
    ErrorDeal,
}

impl ColumnName for BaseoperErrorInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ErrorNo, Self::ErrorLevel, Self::ErrorPrompt, Self::ErrorDeal];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ErrorNo => "error_no",
            Self::ErrorLevel => "error_level",
            Self::ErrorPrompt => "error_prompt",
            Self::ErrorDeal => "error_deal",
        }
    }
}

impl DataTable for BaseoperErrorInfo {
    const ID: &'static str = "tb_baseoper_error_info";
    type Column = BaseoperErrorInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperErrorInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperErrorInfo {}
