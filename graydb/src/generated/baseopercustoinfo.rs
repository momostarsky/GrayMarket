//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_custo_info`（codegen：字段与列名源自 DDL，`BaseoperCustoInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperCustoInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub custo_id: i32,
    pub custo_code: String,
    pub custo_key: String,
    pub custo_name: String,
    pub custo_type: i32,
    pub remark_info: String,
}

/// `tb_baseoper_custo_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperCustoInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CustoId,
    CustoCode,
    CustoKey,
    CustoName,
    CustoType,
    RemarkInfo,
}

impl ColumnName for BaseoperCustoInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CustoId, Self::CustoCode, Self::CustoKey, Self::CustoName, Self::CustoType, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CustoId => "custo_id",
            Self::CustoCode => "custo_code",
            Self::CustoKey => "custo_key",
            Self::CustoName => "custo_name",
            Self::CustoType => "custo_type",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BaseoperCustoInfo {
    const ID: &'static str = "tb_baseoper_custo_info";
    type Column = BaseoperCustoInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperCustoInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperCustoInfo {}
