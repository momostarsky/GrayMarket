//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_margin_underly`（codegen：字段与列名源自 DDL，`SeoperMarginUnderlyColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperMarginUnderly {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub fina_status: i32,
    pub loan_status: i32,
}

/// `tb_seoper_margin_underly` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperMarginUnderlyColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    SecuCode,
    FinaStatus,
    LoanStatus,
}

impl ColumnName for SeoperMarginUnderlyColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::SecuCode, Self::FinaStatus, Self::LoanStatus];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::FinaStatus => "fina_status",
            Self::LoanStatus => "loan_status",
        }
    }
}

impl DataTable for SeoperMarginUnderly {
    const ID: &'static str = "tb_seoper_margin_underly";
    type Column = SeoperMarginUnderlyColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperMarginUnderlyColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperMarginUnderly {}
