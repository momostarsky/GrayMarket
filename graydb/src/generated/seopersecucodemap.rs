//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_secu_code_map`（codegen：字段与列名源自 DDL，`SeoperSecuCodeMapColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSecuCodeMap {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub trade_code: String,
    pub target_code: String,
}

/// `tb_seoper_secu_code_map` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSecuCodeMapColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    SecuCode,
    TradeCode,
    TargetCode,
}

impl ColumnName for SeoperSecuCodeMapColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::SecuCode, Self::TradeCode, Self::TargetCode];
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
            Self::TradeCode => "trade_code",
            Self::TargetCode => "target_code",
        }
    }
}

impl DataTable for SeoperSecuCodeMap {
    const ID: &'static str = "tb_seoper_secu_code_map";
    type Column = SeoperSecuCodeMapColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSecuCodeMapColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSecuCodeMap {}
