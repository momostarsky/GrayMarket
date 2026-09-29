//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_crncy_exchcode_config`（codegen：字段与列名源自 DDL，`SeoperCrncyExchcodeConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperCrncyExchcodeConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub out_sys_no: i32,
    pub crncy_exch_code: String,
    pub crncy_type: i32,
    pub for_crncy_type: i32,
    pub remark_info: String,
}

/// `tb_seoper_crncy_exchcode_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperCrncyExchcodeConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    OutSysNo,
    CrncyExchCode,
    CrncyType,
    ForCrncyType,
    RemarkInfo,
}

impl ColumnName for SeoperCrncyExchcodeConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::OutSysNo, Self::CrncyExchCode, Self::CrncyType, Self::ForCrncyType, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::OutSysNo => "out_sys_no",
            Self::CrncyExchCode => "crncy_exch_code",
            Self::CrncyType => "crncy_type",
            Self::ForCrncyType => "for_crncy_type",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperCrncyExchcodeConfig {
    const ID: &'static str = "tb_seoper_crncy_exchcode_config";
    type Column = SeoperCrncyExchcodeConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperCrncyExchcodeConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperCrncyExchcodeConfig {}
