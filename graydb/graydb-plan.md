# GrayDB 推进计划

> 最后更新：2026-09-30　|　分支：main　|　状态：**阶段 2（日志与恢复）完成（mmap 段文件与 group commit 摊销推到阶段 3 前），下一步：阶段 3（订阅分发）或 stub 逐表转正**

## 一、项目定位

GrayDB 是**交易内核 + 主数据镜像 + 归档网关**，分层如下：

| 数据类 | 事实源 | 内存角色 | 持久化路径 | 恢复方式 |
|---|---|---|---|---|
| 用户 / 账户 / 证券字典 | PG（当前用 JSON 模拟） | 只读镜像 | — | 日初重载 |
| 订单 / 成交（当日） | **内存 + WAL 日志** | 主存储 | 日志同步→从库；日终归档→PG | 日志回放 / 从库接管 |
| 订单 / 成交（历史） | PG | — | 日终 COPY 归档 | 直查 PG |
| 持仓 / 资金余额 | 内存（由成交推导） | 主存储 | 随成交日志恢复 | 回放重算 |

**铁律**（贯穿所有阶段）：

1. 内存侧主数据只读，一切变更走 PG → CDC 回流；
2. 资金/持仓的任何写操作必须产生日志记录，日终归档只是日志的投影；
3. 禁止本地写直接改内存表（写回走 PG，内存只回放 WAL）；
4. 精度损失与溢出必须显式（`Option` / `Result`），绝不静默截断或饱和；
5. 订阅/查询永远不成为写路径的一部分（读侧拖慢主循环 = 事故）。

## 二、当前代码状态

### Workspace 结构

```text
GrayMarket/
├── Cargo.toml              members = ["account", "graydb", "codegen"]；根 package GrayMarket
├── .cargo/config.toml      alias: codegen = "run -p codegen"
├── src/main.rs             冒烟入口（notional 演示）
├── account/                定点数领域库（可跨项目复用）
│   └── src/
│       ├── amount.rs       ~1120 行：Amount<SCALE,B> + 序列化 + 32 个测试
│       ├── mem_tb_row_pd_unit_capit_trade.rs
│       └── product_info.rs
├── codegen/                DDL 驱动代码生成器（独立工具 crate，不被运行期依赖）
│   ├── Cargo.toml          deps: serde, toml, anyhow
│   └── src/main.rs         解析 sql/*.sql + tables.toml → 写 graydb/src/generated/；tests(3) 含真实 dump 守护
└── graydb/                 应用 crate（当前 mock 阶段）
    ├── Cargo.toml          deps: account, rust_decimal, serde, serde_json, anyhow
    ├── sql/schema.sql      演示表事实源（受控子集；待真实表全部接替后退役）
    ├── sql/jzdb_prod_schema.sql   真实 pg_dump（jzdb_prod，45 表），重导：`pg_dump -d <库> --schema-only -n jzdb_prod -f ...`
    ├── sql/jzdb_secu_schema.sql   真实 pg_dump（jzdb_secu，157 表）
    ├── sql/jzdb_base_schema.sql   真实 pg_dump（jzdb_base，89 表，其中 tb_error_log 无主键被 exclude）
    ├── tables.toml         策略与类型映射（[[table]] 显式声明 + [[stub]] 整表搬入；source/table 指定 DDL 来源，register 控制是否进 TABLES）
    ├── data/               mock 数据（手写 JSON = 未来 PG 表的投影）
    │   ├── dict/           security.json(2), user.json(3)
    │   └── state/          account.json(3), asset.json(3), position.json(2), order.json(0), trade.json(0)
    └── src/
        ├── main.rs         启动入口（加载 + 启动报告 + 写路径样板 `demo_engine`（含恢复回放 `[8]`）+ mock_data_tests(2)
        ├── generated/      ← codegen 产物（签入库，勿手改）：299 文件 = 7 登记表演示 + 1 试点 + 289 stub 真实表 + specs(TABLES) + mod
        ├── tables.rs       注册中心：Spec / DataTable(+type Column,:RowValidator) / ColVal / ColumnName / RowValidator / load_table / FkIndex / TableStat + 写路径契约（upsert/replace/delete）+ tests(12)
        ├── domain/mod.rs   业务枚举 + 7 个 impl RowValidator（人写不变量，含 Order/Trade）+ pub use generated
        ├── engine.rs       单线程内核：RejectReason / KernelError / Ack / place / apply_fill / cancel / **recover + replay_record**（阶段 2）+ tests(15)
        ├── mem.rs          Snapshot（强类型 `Table<T>` 字段，7 表）+ 声明式校验 + 资金三原语（`Result<(), RejectReason>`）+ tests(4)
        └── journal.rs      WAL：`Record{term,seq,entry}` / `Entry`（表名取自 `DataTable::ID`）/ `GroupWriter`（唯一写入口）/ `commit`（flush + `sync_data`）/ `read`（回放读取 + 截断判定）+ tests(2)
```

### 关键设计决策（已落地）

