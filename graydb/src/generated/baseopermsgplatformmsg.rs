//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_msgplatformmsg`（codegen：字段与列名源自 DDL，`BaseoperMsgplatformmsgColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperMsgplatformmsg {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub target_user_no: i32,
    pub user_name: String,
    pub company_name: String,
    pub sys_name: String,
    pub customer_hotline: String,
    pub msg_content: String,
    pub msg_no: i64,
    pub msg_temp: String,
    pub phone: String,
    pub email: String,
    pub deal_flag: i32,
}

/// `tb_baseoper_msgplatformmsg` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperMsgplatformmsgColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    TargetUserNo,
    UserName,
    CompanyName,
    SysName,
    CustomerHotline,
    MsgContent,
    MsgNo,
    MsgTemp,
    Phone,
    Email,
    DealFlag,
}

impl ColumnName for BaseoperMsgplatformmsgColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::TargetUserNo, Self::UserName, Self::CompanyName, Self::SysName, Self::CustomerHotline, Self::MsgContent, Self::MsgNo, Self::MsgTemp, Self::Phone, Self::Email, Self::DealFlag];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::TargetUserNo => "target_user_no",
            Self::UserName => "user_name",
            Self::CompanyName => "company_name",
            Self::SysName => "sys_name",
            Self::CustomerHotline => "customer_hotline",
            Self::MsgContent => "msg_content",
            Self::MsgNo => "msg_no",
            Self::MsgTemp => "msg_temp",
            Self::Phone => "phone",
            Self::Email => "email",
            Self::DealFlag => "deal_flag",
        }
    }
}

impl DataTable for BaseoperMsgplatformmsg {
    const ID: &'static str = "tb_baseoper_msgplatformmsg";
    type Column = BaseoperMsgplatformmsgColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperMsgplatformmsgColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperMsgplatformmsg {}
