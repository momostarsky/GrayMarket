pub mod journal;
pub mod domain;
pub mod engine;
pub mod generated;
pub mod mem;
pub mod net;
pub mod pubsub;
pub mod tables;
pub mod wire;

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use account::amount::{Money, Price, Quantity, Rounding, notional};
use tokio::net::TcpListener;

use crate::domain::{PdUnitCapitTrade, Security, Side};
use crate::engine::{Engine, PlaceRequest};
use crate::journal::Journal;
use crate::mem::Snapshot;
use crate::net::{Client, Hub, NetError, serve};
use crate::pubsub::{CatchUp, Frame, Op, RowChange, Subscribe, Topic};
use crate::tables::{ColumnName, DataTable, composite_key_str};
use crate::wire::WireFrame;

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

    // 阶段 3.4 要真 socket，所以这一段自己起一个 runtime。它**没有**改内核的形状：
    // `demo_send_side` 里持 `Engine` 与 `Hub` 的那几段全是同步调用，一次 `await` 都没有。
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime 起不来");
    runtime.block_on(demo_send_side(&data_root));
    // 阶段 3.5 的主题粒度：一条通配过真 socket，命不中的那一档当场被拒。
    runtime.block_on(demo_topics(&data_root));
    // 阶段 4.5 的读侧换源：同一张真实表走两条通道，落地必须逐行相等。
    demo_read_side(&data_root);
}

