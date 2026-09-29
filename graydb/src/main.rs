pub mod journal;
pub mod domain;
pub mod generated;
pub mod mem;
pub mod tables;

use std::path::Path;

use account::amount::{Money, Price, Quantity, Rounding, notional};

use crate::domain::Security;
use crate::mem::Snapshot;
use crate::tables::{ColumnName, DataTable, composite_key_str};

fn main() {
    println!("Hello, GrayDB!");

    let qty = Quantity::from_decimal(rust_decimal::dec!(777)).unwrap();
    let price = Price::from_decimal(rust_decimal::dec!(10.0015)).unwrap();
    let amount: Money = notional(price, qty, Rounding::MidpointAwayFromZero).unwrap();
    println!("notional = {}", amount.format_fixed(4));

    // 阶段 0.5：日初加载 = 按 `TABLES` 声明逐项校验。
    // 失败直接终止启动 —— 带脏字典开盘比启动失败危险得多。
    let data_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let snapshot = Snapshot::load(&data_root).expect("主数据加载/校验失败，终止启动");
    snapshot.print_startup_report();

    // 下面两个函数是「如何用 tables 加载 / 遍历」的可运行样板。
    demo_load(&data_root);
    demo_iterate(&snapshot);
}

/// 加载：三种粒度按需选。
fn demo_load(root: &Path) {
    println!("\n[1] 加载的三种粒度");

    // ① 整份镜像（生产路径）：`Snapshot::load(root)` —— 声明自检 + 逐表加载
    //    + 外键校验 + 聚合不变量，内部对每张表就是下面 ② 那一句（见 `mem.rs::load`）。
    println!("  ① Snapshot::load → 5 张表全部就位（见上方启动报告）");

    // ② 单张表：只关心一类数据时用 `load_table::<T>()`。路径不用手写 ——
    //    由 `T::ID` 反查 `Spec::file`，校验逻辑与 ① 完全一致（不是另一套轻量版本）。
    let securities = tables::load_table::<Security>(root).expect("字典表加载失败");
    println!(
        "  ② load_table::<Security> → {} 行，取自 {}",
        securities.len(),
        securities.spec.file
    );

    // ③ 手上只有表名字符串（配置 / 命令行 / 订阅入参）：先 `spec_of` 拿声明再加载。
    //    阶段 4 把数据源从文件换成 `COPY TO STDOUT` 也走这个入口，所以它单独暴露；
    //    未知表名在这里就是 `None`，必须当场拒绝，不能退化成「读个默认路径」。
    let spec = tables::spec_of("dict_security").expect("表名未声明，拒绝加载");
    let by_id = tables::load_specified_table::<Security>(root, spec).expect("按声明加载失败");
    println!(
        "  ③ spec_of + load_specified_table → {} 行（id={} pk={:?}）",
        by_id.len(),
        spec.id,
        spec.pk
    );
}