| 决策 | 内容 | 出处 |
|---|---|---|
| 标度口径 | 金额 2 位 / 价格 4 位 / 中间量 6 位；`Quantity` = 0 | `amount.rs::scale` |
| 后备类型 | 中间计算 `i128`，余额 `i64`（`AtomicI64` 硬约束） | `Backing` trait |
| 转换入口 | `convert<TO,B2>` 一次完成换标度 + 换后备，杜绝双舍入 | `amount.rs` |
| 舍入策略 | 5 种，默认 `MidpointAwayFromZero`；`rescale_down` 手工整数实现 | `amount.rs` |
| 序列化格式 | `Amount` → `{"units":N}`（纯整数，绝不经浮点） | `amount.rs` |
| 枚举序列化 | `Rounding` / `SettleError` / `Side` / `AccountStatus` … → snake_case 字符串 | 两处 |
| 未知即拒绝 | 浮点混入、未知字段、越界值全部 `Err`，不退化为默认值 | 测试锁定 |
| Decimal 依赖 | `serde-with-str`（弃用 `serde-float`，防浮点往返） | 三个 Cargo.toml |
| 错误处理分层 | 冷路径 `anyhow`；journal `std::io::Result`；热路径将来 `RejectReason` | 约定 |
| 复合主键 | `composite_key(&[ColVal]) = "a:b"` 单一构造函数（另有 `composite_key_str(&[&str])` 便捷入口）；`position_key` 已并入 | `tables.rs` |
| 列取值零拷贝 | `ColVal::{Text(&str), Int(i64)}`（`Copy`）：文本借用零拷贝、整型/`Amount.units()` 内联，整型主键得进注册中心而读侧不分配；否决 `Cow<str>`（百万级 TPS 频繁转换不可接受） | `tables.rs` |
| 列访问编译期检查 | `DataTable::Column` 关联枚举取代字符串列名；`column` 匹配变体（拼错不过编），`ColumnName::as_str` 是列名字面量唯一来源，`parse` 由 `ALL` 反查 | `tables.rs` + `domain/mod.rs` |
| 行键单一来源 | 删除手写 `pk_parts()`，行键 = `Spec::pk` 逐列 `column()` → `composite_key`，`verify_primary_keys` 机械核对「声明拼键 == JSON 外层键」 | `tables.rs` |
| 表身份 | `Spec::id` 同时是内存表 / WAL `table` 字段 / 订阅 topic / PG 表名的唯一身份 | `tables.rs::TABLES` |
| 声明↔类型锚定 | `DataTable::ID` ↔ `Spec::id` 加载期双向核对；`Table::parse_column` 把 Spec 列名翻译成枚举，「声明了枚举没有的列」加载即报错（新捕获点） | `tables.rs` + `domain/mod.rs` |
| 外键声明式 | `fk = (本表列, 目标表 id, 目标表列)`；校验只遍历声明，允许成环（载入后统一校） | `FkIndex::check` |
| 列索引只一份 | `ColumnIndex{values, rows}`：成员判定 + 报错能指认到行；不做 `dyn` 行容器 | `tables.rs` |
| 分级启动 | `Critical` 拒启 / `Optional` 告警降级空表并跳过其外键边 / `Lazy` 不读文件 | `read_rows` |
| DDL 驱动 codegen | 独立 `codegen` crate 从 `sql/schema.sql` + `tables.toml` 生成 struct/列枚举/`impl DataTable`/`Spec`；`impl DataTable` 不含校验，`check_row` 默认转调人写的 `RowValidator`，重跑不覆盖业务 | `codegen/` + `graydb/src/generated/` |
| 写路径三入口分工 | `upsert` = 纯插入（行键已占用即拒绝，不许拿它当「插入或更新」）、`replace(old_key, row)` = 显式旧键迁移（行图 + 全部索引一起搬）、`delete(row_key)` = 同步清行清索引；非索引列原地改走 `rows_mut()`；三者入口均强制 `check_row`（防线 1.9/1.10） | `tables.rs::write_row` |
| 扣减必须再过非负守卫 | `Amount::checked_sub` 只挡算术溢出，减成负数是 `Some(负值)`：金额层允许负（盈亏需要），余额/持仓语义不允许。所有扣减统一走 `engine::sub_or_negative`（资金三原语内部同口径 `filter(!is_negative)`）—— 否则「扣成负余额」与「成交量超委托」都判不出来 | 阶段 1（由守护测试抓出） |
| 内核错误二分流 | `KernelError::Reject(RejectReason)`（业务可预期，走 ACK）/ `State(anyhow::Error)`（不变量崩塌）；journal 写失败与 seq 空洞**不进 `Result`**，直接 panic —— 返 `Err` 就会留下「序号已消耗、日志未落盘」的空洞 | `engine.rs` |
| journal 记实体流水 | 落的是 order / trade 整行，不是 asset/position 的逐字段 delta；恢复（阶段 2）= 按流水重建余额，而非把日志数字往内存上抹 | `engine.rs` 模块头 |
| 落账效应单一来源 | 阶段 2.3：把「下单效应 / 成交结算 / 撤单解冻」抽成 `apply_place_effect` / `settle_fill` / `apply_cancel_effect`，live 与回放调同一函数 —— `replay(journal) == 内存终态` 是**构造保证**而非两套算法碰巧算得一样；回放只跑效应，不重做校验、不再写盘 | `engine.rs` |
| 恢复起点是日初镜像 | 日志记实体，资金/持仓由流水推导 → 重放必须从「orders/trades 为空、available 未扣」的日初镜像起，首条 seq 强制为 1；拿当日 `save()` 的脏镜像重放会重复冻结，直接拒收 | `Engine::recover` |
| WAL 表名 = 注册中心身份 | `Entry` 的外部标签由变体名导出（`{"orders":{…}}`），`table_id()` 取 `DataTable::ID`，两者相等由测试钉住；信封里还带 `term`，跨 term / 行内 seq 与信封不符一律拒恢复 | `journal.rs` |
| 截断丢尾必须上报 | 末条无换行 = 崩溃写在半路 → 丢弃该条并停止，`Recovery::dropped_tail` 非空代表「比崩溃前少一笔」，上层必须停服/对从库而不是接着撮合；中间行坏 / seq 不稠密 → `Err`，不做猜测式补齐 | 阶段 2.4 |
| 稠密性按组而非按条 | 一个事务组本就同 seq 写两条（trade → order），合法序列是 `1,1,2,3,3…`；按「每条 +1」判会把正常日志误判为有洞 | 阶段 2.3（由守护测试抽出） |
| 落盘 = flush + `sync_data` | `BufWriter::flush` 只到 OS 页缓存，不算落盘；`commit()` 额外 `sync_data`（不用 `sync_all`：不需刷 mtime，少一次元数据写） | 阶段 2.1b |
| 市值全量重算 | 每次成交后 `recalc_market_value(account)` 按持仓现算，与 `check_valuation` 同一条算式；增量累加会漂移 | `engine.rs` |
| 真实 pg_dump 多源接入 | 每表 `source`（sql 文件）+ `table`（DDL 名，可后缀匹配 schema 限定）；解析器吃多词类型/内联约束/IDENTITY/非建表语句；numeric 映射优先级 `types` 逐列 > `numeric_default` 整表，都缺即报错 | 阶段 0.8 |
| `register=false` 迁移态 | 只生成 struct（含空 `RowValidator`，文件机器独占）不进 `TABLES`/不加载/不校验 —— 绕开「登记即须 Snapshot 字段+数据文件」的全套接入成本，真实表可一张一张搬 | 阶段 0.8 |
| stub 整表搬入 | `[[stub]] source` 把整份 dump 一键展开成占位 struct：全名 Pascal 防跨模块撞名、pk 统一 `row_id`（缺列精确报错、无主键表 `exclude`）、numeric→`Decimal` 无损占位；转正 = 写同表 `[[table]]` 自动让位 | 阶段 0.9 |

### Mock 数据的自洽不变量（已有测试守护）

```text
available + frozen 只随成交变（买入实扣、卖出实收），每步仍守恒  ← mem.rs 资金三原语 + engine 落账
total_market_value = Σ(quantity × avg_cost)         ← `Snapshot::load` 即校验（`check_valuation`），成交后全量重算
外键（全部是 `TABLES` 里的 `fk` 声明，无手写循环）：
  account.user_id     → user_info.user_id
  account.account_id  → account_asset.account_id   ←「账户必须有资金记录」
  asset.account_id    → account_info.account_id
  position.account_id → account_info.account_id
  position.symbol     → dict_security.symbol
quantity ≥ available_qty                            ← `impl RowValidator for Position`
order.filled_qty ≤ order.quantity                    ← `impl RowValidator for Order`（部分成交不变量的单行部分）
每笔写恰好消耗一个 seq，且「已分配 == 已落盘」            ← `engine::alloc_seq` / `commit_journal`（空洞即 panic）
落盘 = flush + `sync_data`，只有它 Ok 才改内存              ← `journal::Journal::commit`（阶段 2.1b）
replay(日初镜像 + journal) == 崩溃前内存（逐分不差）      ← `Engine::recover` + `state_fingerprint`（阶段 2.3 守护测试）
journal 可采信的三个条件：表名已登记 / 行内 seq == 信封 seq / 组间 seq 稠密 ← `Journal::read`
资金/市值 非负                                        ← `impl RowValidator for Asset`
lot_size > 0 && price_tick > 0                       ← `impl RowValidator for Security`
JSON 外层键 == composite_key(Spec::pk 逐列 column())   ← `load_table` 机械校验（`pk_parts` 已删）
每张表行数 == `Spec::expected_rows`                   ← mock 阶段数据回归哨兵（接 PG 后置 None）
```

