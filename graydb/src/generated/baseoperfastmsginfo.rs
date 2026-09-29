//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_fastmsginfo`（codegen：字段与列名源自 DDL，`BaseoperFastmsginfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperFastmsginfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub user_token: String,
    pub user_co_no: i32,
    pub user_no: i32,
    pub user_pwd: String,
    pub oper_mac: String,
    pub oper_ip: String,
    pub oper_info: String,
    pub oper_way: i32,
    pub terminal_msg_id: String,
    pub menu_no: i32,
    pub func_code: String,
}

/// `tb_baseoper_fastmsginfo` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperFastmsginfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    UserToken,
    UserCoNo,
    UserNo,
    UserPwd,
    OperMac,
    OperIp,
    OperInfo,
    OperWay,
    TerminalMsgId,
    MenuNo,
    FuncCode,
}

impl ColumnName for BaseoperFastmsginfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::UserToken, Self::UserCoNo, Self::UserNo, Self::UserPwd, Self::OperMac, Self::OperIp, Self::OperInfo, Self::OperWay, Self::TerminalMsgId, Self::MenuNo, Self::FuncCode];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::UserToken => "user_token",
            Self::UserCoNo => "user_co_no",
            Self::UserNo => "user_no",
            Self::UserPwd => "user_pwd",
            Self::OperMac => "oper_mac",
            Self::OperIp => "oper_ip",
            Self::OperInfo => "oper_info",
            Self::OperWay => "oper_way",
            Self::TerminalMsgId => "terminal_msg_id",
            Self::MenuNo => "menu_no",
            Self::FuncCode => "func_code",
        }
    }
}

impl DataTable for BaseoperFastmsginfo {
    const ID: &'static str = "tb_baseoper_fastmsginfo";
    type Column = BaseoperFastmsginfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperFastmsginfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperFastmsginfo {}
