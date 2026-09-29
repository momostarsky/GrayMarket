//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_margin_offset_secu`（codegen：字段与列名源自 DDL，`SeoperMarginOffsetSecuColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperMarginOffsetSecu {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub channel_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub mortgage_ratio: Decimal,
}

/// `tb_seoper_margin_offset_secu` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperMarginOffsetSecuColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ChannelNo,
    ExchNo,
    SecuCode,
    MortgageRatio,
}

impl ColumnName for SeoperMarginOffsetSecuColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ChannelNo, Self::ExchNo, Self::SecuCode, Self::MortgageRatio];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ChannelNo => "channel_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::MortgageRatio => "mortgage_ratio",
        }
    }
}

impl DataTable for SeoperMarginOffsetSecu {
    const ID: &'static str = "tb_seoper_margin_offset_secu";
    type Column = SeoperMarginOffsetSecuColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperMarginOffsetSecuColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperMarginOffsetSecu {}
