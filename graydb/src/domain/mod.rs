use account::amount::{Money, Price, Quantity};
use crate::tables::{ColVal, DataTable};
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

// ---------------------------------------------------------------------------
// 表声明绑定（阶段 0.5）
//
// 每个 `impl DataTable` 只回答两件事：我是哪张表、我这一行自己合法吗（不跨表）。
// 列取值经 `column` 暴露（文本列 `ColVal::Text` 零拷贝），只需覆盖 `Spec::pk`
// 与被外键引用的列，其余列一律 `None` —— 它在冷路径，不进撮合。
// 主键拼法由 `Spec::pk` 单方定义，不再在类型侧手写 `pk_parts`。
// ---------------------------------------------------------------------------

impl DataTable for User {
    const ID: &'static str = "user_info";

    fn column(&self, name: &'static str) -> Option<ColVal<'_>> {
        match name {
            "user_id" => Some(ColVal::Text(&self.user_id)),
            _ => None,
        }
    }

    fn check_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.user_id.is_empty(), "user_id 不能为空");
        anyhow::ensure!(!self.username.trim().is_empty(), "用户 {} 缺少姓名", self.user_id);
        Ok(())
    }
}

impl DataTable for Account {
    const ID: &'static str = "account_info";

    fn column(&self, name: &'static str) -> Option<ColVal<'_>> {
        match name {
            "account_id" => Some(ColVal::Text(&self.account_id)),
            "user_id" => Some(ColVal::Text(&self.user_id)),
            _ => None,
        }
    }

    fn check_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.account_id.is_empty(), "account_id 不能为空");
        anyhow::ensure!(
            !self.user_id.is_empty(),
            "账户 {} 未归属任何用户",
            self.account_id
        );
        Ok(())
    }
}

impl DataTable for Security {
    const ID: &'static str = "dict_security";

    fn column(&self, name: &'static str) -> Option<ColVal<'_>> {
        match name {
            "symbol" => Some(ColVal::Text(&self.symbol)),
            _ => None,
        }
    }

    fn check_row(&self) -> anyhow::Result<()> {
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

impl DataTable for Asset {
    const ID: &'static str = "account_asset";

    fn column(&self, name: &'static str) -> Option<ColVal<'_>> {
        match name {
            "account_id" => Some(ColVal::Text(&self.account_id)),
            _ => None,
        }
    }

    /// 资金非负是原 `check_integrity` 的手写循环，现在是单行不变量。
    /// `available + frozen` 守恒由 `mem::try_freeze` 等原语保证，不在此重复。
    fn check_row(&self) -> anyhow::Result<()> {
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

impl DataTable for Position {
    const ID: &'static str = "position";

    fn column(&self, name: &'static str) -> Option<ColVal<'_>> {
        match name {
            "account_id" => Some(ColVal::Text(&self.account_id)),
            "symbol" => Some(ColVal::Text(&self.symbol)),
            _ => None,
        }
    }

    fn check_row(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.quantity >= self.available_qty,
            "持仓 {} 可卖数量大于总数量",
            self.symbol
        );
        anyhow::ensure!(!self.quantity.is_negative(), "持仓 {} 数量为负", self.symbol);
        Ok(())
    }
}