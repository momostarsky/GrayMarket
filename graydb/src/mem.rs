//! 内存镜像：日初按 `tables::TABLES` 声明加载（将来换成 PG 快照 + LSN 锚点），
//! 运行期由单线程内核独占写、多读者读。
use std::collections::HashMap;
use std::path::Path;

use account::amount::{Money, Price, Quantity, Rounding, notional};

use crate::domain::{Account, AccountStatus, Asset, Order, Position, Security, Trade, User};
use crate::engine::RejectReason;
use crate::tables::{
    DataTable, FkIndex, Table, TableStat, check_registry_shape, composite_key_str, load_table,
    save_table,
};

/// 一次性载入的全部主数据 —— 对应「日初加载进内存」的边界。
///
/// 字段保持强类型 `Table<T>`：内核访问零开销、IDE 可自动补全。
/// 刻意不 `impl Default` —— 默认的「五表全空」对象能骗过所有 `len()` 判断，
/// 而「漏一张表」必须靠 `load` 里的显式一行来承诺。
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub users: Table<User>,
    pub accounts: Table<Account>,
    pub securities: Table<Security>,
    /// 可变状态：内核独占写。
    pub assets: Table<Asset>,
    /// key = `composite_key(["account_id", "symbol"])`，拼法唯一。
    pub positions: Table<Position>,
    /// 内核写路径（阶段 1.1）：委托与成交。日初为空，运行期 `upsert` 写入。
    pub orders: Table<Order>,
    pub trades: Table<Trade>,
    /// 加载时的数据源标识（将来是 `lsn`，现在是文件目录）。
    pub source: String,
}

impl Snapshot {
    /// 从 `data/` 目录加载全部主数据：声明自检 → 逐表加载（内含主键机械校验与
    /// 逐行不变量）→ 外键校验 → 跳表派生不变量。
    ///
    /// 失败以 `Err` 返回，由启动流程决定处置 —— 预期行为是终止启动：
    /// 带着脏字典开盘比启动失败危险得多（与 `amount.rs` 的「不静默」原则一致）。
    pub fn load(data_root: impl AsRef<Path>) -> anyhow::Result<Self> {
        let root = data_root.as_ref();
        // 一行数据都不读，先把清单本身的矛盾（id 重复 / 外键目标非法）挡在门口。
        check_registry_shape()?;

        let snapshot = Self {
            users: load_table::<User>(root)?,
            accounts: load_table::<Account>(root)?,
            securities: load_table::<Security>(root)?,
            assets: load_table::<Asset>(root)?,
            positions: load_table::<Position>(root)?,
            orders: load_table::<Order>(root)?,
            trades: load_table::<Trade>(root)?,
            source: root.display().to_string(),
        };
        snapshot.check_integrity()?;
        snapshot.check_valuation()?;
        Ok(snapshot)
    }

    /// 加载即校验：外键全部来自 `Spec::fk` 声明，逐行不变量已在 `load_table` 内跑完。
    /// 这里只把表登记进 `FkIndex` 后触发声明式校验 —— 新表多一行 `register`，不写校验循环。
    pub fn check_integrity(&self) -> anyhow::Result<()> {
        self.fk_index().check()
    }

    /// 市值不变量：`total_market_value == Σ(quantity × avg_cost)`，逐账户聚合，差一分即 `Err`。
    ///
    /// 这类「聚合型」不变量既不属于任何单行，也不适合声明成外键，故留在 `mem.rs`。
    pub fn check_valuation(&self) -> anyhow::Result<()> {
        let mut expected: HashMap<String, Money> = HashMap::new();
        for (key, position) in self.positions.rows() {
            let value: Money = notional(
                Price::from_units(position.avg_cost.units()),
                Quantity::from_units(position.quantity.units()),
                Rounding::MidpointAwayFromZero,
            )
            .ok_or_else(|| {
                anyhow::anyhow!("持仓 {key} 市值计算溢出")
            })?;
            let entry = expected.entry(position.account_id.clone()).or_insert(Money::ZERO);
            *entry = entry
                .checked_add(value)
                .ok_or_else(|| anyhow::anyhow!("账户 {} 市值累加溢出", position.account_id))?;
        }

        for (account_id, asset) in self.assets.rows() {
            let want = expected.get(account_id).copied().unwrap_or(Money::ZERO);
            anyhow::ensure!(
                asset.total_market_value == want,
                "账户 {account_id} 市值不平: 实际 {} != 期望 {want}",
                asset.total_market_value.format_fixed(2)
            );
        }
        Ok(())
    }

