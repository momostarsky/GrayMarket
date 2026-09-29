//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_sys_ctrl_config`（codegen：字段与列名源自 DDL，`BaseoperSysCtrlConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperSysCtrlConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub ctrl_config_type: String,
    pub ctrl_config_type_name: String,
    pub ctrl_config_no: i64,
    pub ctrl_config_name: String,
    pub ctrl_precond: String,
    pub ctrl_config_value: String,
    pub ctrl_do_value: String,
    pub ctrl_config_memo: String,
}

/// `tb_baseoper_sys_ctrl_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperSysCtrlConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    CtrlConfigType,
    CtrlConfigTypeName,
    CtrlConfigNo,
    CtrlConfigName,
    CtrlPrecond,
    CtrlConfigValue,
    CtrlDoValue,
    CtrlConfigMemo,
}

impl ColumnName for BaseoperSysCtrlConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::CtrlConfigType, Self::CtrlConfigTypeName, Self::CtrlConfigNo, Self::CtrlConfigName, Self::CtrlPrecond, Self::CtrlConfigValue, Self::CtrlDoValue, Self::CtrlConfigMemo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::CtrlConfigType => "ctrl_config_type",
            Self::CtrlConfigTypeName => "ctrl_config_type_name",
            Self::CtrlConfigNo => "ctrl_config_no",
            Self::CtrlConfigName => "ctrl_config_name",
            Self::CtrlPrecond => "ctrl_precond",
            Self::CtrlConfigValue => "ctrl_config_value",
            Self::CtrlDoValue => "ctrl_do_value",
            Self::CtrlConfigMemo => "ctrl_config_memo",
        }
    }
}

impl DataTable for BaseoperSysCtrlConfig {
    const ID: &'static str = "tb_baseoper_sys_ctrl_config";
    type Column = BaseoperSysCtrlConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperSysCtrlConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperSysCtrlConfig {}
