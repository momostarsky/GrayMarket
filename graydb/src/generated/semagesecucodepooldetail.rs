//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_secu_code_pool_detail`（codegen：字段与列名源自 DDL，`SemageSecuCodePoolDetailColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageSecuCodePoolDetail {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub secu_code_pool_dim_no: i32,
    pub secu_code_pool_no: i32,
    pub secu_code_pool_type: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub secu_type: i32,
    pub remark_info: String,
}

/// `tb_semage_secu_code_pool_detail` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageSecuCodePoolDetailColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    SecuCodePoolDimNo,
    SecuCodePoolNo,
    SecuCodePoolType,
    ExchNo,
    SecuCode,
    SecuName,
    SecuType,
    RemarkInfo,
}

impl ColumnName for SemageSecuCodePoolDetailColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::SecuCodePoolDimNo, Self::SecuCodePoolNo, Self::SecuCodePoolType, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::SecuType, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::SecuCodePoolDimNo => "secu_code_pool_dim_no",
            Self::SecuCodePoolNo => "secu_code_pool_no",
            Self::SecuCodePoolType => "secu_code_pool_type",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::SecuType => "secu_type",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageSecuCodePoolDetail {
    const ID: &'static str = "tb_semage_secu_code_pool_detail";
    type Column = SemageSecuCodePoolDetailColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageSecuCodePoolDetailColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageSecuCodePoolDetail {}
