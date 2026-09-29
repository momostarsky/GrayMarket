//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_ex_time`（codegen：字段与列名源自 DDL，`SeoperExTimeColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperExTime {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub exch_sub_type: i32,
    pub trd_time_frame: i32,
    pub begin_time: i32,
    pub end_time: i32,
    pub allow_withdrw_flag: i32,
    pub remark_info: String,
}

/// `tb_seoper_ex_time` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperExTimeColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    ExchSubType,
    TrdTimeFrame,
    BeginTime,
    EndTime,
    AllowWithdrwFlag,
    RemarkInfo,
}

impl ColumnName for SeoperExTimeColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::ExchSubType, Self::TrdTimeFrame, Self::BeginTime, Self::EndTime, Self::AllowWithdrwFlag, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::ExchSubType => "exch_sub_type",
            Self::TrdTimeFrame => "trd_time_frame",
            Self::BeginTime => "begin_time",
            Self::EndTime => "end_time",
            Self::AllowWithdrwFlag => "allow_withdrw_flag",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperExTime {
    const ID: &'static str = "tb_seoper_ex_time";
    type Column = SeoperExTimeColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperExTimeColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperExTime {}