    /// 0.5.5 的出口：每张声明表的行数 + 策略，供启动报告、日终对账、内存预算复用。
    #[must_use]
    pub fn stats(&self) -> Vec<TableStat> {
        self.fk_index().stats()
    }

    /// 分级启动的结果：哪些 `Optional` 表加载失败被降级成空表。
    #[must_use]
    pub fn degraded_tables(&self) -> Vec<&'static str> {
        let mut degraded = Vec::new();
        if self.users.degraded {
            degraded.push(User::ID);
        }
        if self.accounts.degraded {
            degraded.push(Account::ID);
        }
        if self.securities.degraded {
            degraded.push(Security::ID);
        }
        if self.assets.degraded {
            degraded.push(Asset::ID);
        }
        if self.positions.degraded {
            degraded.push(Position::ID);
        }
        if self.orders.degraded {
            degraded.push(Order::ID);
        }
        if self.trades.degraded {
            degraded.push(Trade::ID);
        }
        degraded
    }

    /// 启动报告：把 `TABLES` 声明与实载行数平铺一行一张，降级表显式标记。
    pub fn print_startup_report(&self) {
        println!("GrayDB 启动报告 (source = {})", self.source);
        let degraded = self.degraded_tables();
        for stat in self.stats() {
            let mark = if degraded.contains(&stat.id) {
                "  [降级]"
            } else {
                ""
            };
            println!(
                "  {:<16} rows={:<3} policy={:?}{mark}",
                stat.id, stat.rows, stat.policy
            );
        }
    }

    fn fk_index(&self) -> FkIndex {
        let mut index = FkIndex::new();
        index.register(&self.users);
        index.register(&self.accounts);
        index.register(&self.securities);
        index.register(&self.assets);
        index.register(&self.positions);
        index.register(&self.orders);
        index.register(&self.trades);
        index
    }

    #[must_use]
    pub fn asset(&self, account_id: &str) -> Option<&Asset> {
        self.assets.get(account_id)
    }

    #[must_use]
    pub fn position(&self, account_id: &str, symbol: &str) -> Option<&Position> {
        self.positions.get(&composite_key_str(&[account_id, symbol]))
    }

    #[must_use]
    pub fn security(&self, symbol: &str) -> Option<&Security> {
        self.securities.get(symbol)
    }

    #[must_use]
    pub fn is_tradable(&self, account_id: &str, symbol: &str) -> bool {
        let account_active = self
            .accounts
            .get(account_id)
            .is_some_and(|account| account.status == AccountStatus::Active);
        account_active && self.securities.contains_key(symbol)
    }

    /// 日终 dump 回 JSON —— 与 `load` 对偶，路径同样取自 `Spec::file`。
    ///
    /// 只回写 `Kind::State` 的四张表（资金/持仓/订单/成交）；新增可变表时在此多一行。
    pub fn save(&self, data_root: impl AsRef<Path>) -> anyhow::Result<()> {
        let root = data_root.as_ref();
        save_table(&self.assets, root)?;
        save_table(&self.positions, root)?;
        save_table(&self.orders, root)?;
        save_table(&self.trades, root)?;
        Ok(())
    }
}

/// 资金变动的一切入口都走这里，保证 `available + frozen` 守恒。
///
/// 阶段 1.2：返回值从 `bool` 升级为 `Result<(), RejectReason>` —— 原来的 `false` 把
/// 「可用不足」和「冻结账目被写坏」压成同一个信号，调用方无从区分该拒单还是该停内核。
/// 两个分支都保证零状态变化：先算出两个新值，全部算式成立才一次性写回。
///
/// 为什么要额外过一道 `is_negative`：`Amount::checked_sub` 只挡算术溢出，
/// 减成负数是 `Some(负值)` —— 金额层允许负（盈亏需要），余额语义层不允许。
pub fn try_freeze(asset: &mut Asset, amount: Money) -> Result<(), RejectReason> {
    let frozen = asset
        .frozen
        .checked_add(amount)
        .ok_or(RejectReason::InsufficientFunds)?;
    let available = asset
        .available
        .checked_sub(amount)
        .filter(|value| !value.is_negative())
        .ok_or(RejectReason::InsufficientFunds)?;
    asset.frozen = frozen;
    asset.available = available;
    Ok(())
}

