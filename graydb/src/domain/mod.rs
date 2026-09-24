use account::amount::{Money, Price, Quantity};
use serde::{Deserialize, Serialize};

/// 证券字典 —— 未来表: dict_security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    pub symbol: String, // 主键, "09018"
    pub name: String,
    pub currency: String,
    pub lot_size: Quantity,
    pub price_tick: Price,
    pub market: String,
}

/// 资金账户 —— 未来表: account_asset (日终余额 = 初始 + 成交累计)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub account_id: String,
    pub currency: String,
    pub available: Money, // 可用
    pub frozen: Money,    // 冻结
    pub total_market_value: Money,
}

/// 持仓 —— 未来表: position (日终快照)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub account_id: String,
    pub symbol: String,
    pub quantity: Quantity,
    pub available_qty: Quantity,
    pub avg_cost: Price, // 摊薄成本
}

/// 委托单 —— 未来表: orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String, // 主键, 客户端幂等键
    pub account_id: String,
    pub symbol: String,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
    pub filled_qty: Quantity,
    pub status: OrderStatus,
    pub created_at: String, // ISO8601, 模拟阶段先字符串
    pub seq: u64,           // 内核全局序号, 将来 WAL 的 seq
}

/// 成交 —— 未来表: trades (JSONL 追加写)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub order_id: String,
    pub account_id: String,
    pub symbol: String,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
    pub amount: Money, // = notional(price, qty)
    pub seq: u64,
}

/// 用户主数据 —— 未来表: user_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String, // 主键
    pub username: String,
    pub phone: String,
    pub id_type: IdType,
    pub id_number: String,
    pub status: UserStatus,
    pub opened_at: String, // ISO8601
}

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

/// 账户主数据 —— 未来表: account_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: String, // 主键
    pub user_id: String,
    pub account_name: String,
    pub status: AccountStatus,
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