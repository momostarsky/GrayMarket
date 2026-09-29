//! 由 `codegen` 从 `sql/schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use account::amount::{Price, Quantity};
use crate::tables::{ColVal, ColumnName, DataTable};

/// `dict_security`（codegen：字段与列名源自 DDL，`SecurityColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Security {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub lot_size: Quantity,
    pub price_tick: Price,
    pub market: String,
}

/// `dict_security` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityColumn {
    Symbol,
    Name,
    Currency,
    LotSize,
    PriceTick,
    Market,
}

impl ColumnName for SecurityColumn {
    const ALL: &'static [Self] = &[Self::Symbol, Self::Name, Self::Currency, Self::LotSize, Self::PriceTick, Self::Market];
    fn as_str(self) -> &'static str {
        match self {
            Self::Symbol => "symbol",
            Self::Name => "name",
            Self::Currency => "currency",
            Self::LotSize => "lot_size",
            Self::PriceTick => "price_tick",
            Self::Market => "market",
        }
    }
}

impl DataTable for Security {
    const ID: &'static str = "dict_security";
    type Column = SecurityColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SecurityColumn::Symbol => Some(ColVal::Text(&self.symbol)),
            _ => None,
        }
    }
}
