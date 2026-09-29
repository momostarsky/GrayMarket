//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_ctm_info`（codegen：字段与列名源自 DDL，`BasemageCtmInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageCtmInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub ctm_key: String,
    pub ctm_name: String,
    pub ctm_type: i32,
    pub ctm_flag: i32,
    pub ctm_acc: String,
    pub ctm_pwd: String,
    pub country: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub remark_info: String,
}

/// `tb_basemage_ctm_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageCtmInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    CtmKey,
    CtmName,
    CtmType,
    CtmFlag,
    CtmAcc,
    CtmPwd,
    Country,
    BusiCtrlStr,
    EnableCtrlStr,
    RemarkInfo,
}

impl ColumnName for BasemageCtmInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::CtmKey, Self::CtmName, Self::CtmType, Self::CtmFlag, Self::CtmAcc, Self::CtmPwd, Self::Country, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::CtmKey => "ctm_key",
            Self::CtmName => "ctm_name",
            Self::CtmType => "ctm_type",
            Self::CtmFlag => "ctm_flag",
            Self::CtmAcc => "ctm_acc",
            Self::CtmPwd => "ctm_pwd",
            Self::Country => "country",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageCtmInfo {
    const ID: &'static str = "tb_basemage_ctm_info";
    type Column = BasemageCtmInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageCtmInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageCtmInfo {}
