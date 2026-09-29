//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdeva_eva_file_import_info`（codegen：字段与列名源自 DDL，`PdevaEvaFileImportInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdevaEvaFileImportInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub busi_date: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub upload_status: i32,
    pub import_status: i32,
    pub total_count: i32,
    pub log_memo: String,
}

/// `tb_pdeva_eva_file_import_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdevaEvaFileImportInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    BusiDate,
    CoNo,
    PdNo,
    UploadStatus,
    ImportStatus,
    TotalCount,
    LogMemo,
}

impl ColumnName for PdevaEvaFileImportInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::BusiDate, Self::CoNo, Self::PdNo, Self::UploadStatus, Self::ImportStatus, Self::TotalCount, Self::LogMemo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::BusiDate => "busi_date",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::UploadStatus => "upload_status",
            Self::ImportStatus => "import_status",
            Self::TotalCount => "total_count",
            Self::LogMemo => "log_memo",
        }
    }
}

impl DataTable for PdevaEvaFileImportInfo {
    const ID: &'static str = "tb_pdeva_eva_file_import_info";
    type Column = PdevaEvaFileImportInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdevaEvaFileImportInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdevaEvaFileImportInfo {}
