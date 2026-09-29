//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_posi_part_jour`（codegen：字段与列名源自 DDL，`SemagePosiPartJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemagePosiPartJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub co_no: i32,
    pub jour_no: i32,
    pub posi_part_batch_no: i32,
    pub posi_part_no: i32,
    pub user_no: i32,
    pub user_name: String,
    pub remark_info: String,
}

/// `tb_semage_posi_part_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemagePosiPartJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    CoNo,
    JourNo,
    PosiPartBatchNo,
    PosiPartNo,
    UserNo,
    UserName,
    RemarkInfo,
}

impl ColumnName for SemagePosiPartJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::CoNo, Self::JourNo, Self::PosiPartBatchNo, Self::PosiPartNo, Self::UserNo, Self::UserName, Self::RemarkInfo];
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
            Self::JourNo => "jour_no",
            Self::PosiPartBatchNo => "posi_part_batch_no",
            Self::PosiPartNo => "posi_part_no",
            Self::UserNo => "user_no",
            Self::UserName => "user_name",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemagePosiPartJour {
    const ID: &'static str = "tb_semage_posi_part_jour";
    type Column = SemagePosiPartJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemagePosiPartJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemagePosiPartJour {}
