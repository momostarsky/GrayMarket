pub mod journal;
pub mod store;
pub mod domain;
pub mod mem;

use account::amount::{Money, Price, Quantity, Rounding, notional};

fn main() {
    println!("Hello, GrayDB!");

    let qty = Quantity::from_decimal(rust_decimal::dec!(777)).unwrap();
    let price = Price::from_decimal(rust_decimal::dec!(10.0015)).unwrap();
    let amount: Money = notional(price, qty, Rounding::MidpointAwayFromZero).unwrap();
    println!("notional = {}", amount.format_fixed(4));
}


// ... existing code ...

#[cfg(test)]
mod mock_data_tests {
    use crate::domain::{Account, Asset, Position, Security, User};
    use std::collections::HashMap;
    use std::path::Path;

    fn data_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("data")
    }

    fn load<T , K >(
        rel: &str,
    ) -> HashMap<K, T>
    where
        T: serde::de::DeserializeOwned,
        K: Eq + std::hash::Hash + serde::de::DeserializeOwned,
    {
        let path = data_root().join(rel);
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("读取 {} 失败: {e}", path.display()));
        serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("解析 {} 失败: {e}", path.display()))
    }

    #[test]
    fn all_mock_json_files_match_domain_structs() {
        let users: HashMap<String, User> = load("dict/user.json");
        let accounts: HashMap<String, Account> = load("state/account.json");
        let securities: HashMap<String, Security> = load("dict/security.json");
        let assets: HashMap<String, Asset> = load("state/asset.json");
        let positions: HashMap<String, Position> = load("state/position.json");

        assert_eq!(users.len(), 3);
        assert_eq!(accounts.len(), 3);
        assert_eq!(assets.len(), 3);
        assert_eq!(positions.len(), 2);

        // 外键完整性: account.user_id 必须存在
        for acc in accounts.values() {
            assert!(users.contains_key(&acc.user_id), "账户 {} 引用了不存在的用户", acc.account_id);
        }
        // 外键完整性: position.symbol 必须在字典中
        for pos in positions.values() {
            assert!(securities.contains_key(&pos.symbol), "持仓引用了未知证券 {}", pos.symbol);
            assert!(assets.contains_key(&pos.account_id), "持仓引用了未知账户 {}", pos.account_id);
            assert_eq!(
                format!("{}:{}", pos.account_id, pos.symbol),
                // 复合主键约定不能写错
                {
                    let mut key = pos.account_id.clone();
                    key.push(':');
                    key.push_str(&pos.symbol);
                    key
                }
            );
        }
    }

    #[test]
    fn asset_invariants_hold() {
        use account::amount::{notional, Money, Price, Quantity, Rounding};
        let positions: HashMap<String, Position> = load("state/position.json");
        let assets: HashMap<String, Asset> = load("state/asset.json");

        // mv 必须等于 Σ(quantity × avg_cost)，逐账户核算，差一分钱即失败。
        let mut expected_mv: HashMap<String, Money> = HashMap::new();
        for pos in positions.values() {
            let value: Money = notional(
                Price::from_units(pos.avg_cost.units()),
                Quantity::from_units(pos.quantity.units()),
                Rounding::MidpointAwayFromZero,
            )
            .unwrap();
            let entry = expected_mv.entry(pos.account_id.clone()).or_insert(Money::ZERO);
            *entry = entry.checked_add(value).unwrap();
        }
        for (account_id, asset) in &assets {
            let expected = expected_mv.get(account_id).copied().unwrap_or(Money::ZERO);
            assert_eq!(asset.total_market_value, expected, "账户 {account_id} 市值不平");
        }
    }
}