//! 领域层：codegen 产物的重导出 + 业务枚举 + 人写的单行不变量。
//!
//! 分工（阶段 0.7）：
//! - `struct` 字段、`XxxColumn` 列枚举、`impl DataTable` 由 `codegen` 从
//!   `sql/schema.sql` + `tables.toml` 生成（见 `crate::generated`），本模块 `pub use` 转出，
//!   保持 `crate::domain::Security` 等旧路径不变；
//! - 业务枚举（`Side` / `IdType` / …）表达语义、不由 DDL 物理类型推出，留此手写；
//! - 单行不变量写在 `impl RowValidator` 里 —— `check_row` 默认转调它，与生成的
//!   `impl DataTable` 各占一块，重跑 codegen 不覆盖业务逻辑。

use crate::tables::RowValidator;
use serde::{Deserialize, Serialize};

// codegen 生成的表类型与列枚举：从 `crate::generated` 转出，对外路径仍是 `crate::domain::*`。
pub use crate::generated::{
    Account, AccountColumn, Asset, AssetColumn, Order, OrderColumn, Position, PositionColumn,
    Security, SecurityColumn, Trade, TradeColumn, User, UserColumn,
};

// ---------------------------------------------------------------------------
// 业务枚举（非表结构，DDL 推不出语义，故手写；`numeric`→Amount 的映射在 tables.toml）
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdType {
    IdCard,
    Passport,
    BusinessLicense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Frozen,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

// ---------------------------------------------------------------------------
// 单行不变量（人写，跨列/业务约束）：`DataTable::check_row` 默认转调 `validate_row`。
// 跨表不变量由 `Spec::fk` 声明 + `FkIndex` 承担；聚合不变量（市值守恒）留在 `mem.rs`。
// ---------------------------------------------------------------------------

/// 订单：数量/价格为正，部分成交不得超过委托量（成交不变量的单行部分）。
impl RowValidator for Order {
    fn validate_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.order_id.is_empty(), "order_id 不能为空（幂等键）");
        anyhow::ensure!(self.quantity.is_positive(), "订单 {} 委托数量必须为正", self.order_id);
        anyhow::ensure!(self.price.is_positive(), "订单 {} 价格必须为正", self.order_id);
        anyhow::ensure!(
            self.filled_qty <= self.quantity,
            "订单 {} 已成交量 {} 超过委托量 {}",
            self.order_id,
            self.filled_qty,
            self.quantity
        );
        Ok(())
    }
}

/// 成交：数量与金额均为正（金额由 `notional` 算出，不合法即说明上游算错）。
impl RowValidator for Trade {
    fn validate_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(self.quantity.is_positive(), "成交 {} 数量必须为正", self.trade_id);
        anyhow::ensure!(self.price.is_positive(), "成交 {} 价格必须为正", self.trade_id);
        anyhow::ensure!(!self.amount.is_negative(), "成交 {} 金额为负", self.trade_id);
        Ok(())
    }
}

impl RowValidator for User {
    fn validate_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.user_id.is_empty(), "user_id 不能为空");
        anyhow::ensure!(!self.username.trim().is_empty(), "用户 {} 缺少姓名", self.user_id);
        Ok(())
    }
}

impl RowValidator for Account {
    fn validate_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.account_id.is_empty(), "account_id 不能为空");
        anyhow::ensure!(
            !self.user_id.is_empty(),
            "账户 {} 未归属任何用户",
            self.account_id
        );
        Ok(())
    }
}

impl RowValidator for Security {
    fn validate_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.lot_size.units() > 0,
            "证券 {} 的 lot_size 必须为正（手数 0 会造成下单不整手）",
            self.symbol
        );
        anyhow::ensure!(
            self.price_tick.units() > 0,
            "证券 {} 的 price_tick 必须为正",
            self.symbol
        );
        Ok(())
    }
}

impl RowValidator for Asset {
    /// 资金非负是原 `check_integrity` 的手写循环，现在是单行不变量。
    /// `available + frozen` 守恒由 `mem::try_freeze` 等原语保证，不在此重复。
    fn validate_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            !self.available.is_negative() && !self.frozen.is_negative(),
            "账户 {} 资金为负",
            self.account_id
        );
        anyhow::ensure!(
            !self.total_market_value.is_negative(),
            "账户 {} 市值为负",
            self.account_id
        );
        Ok(())
    }
}

impl RowValidator for Position {
    fn validate_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.quantity >= self.available_qty,
            "持仓 {} 可卖数量大于总数量",
            self.symbol
        );
        anyhow::ensure!(!self.quantity.is_negative(), "持仓 {} 数量为负", self.symbol);
        Ok(())
    }
}
