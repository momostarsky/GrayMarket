//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_secu_code_pool_level`（codegen：字段与列名源自 DDL，`SemageSecuCodePoolLevelColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageSecuCodePoolLevel {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub secu_code_pool_dim_level: i32,
    pub secu_code_pool_dim_no: i32,
    pub secu_code_pool_dim_name: String,
    pub secu_code_pool_dim_type: i32,
    pub pd_no_str: String,
    pub remark_info: String,
}

/// `tb_semage_secu_code_pool_level` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageSecuCodePoolLevelColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    SecuCodePoolDimLevel,
    SecuCodePoolDimNo,
    SecuCodePoolDimName,
    SecuCodePoolDimType,
    PdNoStr,
    RemarkInfo,
}

impl ColumnName for SemageSecuCodePoolLevelColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::SecuCodePoolDimLevel, Self::SecuCodePoolDimNo, Self::SecuCodePoolDimName, Self::SecuCodePoolDimType, Self::PdNoStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::SecuCodePoolDimLevel => "secu_code_pool_dim_level",
            Self::SecuCodePoolDimNo => "secu_code_pool_dim_no",
            Self::SecuCodePoolDimName => "secu_code_pool_dim_name",
            Self::SecuCodePoolDimType => "secu_code_pool_dim_type",
            Self::PdNoStr => "pd_no_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageSecuCodePoolLevel {
    const ID: &'static str = "tb_semage_secu_code_pool_level";
    type Column = SemageSecuCodePoolLevelColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageSecuCodePoolLevelColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageSecuCodePoolLevel {}
