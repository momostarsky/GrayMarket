//! 由 `codegen` 从 `sql/schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use account::amount::Money;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `account_asset`（codegen：字段与列名源自 DDL，`AssetColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Asset {
    pub account_id: String,
    pub currency: String,
    pub available: Money,
    pub frozen: Money,
    pub total_market_value: Money,
}

/// `account_asset` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetColumn {
    AccountId,
    Currency,
    Available,
    Frozen,
    TotalMarketValue,
}

impl ColumnName for AssetColumn {
    const ALL: &'static [Self] = &[Self::AccountId, Self::Currency, Self::Available, Self::Frozen, Self::TotalMarketValue];
    fn as_str(self) -> &'static str {
        match self {
            Self::AccountId => "account_id",
            Self::Currency => "currency",
            Self::Available => "available",
            Self::Frozen => "frozen",
            Self::TotalMarketValue => "total_market_value",
        }
    }
}

impl DataTable for Asset {
    const ID: &'static str = "account_asset";
    type Column = AssetColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            AssetColumn::AccountId => Some(ColVal::Text(&self.account_id)),
            _ => None,
        }
    }
}