当前测试清单：

| 位置 | 测试 |
|---|---|
| `graydb/src/tables.rs` | 12 个守护测试：`table_ids_are_unique_and_match_files`、`every_declared_table_is_loaded_and_nonempty_or_marked`、`critical_table_missing_refuses_startup`、`optional_table_degrades_to_empty_and_is_marked`、`fk_violation_is_caught_from_declaration_only`、`primary_key_must_match_json_outer_key`、`composite_key_mixed_segments_render_stably`、`column_name_round_trips_and_rejects_unknown`、`int_primary_key_is_supported_end_to_end`、`generated_column_enums_cover_declared_columns`、`upsert_clears_old_index_values_on_reindex_column`（防线①）、`upsert_enforces_check_row_before_any_mutation`（防线②） |
| `graydb/src/engine.rs` | 15 个全链路测试：`buy_fill_conserves_cash_position_and_seq`、`rejection_leaves_zero_state_delta`、`duplicate_order_id_is_idempotent_not_double_frozen`、`duplicate_trade_id_is_idempotent_not_double_settled`、`partial_fill_then_cancel_settles_exactly`、`partial_sell_fill_keeps_remaining_lock`、`t_plus_one_blocks_same_day_buy_from_selling`、`sell_fill_settles_lock_and_credit`、`fill_validation_rejects_bad_price_and_overfill`、`seq_is_monotonic_and_gapless`、`journal_records_the_whole_group_under_one_seq`（读回取证信封：seq 稠密 + 表名 + 行内 seq）、`weighted_avg_price_rounds_once_at_the_end`、`replay_reconstructs_the_exact_final_state`（阶段 2.3 守护）、`replay_drops_partial_tail_and_stops_at_the_last_complete_record`（阶段 2.4）、`replay_refuses_interior_corruption_and_seq_gap`（坏日志一律拒收） |
| `graydb/src/journal.rs` | `table_tag_matches_registry_identity`（serde 外部标签 == `DataTable::ID` == `Spec::id`）、`record_round_trips_with_envelope`（`{term,seq,entry}` 写读往返） |
| `graydb/src/mem.rs` | `loads_all_mock_json_into_memory`（7 表，orders/trades 日初为空）、`tradability_respects_account_and_dict`、`freeze_conserves_available_plus_frozen`（含 `checked_sub` 允许负值的拦截）、`composite_key_is_stable_and_matches_json_layout` |
| `graydb/src/main.rs` | `all_mock_json_files_match_domain_structs`（改用 `Snapshot::load`）、`asset_invariants_hold`（测试自行复算市值，不复用生产实现） |
| `account/src/amount.rs` | 32 个：标度显示 / 边界 / widen-narrow / 5 种舍入 / Decimal 互转 / notional / 三段式落账 / settle 错误 / 序列化格式 / 越界拒绝 / 枚举 snake_case |
| `codegen/src/main.rs` | 4 个：`parses_real_pg_dump`（jzdb_prod 45 表全解析 + 多词类型/引号列名/内联约束）、`parses_demo_schema`、`stub_expansion_rules_hold`（全名 Pascal 不撞车 + 无 row_id 报错）、`snake_case_handles_acronyms` |

## 三、遗留问题（进入阶段 1 前清完）

| # | 问题 | 处理 |
|---|---|---|
| 1 | ✅ 已清：`mem.rs` 的 `impl Snapshot` 内重复 `load_json` / `save_json` → `dead_code` 警告 | 阶段 0.5 连模块级同名自由函数一起删除，统一走 `load_table` / `save_table` |
| 2 | ✅ 已清：`mem.rs::load` 文档写「直接 panic」，实际返回 `Result` | 改为「失败由启动流程决定处置，预期终止启动」 |
| 3 | ✅ 已清：`domain/mod.rs` 残留 `// ... existing code ...` 粘贴标记 | 删除 |
| 4 | ✅ 已清：`main.rs` 第 18 行同样的残留标记 | 删除 |
| 5 | ✅ 已清：`store/mod.rs::load_all` 用 `type_name::<T>()` 当文件名（路径含泛型名，实际不可用），且与 `Snapshot::load` 语义重叠 | 已整目录删除（0.5.7）；阶段 4 接 PG 时按 `Spec::id` 重新设计 store |
| 6 | ✅ 已清：`journal.rs` 有 `Order`/`Trade` 类型但无样本文件、无调用方 | 阶段 1：`engine` 独占写入，每笔提交落 JSONL（`journal_records_the_whole_group_under_one_seq` 直接读回取证）；`demo_engine` 把 journal 落在临时目录，不脏化 `data/` |
| 7 | ✅ 已清：`Order.created_at: String`，时间类型策略未定 | 按既定裁决保持 ISO8601 字符串，入口集中在 `PlaceRequest::created_at`（单一转换点）；接真实时钟时只改这一处 |
| 8 | ✅ 已清：资金原语返回 `bool`，丢失了拒单原因 | 阶段 1.2：`RejectReason`（14 变体 + `Display`），三原语升为 `Result<(), RejectReason>`；「可用不足」与「冻结账目不平」从此可区分 |
| 9 | ✅ 已清：`Order` / `Trade` 尚无 `impl DataTable`，也不在 `TABLES` 里 | 阶段 1.1：DDL 进 `schema.sql` + `tables.toml` 登记，struct/列枚举/`impl DataTable`/`Spec` 全部 codegen 产出（手写定义已删），`Snapshot` 加两张表字段 |
| 10 | ✅ 已处理：**`Snapshot` 硬编码 5 张表**：struct 字段 / load 路径 / check_integrity / save 四处手工同步；`#[derive(Default)]` 使「漏注册」不报错 | 阶段 0.5 `tables.rs` 表清单单一事实源；`Snapshot` 不再 `impl Default`（空表无法蒙过 `len()` 判定） |

## 四、阶段计划

### ✅ 阶段 0：Mock 数据与内存加载（已完成）

领域结构 + JSON 数据 + 加载即校验 + 不变量测试。

**验收**：`cargo test -p graydb` 6 passed；`cargo clippy -p graydb --all-targets` 无警告（依赖遗留 1–4 清理）。

### ✅ 阶段 0.5：表清单与注册中心（已完成）

