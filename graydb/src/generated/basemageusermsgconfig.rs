//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_user_msg_config`（codegen：字段与列名源自 DDL，`BasemageUserMsgConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageUserMsgConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub user_no: i32,
    pub msg_config_no: i32,
    pub remind_mode_str: String,
    pub msg_tone: String,
    pub resp_mode: i32,
    pub menu_activate_mode: i32,
    pub remark_info: String,
}

/// `tb_basemage_user_msg_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageUserMsgConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    UserNo,
    MsgConfigNo,
    RemindModeStr,
    MsgTone,
    RespMode,
    MenuActivateMode,
    RemarkInfo,
}

impl ColumnName for BasemageUserMsgConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::UserNo, Self::MsgConfigNo, Self::RemindModeStr, Self::MsgTone, Self::RespMode, Self::MenuActivateMode, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::UserNo => "user_no",
            Self::MsgConfigNo => "msg_config_no",
            Self::RemindModeStr => "remind_mode_str",
            Self::MsgTone => "msg_tone",
            Self::RespMode => "resp_mode",
            Self::MenuActivateMode => "menu_activate_mode",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageUserMsgConfig {
    const ID: &'static str = "tb_basemage_user_msg_config";
    type Column = BasemageUserMsgConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageUserMsgConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageUserMsgConfig {}
