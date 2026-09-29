//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdeva_sys_sub_busi_flag`（codegen：字段与列名源自 DDL，`PdevaSysSubBusiFlagColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdevaSysSubBusiFlag {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub busi_flag: i32,
    pub invest_area: i32,
    pub subject_code: String,
    pub calc_dir: i32,
}

/// `tb_pdeva_sys_sub_busi_flag` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdevaSysSubBusiFlagColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    BusiFlag,
    InvestArea,
    SubjectCode,
    CalcDir,
}

impl ColumnName for PdevaSysSubBusiFlagColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::BusiFlag, Self::InvestArea, Self::SubjectCode, Self::CalcDir];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::BusiFlag => "busi_flag",
            Self::InvestArea => "invest_area",
            Self::SubjectCode => "subject_code",
            Self::CalcDir => "calc_dir",
        }
    }
}

impl DataTable for PdevaSysSubBusiFlag {
    const ID: &'static str = "tb_pdeva_sys_sub_busi_flag";
    type Column = PdevaSysSubBusiFlagColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdevaSysSubBusiFlagColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdevaSysSubBusiFlag {}
