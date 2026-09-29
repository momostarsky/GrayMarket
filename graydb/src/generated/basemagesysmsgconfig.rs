//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_sys_msg_config`（codegen：字段与列名源自 DDL，`BasemageSysMsgConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageSysMsgConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub msg_config_no: i32,
    pub msg_config_name: String,
    pub sys_msg_type: i32,
    pub relation_menu_no: i32,
    pub menu_activate_mode: i32,
    pub subscribe_mode: i32,
    pub remind_mode_str: String,
    pub msg_get_mode: i32,
    pub remind_frequency: i32,
    pub msg_tone: String,
    pub resp_mode: i32,
    pub remind_group_no_str: String,
    pub msg_template_str: String,
    pub param_no_str: String,
    pub remark_info: String,
}

/// `tb_basemage_sys_msg_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageSysMsgConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    MsgConfigNo,
    MsgConfigName,
    SysMsgType,
    RelationMenuNo,
    MenuActivateMode,
    SubscribeMode,
    RemindModeStr,
    MsgGetMode,
    RemindFrequency,
    MsgTone,
    RespMode,
    RemindGroupNoStr,
    MsgTemplateStr,
    ParamNoStr,
    RemarkInfo,
}

impl ColumnName for BasemageSysMsgConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::MsgConfigNo, Self::MsgConfigName, Self::SysMsgType, Self::RelationMenuNo, Self::MenuActivateMode, Self::SubscribeMode, Self::RemindModeStr, Self::MsgGetMode, Self::RemindFrequency, Self::MsgTone, Self::RespMode, Self::RemindGroupNoStr, Self::MsgTemplateStr, Self::ParamNoStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::MsgConfigNo => "msg_config_no",
            Self::MsgConfigName => "msg_config_name",
            Self::SysMsgType => "sys_msg_type",
            Self::RelationMenuNo => "relation_menu_no",
            Self::MenuActivateMode => "menu_activate_mode",
            Self::SubscribeMode => "subscribe_mode",
            Self::RemindModeStr => "remind_mode_str",
            Self::MsgGetMode => "msg_get_mode",
            Self::RemindFrequency => "remind_frequency",
            Self::MsgTone => "msg_tone",
            Self::RespMode => "resp_mode",
            Self::RemindGroupNoStr => "remind_group_no_str",
            Self::MsgTemplateStr => "msg_template_str",
            Self::ParamNoStr => "param_no_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageSysMsgConfig {
    const ID: &'static str = "tb_basemage_sys_msg_config";
    type Column = BasemageSysMsgConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageSysMsgConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageSysMsgConfig {}
