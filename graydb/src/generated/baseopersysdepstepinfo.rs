//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_sys_dep_step_info`（codegen：字段与列名源自 DDL，`BaseoperSysDepStepInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperSysDepStepInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub dep_step_no: i32,
    pub dep_step_name: String,
    pub init_date: i32,
    pub dep_step_status: i32,
    pub remark_info: String,
}

/// `tb_baseoper_sys_dep_step_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperSysDepStepInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    DepStepNo,
    DepStepName,
    InitDate,
    DepStepStatus,
    RemarkInfo,
}

impl ColumnName for BaseoperSysDepStepInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::DepStepNo, Self::DepStepName, Self::InitDate, Self::DepStepStatus, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::DepStepNo => "dep_step_no",
            Self::DepStepName => "dep_step_name",
            Self::InitDate => "init_date",
            Self::DepStepStatus => "dep_step_status",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BaseoperSysDepStepInfo {
    const ID: &'static str = "tb_baseoper_sys_dep_step_info";
    type Column = BaseoperSysDepStepInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperSysDepStepInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperSysDepStepInfo {}