目标：把「一张表 = 四处硬编码」收敛为「一处声明 + 一个 impl」，为多表、订阅、PG 接入解锁。

| 任务 | 说明 |
|---|---|
| 0.5.1 `tables.rs` | `Spec { id, file, kind, policy, pk, fk }` + `TABLES: &[Spec]` 清单；`Kind::{Dict,State}`、`LoadPolicy::{Critical,Optional,Lazy}` |
| 0.5.2 `DataTable` trait | `const ID` 绑定类型与声明；~~`pk_parts()` 统一主键拼法~~（**0.6 已删**，行键改由 `Spec::pk` × `column` 现算）；`check_row()` 承载逐行不变量 |
| 0.5.3 通用加载器 | `load_table::<T>()`：读文件 + 「JSON 键 == 主键」机械校验 + `check_row`；漏写 `impl DataTable` 因泛型约束编译失败 |
| 0.5.4 声明式外键校验 | 遍历 `Spec::fk` 的通用检查器，删除 `check_integrity` 里手写的 `for` 循环 |
| 0.5.5 `TableStat` | `{ id, rows, policy, lsn }`：LSN 锚点、内存预算、日终对账的公共底座（现在只填 rows） |
| 0.5.6 分级启动 | `Critical` 失败拒绝启动；`Optional` 失败告警降级；`Lazy` 不进启动清单 |
| 0.5.7 删除 `store/` | `Store<T,K>` + `type_name::<T>()` 方案与 0.5.1 冲突（表名必须显式），见遗留问题 5 |
| 0.5.8 守护测试 | `every_declared_table_is_loaded_and_nonempty_or_marked`、`table_ids_are_unique_and_match_files` |

**验收结果**（2026-09-29）：
- `cargo test -p graydb` 12 passed（守护 6 + mem 4 + mock 2，原 6 个测试语义不变）；`cargo clippy -p graydb --all-targets` 无警告；`cargo run -p graydb` 输出由 `TABLES` 生成的启动报告；
- 守护测试能实际跑通三类错：`primary_key_must_match_json_outer_key`（主键拼写不符）、`every_declared_table_is_loaded_and_nonempty_or_marked`（声明了没加载 / 加载成空表）、`table_ids_are_unique_and_match_files`（声明与领域类型不一一对应）；
- 新增一张表的改动面：`TABLES` 1 行 + `domain` 1 个 `impl DataTable` + `Snapshot` 1 字段 + `load`/`fk_index`/（若可变）`save` 各 1 行。后三处是保留热路径强类型的必然代价，由「不 `impl Default`」+守护测试保证漏加载不会静默。

**与原计划的偏差**（已验证更优，后续阶段据此对齐）：
- `Spec::fk` 从 `(列, 目标表)` 扩为 `(本表列, 目标表 id, 目标表列)`：目标列必须显式给出，才能机械核对「外键指向的是主键列」；
- 新增 `Spec::expected_rows`（mock 阶段的行数哨兵，接真实库后置 `None` 交给日终对账）；
- 外键校验不再要求声明顺序：`account_info ↔ account_asset` 互指是关系型正常建模，校验在所有表载入后统一跑；
- 市值不变量从测试上升为加载即校验（`Snapshot::check_valuation`），但 `asset_invariants_hold` 仍保留并自行复算一遍，避免测试只重跑生产实现而失去发现力。

**刻意不做**（避免过度设计）：
- 不引入运行时动态注册 / 插件式表发现 —— 表清单是代码，必须可 grep、可静态核对；
- 不把热路径的 `HashMap<String, Asset>` 改成 `dyn Any` 容器 —— 内核访问必须零开销且类型安全；
- 不做百万行大表的列式/arena 存储 —— 等阶段 4 有真实体量数据再决策（`Kind`/`LoadPolicy` 已预留）。

### ✅ 阶段 0.6：列访问契约修订（ColVal + 每表列枚举，已完成）

背景：面向百表规模，手写 `impl DataTable` 存在「列名 ↔ 字段多处同步、仅运行期校验、整型主键无法表达」三类隐患。分两步独立合入，每步 `test`/`clippy` 全绿。

| 任务 | 说明 |
|---|---|
| 0.6.1 `ColVal` | `enum ColVal{ Text(&str), Int(i64) }`（`Copy`）：`column` 读侧零分配，整型/`Amount.units()` 主键可进注册中心；`composite_key` 改收 `&[ColVal]`，新增 `composite_key_str` 便捷入口 |
| 0.6.2 删 `pk_parts` | 行键唯一由 `Spec::pk` 逐列 `column()` 现算；`verify_primary_keys` 三方比对降为两方（保留「JSON 外层键 == 复合主键」守护）；`FkIndex` 不动 |
| 0.6.3 `ColumnName` 桥接 | `DataTable::Column` 关联枚举取代字符串列名；`column` 参数升级为每表列枚举（`UserColumn`…），`as_str` 为列名字面量唯一来源，`parse` 默认由 `ALL` 反查 |
| 0.6.4 `parse_column` 守卫 | `upsert`/`build_columns`/`verify_primary_keys` 在加载边界把 `Spec` 字符串翻译成枚举，未定义即 `Err` |

**验收结果**（2026-09-29）：`cargo test -p graydb` 15 passed（tables 9 + mem 4 + mock 2）；`cargo clippy -p graydb --all-targets` 无警告；新增 `int_primary_key_is_supported_end_to_end`（整型主键全链路）、`composite_key_mixed_segments_render_stably`（Text+Int 混拼）、`column_name_round_trips_and_rejects_unknown`（as_str↔parse 往返 + 未知列名拒）。

**为下一步铺路**：手写的 `XxxColumn` enum + `ColumnName` impl 是纯机械样板，正是 0.7 codegen 要生成的目标；接口已就绪，替换手写不改调用方。

### ✅ 阶段 0.7：DDL 驱动 codegen（已完成）

目标：把「每表 struct 字段 + `XxxColumn` enum + `ColumnName` impl + `impl DataTable` + `Spec` 条目」交给代码生成，列名/类型从 PG DDL 单一事实源产出，人只保留 `check_row` 业务不变量。

| 任务 | 说明 |
|---|---|
| 0.7.1 事实源 | `sql/schema.sql`（`pg_dump --schema-only` 维护，DDL 列名 = 唯一来源，含引号大小写如 `"Symbol"`）+ `tables.toml`（策略 `file/kind/policy` 与 `numeric→Money/Price/Quantity` 类型映射，DDL 推不出的语义在此声明） |
| 0.7.2 codegen 二进制 | 独立 crate `codegen`（非 proc-macro：增量编译友好）；解析受控的 `CREATE TABLE` 子集；**每表产出一个 `src/generated/<type>.rs`**，改一张表只重编一个文件 |
| 0.7.3 产物 | struct + `#[serde(rename)]` + `XxxColumn` enum + `ColumnName` impl（`as_str` 携带精确列名，字段↔列名同源不可能写错）+ `impl DataTable`（`check_row` 委托给旁路 `RowValidator`）+ `Spec`/`TABLES` 条目 |
| 0.7.4 业务钩子 | `tables.rs` 加 `trait RowValidator{ fn validate_row(&self)->Result<()>{Ok(())} }`，人把 `check_row` 逻辑迁到 `impl RowValidator for Xxx`；样板归宏、不变量归人 |
| 0.7.5 防漂移 | `.cargo/config.toml` 别名 `cargo codegen`；CI 跑 codegen 后 `git diff --exit-code generated/`，DDL 改了忘重跑即挂 |
| 0.7.6 `Snapshot` 仍手写 | 强类型字段 + 语义方法（`position`/`is_tradable`）不生成；守护测试扩一条「`generated` 每张表都在 `Snapshot` 有 register」 |

