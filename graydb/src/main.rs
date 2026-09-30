pub mod journal;
pub mod domain;
pub mod engine;
pub mod generated;
pub mod mem;
pub mod tables;

use std::path::Path;

use account::amount::{Money, Price, Quantity, Rounding, notional};

use crate::domain::{Security, Side};
use crate::engine::{Engine, PlaceRequest};
use crate::journal::Journal;
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

    // 下面三个函数是「如何用 tables 加载 / 遍历 / 写路径」的可运行样板。
    demo_load(&data_root);
    demo_iterate(&snapshot);
    demo_engine(&data_root);
}

/// 加载：三种粒度按需选。
fn demo_load(root: &Path) {
    println!("\n[1] 加载的三种粒度");

    // ① 整份镜像（生产路径）：`Snapshot::load(root)` —— 声明自检 + 逐表加载
    //    + 外键校验 + 聚合不变量，内部对每张表就是下面 ② 那一句（见 `mem.rs::load`）。
    println!("  ① Snapshot::load → 7 张表全部就位（见上方启动报告）");

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

/// 内核写路径演示（阶段 1）：下单 → 冻结 → 先写 journal → 改内存 → 成交落账，
/// 最后用同一份 journal 做一次恢复回放（阶段 2.3）。
///
/// 与上面两个只读样板不同，这里会真改内存镜像：为了不把演示结果写回 `data/`，
/// 现取一份干净镜像喂给内核，journal 落在临时目录 —— 进程退出即丢，
/// 但序号与落盘顺序已按阶段 1.3/1.5 的规则走完，恢复回放能对得上账。
fn demo_engine(root: &Path) {
    println!("\n[7] 内核写路径：place → 冻结 → apply_fill → 落账");

    let mut path = std::env::temp_dir();
    path.push(format!("graydb-demo-{}.journal.jsonl", std::process::id()));
    // 先清掉同名的上次残留：回放起点必须是「日初 + 仅本进程的流水」，否则 seq 不从 1 起。
    let _ = std::fs::remove_file(&path);
    let journal = Journal::open(&path).expect("演示 journal 打开失败");
    let mut engine = Engine::new(Snapshot::load(root).expect("重新加载一份干净镜像"), journal);

    let asset_before = engine.snapshot.asset("A001").expect("A001 应有资产行");
    let (available_before, total_before) = (asset_before.available, asset_before.total_market_value);

    // 买 100 股 600000，限价 6.0600，立即全成 —— A001 原本不持此券，走的是首次建仓 `upsert`。
    let ack = engine
        .place_and_fill(PlaceRequest {
            order_id: "O-DEMO-1",
            account_id: "A001",
            symbol: "600000",
            side: Side::Buy,
            price: Price::from_units(60_600),
            quantity: Quantity::from_units(100),
            created_at: "2026-09-29T09:30:00Z",
        })
        .expect("演示下单成交不应被拒");

    let asset = engine.snapshot.asset("A001").expect("A001 应有资产行");
    let position = engine.snapshot.position("A001", "600000").expect("成交后必有持仓行");
    println!("  受理 {ack:?}，内核序号已分配 {} / 已落盘 {}", engine.seq(), engine.durable());
    println!("  可用资金 {} → {}（买入冻结与成交扣款均逐笔可追）",
        available_before.format_fixed(2), asset.available.format_fixed(2));
    println!("  新持仓 qty={} 可卖={}（T+1：当日买入不可卖） avg_cost={}",
        position.quantity, position.available_qty, position.avg_cost);
    println!("  市值 {} → {}（每次成交后按持仓全量重算，非增量累加）",
        total_before.format_fixed(2), asset.total_market_value.format_fixed(2));
    println!("  orders={} trades={}", engine.snapshot.orders.len(), engine.snapshot.trades.len());

    // 拒单零状态变化：资金不足直接被拒，镜像与序号一分未动。
    let seq_before = engine.seq();
    let rejected = engine.place(PlaceRequest {
        order_id: "O-DEMO-2",
        account_id: "A001",
        symbol: "600000",
        side: Side::Buy,
        price: Price::from_units(999_999_999),
        quantity: Quantity::from_units(100_000),
        created_at: "2026-09-29T09:31:00Z",
    });
    println!("  高价大额下单 → {rejected:?}，序号仍为 {seq_before}（拒单发生在写盘之前）");
    assert_eq!(engine.seq(), seq_before, "拒单不得消耗序号");
    engine.snapshot.check_valuation().expect("演示结束后市值仍应平");

    // 阶段 2.3：拿同一份 journal 从「日初镜像」重放，结果必须与上面的内存逐分不差。
    // 两边用的是同一套落账函数（冻结 / 结算 / 持仓 / 市值），对不上只可能出在流水本身。
    println!("\n[8] 日志恢复：replay(日初镜像 + journal) == 崩溃前内存");
    let recovery = Engine::recover(Snapshot::load(root).expect("重新加载日初镜像作回放起点"), &path)
        .expect("演示 journal 完整，回放应成功重建终态");
    let (live, reborn) = (&engine.snapshot, &recovery.engine.snapshot);
    println!(
        "  回放 {} 条记录，丢尾={}；序号 {} → {}，orders {} → {}，trades {} → {}",
        recovery.replayed,
        recovery.dropped_tail.is_some(),
        engine.seq(),
        recovery.engine.seq(),
        live.orders.len(),
        reborn.orders.len(),
        live.trades.len(),
        reborn.trades.len(),
    );
    let (live_asset, reborn_asset) = (
        live.asset("A001").expect("A001 应有资产行"),
        reborn.asset("A001").expect("回放后 A001 应有资产行"),
    );
    println!(
        "  可用 {} ↔ {}，冻结 {} ↔ {}，市值 {} ↔ {}",
        live_asset.available.format_fixed(2),
        reborn_asset.available.format_fixed(2),
        live_asset.frozen.format_fixed(2),
        reborn_asset.frozen.format_fixed(2),
        live_asset.total_market_value.format_fixed(2),
        reborn_asset.total_market_value.format_fixed(2),
    );
    let (live_position, reborn_position) = (
        live.position("A001", "600000").expect("成交后必有持仓行"),
        reborn.position("A001", "600000").expect("回放后应有同一持仓行"),
    );
    println!(
        "  新持仓 qty {} ↔ {}，可卖 {} ↔ {}，avg_cost {} ↔ {}",
        live_position.quantity,
        reborn_position.quantity,
        live_position.available_qty,
        reborn_position.available_qty,
        live_position.avg_cost,
        reborn_position.avg_cost,
    );
    assert_eq!(recovery.engine.seq(), engine.seq(), "恢复后的序号应接在崩溃前");
    assert_eq!(recovery.engine.durable(), recovery.engine.seq(), "恢复后不得留空洞");
    assert_eq!(reborn_asset.available, live_asset.available, "可用资金逐分不差");
    assert_eq!(reborn_asset.frozen, live_asset.frozen, "冻结逐分不差");
    assert_eq!(
        reborn_asset.total_market_value,
        live_asset.total_market_value,
        "市值逐分不差"
    );
    assert_eq!(reborn_position.quantity, live_position.quantity);
    assert_eq!(reborn_position.available_qty, live_position.available_qty);
    assert_eq!(reborn_position.avg_cost, live_position.avg_cost);
    assert_eq!(reborn.orders.len(), live.orders.len(), "订单行数一致");
    assert_eq!(reborn.trades.len(), live.trades.len(), "成交行数一致");
    recovery
        .engine
        .snapshot
        .check_valuation()
        .expect("回放后的市值同样该平");
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

    /// 七份 JSON → 领域结构 → 声明式校验全跑一遍。
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
        // 阶段 1 的流水表日初为空，只能由内核写入（与 `tables.toml` 的 `expected_rows = 0` 同源）
        assert_eq!(snap.orders.len(), 0);
        assert_eq!(snap.trades.len(), 0);

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