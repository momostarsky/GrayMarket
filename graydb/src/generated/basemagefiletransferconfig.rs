//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_file_transfer_config`（codegen：字段与列名源自 DDL，`BasemageFileTransferConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageFileTransferConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub file_config_no: i64,
    pub file_config_name: String,
    pub file_type: i32,
    pub file_export_name: String,
    pub file_export_flag: i32,
    pub file_class: i32,
}

/// `tb_basemage_file_transfer_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageFileTransferConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    FileConfigNo,
    FileConfigName,
    FileType,
    FileExportName,
    FileExportFlag,
    FileClass,
}

impl ColumnName for BasemageFileTransferConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::FileConfigNo, Self::FileConfigName, Self::FileType, Self::FileExportName, Self::FileExportFlag, Self::FileClass];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::FileConfigNo => "file_config_no",
            Self::FileConfigName => "file_config_name",
            Self::FileType => "file_type",
            Self::FileExportName => "file_export_name",
            Self::FileExportFlag => "file_export_flag",
            Self::FileClass => "file_class",
        }
    }
}

impl DataTable for BasemageFileTransferConfig {
    const ID: &'static str = "tb_basemage_file_transfer_config";
    type Column = BasemageFileTransferConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageFileTransferConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageFileTransferConfig {}