**验收结果**（2026-09-29）：新增独立 `codegen` crate + `sql/schema.sql` + `tables.toml` + `.cargo/config.toml` 别名；`cargo codegen` 一次产出 5 表到 `graydb/src/generated/`（struct/列枚举/`ColumnName`/`impl DataTable`）+ `specs.rs`（`TABLES`）+ `mod.rs`；`domain/mod.rs` 改为 `pub use` 生成物 + 保留业务枚举与 `impl RowValidator`；`tables.rs` 手写 `TABLES` 换为 `pub use crate::generated::TABLES`。`cargo test -p graydb` 16 passed（tables 10 + mem 4 + mock 2）——**关键证明：换成生成样板后，加载/主键/外键/市值全部测试零修改通过**；`cargo clippy --workspace --all-targets` 零警告；`cargo run -p graydb` 输出不变。numeric/浮点列无显式映射时 codegen 直接报错（不猜、不静默落浮点）。

**为下一步铺路**：codegen 从此直接吃 `pg_dump --schema-only` 产物（多源），真实表用 `register=false` 渐进接入 —— 见阶段 0.8。

### ✅ 阶段 0.8：codegen 接真实 pg_dump（jzdb_prod 试点，已完成）

背景：真实库 `jzdb_prod`（同库另有 `jzdb_base`/`jzdb_secu`，表最少）已有 45 张表的 `pg_dump` 产物 `sql/jzdb_prod_schema.sql`。目标：生成器直接吃 dump，真实表渐进接入，不动已登记的 5 张演示表。

| 任务 | 说明 |
|---|---|
| 0.8.1 多源解析 | 每表 `source`（`sql/` 下文件名，默认 `schema.sql`）+ `table`（DDL 名，默认 = `id`）；同名源只解析一次；后缀匹配允许写 `tb_x` 命中 `jzdb_prod.tb_x`，歧义即报错并列出可用表 |
| 0.8.2 pg_dump 抗性 | `extract_type` 吃多词类型（`character varying(64)`）与括号精度，遇约束词（NOT/DEFAULT/CONSTRAINT/GENERATED…）截断；表级约束、`ALTER TABLE OWNER/IDENTITY`、`\restrict` 等非建表语句天然跳过 |
| 0.8.3 缩略语字段名 | `to_snake` 重写：`SEC_charges` → `sec_charges`（旧版会碎成 `s_e_c__charges`），`serde(rename)` 回原列名 |
| 0.8.4 numeric 批量映射 | `numeric_default` 整表默认（试点表 30 个金额列→`Money`，与 `account::MemTbRowPdUnitCapitTrade` 既有裁决一致）；个别列（如利率）用 `types` 逐列覆盖；两者都缺仍报错 |
| 0.8.5 `register=false` | 只生成 struct 不登记 `TABLES`（避开守护测试的「登记即须 Snapshot 字段 + 数据文件」全套接入）；文件机器独占故空 `impl RowValidator` 也生成；转正时改 true，人在 `domain` 接手校验 |
| 0.8.6 守护测试 | codegen 新增 3 测：真实 dump 45 表全解析（含逐列抽查）、演示 schema 回归、缩略语蛇形 |

**验收结果**（2026-09-29）：`cargo test -p codegen` 3 passed（含 45 表/45 列逐项断言）；`cargo codegen` 产 6 表（5 登记 + 试点 `pdunitcapittrade.rs` 不登记，`specs.rs` 验证无 `capit_trade`）；试点 struct 与 DDL 逐列吻合（`bigint→i64`、`integer→i32`、`varchar→String`、30 个 `numeric→Money`、`"SEC_charges"` 带 rename、`row_id` 主键进 `ColVal::Int`）；`cargo test -p graydb` 16 passed 零修改；`cargo clippy --workspace --all-targets` 零警告。

### ✅ 阶段 0.9：stub 整表搬入（prod + secu 共 201 张真实表，已完成）

背景：secu dump 到位（157 表）后共 202 张真实表，逐表手写 `[[table]]` 不可持续。codegen 新增 `[[stub]] source = "..."` 模式：整份 dump 一键展开成占位 struct，机器裁决固定规则，人只在转正时介入。

| 规则 | 内容 |
|---|---|
| 命名 | id = 物理表名；struct = 去 `tb_` 的**全表名 Pascal（保留模块段）** —— 侦察发现去模块后 `pdmage/pdswap/pdotcsecu` 的同名 `pd_unit_capit` 撞车，全名规则数学上唯一；stem 重复即报错 |
| 主键 | 统一 `row_id`（库里每表都有 bigint identity）；缺列即精确报错指向 `[[table]]`/`exclude` 出路 |
| numeric | → `Decimal` 无损占位：库里 scale 从 2 到 12（利率 8/12 位），统一落 Money(2) = 静默丢精度违反铁律 4；转正时逐列裁决成 Money/Price/Quantity |
| 边界 | stub 永远 `register=false`（不进 TABLES/不加载/不校验，文件机器独占含空 `impl RowValidator`）；同表写了显式 `[[table]]` 则 stub 自动让位 —— 转正零协调 |

**验收结果**（2026-09-29）：`cargo codegen` 产 207 表（5 登记 + 202 未登记）→ `generated/` 209 文件；`cargo test -p codegen` 4 passed（新增 `stub_expansion_rules_hold`）；`cargo test -p graydb` 16 passed 零修改（200+ 个新 struct 全部参与编译）；`cargo clippy --workspace --all-targets` 零警告；`specs.rs` 仍恰 5 条。

### ✅ 阶段 0.9.1：jzdb_base 接入（三 schema 289 张搬齐，已完成）

`tables.toml` 加第三行 `[[stub]] source = "jzdb_base_schema.sql"` 即完成接入 —— 守卫按设计工作：全库唯一缺 `row_id` 的表是 **`tb_error_log`**（单列 `remark_info` 的日志黑洞表，无合理行键），以 `exclude = ["tb_error_log"]` 排除，将来要接它先由业务定身份列。

