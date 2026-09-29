//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_oper_log`（codegen：字段与列名源自 DDL，`BaseoperOperLogColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperOperLog {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub user_co_no: i32,
    pub user_no: i32,
    pub oper_mac: String,
    pub oper_ip: String,
    pub oper_info: String,
    pub oper_way: i32,
    pub menu_no: i32,
    pub func_code: String,
    pub log_type: i32,
    pub busi_oper_context: String,
    pub jour_occur_field: String,
    pub jour_occur_info: String,
    pub jour_after_occur_info: String,
    pub log_error_code: String,
    pub log_error_info: String,
    pub remark_info: String,
}

/// `tb_baseoper_oper_log` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperOperLogColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    UserCoNo,
    UserNo,
    OperMac,
    OperIp,
    OperInfo,
    OperWay,
    MenuNo,
    FuncCode,
    LogType,
    BusiOperContext,
    JourOccurField,
    JourOccurInfo,
    JourAfterOccurInfo,
    LogErrorCode,
    LogErrorInfo,
    RemarkInfo,
}

impl ColumnName for BaseoperOperLogColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::UserCoNo, Self::UserNo, Self::OperMac, Self::OperIp, Self::OperInfo, Self::OperWay, Self::MenuNo, Self::FuncCode, Self::LogType, Self::BusiOperContext, Self::JourOccurField, Self::JourOccurInfo, Self::JourAfterOccurInfo, Self::LogErrorCode, Self::LogErrorInfo, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::UserCoNo => "user_co_no",
            Self::UserNo => "user_no",
            Self::OperMac => "oper_mac",
            Self::OperIp => "oper_ip",
            Self::OperInfo => "oper_info",
            Self::OperWay => "oper_way",
            Self::MenuNo => "menu_no",
            Self::FuncCode => "func_code",
            Self::LogType => "log_type",
            Self::BusiOperContext => "busi_oper_context",
            Self::JourOccurField => "jour_occur_field",
            Self::JourOccurInfo => "jour_occur_info",
            Self::JourAfterOccurInfo => "jour_after_occur_info",
            Self::LogErrorCode => "log_error_code",
            Self::LogErrorInfo => "log_error_info",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BaseoperOperLog {
    const ID: &'static str = "tb_baseoper_oper_log";
    type Column = BaseoperOperLogColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperOperLogColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperOperLog {}
