//! 内存镜像：日初按 `tables::TABLES` 声明加载（将来换成 PG 快照 + LSN 锚点），
//! 运行期由单线程内核独占写、多读者读。
use std::collections::HashMap;
use std::path::Path;

use account::amount::{Money, Price, Quantity, Rounding, notional};

use crate::domain::{Account, AccountStatus, Asset, Order, PdUnitCapitTrade, Position, Security, Trade, User};
use crate::engine::RejectReason;
use crate::tables::{
    ColumnName, DataTable, FkIndex, Table, TableStat, check_registry_shape, composite_key_str,
    load_table, save_table,
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
    /// 阶段 4.5 首张转正的真实表（jzdb_prod，45 列、单主键 `row_id`）。
    /// graydb 对它是**只读**（写权在 PG），所以不进取 `save` 的可变表名单。
    pub pd_unit_capit_trades: Table<PdUnitCapitTrade>,
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
            pd_unit_capit_trades: load_table::<PdUnitCapitTrade>(root)?,
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
        if self.pd_unit_capit_trades.degraded {
            degraded.push(PdUnitCapitTrade::ID);
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
        index.register(&self.pd_unit_capit_trades);
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

    // ── 读侧按表身份取行（阶段 3.2 / 3.6）──────────────────────────
    //
    // 三个入口共用同一张分派表：出口发布取一行、快照下发扫全表、订阅校验取列名。
    // 新表登记后必须在这里加臂 —— 漏臂的后果是 `panic`，而不是给下游静默少一行／少一张表。

    /// 表身份（`Spec::id`）→ 单行 JSON。行→JSON 只走 serde 一条路：与 journal 落盘、
    /// [`Snapshot::save`] 同一份序列化实现，订阅端拿到的数字与日志里的不可能不一致。
    #[must_use]
    pub fn row_json(&self, table: &str, key: &str) -> Option<serde_json::Value> {
        match table {
            User::ID => self.users.get(key).map(encode_row),
            Account::ID => self.accounts.get(key).map(encode_row),
            Security::ID => self.securities.get(key).map(encode_row),
            Asset::ID => self.assets.get(key).map(encode_row),
            Position::ID => self.positions.get(key).map(encode_row),
            Order::ID => self.orders.get(key).map(encode_row),
            Trade::ID => self.trades.get(key).map(encode_row),
            PdUnitCapitTrade::ID => self.pd_unit_capit_trades.get(key).map(encode_row),
            other => panic!("表 {other:?} 未接入读侧取行分派（row_json 缺臂）"),
        }
    }

    /// 表身份 → 全表 `(行键, 行 JSON)`，按键排序（3.6）。
    ///
    /// 快照下发的表清单来自订阅请求（已按 `spec_of` 校验），取行只靠这一层分派，
    /// 协议代码不为新表改一行。排序是为了让下发顺序可重复、逐帧对得上。
    #[must_use]
    pub fn rows_json(&self, table: &str) -> Vec<(String, serde_json::Value)> {
        let mut rows: Vec<(String, serde_json::Value)> = match table {
            User::ID => mirror_rows(&self.users),
            Account::ID => mirror_rows(&self.accounts),
            Security::ID => mirror_rows(&self.securities),
            Asset::ID => mirror_rows(&self.assets),
            Position::ID => mirror_rows(&self.positions),
            Order::ID => mirror_rows(&self.orders),
            Trade::ID => mirror_rows(&self.trades),
            PdUnitCapitTrade::ID => mirror_rows(&self.pd_unit_capit_trades),
            other => panic!("表 {other:?} 未接入读侧取行分派（rows_json 缺臂）"),
        };
        rows.sort_by(|left, right| left.0.cmp(&right.0));
        rows
    }

    /// 某表的全部列名。`Spec` 本身不带列清单（只有 `pk`/`fk`），列名的唯一事实源
    /// 是 codegen 生成的列枚举 `Column::ALL` —— 订阅请求里的 `columns` / `filter`
    /// 靠这份清单核对，未声明的列名在 attach 时就被拒，不留到运行期。
    #[must_use]
    pub fn columns_of(table: &str) -> Vec<&'static str> {
        match table {
            User::ID => column_names::<User>(),
            Account::ID => column_names::<Account>(),
            Security::ID => column_names::<Security>(),
            Asset::ID => column_names::<Asset>(),
            Position::ID => column_names::<Position>(),
            Order::ID => column_names::<Order>(),
            Trade::ID => column_names::<Trade>(),
            PdUnitCapitTrade::ID => column_names::<PdUnitCapitTrade>(),
            other => panic!("表 {other:?} 未接入读侧取行分派（columns_of 缺臂）"),
        }
    }

    /// 日终 dump 回 JSON —— 与 `load` 对偶，路径同样取自 `Spec::file`。
    ///
    /// 只回写 `Kind::State` 且**内核会写**的四张表（资金/持仓/订单/成交）；新增可变表时在此多一行。
    /// 阶段 4.5 转正的 `tb_pdmage_pd_unit_capit_trade` 故意不在名单里：它是从 PG 读进来的
    /// 真实表，写权在库那边 —— 拿内存里那份去覆写文件，等于造出一个两边都不认账的副本。
    pub fn save(&self, data_root: impl AsRef<Path>) -> anyhow::Result<()> {
        let root = data_root.as_ref();
        save_table(&self.assets, root)?;
        save_table(&self.positions, root)?;
        save_table(&self.orders, root)?;
        save_table(&self.trades, root)?;
        Ok(())
    }
}