/// 加载：三种粒度按需选。
fn demo_load(root: &Path) {
    println!("\n[1] 加载的三种粒度");

    // ① 整份镜像（生产路径）：`Snapshot::load(root)` —— 声明自检 + 逐表加载
    //    + 外键校验 + 聚合不变量，内部对每张表就是下面 ② 那一句（见 `mem.rs::load`）。
    println!("  ① Snapshot::load → {} 张表全部就位（见上方启动报告）", tables::TABLES.len());

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
        let topics = spec.topics.join(",");
        match engine.subscribe(spec) {
            Ok(_) => panic!("{topics} 这条请求本应被拒（{expect}）"),
            Err(error) => println!("  拒订 {topics}：{error}（{expect}）"),
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

/// 演示用的一笔下单：A002 买 100 股 600000（只下单不成交，一组 = 订单一行 + 资金一行）。
/// 抽出来是因为 [11] 要把同一个写动作重复几十组，而不必每次重抄七个字段。
fn place_one(engine: &mut Engine, order_id: &str) -> i64 {
    engine
        .place(PlaceRequest {
            order_id,
            account_id: "A002",
            symbol: "600000",
            side: Side::Buy,
            price: Price::from_units(60_600),
            quantity: Quantity::from_units(100),
            created_at: "2026-09-29T09:40:00Z",
        })
        .expect("演示下单不应被拒")
        .seq()
}

/// 发送侧的对账：消费者镜像（表名是 `String`）与内核当前行集逐表相等。
/// 两边都是 `BTreeMap`，键序稳定，所以比的是值本身，不需要先排序再凑。
fn assert_matches_kernel(mirror: &crate::wire::Mirror, kernel: &Mirror) {
    assert_eq!(mirror.tables().len(), kernel.len(), "镜像里的表数与订阅不符");
    for (table, rows) in kernel {
        assert_eq!(mirror.table(table), Some(rows), "表 {table} 两端逐行不等");
    }
}

/// 阶段 3.4：发送侧。真 socket 上跑一遍「快照 → 增量 → 越界拒订 → 慢连接被降级 → 收摊摘除」。
/// 铁律 5 在这一层的验收标准只有一条：**持 `Engine` 与 `Hub` 的那段代码一次 `await` 都不许有**；
/// 下面的 `drain_commands` / `fan_out` 全是同步调用，跨线程只传已拥有的 `String`。
async fn demo_send_side(root: &Path) {
    println!("\n[11] 发送侧：Hub 扇出 + 每连接独立队列（慢连接只慢自己）");

    const TABLE_LIST: &[&str] = &["account_asset", "orders", "trades", "position"];

    let mut path = std::env::temp_dir();
    path.push(format!("graydb-demo-net-{}.journal.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let journal = Journal::open(&path).expect("演示 journal 打开失败");
    // 补发窗口只留 8 行（≈ 4 组）：让「慢连接跌出窗口」在这段演示里真会发生，而不是只在测试里。
    let mut engine = Engine::with_ring(Snapshot::load(root).expect("重新加载一份干净镜像"), journal, 8);

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("回环监听起不来");
    let addr = listener.local_addr().expect("已绑定就该有端口");
    // 队列容量 4 批是**服务端**策略，不能来自请求：那等于让客户端决定内核侧占多少内存。
    let (commands, mut hub) = Hub::new(4);
    tokio::spawn(serve(listener, commands));

    let spec = Subscribe::new(TABLE_LIST);
    let mut quick = Client::connect(addr, &spec).await.expect("A 的连接应能建立");
    let mut idle = Client::connect(addr, &spec).await.expect("B 的连接应能建立");
    let mut out_of_scope =
        Client::connect(addr, &Subscribe::new(&["orders"]).accounts(&["A001"])).await
            .expect("C 的连接能建立，被拒的是请求");
    // 内核不因为有人连上来就等：握手只在下一次 `drain_commands` 里被 `try_recv` 收干，
    // 所以这里得转几圈。真循环里这几圈就是写路径的下一组，不是给连接让的路。
    for _ in 0..100 {
        hub.drain_commands(&engine);
        if hub.connections() == 2 && hub.stats().refused == 1 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert_eq!(hub.connections(), 2, "合规的两条该登记，越界那条不该");

    // ① 越界请求在真 socket 上拿到的是那句人话：只关连接的话，客户端只能从 EOF 反推，
    //    而 EOF 与「网络掉了」分不开。
    let refused = out_of_scope.finish_snapshot().await.expect_err("越界请求不该读到快照");
    assert!(matches!(refused, NetError::Refused(_)), "实得：{refused}");
    println!("  C 越界请求 → {refused}");

    // ② 两条合规连接各自收下全表快照，镜像与内核逐行相等 —— 编解码边界的端到端证明。
    quick.finish_snapshot().await.expect("A 该读完快照");
    idle.finish_snapshot().await.expect("B 该读完快照");
    for (who, client) in [("A", &quick), ("B", &idle)] {
        assert_matches_kernel(client.mirror(), &kernel_mirror(&engine, TABLE_LIST));
        println!("  {who} 快照读完：{}", client.mirror().summary());
    }

    // ③ 十组下单：内核写 → 同步扇出 → 只有 A 去读。B 一行不读，但它自己的任务在把批
    //    写进自己的 socket 缓冲，所以它慢在自己的那头；内核这边一帧都没少发、一步都没等。
    for index in 1..=10 {
        place_one(&mut engine, &format!("O-NET-{index}"));
        let round = hub.fan_out(&engine);
        quick.advance_to(engine.durable()).await.expect("A 该一直跟得上");
        if index <= 2 {
            println!(
                "  第 {index} 组：seq={} 本轮扇出 {} 帧、退让 {} 批",
                engine.seq(), round.frames, round.stalled
            );
        }
    }
    assert_matches_kernel(quick.mirror(), &kernel_mirror(&engine, TABLE_LIST));
    println!("  A 跟到水位 {}：{}", quick.mirror().last_through(), quick.mirror().summary());

    // ④ 慢连接被降级。这里再登记一条连接，把它的 `rx` 握在演示手里不读：它就是
    //    「对端还在、但一个字节都不取」的精确模型 —— 背压走的是与 socket 完全同一条代码，
    //    而只有把它放在内核侧才确定性地能推到跌出补发窗口（不靠网速撞运气）。
    let slow = hub
        .attach_with(&engine, Subscribe::new(TABLE_LIST), 2)
        .expect("演示用的慢连接应能登记");
    let slow_id = slow.id;
    let mut slow_rx = slow.rx;
    println!("  登记慢连接 D（id={slow_id}，队列 2 批）");
    for index in 11..=14 {
        place_one(&mut engine, &format!("O-NET-{index}"));
        let round = hub.fan_out(&engine);
        quick.advance_to(engine.durable()).await.expect("A 该一直跟得上");
        println!(
            "  第 {index} 组（D 一帧不取）：扇出 {} 帧、退让 {} 批、判落后 {} 条",
            round.frames, round.stalled, round.lagged
        );
    }
    // D 每收一次就又要它「一帧不取」六组：队列 2 批先填满，之后的扇出全记成退让，而环继续
    // 淘汰，直到把它的游标甩出补发窗口。队列满的时候连降级信号也投不出去 —— 投不出去
    // 就是还没退让成功，不能当作已降级，所以只有腾出位置的那一轮才可能收到它。
    let mut degraded = None;
    let mut group = 15;
    for phase in 1..=4 {
        // 先把上一阶段攒着的取走（只看字节不看内容），给队列腾出位置。
        let mut lines: Vec<String> = Vec::new();
        while let Ok(batch) = slow_rx.try_recv() {
            lines.extend(batch);
        }
        for _ in 0..6 {
            place_one(&mut engine, &format!("O-NET-{group}"));
            group += 1;
            hub.fan_out(&engine);
            quick.advance_to(engine.durable()).await.expect("A 该一直跟得上");
        }
        while let Ok(batch) = slow_rx.try_recv() {
            lines.extend(batch);
        }
        for line in &lines {
            // 服务端发出的行必能解回来：这一步顺手拿真数据把解码边界也走了一遍。
            if let WireFrame::RebuildRequired { lost_through } = wire::decode_line(line).expect("行该解得开") {
                degraded = Some(lost_through);
            }
        }
        println!(
            "  D 第 {phase} 阶段取走 {} 行，{}",
            lines.len(),
            if degraded.is_some() { "已收到重建信号" } else { "游标仍在窗口内，本阶段只拿到补发增量" }
        );
        if degraded.is_some() {
            break;
        }
    }
    let lost_through = degraded.expect("D 一直不消费，该跌出补发窗口拿到重建信号");

    // 降级之后内核不再给它增量（继续发只会让它以为自己能续上）：再走一轮，D 的游标一分不动，
    // 而 A 照常追到当前位。缺口不能靠推进掩盖 —— 这条与 3.6 的订阅者口径是同一份。
    let cursor_before = hub.cursor_of(slow_id).expect("D 还在登记里");
    place_one(&mut engine, "O-NET-AFTER-REBUILD");
    hub.fan_out(&engine);
    quick.advance_to(engine.durable()).await.expect("A 该一直跟得上");
    assert_matches_kernel(quick.mirror(), &kernel_mirror(&engine, TABLE_LIST));
    assert_eq!(hub.cursor_of(slow_id), Some(cursor_before), "已降级的连接游标不该被推进");
    assert!(cursor_before <= lost_through, "游标停在缺口之内才会被判落后：{cursor_before} vs {lost_through}");
    let (_, d_cursor, d_paused, d_stalled) = hub
        .cursors()
        .into_iter()
        .find(|(id, ..)| *id == slow_id)
        .expect("D 还在登记里");
    println!(
        "  D 降级后：游标停在 {d_cursor}（已降级={d_paused}，自己退让 {d_stalled} 批），而 A 已到 {}",
        quick.mirror().last_through()
    );
    assert!(d_paused, "被判落后的连接不该再收增量");

    // ⑤ 对端收摊：`rx` 被 drop 之后，下一轮扇出用 `try_reserve` 探到 Closed 并当场摘除。
    //   不探这一步，一条写不出帧的死连接永远没机会被发现（没新组 → 不 try_send → 不看错）。
    drop(slow_rx);
    let round = hub.fan_out(&engine);
    assert_eq!(round.detached, 1, "对端收摊该在这一轮就被摘掉");
    quick.advance_to(engine.durable()).await.expect("A 该一直跟得上");
    println!(
        "  D 收摊 → 本轮摘除 {} 条，登记表剩 {} 条连接",
        round.detached,
        hub.connections()
    );

    println!("  最终登记（id, 游标, 已降级, 退让批数）：{:?}", hub.cursors());
    // B 一行不读，它的客户端镜像停在快照那 5 行；而服务端侧的游标已跟着走到当前位 ——
    // 那批字节正待在它自己的 socket 缓冲里。「慢」是它自己的事，不是内核少发了。
    println!("  B（从不读一行）的镜像：{}", idle.mirror().summary());
    println!("  发送侧累计：{}", hub.stats());
    println!(
        "  内核：seq={} durable={} orders={}（全程没有为连接推迟过任何一笔写入）",
        engine.seq(),
        engine.durable(),
        engine.snapshot.orders.len()
    );
    assert_eq!(engine.durable(), engine.seq(), "到最后落盘应追平受理");
    assert_matches_kernel(quick.mirror(), &kernel_mirror(&engine, TABLE_LIST));
}

/// 阶段 3.5：主题粒度。一条 `table:*` 换来整份注册中心，一条 `table:{schema}.*` 在真实库
/// 还没接表时被当场拒 —— 「命中零张」绝不留下一个看起来成功、其实永远静默的订阅。
async fn demo_topics(root: &Path) {
    println!("\n[12] 主题粒度：一条主题展开成一组表，命中零张当场拒订");

    let mut path = std::env::temp_dir();
    path.push(format!("graydb-demo-topic-{}.journal.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let journal = Journal::open(&path).expect("演示 journal 打开失败");
    let mut engine = Engine::with_ring(Snapshot::load(root).expect("重新加载一份干净镜像"), journal, 64);

    // ① 展开本身：通配与精确名重叠只算一张 —— 不去重就会下两遍快照，
    //    消费者的镜像会在第二道 begin 上撞 `DuplicateBegin`。
    let all = Subscribe::of_topics(&["table:*"])
        .validate()
        .expect("全库主题应放行");
    let overlapped = Subscribe::of_topics(&["table:*", "table:orders"])
        .validate()
        .expect("重叠主题应放行");
    assert_eq!(all.len(), tables::TABLES.len(), "table:* 就是注册中心全部");
    assert_eq!(overlapped, all, "通配与精确名重叠不该多出一张表");
    println!("  table:* 展开成 {} 张表；再加一条 table:orders 仍是 {} 张", all.len(), overlapped.len());

    // ② 命不中就说不中：反例用库里根本没的 schema —— 这一档必须靠拒订说出口，
    //    不能编一条通配假装命中。正例则是 4.5 转正的那张真实表：它带 schema，
    //    所以 `table:{schema}.*` 从「恒为空」变成了第一次真能展开。
    let missed = Subscribe::of_topics(&["table:jzdb_nosuch.*"])
        .validate()
        .expect_err("库里没这个 schema，按订本该被拒");
    println!("  table:jzdb_nosuch.* → {missed}");
    let by_schema = Subscribe::of_topics(&["table:jzdb_prod.*"])
        .validate()
        .expect("jzdb_prod 已有转正的表，这条通配不该再被拒");
    println!("  table:jzdb_prod.* 展开成 {:?}", by_schema);
    // 「形状不对」与「这张表恰好没变更」是两种病，报错必须分得开。
    let malformed = Topic::parse("orders").expect_err("裸表名不该被猜成主题");
    println!("  裸表名 orders → {malformed}");

    // ③ 一条通配过真 socket：客户端等的屏障数按展开后的表集算，而不是按主题串数。
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("回环监听起不来");
    let addr = listener.local_addr().expect("已绑定就该有端口");
    let (commands, mut hub) = Hub::new(8);
    tokio::spawn(serve(listener, commands));
    let mut client = Client::connect(addr, &Subscribe::of_topics(&["table:*"]))
        .await
        .expect("全库主题的请求该能写出");
    for _ in 0..100 {
        hub.drain_commands(&engine);
        if hub.connections() == 1 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    client.finish_snapshot().await.expect("全库快照该读完");
    assert_matches_kernel(client.mirror(), &kernel_mirror(&engine, &all));
    println!("  一条主题的连接收到 {} 帧，镜像：{}", client.frames(), client.mirror().summary());

    // ④ 增量口径不因通配而变：同一个水位、同一批行。
    place_one(&mut engine, "O-TOPIC-1");
    let round = hub.fan_out(&engine);
    client.advance_to(engine.durable()).await.expect("增量该读到");
    assert_matches_kernel(client.mirror(), &kernel_mirror(&engine, &all));
    println!(
        "  下一单后：本轮扇出 {} 帧，镜像 {}（水位 {}）",
        round.frames,
        client.mirror().summary(),
        engine.durable()
    );
}

/// 阶段 4.5：读侧换源。表清单与校验口径只有一份，换源只换「行从哪来」：
/// 文件通道拿 `{"外层键": {…行}}`，PG 通道拿 `COPY … WITH (FORMAT json, ARRAY true)` 的产物
/// —— 一份**没有外层键**的数组。两条通道只在取行一步分叉，建行 / 分级 / 哨兵全部合流。
fn demo_read_side(root: &Path) {
    println!("\n[13] 读侧换源：同一张表两条通道，落地逐行相等");

    let spec = tables::spec_of(PdUnitCapitTrade::ID).expect("试点表应已登记");
    let from_file =
        tables::load_specified_table::<PdUnitCapitTrade>(root, spec).expect("文件通道加载失败");
    let pg_path = root.join("pg").join("tb_pdmage_pd_unit_capit_trade.pg.json");
    let from_pg = tables::load_from_source::<PdUnitCapitTrade>(
        spec,
        &tables::Source::PgJsonArray { path: &pg_path },
    )
    .expect("PG 形状通道加载失败");

    println!(
        "  {}：schema={:?} pk={:?} policy={:?}，文件 {} 行 / PG {} 行",
        spec.id, spec.schema, spec.pk, spec.policy, from_file.len(), from_pg.len()
    );

    let mut file_keys: Vec<&String> = from_file.rows().keys().collect();
    let mut pg_keys: Vec<&String> = from_pg.rows().keys().collect();
    file_keys.sort();
    pg_keys.sort();
    assert_eq!(file_keys, pg_keys, "行键集合必须一致 —— 现算主键不认文件顺序");
    // 两份 fixture 的行序刻意不同（文件 1/2/3，PG 数组 1/3/2）：
    // 逐行相等只能来自「按 `Spec::pk` 现算的行键对齐」，不来自顺序巧合。
    for key in &file_keys {
        let left = serde_json::to_string(from_file.get(key).expect("刚列出的键"))
            .expect("行应可序列化");
        let right = serde_json::to_string(from_pg.get(key).expect("刚列出的键"))
            .expect("行应可序列化");
        println!("  行键 {key:<3} 逐字段相等：{}（{} 字节）", left == right, left.len());
        assert_eq!(left, right, "行键 {key} 换源后不该有一个字节不同");
    }
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

    /// 八份 JSON → 领域结构 → 声明式校验全跑一遍。
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
        // 阶段 4.5 转正的首张真实表：同一份 `data/` 目录，同一条加载链。
        assert_eq!(snap.pd_unit_capit_trades.len(), 3);

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