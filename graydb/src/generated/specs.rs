//! 由 `codegen` 从 `tables.toml` 生成的表清单 —— 请勿手改。
//! 改策略请编辑 `tables.toml` 后 `cargo codegen`。

use crate::tables::{Kind, LoadPolicy, Spec};

/// 表清单：`codegen` 产出（仅 `register = true` 的表），`tables.rs` 以 `pub use` 重导出为唯一事实源。
pub const TABLES: &[Spec] = &[
    Spec {
        id: "user_info",
        schema: None,
        file: "dict/user.json",
        kind: Kind::Dict,
        policy: LoadPolicy::Critical,
        pk: &["user_id"],
        fk: &[],
        expected_rows: Some(3),
    },
    Spec {
        id: "account_info",
        schema: None,
        file: "state/account.json",
        kind: Kind::Dict,
        policy: LoadPolicy::Critical,
        pk: &["account_id"],
        fk: &[("user_id", "user_info", "user_id"), ("account_id", "account_asset", "account_id")],
        expected_rows: Some(3),
    },
    Spec {
        id: "dict_security",
        schema: None,
        file: "dict/security.json",
        kind: Kind::Dict,
        policy: LoadPolicy::Critical,
        pk: &["symbol"],
        fk: &[],
        expected_rows: Some(2),
    },
    Spec {
        id: "account_asset",
        schema: None,
        file: "state/asset.json",
        kind: Kind::State,
        policy: LoadPolicy::Critical,
        pk: &["account_id"],
        fk: &[("account_id", "account_info", "account_id")],
        expected_rows: Some(3),
    },
    Spec {
        id: "position",
        schema: None,
        file: "state/position.json",
        kind: Kind::State,
        policy: LoadPolicy::Critical,
        pk: &["account_id", "symbol"],
        fk: &[("account_id", "account_info", "account_id"), ("symbol", "dict_security", "symbol")],
        expected_rows: Some(2),
    },
    Spec {
        id: "orders",
        schema: None,
        file: "state/order.json",
        kind: Kind::State,
        policy: LoadPolicy::Critical,
        pk: &["order_id"],
        fk: &[("account_id", "account_info", "account_id"), ("symbol", "dict_security", "symbol")],
        expected_rows: Some(0),
    },
    Spec {
        id: "trades",
        schema: None,
        file: "state/trade.json",
        kind: Kind::State,
        policy: LoadPolicy::Critical,
        pk: &["trade_id"],
        fk: &[("order_id", "orders", "order_id"), ("account_id", "account_info", "account_id"), ("symbol", "dict_security", "symbol")],
        expected_rows: Some(0),
    },
];