/// 遍历：行 / 键 / 跨表 / 点查 / 列索引 / 声明本身。
fn demo_iterate(snapshot: &Snapshot) {
    println!("\n[2] 遍历一张表（键 = `composite_key(Spec::pk)`）");

    // `rows()` 直接暴露底层 HashMap，不造中间包装。
    // 注意：HashMap 顺序不稳定 → 想要可复现的输出先把键排序（只有冷路径打印才这么做）。
    let mut symbols: Vec<&String> = snapshot.securities.rows().keys().collect();
    symbols.sort_unstable();
    for symbol in symbols {
        let security = snapshot.securities.get(symbol).expect("刚列出来的键必然在表里");
        println!(
            "  {symbol:<8} {:<8} lot={} tick={}",
            security.name, security.lot_size, security.price_tick
        );
    }

    println!("\n[3] 遍历值 + 沿外键点查另一张表");
    // 只要值不要键：`values()`；键值都要：`rows().iter()`。
    for (key, position) in snapshot.positions.rows() {
        // 行键由加载期按 `Spec::pk` 现算（impl 不再手写 `pk_parts`），这里直接用 map 键。
        // 声明式外键已保证引用得到，这里不需要 `ok_or_else` 兼容缺失。
        let security = snapshot.securities.get(&position.symbol).expect("字典缺此证券");
        println!(
            "  {key:<16} qty={} 可卖={} 成本={} | {}",
            position.quantity, position.available_qty, position.avg_cost, security.name
        );
    }

    println!("\n[4] 点查与列索引判定");
    // 单列主键：`get` 直接给主键值（入参是行键，不是字段名）。
    println!("  securities.get(\"09018\")            → {}", snapshot.securities.get("09018").is_some());
    // 复合主键：必须用 `composite_key` 拼，段顺序按 `Spec::pk`（`account_id` 在前）。
    let hit = snapshot.positions.get(&composite_key_str(&["A001", "09018"]));
    println!("  positions.get(composite_key)     → {}", hit.is_some());
    // `Snapshot` 上的语义化封装，本质同上，业务代码优先用它。
    println!("  snapshot.position(A001, 09018)   → {}", snapshot.position("A001", "09018").is_some());
    // `has(column, value)`：走加载期建好的列索引，O(1) 回答「这张表有没有某列等于某值的行」。
    // 只能查声明里出现过的列（pk + 本表外键列 + 被别表引用的目标列），它只为校验服务。
    println!("  positions.has(symbol, 09018)     → {}", snapshot.positions.has("symbol", "09018"));
    println!("  securities.has(symbol, 600000)   → {}", snapshot.securities.has("symbol", "600000"));
    println!("  is_tradable(A001, 09018)         → {}", snapshot.is_tradable("A001", "09018"));

    println!("\n[5] 遍历声明本身（只看清单，不碰数据）");
    // 清单是普通静态数组，常规 `for` 即可；拿不到的只有「行数据」——
    // 表对象是强类型 `Table<T>`，按字符串 id 取表在这套设计里不存在（阶段 0.5 刻意不做）。
    for spec in tables::TABLES {
        let expected = spec.expected_rows.map_or("-".to_string(), |rows| rows.to_string());
        println!(
            "  {:<16} {:<20} kind={:?} policy={:?} pk={:?} fk={}行 期望={}",
            spec.id, spec.file, spec.kind, spec.policy, spec.pk, spec.fk.len(), expected
        );
    }
    // 统计口径同样源于声明（0.5.5）：`TableStat` 是 LSN 锚点 / 内存预算 / 日终对账的底座。
    for stat in snapshot.stats() {
        println!("  stat {:<16} rows={:<3} lsn={:?}", stat.id, stat.rows, stat.lsn);
    }

    println!("\n[6] 一份泛型代码遍历任意张表（取哪些列全由声明决定）");
    dump_pk_columns(&snapshot.positions);
    dump_pk_columns(&snapshot.securities);
}

/// 泛型遍历：编译期落到具体 `Table<T>`，运行时零擦除（没进 `dyn`，也没造行包装）。
/// 新表登记进 `TABLES` 并 `impl DataTable` 后，这个函数一行不用改 —— 这就是声明驱动的意义。
fn dump_pk_columns<T: DataTable>(table: &tables::Table<T>) {
    let mut keys: Vec<&String> = table.rows().keys().collect();
    keys.sort_unstable();
    for key in keys {
        let row = table.rows().get(key).expect("刚列出来的键必然在表里");
        // 声明里的列名先 `parse` 成本表列枚举，再经 `DataTable::column` 取值（只开放声明用得到的列）。
        let rendered: Vec<String> = table
            .spec
            .pk
            .iter()
            .filter_map(|column| {
                T::Column::parse(column)
                    .and_then(|col| row.column(col))
                    .map(|value| format!("{column}={value}"))
            })
            .collect();
        println!("  {:<14} {key:<16} {}", table.spec.id, rendered.join(" "));
    }
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