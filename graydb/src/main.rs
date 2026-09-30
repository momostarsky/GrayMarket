pub mod journal;
pub mod domain;
pub mod engine;
pub mod generated;
pub mod mem;
pub mod pubsub;
pub mod tables;

use std::collections::BTreeMap;
use std::path::Path;

use account::amount::{Money, Price, Quantity, Rounding, notional};

use crate::domain::{Security, Side};
use crate::engine::{Engine, PlaceRequest};
use crate::journal::Journal;
use crate::mem::Snapshot;
use crate::pubsub::{CatchUp, Frame, Op, RowChange, Subscribe};
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
    demo_pubsub(&data_root);
    demo_protocol(&data_root);
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

/// 阶段 3.1 / 3.3：写路径的终点多了一个订阅出口 —— 每个事务组改了哪些行，
/// 消费者按自己的游标来拉；拉得慢就整组淘汰，但内核照写不误（铁律 5：
/// 读侧的速度不能变成写侧的债）。发布的值是**落账完成后的终态镜像**，不是中间态。
fn demo_pubsub(root: &Path) {
    println!("\n[9] 订阅出口：按组发布 + 按游标补发");

    let mut path = std::env::temp_dir();
    path.push(format!("graydb-demo-pubsub-{}.journal.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let journal = Journal::open(&path).expect("演示 journal 打开失败");
    let mut engine = Engine::new(Snapshot::load(root).expect("重新加载一份干净镜像"), journal);

    // 卖光持仓：这一组里 `position` 行被 `delete`（不带行值），其余三张表各一条 upsert。
    engine
        .place_and_fill(PlaceRequest {
            order_id: "O-PUB-S1",
            account_id: "A001",
            symbol: "09018",
            side: Side::Sell,
            price: Price::from_units(90_025),
            quantity: Quantity::from_units(800),
            created_at: "2026-09-29T09:30:00Z",
        })
        .expect("演示卖出成交不应被拒");
    // 再走一组「下单冻结 + 撤单解冻」，看两个组各自独立。
    engine
        .place(PlaceRequest {
            order_id: "O-PUB-B1",
            account_id: "A001",
            symbol: "600000",
            side: Side::Buy,
            price: Price::from_units(60_600),
            quantity: Quantity::from_units(100),
            created_at: "2026-09-29T09:31:00Z",
        })
        .expect("演示买入下单不应被拒");
    engine.cancel("O-PUB-B1").expect("演示撤单不应失败");

    println!(
        "  内核：已落盘 seq={}，出口已发布 {} 组 / 留存 {} 行（容量 {} 行）",
        engine.durable(),
        engine.broadcast().groups(),
        engine.broadcast().len(),
        engine.broadcast().capacity()
    );

    // 消费者刚接入：游标 0，一次补到当前位。
    let CatchUp::Delta { from, through, changes } = engine.catch_up(0) else {
        panic!("默认大环装得下本进程全部历史，不该降级：{}", engine.catch_up(0).summary());
    };
    println!("  补发 {from}..{through}，{} 行：", changes.len());
    for change in &changes {
        println!(
            "    seq={} {} 键={} op={:?} 带行={}",
            change.seq, change.table, change.key, change.op, change.row.is_some()
        );
    }
    assert_eq!(from, 1);
    assert_eq!(through, engine.seq());
    assert!(
        changes.iter().any(|c| c.op == Op::Delete && c.row.is_none()),
        "卖光持仓应发一条不带行值的 Delete"
    );

    // 已跟上的游标：空 Delta，而不是降级 —— 消费完不欠账。
    assert!(
        matches!(engine.catch_up(through), CatchUp::Delta { ref changes, .. } if changes.is_empty()),
        "跟上当前位后应拿到空 Delta：{}", engine.catch_up(through).summary()
    );

    // 小环（只装得下一组）：慢消费者的历史被整组淘汰，写路径一分不受影响。
    let mut tiny_path = std::env::temp_dir();
    tiny_path.push(format!("graydb-demo-pubsub-tiny-{}.journal.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&tiny_path);
    let tiny_journal = Journal::open(&tiny_path).expect("小环演示 journal 打开失败");
    let mut tiny = Engine::with_ring(Snapshot::load(root).expect("重新加载日初镜像"), tiny_journal, 1);
    for (order_id, symbol) in [("O-TINY-1", "600000"), ("O-TINY-2", "09018")] {
        tiny
            .place(PlaceRequest {
                order_id,
                account_id: "A002",
                symbol,
                side: Side::Buy,
                price: Price::from_units(60_600),
                quantity: Quantity::from_units(100),
                created_at: "2026-09-29T09:32:00Z",
            })
            .expect("小环不影响受理");
    }
    println!(
        "  小环（容量 1 行）：已发布 {} 组 / 留存 {} 行 / 已淘汰 {} 行，游标 0 拿到：{}",
        tiny.broadcast().groups(),
        tiny.broadcast().len(),
        tiny.broadcast().dropped_rows(),
        tiny.catch_up(0).summary()
    );
    assert!(matches!(tiny.catch_up(0), CatchUp::Lagged { .. }), "跌出窗口的游标必须降级");
    assert_eq!(tiny.seq(), tiny.durable(), "淘汰历史不动落盘序号，写路径无感");
    assert!(tiny.broadcast().dropped_rows() > 0, "慢消费者的代价只落在它自己的历史上");
}

/// 消费者本地镜像：`表 -> (行键 -> 行 JSON)`，只从帧里长出来，不直接读内核。
type Mirror = BTreeMap<&'static str, BTreeMap<String, serde_json::Value>>;

/// 把一个帧序列打进本地镜像：`SNAPSHOT_BEGIN` 建桶、快照行覆盖、`Delta` 逐条应用。
/// 与测试里那份 `apply_frames` 同构，但这里只负责把协议跑给人看，不做断言密集的对账。
fn replay(mirror: &mut Mirror, frames: &[Frame]) {
    for frame in frames {
        match frame {
            Frame::SnapshotBegin { table, .. } => {
                mirror.insert(*table, BTreeMap::new());
            }
            Frame::SnapshotRow(change) => apply_change(mirror, change),
            Frame::Delta { changes, .. } => {
                for change in changes {
                    apply_change(mirror, change);
                }
            }
            Frame::SnapshotEnd { .. } | Frame::RebuildRequired { .. } => {}
        }
    }
}

/// `Upsert` 覆盖本地行，`Delete` 摘行（不带行值，靠行键定位）。
fn apply_change(mirror: &mut Mirror, change: &RowChange) {
    let bucket = mirror
        .get_mut(change.table)
        .unwrap_or_else(|| panic!("帧里出现了没订的表 {}", change.table));
    match change.op {
        Op::Upsert => {
            bucket.insert(
                change.key.clone(),
                change.row.clone().expect("Upsert 必带行"),
            );
        }
        Op::Delete => {
            bucket.remove(&change.key);
        }
    }
}

/// 3.2 + 3.6：订阅协议与快照下发。表清单靠注册中心校验、取行靠 `Snapshot` 分派，
/// 所以新表接入不必改协议代码；快照与增量共用同一份过滤与裁列口径。
fn demo_protocol(root: &Path) {
    println!("\n[10] 订阅协议与快照下发：帧序列 → 消费者镜像");

    let mut path = std::env::temp_dir();
    path.push(format!("graydb-demo-proto-{}.journal.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let journal = Journal::open(&path).expect("演示 journal 打开失败");
    let mut engine = Engine::new(Snapshot::load(root).expect("重新加载一份干净镜像"), journal);

    // ① 订约：四张可变表全列 + 全量快照。水位 = 当前 durable，快照行全打在这个 seq 上。
    let (mut sub, frames) = engine
        .subscribe(Subscribe::new(&["account_asset", "position", "orders", "trades"]))
        .expect("四张可变表应可订全量快照");
    let (mut barriers, mut snapshot_rows) = (0_usize, 0_usize);
    for frame in &frames {
        match frame {
            Frame::SnapshotBegin { .. } | Frame::SnapshotEnd { .. } => barriers += 1,
            Frame::SnapshotRow(_) => snapshot_rows += 1,
            other => panic!("attach 不该发出增量帧：{other:?}"),
        }
    }
    println!(
        "  attach：水位 seq={}，{} 帧 = {barriers} 道屏障（{} 表 × 2）+ {snapshot_rows} 行快照",
        sub.cursor(),
        frames.len(),
        sub.tables().len(),
    );
    let mut mirror: Mirror = BTreeMap::new();
    replay(&mut mirror, &frames);
    assert_eq!(
        mirror,
        kernel_mirror(&engine, sub.tables()),
        "接入那一刻消费者镜像就该等于内核镜像"
    );

    // ② 写一组：卖光 09018。增量只发 seq 大于水位的行，快照与增量交接不重不漏。
    engine
        .place_and_fill(PlaceRequest {
            order_id: "O-PROTO-S1",
            account_id: "A001",
            symbol: "09018",
            side: Side::Sell,
            price: Price::from_units(90_025),
            quantity: Quantity::from_units(800),
            created_at: "2026-09-29T09:30:00Z",
        })
        .expect("演示卖出成交不应被拒");
    let frames = engine.poll(&mut sub);
    for frame in &frames {
        if let Frame::Delta { through, changes } = frame {
            println!("  poll：Delta through={through}，{} 行：", changes.len());
            for change in changes {
                println!(
                    "    seq={} {:<13} 键={:<12} op={:?} 带行={}",
                    change.seq, change.table, change.key, change.op, change.row.is_some()
                );
            }
        }
    }
    replay(&mut mirror, &frames);
    assert_eq!(mirror, kernel_mirror(&engine, sub.tables()), "增量之后两边仍逐行相等");
    assert_eq!(sub.cursor(), engine.durable(), "跟上当前位后水位应齐平");
    assert!(
        !mirror["position"].contains_key(&composite_key_str(&["A001", "09018"])),
        "卖光后消费者本地不该残留那行仓位"
    );

    // ③ 先给 A001 建一笔新仓位（09018 已卖光），再看裁列 + 账户过滤：快照行与增量行走同一个
    // 口径，否则消费者会得到「没裁过的快照 + 裁过的增量」。
    engine
        .place_and_fill(PlaceRequest {
            order_id: "O-PROTO-B1",
            account_id: "A001",
            symbol: "600000",
            side: Side::Buy,
            price: Price::from_units(60_600),
            quantity: Quantity::from_units(100),
            created_at: "2026-09-29T09:31:00Z",
        })
        .expect("演示买入成交不应被拒");
    replay(&mut mirror, &engine.poll(&mut sub));
    assert_eq!(mirror, kernel_mirror(&engine, sub.tables()), "买入成交后两边仍逐行相等");

    let (mut filtered, frames) = engine
        .subscribe(
            Subscribe::new(&["position"])
                .columns(&["quantity"])
                .accounts(&["A001"]),
        )
        .expect("position 的主键首列是 account_id，应放行按账户分流");
    let picked: Vec<(String, Vec<String>)> = frames
        .iter()
        .filter_map(|frame| match frame {
            Frame::SnapshotRow(change) => {
                let mut columns: Vec<String> = change
                    .row
                    .as_ref()
                    .and_then(serde_json::Value::as_object)
                    .map(|object| object.keys().cloned().collect())
                    .unwrap_or_default();
                columns.sort();
                Some((change.key.clone(), columns))
            }
            _ => None,
        })
        .collect();
    println!(
        "  裁列 + 过滤（只订 A001 的 position，只要 quantity）：{} 行，水位 seq={}",
        picked.len(),
        filtered.cursor(),
    );
    for (key, columns) in &picked {
        println!("    键={key} 列={}", columns.join(","));
    }
    assert_eq!(
        picked.len(),
        1,
        "A001 此刻只应有 600000 一行仓位（09018 已被上面的卖光消掉），否则下面的断言是空话"
    );
    assert!(
        picked.iter().all(|(key, _)| key.starts_with("A001")),
        "快照不该漏进别人的行"
    );
    assert!(
        picked.iter().all(|(_, columns)| columns == &["account_id", "quantity", "symbol"]),
        "裁列后应剩请求列 ∪ 主键列（裁掉主键会让消费者拼不回行）"
    );

    // 本组没有本订阅者的行：帧仍要发、水位仍要推 —— 「≤ through 已确认无你的行」必须说出口。
    engine
        .place(PlaceRequest {
            order_id: "O-PROTO-B2",
            account_id: "A002",
            symbol: "600000",
            side: Side::Buy,
            price: Price::from_units(60_600),
            quantity: Quantity::from_units(100),
            created_at: "2026-09-29T09:32:00Z",
        })
        .expect("A002 下单不应被拒");
    let frames = engine.poll(&mut filtered);
    println!(
        "  只动别人的一组之后：{}，游标 → {}",
        describe_frames(&frames),
        filtered.cursor(),
    );
    let [Frame::Delta { through, changes }] = frames.as_slice() else {
        panic!("应拿到恰好一个增量帧：{frames:?}");
    };
    assert!(changes.is_empty(), "别人的行不该泄给只订 A001 的订阅者");
    assert_eq!(*through, engine.durable(), "空批也要把水位推到当前位");

    // ④ 越界拒订：过滤与列名都在 attach 当场拒，不退成一个笼统的 Invalid。
    for (spec, expect) in [
        (
            Subscribe::new(&["orders"]).accounts(&["A001"]),
            "orders 的主键是 order_id，Delete 无法按账户归属",
        ),
        (
            Subscribe::new(&["position"]).columns(&["qty"]),
            "列名拼错要指认到列",
        ),
        (
            Subscribe::new(&["orders"]).ops(&[Op::Delete]),
            "要快照就必须订 Upsert",
        ),
    ] {
        let tables = spec.tables.join(",");
        match engine.subscribe(spec) {
            Ok(_) => panic!("{tables} 这条请求本应被拒（{expect}）"),
            Err(error) => println!("  拒订 {tables}：{error}（{expect}）"),
        }
    }

    // ⑤ 落后到窗口外：协议层只说明一件事 —— 重新下发快照，而且不推游标。
    let mut tiny_path = std::env::temp_dir();
    tiny_path.push(format!("graydb-demo-proto-tiny-{}.journal.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&tiny_path);
    let tiny_journal = Journal::open(&tiny_path).expect("小环演示 journal 打开失败");
    let mut tiny = Engine::with_ring(Snapshot::load(root).expect("重新加载日初镜像"), tiny_journal, 1);
    let (mut slow, _) = tiny
        .subscribe(Subscribe::new(&["orders"]).delta_only())
        .expect("只增量接入应放行");
    for (order_id, symbol) in [("O-TINY-P1", "600000"), ("O-TINY-P2", "09018")] {
        tiny
            .place(PlaceRequest {
                order_id,
                account_id: "A002",
                symbol,
                side: Side::Buy,
                price: Price::from_units(60_600),
                quantity: Quantity::from_units(100),
                created_at: "2026-09-29T09:33:00Z",
            })
            .expect("小环不影响受理");
    }
    let frames = tiny.poll(&mut slow);
    println!(
        "  落后订阅者：{}，游标仍停在 {}（缺口不能靠推进掩盖）",
        describe_frames(&frames),
        slow.cursor(),
    );
    assert!(
        matches!(frames.as_slice(), [Frame::RebuildRequired { .. }]),
        "跌出窗口的订阅者只能拿到重建信号：{frames:?}"
    );
    assert_eq!(slow.cursor(), 0, "重建前不得推进游标");
    // 重建的正确姿势就是重新走一遍 attach。
    let (rebuilt, frames) = tiny
        .subscribe(Subscribe::new(&["orders"]))
        .expect("重新订约应被接受");
    let mut mirror: Mirror = BTreeMap::new();
    replay(&mut mirror, &frames);
    assert_eq!(mirror, kernel_mirror(&tiny, rebuilt.tables()), "重建后镜像应等于内核");
    println!(
        "  重新 attach：{} 帧，水位 seq={}，镜像已与内核一致",
        frames.len(),
        rebuilt.cursor(),
    );
}

/// 内核当前镜像（只取本订阅者订的那几张表）：走 `Snapshot::rows_json` 的分派臂。
fn kernel_mirror(engine: &Engine, tables: &[&'static str]) -> Mirror {
    tables
        .iter()
        .map(|table| (*table, engine.snapshot.rows_json(table).into_iter().collect()))
        .collect()
}

/// 把帧序列压成一句能打印的话。
fn describe_frames(frames: &[Frame]) -> String {
    frames
        .iter()
        .map(|frame| match frame {
            Frame::SnapshotBegin { table, seq, .. } => format!("SNAPSHOT_BEGIN({table}@{seq})"),
            Frame::SnapshotRow(change) => format!("SNAPSHOT_ROW({})", change.key),
            Frame::SnapshotEnd { table, rows } => format!("SNAPSHOT_END({table}:{rows})"),
            Frame::Delta { through, changes } => format!("DELTA(≤{through},{}行)", changes.len()),
            Frame::RebuildRequired { lost_through } => {
                format!("REBUILD_REQUIRED（≤{lost_through} 已被淘汰，需重新下发快照）")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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