**验收结果**（2026-09-29）：base 88 张落盘 → `generated/` 297 文件（5 登记 + 1 试点 + 289 stub + specs + mod + 其余为演示产物）；`cargo test --workspace` 52 passed（account 32 + graydb 16 + codegen 4）零修改；`cargo clippy --workspace --all-targets` 零警告；`specs.rs` 仍恰 5 条；`cargo run -p graydb` 输出不变。真实表三 schema 已全部在册，后续只涉及逐表转正与 Snapshot 接线。

### ✅ 阶段 1：内核写路径（已完成）

目标：跑通「下单 → 校验 → 冻结资金 → 模拟成交 → 记账 → 写日志」全链路。

| 任务 | 说明 |
|---|---|
| 1.1 `engine.rs` | ✅ 单线程内核：持 `Snapshot` + `seq: i64`（跟 DDL `bigint` 对齐，原计划的 `u64` 作废）+ `Journal`，暴露 `place` / `cancel` / `apply_fill` / `place_and_fill`；访问表走 `Snapshot` 强类型字段（零开销）；`orders` / `trades` 同时登记进 `TABLES`（遗留 9）。入参用具名 `PlaceRequest`（7 个位置参数极易调错序），返回 `Ack{Accepted/Duplicate{seq}}` |
| 1.2 `RejectReason` | ✅ `engine.rs` 里 14 个变体（计划 6 个 + 实现全链路必然要区分的 `InsufficientPosition` / `InvalidPrice` / `InvalidFillPrice` / `UnknownOrder` / `OrderNotLive` / `OverFill` / `FrozenUnderflow` / `DuplicateFill`），带 `Display`；`try_freeze` / `settle_frozen_out` / `unfreeze` 均改为 `Result<(), RejectReason>`（宁多列不合并成 `Other`） |
| 1.3 seq 单调 + 空洞检测 | ✅ `alloc_seq` 先校「已分配 == 已落盘」，不等即 `panic!`（fail-fast，宁停不脏）；`commit_journal` 写失败同样 panic —— 这两个分支故意不返 `Result`，返 `Err` 就是把空洞留给下一步 |
| 1.4 幂等键 | ✅ `order_id` 重复提交 → `Ack::Duplicate{原 seq}`，不重复冻结；同键不同内容 → `DuplicateOrder` 拒。**实现时发现仅靠 `order_id` 不够**：成交侧若由内核自己拼 `trade_id`，同一回报重放两次会拼出另一个号、静默重复扣款，所以 `apply_fill(trade_id, …)` 改成外部传入成交号（对应交易所 exec id），同交易同样幂等 |
| 1.5 写路径顺序 | ✅ 校验 → **先写 journal**（一次 `flush`）→ 改内存 → 回 ACK（将来升级为「+ fsync + 从库确认」）；journal 记 order/trade 整行实体，恢复口径 = 按流水重建余额 |
| 1.6 持仓更新 | ✅ 买入 `quantity += q`、`available_qty` 按 T+1 不变（新建仓位为 0）；`avg_cost` 走 `weighted_avg_price`：旧成本与新成本都先落 `MicroAmount`（6 位）相加，除以总数量后**只一次** `Price::from_decimal_with(…, MidpointAwayFromZero)`；新增行走 `upsert`、改已有行走 `replace`（两者区分已由机制拦截，不能混用）；卖完 `delete` 不留零行（注意不能按 `fully_filled` 删 —— 全成但仓上还剩货必须保留） |
| 1.7 事务边界 | ✅ 一笔成交引发的多表变更（asset + position + order + trade）共用同 `seq`，整组（trade → 更新后 order）一次 `flush` 落盘后才改内存 |
| 1.8 清理 | ✅ 遗留问题 6、7、8、9 已全部关闭（1–5、10 已在阶段 0.5 清完，见本节上方遗留表） |
| 1.9 `upsert` 防线①：索引清旧值 | ✅ 已清（下列为缺陷记录）：原 `ColumnIndex::push` **只增不删**（append-only）—— 同一行二次写入改已索引列后，旧值仍被 `has` 命中（假阳性，外键校验会放过已不存在的引用）。落地选了「先清旧」而非「禁止改已索引列」：新增 `ColumnIndex::remove_row(row_key)`（`retain` + 重建 `values`），`write_row` 入口先按 `row_key` 抹掉本列旧对再插新；改主键时旧行条目连同**全部**索引一起迁走（这一条正是守护测试最先抓到的：`rows.insert(新键)` 不删旧键 → 行图与索引双失真） |
| 1.10 `upsert` 防线②：强制 `check_row` | ✅ 已清：`write_row` 第一行即 `row.check_row()?`（先于一切变更，包含算行键与建 `fresh` 列表）；分工从注释升为机制：`upsert` = 纯插入且行键已被占用即拒（不再当「插入或更新」用）、`replace(old_key, row)` = 显式旧键迁移、`delete(row_key)` 同步清行清索引、非索引列原地改走 `rows_mut()` |

**验收标准（每条都要有测试）**：

- 全链路后 `available + frozen` 与初始资金守恒；
- 拒单不产生任何状态变化（原子性）；
- 重复 `order_id` 不重复扣款；
- 部分成交（`PartiallyFilled`）下 `filled_qty` 与资金增量一致；
- 撮合规则保持极简：限价单立即全成（价格/时间优先后续迭代）；
- （1.9）同一行二次 `upsert` 改已索引列：旧值 `has == false`、索引 `(key, value)` 对数与实际行数一致；
- （1.10）违反 `check_row` 的行经 `upsert` 写入必 `Err`，且表内容与索引均未被触碰。

**验收结果**（2026-09-29）：

- `cargo test --workspace` **66 passed**（account 32 + graydb 30 + codegen 4），其中 `engine.rs` 新增 12 个全链路测试；`cargo clippy --workspace --all-targets` 零警告；`cargo run -p graydb` 多一段 `[7] 内核写路径`（买 100 股 6.06：可用 992798.00 → 992192.00，新持仓可卖 0，市值 7202.00 → 7808.00，orders=1 trades=1，高价大额单被拒且序号不动）。
- 七条验收逐条对应测试：守恒 `buy_fill_conserves_cash_position_and_seq` / 拒单零变化 `rejection_leaves_zero_state_delta`（7 类拒单共用一份前置镜像比对）/ 幂等 `duplicate_order_id_*` + `duplicate_trade_id_*` / 部分成交 `partial_fill_then_cancel_settles_exactly` + `partial_sell_fill_keeps_remaining_lock` / T+1 `t_plus_one_blocks_same_day_buy_from_selling` / 序号无空洞 `seq_is_monotonic_and_gapless` + `journal_records_the_whole_group_under_one_seq` / 单舍入 `weighted_avg_price_rounds_once_at_the_end`；防线①② 在 `tables.rs`。
- **守护测试抽出的真缺陷（两个，同一根因）**：`Amount::checked_sub` 只挡算术溢出，减成负数是 `Some(负值)`。于是 ① 资金原语能把 `available` 扣成负数（旧 `-> bool` 测试也没发现，因为它只测了溢出分支）；② `apply_fill` 拿 `quantity.checked_sub(fill_qty)` 判「超量成交」永远不成立（OverFill 分不出），反而在下游报成 `FrozenUnderflow`。修法：所有扣减再过一道 `!is_negative`（`engine::sub_or_negative`），超量改用 `Ord` 比较。这类「金额层允许负、余额语义不允许负」的缺口，单看 `amount.rs` 的 API 看不出来，必须靠带真实数值的守恒测试反推。
- 遗留 6–9 全部关闭；`Order.seq` 类型由手写时代的 `u64` 统一为 codegen 从 DDL `bigint` 推出的 `i64`（`Seq` 列不开放取值：不在 pk/fk 声明里，`column()` 返回 `None`）；`expected_rows = 0` 是**日初**哨兵，当日撮合后 `save()` 回盘再 `load()` 会被行数哨兵拒启 —— 跨日重启需先做日切归档（阶段 2/4 一并处理）。

