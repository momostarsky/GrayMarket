//! 由 `codegen` 从 `sql/schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use account::amount::{Price, Quantity};
use crate::domain::{OrderStatus, Side};
use crate::tables::{ColVal, ColumnName, DataTable};

/// `orders`（codegen：字段与列名源自 DDL，`OrderColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Order {
    pub order_id: String,
    pub account_id: String,
    pub symbol: String,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
    pub filled_qty: Quantity,
    pub status: OrderStatus,
    pub created_at: String,
    pub seq: i64,
}

/// `orders` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderColumn {
    OrderId,
    AccountId,
    Symbol,
    Side,
    Price,
    Quantity,
    FilledQty,
    Status,
    CreatedAt,
    Seq,
}

impl ColumnName for OrderColumn {
    const ALL: &'static [Self] = &[Self::OrderId, Self::AccountId, Self::Symbol, Self::Side, Self::Price, Self::Quantity, Self::FilledQty, Self::Status, Self::CreatedAt, Self::Seq];
    fn as_str(self) -> &'static str {
        match self {
            Self::OrderId => "order_id",
            Self::AccountId => "account_id",
            Self::Symbol => "symbol",
            Self::Side => "side",
            Self::Price => "price",
            Self::Quantity => "quantity",
            Self::FilledQty => "filled_qty",
            Self::Status => "status",
            Self::CreatedAt => "created_at",
            Self::Seq => "seq",
        }
    }
}

impl DataTable for Order {
    const ID: &'static str = "orders";
    type Column = OrderColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            OrderColumn::OrderId => Some(ColVal::Text(&self.order_id)),
            OrderColumn::AccountId => Some(ColVal::Text(&self.account_id)),
            OrderColumn::Symbol => Some(ColVal::Text(&self.symbol)),
            _ => None,
        }
    }
}
