//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_user_login_info`（codegen：字段与列名源自 DDL，`BasemageUserLoginInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageUserLoginInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub user_no: i32,
    pub oper_way: i32,
    pub co_no: i32,
    pub exor_no: i32,
    pub asac_no: i32,
    pub channel_no: i32,
    pub online_status: i32,
    pub login_count: i32,
    pub login_error_count: i32,
    pub client_ver: String,
    pub last_login_date: i32,
    pub last_login_time: i32,
    pub last_login_ip: String,
    pub last_login_mac: String,
    pub change_pwd_flag: i32,
    pub last_oper_info: String,
    pub unlock_date: i32,
    pub unlock_time: i32,
}

/// `tb_basemage_user_login_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageUserLoginInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    UserNo,
    OperWay,
    CoNo,
    ExorNo,
    AsacNo,
    ChannelNo,
    OnlineStatus,
    LoginCount,
    LoginErrorCount,
    ClientVer,
    LastLoginDate,
    LastLoginTime,
    LastLoginIp,
    LastLoginMac,
    ChangePwdFlag,
    LastOperInfo,
    UnlockDate,
    UnlockTime,
}

impl ColumnName for BasemageUserLoginInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::UserNo, Self::OperWay, Self::CoNo, Self::ExorNo, Self::AsacNo, Self::ChannelNo, Self::OnlineStatus, Self::LoginCount, Self::LoginErrorCount, Self::ClientVer, Self::LastLoginDate, Self::LastLoginTime, Self::LastLoginIp, Self::LastLoginMac, Self::ChangePwdFlag, Self::LastOperInfo, Self::UnlockDate, Self::UnlockTime];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::UserNo => "user_no",
            Self::OperWay => "oper_way",
            Self::CoNo => "co_no",
            Self::ExorNo => "exor_no",
            Self::AsacNo => "asac_no",
            Self::ChannelNo => "channel_no",
            Self::OnlineStatus => "online_status",
            Self::LoginCount => "login_count",
            Self::LoginErrorCount => "login_error_count",
            Self::ClientVer => "client_ver",
            Self::LastLoginDate => "last_login_date",
            Self::LastLoginTime => "last_login_time",
            Self::LastLoginIp => "last_login_ip",
            Self::LastLoginMac => "last_login_mac",
            Self::ChangePwdFlag => "change_pwd_flag",
            Self::LastOperInfo => "last_oper_info",
            Self::UnlockDate => "unlock_date",
            Self::UnlockTime => "unlock_time",
        }
    }
}

impl DataTable for BasemageUserLoginInfo {
    const ID: &'static str = "tb_basemage_user_login_info";
    type Column = BasemageUserLoginInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageUserLoginInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageUserLoginInfo {}