### ✅ 阶段 2：日志与恢复（已完成 2.1 / 2.1b / 2.1c / 2.3 / 2.4，mmap 段文件与 group commit 摊销推迟）

目标：让「先写日志再改内存」真的守得住 —— 范围裁决为先做**恢复语义**（信封 + 真落盘 + 回放守护测试 + 截断处置），性能改造推到阶段 3 前（现在只需顺序回放，mmap 与攒批的收益要等按 `seq` 随机补发时才兑现）。

| 任务 | 说明 |
|---|---|
| 2.1 WAL 信封 | `Record{term, seq, entry}` 一行一条，`entry` 是 `Entry::{Orders, Trades}` 外部标签 enum（落盘形如 `{"term":0,"seq":1,"entry":{"orders":…整行…}}`）。表名不另写字面量：`Entry::table_id()` 取 `DataTable::ID`，与 serde 由变体名导出的标签由 `table_tag_matches_registry_identity` 钉死相等 → WAL 表名 == 注册中心 == 订阅 topic == PG 表名，不可能写岔 |
| 2.1b 真落盘 | `Journal::commit()` = `BufWriter::flush` + `File::sync_data`（不用 `sync_all`：不需要刷 mtime，少一次元数据写）。阶段 1.5 那句「先写日志再改内存」的「写」到这一层才算真的写完 |
| 2.1c 唯一写入口 | `GroupWriter{journal, term, seq}` 字段全私有、只有 `pub(crate) fn new`：一个事务组内 `group.trade(…)?; group.order(…)` 都盖同一个 seq —— 「带信封写入」是结构上唯一的路，「忘了 sync」写不出来 |
| 2.3 回放恢复 | `Engine::recover(day_open_snapshot, path) -> Recovery{engine, replayed, dropped_tail}`。`replay_record` 分流：orders 首见 = 下单流水（重新冻结）、再见 = 只搬行（仅「活单 → Cancelled」那一跳施加解冻）、trades = 结算 + `upsert`（trade 记录排在同组 order 之前，故成交时表里仍是**成交前**那行，委托价/可卖量与当时一致）。落账效应抽成 `apply_place_effect` / `settle_fill` / `apply_cancel_effect`，live 与回放调同一函数 —— `replay == 终态` 是构造保证而非两套算法碰巧算得一样 |
| 2.4 截断与拒收 | `Journal::read` 的采信判定：末条无换行 = 崩在写入中途 → 丢弃并停止（`dropped_tail` 上报，非空代表比崩溃前少一笔，上层必须停服 / 对从库而不是接着撮合）；中间行解析失败 / 表名未登记 / 行内 seq ≠ 信封 seq / 组间 seq 不稠密 / 跨 term → 直接 `Err`，不做「猜测式补齐」 |
| 2.2 group commit | ⏸ 推迟：当前每笔一次 `sync_data`，正确性已足；攒批摊销要等阶段 3 有多消费者压力测试再量化 |
| 2.5 mmap 段文件 | ⏸ 推迟：`(term, seq)` 段文件 + 按 seq 随机读的收益要等阶段 3 补发（ring buffer 溢出后回读段文件）才兑现 |

**验收结果**（2026-09-30）：

- `cargo test --workspace` **71 passed**（account 32 + graydb 35 + codegen 4），其中 `journal.rs` 新增 2 个、`engine.rs` 12 → 15（回放三件）；`cargo clippy --workspace --all-targets` 零警告；`cargo run -p graydb` 新增 `[8] 日志恢复` 一段：回放 3 条记录、丢尾=false，序号 2 → 2、orders 1→1、trades 1→1，可用 992192.00 ↔ 992192.00、冻结 0.00 ↔ 0.00、市值 7808.00 ↔ 7808.00，持仓 qty 100 ↔ 100、可卖 0 ↔ 0、avg_cost 6.06 ↔ 6.06，恢复后的引擎还能继续 `place`。
- 比对口径：`state_fingerprint` 用 `serde_json::to_value` 把四张可变表（assets / positions / orders / trades）整表取出来逐字段比（`Amount` 序列化为 `{"units":N}` 纯整数，故「指纹相等」== 逐分不差），不是挑几个字段抽查。
- **守护测试抽出的三个真问题**：① `BufWriter::flush` 不是落盘（阶段 1 遗留，无 fsync → 掉电丢已 ACK 的组）→ 2.1b 补 `sync_data`；② 稠密性最初按「每条记录 +1」判定，而 1.7 的事务组本就同 seq 写两条，正常日志被误判有洞 → 改成 `seq == last.seq || last.seq + 1`（合法序列 `1,1,2,3,3…`）；③ 误以为 `#[serde(rename_all)]` 会产出内部字段 `"table"`，普通 enum 得到的是**外部标签** → 表名校验改走 `Entry::table_id()`，测试断言 entry 对象的唯一键。
- 口径纠正：`Recovery::replayed` 数的是**记录条数**（该场景 11 条），`seq` 数的是**事务组数**（8 组）—— 三个成交组各 2 条，两者天然不相等。
- 遗留（不在本阶段做）：恢复起点强制「日初镜像 + 首条 seq=1」（拿当日 `save()` 的脏镜像重放会重复冻结，`recover` 直接拒收），而 `expected_rows` 的日初哨兵使脏镜像无法 `load()` → 真跨日重启需先做日切归档（阶段 4 接 PG 时一并处理）；跨 term 日志与从库接管属阶段 5。

### 阶段 3：订阅分发

- 3.1 在写路径唯一出口挂 `tokio::sync::broadcast<RowChange>`；
- 3.2 协议：`Subscribe { tables, ops, snapshot, columns, filter }`；帧含 `SNAPSHOT_BEGIN / ROW / SNAPSHOT_END` 屏障；
- 3.3 断线按 `seq` 从 ring buffer 补发，补不齐降级为重新下发快照；
- 3.4 慢消费者隔离：broadcast 丢包 → 客户端触发快照重建，不拖累内核主循环；
- 3.5 主题粒度：`table:{spec.id}`、`table:{schema}.*`、`table:*`；`tables` 入参用 `spec_of()` 校验，未知表名直接拒绝订阅；
- 3.6 全量快照下发遍历 `TABLES`，无需为新表改订阅代码。

