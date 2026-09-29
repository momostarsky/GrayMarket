//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_user_info`（codegen：字段与列名源自 DDL，`BasemageUserInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageUserInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub user_no: i32,
    pub co_no: i32,
    pub user_name: String,
    pub user_type: i32,
    pub user_pwd: String,
    pub user_status: i32,
    pub user_nickname: String,
    pub user_image: i32,
    pub phone: String,
    pub email: String,
    pub exor_no: i32,
    pub asac_no: i32,
    pub channel_no: i32,
    pub exch_role_no_str: String,
    pub mage_role_no_str: String,
    pub back_role_no_str: String,
    pub exch_menu_func_rights_str: String,
    pub mage_menu_func_rights_str: String,
    pub back_menu_func_rights_str: String,
    pub allow_login_type: String,
    pub allow_oper_ip: String,
    pub allow_oper_mac: String,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub remark_info: String,
    pub cert_code: String,
    pub cert_issue_place: String,
    pub conta_addr: String,
    pub exor_code: String,
}

/// `tb_basemage_user_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageUserInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    UserNo,
    CoNo,
    UserName,
    UserType,
    UserPwd,
    UserStatus,
    UserNickname,
    UserImage,
    Phone,
    Email,
    ExorNo,
    AsacNo,
    ChannelNo,
    ExchRoleNoStr,
    MageRoleNoStr,
    BackRoleNoStr,
    ExchMenuFuncRightsStr,
    MageMenuFuncRightsStr,
    BackMenuFuncRightsStr,
    AllowLoginType,
    AllowOperIp,
    AllowOperMac,
    BusiCtrlStr,
    EnableCtrlStr,
    RemarkInfo,
    CertCode,
    CertIssuePlace,
    ContaAddr,
    ExorCode,
}

impl ColumnName for BasemageUserInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::UserNo, Self::CoNo, Self::UserName, Self::UserType, Self::UserPwd, Self::UserStatus, Self::UserNickname, Self::UserImage, Self::Phone, Self::Email, Self::ExorNo, Self::AsacNo, Self::ChannelNo, Self::ExchRoleNoStr, Self::MageRoleNoStr, Self::BackRoleNoStr, Self::ExchMenuFuncRightsStr, Self::MageMenuFuncRightsStr, Self::BackMenuFuncRightsStr, Self::AllowLoginType, Self::AllowOperIp, Self::AllowOperMac, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::RemarkInfo, Self::CertCode, Self::CertIssuePlace, Self::ContaAddr, Self::ExorCode];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::UserNo => "user_no",
            Self::CoNo => "co_no",
            Self::UserName => "user_name",
            Self::UserType => "user_type",
            Self::UserPwd => "user_pwd",
            Self::UserStatus => "user_status",
            Self::UserNickname => "user_nickname",
            Self::UserImage => "user_image",
            Self::Phone => "phone",
            Self::Email => "email",
            Self::ExorNo => "exor_no",
            Self::AsacNo => "asac_no",
            Self::ChannelNo => "channel_no",
            Self::ExchRoleNoStr => "exch_role_no_str",
            Self::MageRoleNoStr => "mage_role_no_str",
            Self::BackRoleNoStr => "back_role_no_str",
            Self::ExchMenuFuncRightsStr => "exch_menu_func_rights_str",
            Self::MageMenuFuncRightsStr => "mage_menu_func_rights_str",
            Self::BackMenuFuncRightsStr => "back_menu_func_rights_str",
            Self::AllowLoginType => "allow_login_type",
            Self::AllowOperIp => "allow_oper_ip",
            Self::AllowOperMac => "allow_oper_mac",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::RemarkInfo => "remark_info",
            Self::CertCode => "cert_code",
            Self::CertIssuePlace => "cert_issue_place",
            Self::ContaAddr => "conta_addr",
            Self::ExorCode => "exor_code",
        }
    }
}

impl DataTable for BasemageUserInfo {
    const ID: &'static str = "tb_basemage_user_info";
    type Column = BasemageUserInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageUserInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageUserInfo {}
