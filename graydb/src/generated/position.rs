//! 由 `codegen` 从 `sql/schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use account::amount::{Price, Quantity};
use crate::tables::{ColVal, ColumnName, DataTable};

/// `position`（codegen：字段与列名源自 DDL，`PositionColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub account_id: String,
    pub symbol: String,
    pub quantity: Quantity,
    pub available_qty: Quantity,
    pub avg_cost: Price,
}

/// `position` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionColumn {
    AccountId,
    Symbol,
    Quantity,
    AvailableQty,
    AvgCost,
}

impl ColumnName for PositionColumn {
    const ALL: &'static [Self] = &[Self::AccountId, Self::Symbol, Self::Quantity, Self::AvailableQty, Self::AvgCost];
    fn as_str(self) -> &'static str {
        match self {
            Self::AccountId => "account_id",
            Self::Symbol => "symbol",
            Self::Quantity => "quantity",
            Self::AvailableQty => "available_qty",
            Self::AvgCost => "avg_cost",
        }
    }
}

impl DataTable for Position {
    const ID: &'static str = "position";
    type Column = PositionColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PositionColumn::AccountId => Some(ColVal::Text(&self.account_id)),
            PositionColumn::Symbol => Some(ColVal::Text(&self.symbol)),
            _ => None,
        }
    }
}
