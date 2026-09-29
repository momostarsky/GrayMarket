//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_sys_info`（codegen：字段与列名源自 DDL，`BaseoperSysInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperSysInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub sys_name: String,
    pub sys_status: i32,
    pub init_date: i32,
    pub no_exch_date_str: String,
    pub datatohis_date: i32,
    pub create_pdunit_enable_flag: i32,
    pub sys_type: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub remark_info: String,
}

/// `tb_baseoper_sys_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperSysInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    SysName,
    SysStatus,
    InitDate,
    NoExchDateStr,
    DatatohisDate,
    CreatePdunitEnableFlag,
    SysType,
    BusiCtrlStr,
    EnableCtrlStr,
    RemarkInfo,
}

impl ColumnName for BaseoperSysInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::SysName, Self::SysStatus, Self::InitDate, Self::NoExchDateStr, Self::DatatohisDate, Self::CreatePdunitEnableFlag, Self::SysType, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::SysName => "sys_name",
            Self::SysStatus => "sys_status",
            Self::InitDate => "init_date",
            Self::NoExchDateStr => "no_exch_date_str",
            Self::DatatohisDate => "datatohis_date",
            Self::CreatePdunitEnableFlag => "create_pdunit_enable_flag",
            Self::SysType => "sys_type",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BaseoperSysInfo {
    const ID: &'static str = "tb_baseoper_sys_info";
    type Column = BaseoperSysInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperSysInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperSysInfo {}