/// 行 → JSON：与 journal / [`Snapshot::save`] 同一份 serde 实现，出口与快照共用。
fn encode_row<T: serde::Serialize>(row: &T) -> serde_json::Value {
    serde_json::to_value(row).expect("行应可序列化 —— 与 journal/save 同一份 serde 实现")
}

/// 强类型表 → `(行键, 行 JSON)`（排序由调用方统一做）。
fn mirror_rows<T: DataTable + serde::Serialize>(table: &Table<T>) -> Vec<(String, serde_json::Value)> {
    table
        .rows()
        .iter()
        .map(|(key, row)| (key.clone(), encode_row(row)))
        .collect()
}

/// 列枚举 → 列名清单。
fn column_names<T: DataTable>() -> Vec<&'static str> {
    T::Column::ALL
        .iter()
        .copied()
        .map(|column| column.as_str())
        .collect()
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
    use crate::tables::TABLES;
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

    /// 3.6 的守护：读侧取行 / 取列分派必须覆盖**注册中心里的每一张表**。
    /// 新表登记而忘在 `Snapshot` 的三个分派里加臂，这里立刻 panic（而不是下游静默少一张表）；
    /// 顺带核对主键列都能从列名清单找到 —— 订阅请求的 `columns` / `filter` 就靠这份清单校验。
    #[test]
    fn read_side_dispatch_covers_every_registered_table() {
        let snap = snapshot();
        for spec in TABLES {
            let columns = Snapshot::columns_of(spec.id);
            assert!(!columns.is_empty(), "{} 的列名清单为空", spec.id);
            for pk in spec.pk {
                assert!(columns.contains(pk), "{} 的主键列 {pk} 不在列名清单里", spec.id);
            }

            let rows = snap.rows_json(spec.id);
            assert_eq!(
                rows.len(),
                spec.expected_rows.unwrap_or(0),
                "{} 全表扫描的行数应与注册中心一致",
                spec.id
            );
            for pair in rows.windows(2) {
                assert!(pair[0].0 <= pair[1].0, "{} 的 rows_json 未按行键排序", spec.id);
            }
            for (key, row) in &rows {
                assert_eq!(
                    snap.row_json(spec.id, key).as_ref(),
                    Some(row),
                    "{}/{} 单行取回应与全表扫描一致",
                    spec.id,
                    key
                );
                // 行 JSON 的键必须都落在列名清单里（出现未声明字段 = 类型与声明不同步）。
                let object = row.as_object().expect("行应序列化为 JSON 对象");
                for name in object.keys() {
                    assert!(
                        columns.iter().any(|column| *column == name),
                        "{} 的行里出现未声明的列 {name}",
                        spec.id
                    );
                }
            }
        }
    }
}