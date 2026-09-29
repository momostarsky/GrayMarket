//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_sys_msg_jour`（codegen：字段与列名源自 DDL，`BasemageSysMsgJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageSysMsgJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub msg_jour_no: i64,
    pub msg_date: i32,
    pub msg_time: i32,
    pub msg_config_no: i32,
    pub msg_level: i32,
    pub opor_no: i32,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub msg_content: String,
    pub param_value_str: String,
    pub expire_date: i32,
    pub remark_info: String,
}

/// `tb_basemage_sys_msg_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageSysMsgJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    MsgJourNo,
    MsgDate,
    MsgTime,
    MsgConfigNo,
    MsgLevel,
    OporNo,
    PdNo,
    PdUnitNo,
    AsacNo,
    ExchNo,
    SecuCode,
    MsgContent,
    ParamValueStr,
    ExpireDate,
    RemarkInfo,
}

impl ColumnName for BasemageSysMsgJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::MsgJourNo, Self::MsgDate, Self::MsgTime, Self::MsgConfigNo, Self::MsgLevel, Self::OporNo, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::MsgContent, Self::ParamValueStr, Self::ExpireDate, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::MsgJourNo => "msg_jour_no",
            Self::MsgDate => "msg_date",
            Self::MsgTime => "msg_time",
            Self::MsgConfigNo => "msg_config_no",
            Self::MsgLevel => "msg_level",
            Self::OporNo => "opor_no",
            Self::PdNo => "pd_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::MsgContent => "msg_content",
            Self::ParamValueStr => "param_value_str",
            Self::ExpireDate => "expire_date",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageSysMsgJour {
    const ID: &'static str = "tb_basemage_sys_msg_jour";
    type Column = BasemageSysMsgJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageSysMsgJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageSysMsgJour {}
