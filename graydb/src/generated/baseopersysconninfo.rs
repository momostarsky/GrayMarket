//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_sys_conn_info`（codegen：字段与列名源自 DDL，`BaseoperSysConnInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperSysConnInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub terminal_conn_type: i32,
    pub terminal_conn_name: String,
    pub terminal_conn_in_ip: String,
    pub terminal_conn_out_ip: String,
    pub terminal_conn_port: i32,
    pub terminal_conn_pwd: String,
    pub remark_info: String,
}

/// `tb_baseoper_sys_conn_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperSysConnInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    TerminalConnType,
    TerminalConnName,
    TerminalConnInIp,
    TerminalConnOutIp,
    TerminalConnPort,
    TerminalConnPwd,
    RemarkInfo,
}

impl ColumnName for BaseoperSysConnInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::TerminalConnType, Self::TerminalConnName, Self::TerminalConnInIp, Self::TerminalConnOutIp, Self::TerminalConnPort, Self::TerminalConnPwd, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::TerminalConnType => "terminal_conn_type",
            Self::TerminalConnName => "terminal_conn_name",
            Self::TerminalConnInIp => "terminal_conn_in_ip",
            Self::TerminalConnOutIp => "terminal_conn_out_ip",
            Self::TerminalConnPort => "terminal_conn_port",
            Self::TerminalConnPwd => "terminal_conn_pwd",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BaseoperSysConnInfo {
    const ID: &'static str = "tb_baseoper_sys_conn_info";
    type Column = BaseoperSysConnInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperSysConnInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperSysConnInfo {}