/// 冻结转已用（成交扣款）：冻结金额减少，成交额离开账户体系。
/// 冻结额不够说明上游按限价冻结算错了 —— 属状态缺陷，不是市场原因，返回 `FrozenUnderflow`。
pub fn settle_frozen_out(asset: &mut Asset, amount: Money) -> Result<(), RejectReason> {
    let frozen = asset
        .frozen
        .checked_sub(amount)
        .filter(|value| !value.is_negative())
        .ok_or(RejectReason::FrozenUnderflow)?;
    asset.frozen = frozen;
    Ok(())
}

/// 解冻回可用（撤单/废单/成交价优于限价）。任一算式失败都意味着冻结账目不平。
pub fn unfreeze(asset: &mut Asset, amount: Money) -> Result<(), RejectReason> {
    let frozen = asset
        .frozen
        .checked_sub(amount)
        .filter(|value| !value.is_negative())
        .ok_or(RejectReason::FrozenUnderflow)?;
    let available = asset
        .available
        .checked_add(amount)
        .ok_or(RejectReason::FrozenUnderflow)?;
    asset.frozen = frozen;
    asset.available = available;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn snapshot() -> Snapshot {
        Snapshot::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("data")).unwrap()
    }

    #[test]
    fn loads_all_mock_json_into_memory() {
        let snap = snapshot();
        assert_eq!(snap.users.len(), 3);
        assert_eq!(snap.accounts.len(), 3);
        assert_eq!(snap.securities.len(), 2);
        assert_eq!(snap.assets.len(), 3);
        assert_eq!(snap.positions.len(), 2);
        // 阶段 1 的两张流水表：日初为空（`expected_rows = 0`），只能由内核写入。
        assert_eq!(snap.orders.len(), 0);
        assert_eq!(snap.trades.len(), 0);
        assert!(snap.check_integrity().is_ok());
    }

    #[test]
    fn tradability_respects_account_and_dict() {
        let snap = snapshot();
        assert!(snap.is_tradable("A001", "09018"));
        // A003 账户被冻结
        assert!(!snap.is_tradable("A003", "09018"));
        // 未知证券
        assert!(!snap.is_tradable("A001", "999999"));
        // 未知账户
        assert!(!snap.is_tradable("A404", "09018"));
    }

    #[test]
    fn freeze_conserves_available_plus_frozen() {
        let snap = snapshot();
        let mut asset = snap.asset("A001").unwrap().clone();
        let before = asset.available.checked_add(asset.frozen).unwrap();

        // 冻结 7202.00 元
        assert!(try_freeze(&mut asset, Money::from_units(720_200)).is_ok());
        assert_eq!(asset.available.checked_add(asset.frozen).unwrap(), before);

        // 超额冻结被拒，且零状态变化（1.2：结构化原因取代丢信息的 `false`）
        let mut oversized = asset.clone();
        assert_eq!(
            try_freeze(&mut oversized, Money::from_units(i64::MAX)),
            Err(RejectReason::InsufficientFunds)
        );
        assert_eq!(oversized.available, asset.available);
        assert_eq!(oversized.frozen, asset.frozen);

        // 解冻回原额；解冻多于冻结额是账目不平，不是市场原因
        assert!(unfreeze(&mut asset, Money::from_units(720_200)).is_ok());
        assert_eq!(asset.available.checked_add(asset.frozen).unwrap(), before);
        assert!(asset.frozen.is_zero());
        assert_eq!(
            settle_frozen_out(&mut asset, Money::from_units(1)),
            Err(RejectReason::FrozenUnderflow),
            "冻结已清空，再扣一分都是账目不平（`checked_sub` 允许负值，这里必须拦）"
        );
        assert_eq!(
            unfreeze(&mut asset, Money::from_units(1)),
            Err(RejectReason::FrozenUnderflow)
        );
        assert!(asset.frozen.is_zero() && asset.available.checked_add(asset.frozen).unwrap() == before);
    }

    /// 主键拼法唯一：`position_key` 已并入 `tables::composite_key`，与 `Spec::pk` 共用一处定义。
    #[test]
    fn composite_key_is_stable_and_matches_json_layout() {
        assert_eq!(composite_key_str(&["A001", "09018"]), "A001:09018");
        let snap = snapshot();
        assert!(snap.position("A001", "09018").is_some());
        assert!(snap.position("A001", "600000").is_none());
    }
}