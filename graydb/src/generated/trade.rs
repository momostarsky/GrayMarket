//! 由 `codegen` 从 `sql/schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use account::amount::{Money, Price, Quantity};
use crate::domain::Side;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `trades`（codegen：字段与列名源自 DDL，`TradeColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub order_id: String,
    pub account_id: String,
    pub symbol: String,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
    pub amount: Money,
    pub seq: i64,
}

/// `trades` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeColumn {
    TradeId,
    OrderId,
    AccountId,
    Symbol,
    Side,
    Price,
    Quantity,
    Amount,
    Seq,
}

impl ColumnName for TradeColumn {
    const ALL: &'static [Self] = &[Self::TradeId, Self::OrderId, Self::AccountId, Self::Symbol, Self::Side, Self::Price, Self::Quantity, Self::Amount, Self::Seq];
    fn as_str(self) -> &'static str {
        match self {
            Self::TradeId => "trade_id",
            Self::OrderId => "order_id",
            Self::AccountId => "account_id",
            Self::Symbol => "symbol",
            Self::Side => "side",
            Self::Price => "price",
            Self::Quantity => "quantity",
            Self::Amount => "amount",
            Self::Seq => "seq",
        }
    }
}

impl DataTable for Trade {
    const ID: &'static str = "trades";
    type Column = TradeColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            TradeColumn::TradeId => Some(ColVal::Text(&self.trade_id)),
            TradeColumn::OrderId => Some(ColVal::Text(&self.order_id)),
            TradeColumn::AccountId => Some(ColVal::Text(&self.account_id)),
            TradeColumn::Symbol => Some(ColVal::Text(&self.symbol)),
            _ => None,
        }
    }
}
