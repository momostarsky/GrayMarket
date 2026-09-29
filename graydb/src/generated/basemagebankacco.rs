//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_bank_acco`（codegen：字段与列名源自 DDL，`BasemageBankAccoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageBankAcco {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub bank_acco_no: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub bank_code: String,
    pub bank_name: String,
    pub bank_acco: String,
    pub remark_info: String,
}

/// `tb_basemage_bank_acco` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageBankAccoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    BankAccoNo,
    CoNo,
    PdNo,
    BankCode,
    BankName,
    BankAcco,
    RemarkInfo,
}

impl ColumnName for BasemageBankAccoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::BankAccoNo, Self::CoNo, Self::PdNo, Self::BankCode, Self::BankName, Self::BankAcco, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::BankAccoNo => "bank_acco_no",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::BankCode => "bank_code",
            Self::BankName => "bank_name",
            Self::BankAcco => "bank_acco",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageBankAcco {
    const ID: &'static str = "tb_basemage_bank_acco";
    type Column = BasemageBankAccoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageBankAccoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageBankAcco {}
