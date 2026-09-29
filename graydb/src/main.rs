pub mod journal;
pub mod domain;
pub mod mem;
pub mod tables;

use account::amount::{Money, Price, Quantity, Rounding, notional};

fn main() {
    println!("Hello, GrayDB!");

    let qty = Quantity::from_decimal(rust_decimal::dec!(777)).unwrap();
    let price = Price::from_decimal(rust_decimal::dec!(10.0015)).unwrap();
    let amount: Money = notional(price, qty, Rounding::MidpointAwayFromZero).unwrap();
    println!("notional = {}", amount.format_fixed(4));

    // 阶段 0.5：日初加载 = 按 `TABLES` 声明逐项校验。
    // 失败直接终止启动 —— 带脏字典开盘比启动失败危险得多。
    let data_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let snapshot = mem::Snapshot::load(data_root).expect("主数据加载/校验失败，终止启动");
    snapshot.print_startup_report();
}

#[cfg(test)]
mod mock_data_tests {
    use crate::mem::Snapshot;
    use std::collections::HashMap;
    use std::path::Path;

    fn snapshot() -> Snapshot {
        Snapshot::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("data")).unwrap()
    }

    /// 五份 JSON → 领域结构 → 声明式校验全跑一遍。
    ///
    /// 这里原先手写的两段外键 `for` 循环已删：语义等价地搬进了 `TABLES` 的 `fk` 声明
    /// （`account_info.user_id → user_info`、`position.symbol → dict_security` 等），
    /// 由 `Snapshot::load` 里的 `FkIndex::check` 统一承担；复合主键拼法则由
    /// `load_table` 的「JSON 外层键 == 复合主键」机械校验兜住，写错根本加载不进来。
    #[test]
    fn all_mock_json_files_match_domain_structs() {
        let snap = snapshot();

        assert_eq!(snap.users.len(), 3);
        assert_eq!(snap.accounts.len(), 3);
        assert_eq!(snap.securities.len(), 2);
        assert_eq!(snap.assets.len(), 3);
        assert_eq!(snap.positions.len(), 2);

        snap.check_integrity().unwrap();

        // 复合主键约定不能写错：外层键就是 `account_id:symbol`。
        assert!(snap.position("A001", "09018").is_some());
        assert!(snap.positions.rows().contains_key("A002:600000"));
    }

    /// 逐账户独立复算市值，与 `Asset` 里落盘的 `total_market_value` 对比，差一分即失败。
    ///
    /// 刻意不调 `Snapshot::check_valuation`：测试自己算一遍，才能当住「生产实现被改错」。
    #[test]
    fn asset_invariants_hold() {
        use account::amount::{Money, Price, Quantity, Rounding, notional};
        let snap = snapshot();

        let mut expected_mv: HashMap<String, Money> = HashMap::new();
        for position in snap.positions.rows().values() {
            let value: Money = notional(
                Price::from_units(position.avg_cost.units()),
                Quantity::from_units(position.quantity.units()),
                Rounding::MidpointAwayFromZero,
            )
            .unwrap();
            let entry = expected_mv
                .entry(position.account_id.clone())
                .or_insert(Money::ZERO);
            *entry = entry.checked_add(value).unwrap();
        }
        for (account_id, asset) in snap.assets.rows() {
            let expected = expected_mv.get(account_id).copied().unwrap_or(Money::ZERO);
            assert_eq!(asset.total_market_value, expected, "账户 {account_id} 市值不平");
        }
    }
}