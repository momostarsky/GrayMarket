//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_param_default`（codegen：字段与列名源自 DDL，`BasemageParamDefaultColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageParamDefault {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub param_no: i32,
    pub param_name: String,
    pub param_value: String,
    pub param_type: i32,
    pub param_dim: i32,
    pub param_describe: String,
}

/// `tb_basemage_param_default` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageParamDefaultColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ParamNo,
    ParamName,
    ParamValue,
    ParamType,
    ParamDim,
    ParamDescribe,
}

impl ColumnName for BasemageParamDefaultColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ParamNo, Self::ParamName, Self::ParamValue, Self::ParamType, Self::ParamDim, Self::ParamDescribe];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ParamNo => "param_no",
            Self::ParamName => "param_name",
            Self::ParamValue => "param_value",
            Self::ParamType => "param_type",
            Self::ParamDim => "param_dim",
            Self::ParamDescribe => "param_describe",
        }
    }
}

impl DataTable for BasemageParamDefault {
    const ID: &'static str = "tb_basemage_param_default";
    type Column = BasemageParamDefaultColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageParamDefaultColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageParamDefault {}