### 阶段 4：接入真实 PG

- 4.1 `tokio-postgres` + `COPY`；`store` 模块此时正式成型（对齐 `Snapshot` 的加载接口）；
- 4.2 日初加载记录 `(lsn, batch_seq)` 锚点，日终校验主数据在窗口内未被改写；锚点落在 `TableStat.lsn`（按表粒度）；
- 4.3 逻辑复制槽（wal2json 验证 → pgoutput 定稿）单向同步主数据变更进内存；PG 表名 = `Spec::id`，订阅槽与清单自动对齐；
- 4.4 日终归档：重放日志 → `COPY` 进 PG，顺带产出 Parquet 冷备；导出 SQL 由 `TABLES` 生成；
- 4.5 替换数据源：`load_table::<T>()` 的文件读取换成 `COPY TO STDOUT` / 流式查询，**表清单与校验逻辑完全复用**，`data/` 退化为测试 fixture。

### 阶段 5：高可用

- 5.1 同步复制后才 ACK（同机房 0.1~0.5ms；跨城降为异步，不为容灾牺牲撮合延迟）；
- 5.2 etcd / Consul lease 仲裁 + **fencing**（租约丢失立即自我降级为只读）；
- 5.3 `term` 递增，带旧 term 的请求直接拒绝；
- 5.4 接管演练测试：杀主 → 从接管 → 客户端重试不重复下单；
- 5.5 前置条件：1.4 的幂等键 + 1.3 的 seq 单调，缺这两条不得开始本阶段。

### 阶段 6（可选）：分析出口

- 6.1 日终对账：DataFusion 比对日初/日终快照（`EXCEPT` / `JOIN`），行数用 `TableStat.rows` 对 PG `count(*)`；
- 6.2 盘中只读分析：微批 sink 进 DataFusion 内存表（或 `rusqlite :memory:`）；
- 6.3 归档流水线：日志重放 → DataFusion 转换 → `COPY` / Parquet；
- 6.4 判断准则：点查走内存表（µs 级），聚合/对账才上 DataFusion；若只需简单汇总，单用 `arrow` 不引 `datafusion`（重依赖）。

## 五、常用命令

```powershell
cargo test                      # 全 workspace
cargo test -p account           # 定点数与序列化测试
cargo test -p graydb            # 35 个测试：tables 12 + engine 15 + journal 2 + mem 4 + mock 2
cargo test -p graydb tables::    # 只跑表清单/注册中心守护测试
cargo test -p graydb engine::    # 只跑内核写路径与回放恢复测试
cargo test -p graydb journal::   # 只跑 WAL 信封 / 落盘 / 截断判定测试
cargo clippy --workspace --all-targets
cargo codegen                   # 改 DDL/tables.toml 后重生成 graydb/src/generated（= run -p codegen）
cargo run -p graydb             # 加载 + 按 TABLES 打印启动报告 + `[7]` 写路径 + `[8]` 日志恢复
cargo run                       # 根 package GrayMarket
```

> PowerShell 控制台乱码时先 `chcp 65001`（Rust 输出 UTF-8，GBK 代码页会把中文解成乱码）。

RustRover：Cargo 面板 → `graydb > tests` 可批量运行；`Cargo.toml` 变更后需 Reload。

## 六、单位速查（写数据最容易错的地方）

| 类型 | SCALE | 示例值 | `units` |
|---|---|---|---|
| `Money` | 2 | 992,798.00 元 | `99279800` |
| `Price` | 4 | 9.0025 元 | `90025` |
| `Quantity` | 0 | 800 股 | `800` |
| `MicroAmount` | 6 | 12,006.00 元 | `12006000000` |

## 七、风险清单

| 风险 | 影响 | 缓解 |
|---|---|---|
| 日终落库当作唯一持久化 | 崩溃丢失当日全部订单 | 阶段 2 WAL + 回放已具备（`replay(日初镜像 + journal) == 崩溃前内存`）；阶段 5 同步复制从库 |
| 把 `BufWriter::flush` 当落盘 | 数据只到 OS 页缓存，掉电/kill 后丢掉已回 ACK 的事务组 | ✅ 已缓解（阶段 2.1b）：落盘 = `flush` + `sync_data`，且只有它 `Ok` 才改内存 |
| 主从异步复制 | 已 ACK 的订单被抹掉 | 阶段 5.1 |
| 无仲裁的主从 | 脑裂双写，比丢数据更严重 | 阶段 5.2 fencing |
| 双舍入（先中间标度再目标标度） | 差一分钱，清算时才发现 | `convert` 一次完成 + `amount.rs` 测试锁定 |
| `i128 → i64` 静默截断 | 账面值被悄悄改小 | 全 `Option`/`Result`，无饱和路径 |
| 手写 mock JSON 字段/单位错误 | 流程跑一半对不平 | `mock_data_tests` + `mem::tests` + `Spec::expected_rows` 行数哨兵 |
| `SCALE` 不存在数据里，喂错容器 | 同一份 JSON 被按不同口径解读 | 测试已文档化；加载时校验数量级 |
| 表数量增长导致加载/校验/保存三处不同步 | 静默少加载一张表，业务在缺数据的情况下继续跑 | ✅ 已缓解：阶段 0.5 表清单单一事实源 + 守护测试 |
| 表名在 topic / WAL / COPY 三处各写一遍字符串 | 拼写不一致，订阅收不到、归档进错表 | ✅ 已缓解：`Spec::id` 作为唯一身份贯穿三处 |
| 所有表同等重要（一张附表坏 = 全停） | 可用性被最弱依赖绑架 | ✅ 已缓解：`LoadPolicy` 分级启动（`Critical`/`Optional`/`Lazy`） |
| 表名/列名在多处手写样板（column 字面量 + enum + serde）各自写错 | 一致地错，运行期难发现 | ✅ 已缓解：阶段 0.7 codegen 从 DDL 单一源生成，重跑 `cargo codegen` + CI `git diff` 防漂移 |
| `Table::upsert` 索引只增不删、入口不校验 | 写路径一开即脏写入口 + 外键假阳性 | ✅ 已处：阶段 1.9–1.10（入口强制 `check_row`、索引清旧值、`upsert`/`replace`/`delete` 分工），各配守护测试 |
| seq 空洞（panic 回滚 / 任务丢包） | 日志与内存分叉，无法收敛 | 阶段 1.3 fail-fast |
| 把 `Amount::checked_sub` 当「不够减」用 | 余额/持仓被扣成负值，而金额层不报错；超量成交这类拒单分不出，到下游才变成不相干的错误 | 阶段 1：扣减统一过 `!is_negative`（`sub_or_negative`），「够不够」用 `Ord` 比较 |
| 订阅端拖慢内核主循环 | 全市场延迟劣化 | 阶段 3.4 慢消费者隔离 |
| `Rounding` 反序列化退化为默认值 | 静默改变金额 | 已禁 `#[serde(other)]`，未知即 `Err` |