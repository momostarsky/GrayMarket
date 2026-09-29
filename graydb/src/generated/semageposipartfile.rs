//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_posi_part_file`（codegen：字段与列名源自 DDL，`SemagePosiPartFileColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemagePosiPartFile {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub co_no: i32,
    pub posi_part_batch_no: i32,
    pub pd_no: i32,
    pub pd_code: String,
    pub file_addr: String,
    pub file_name: String,
    pub remark_info: String,
}

/// `tb_semage_posi_part_file` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemagePosiPartFileColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    CoNo,
    PosiPartBatchNo,
    PdNo,
    PdCode,
    FileAddr,
    FileName,
    RemarkInfo,
}

impl ColumnName for SemagePosiPartFileColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::CoNo, Self::PosiPartBatchNo, Self::PdNo, Self::PdCode, Self::FileAddr, Self::FileName, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::CoNo => "co_no",
            Self::PosiPartBatchNo => "posi_part_batch_no",
            Self::PdNo => "pd_no",
            Self::PdCode => "pd_code",
            Self::FileAddr => "file_addr",
            Self::FileName => "file_name",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemagePosiPartFile {
    const ID: &'static str = "tb_semage_posi_part_file";
    type Column = SemagePosiPartFileColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemagePosiPartFileColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemagePosiPartFile {}
