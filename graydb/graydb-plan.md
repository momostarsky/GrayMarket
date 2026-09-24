# GrayDB 推进计划

> 最后更新：2026-09-24　|　分支：master　|　状态：**阶段 0 完成，待阶段 1（内核写路径）**

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
├── Cargo.toml              members = ["account", "graydb"]；根 package GrayMarket
├── src/main.rs             冒烟入口（notional 演示）
├── account/                定点数领域库（可跨项目复用）
│   └── src/
│       ├── amount.rs       ~1120 行：Amount<SCALE,B> + 序列化 + 19 个测试
│       ├── mem_tb_row_pd_unit_capit_trade.rs
│       └── product_info.rs
└── graydb/                 应用 crate（当前 mock 阶段）
    ├── Cargo.toml          deps: account, rust_decimal, serde, serde_json, anyhow
    ├── data/               mock 数据（手写 JSON = 未来 PG 表的投影）
    │   ├── dict/           security.json(2), user.json(3)
    │   └── state/          account.json(3), asset.json(3), position.json(2)
    └── src/
        ├── main.rs         模块声明 + mock_data_tests(2)
        ├── domain/mod.rs   领域结构 + snake_case 枚举
        ├── mem.rs          Snapshot 加载/校验 + 资金三原语 + tests(4)
        ├── journal.rs      JSONL 流水（std::io::Result）
        └── store/mod.rs    Store trait + JsonFileStore（⚠️ 遗留问题 5）
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
| 复合主键 | `position_key(account_id, symbol) = "account:symbol"` 单一构造函数 | `mem.rs` |

### Mock 数据的自洽不变量（已有测试守护）

```text
available + frozen = 初始资金
total_market_value = Σ(quantity × avg_cost)   ← 用 notional() 复算，差 1 分即失败
外键：account.user_id ∈ users；position.(account_id,symbol) ∈ assets × securities
quantity ≥ available_qty
position 主键 == position_key(...)
```

当前测试清单：

| 位置 | 测试 |
|---|---|
| `graydb/src/mem.rs` | `loads_all_mock_json_into_memory`、`tradability_respects_account_and_dict`、`freeze_conserves_available_plus_frozen`、`position_key_is_stable_and_matches_json_layout` |
| `graydb/src/main.rs` | `all_mock_json_files_match_domain_structs`、`asset_invariants_hold` |
| `account/src/amount.rs` | 19 个：标度显示 / 边界 / widen-narrow / 5 种舍入 / Decimal 互转 / notional / 三段式落账 / settle 错误 / 序列化格式 / 越界拒绝 / 枚举 snake_case |

## 三、遗留问题（进入阶段 1 前清完）

| # | 问题 | 处理 |
|---|---|---|
| 1 | `mem.rs` 的 `impl Snapshot` 内有重复 `load_json` / `save_json`（实际调用的是模块级同名自由函数）→ `dead_code` 警告 | 删除 impl 内两个 |
| 2 | `mem.rs::load` 文档写「直接 panic」，实际返回 `Result` | 改为「失败由启动流程决定处置，预期终止启动」 |
| 3 | `domain/mod.rs` 第 64、109、126 行残留 `// ... existing code ...` 粘贴标记 | 删除 |
| 4 | `main.rs` 第 18 行同样的残留标记 | 删除 |
| 5 | `store/mod.rs::load_all` 用 `std::any::type_name::<T>()` 当文件名（路径含泛型名，实际不可用），且与 `mem::Snapshot::load` 语义重叠 | **待定**：A 表名参数化改造 / B 删除（建议 B，阶段 4 接 PG 时重新设计） |
| 6 | `journal.rs` 有 `Order`/`Trade` 类型但无样本文件、无调用方 | 阶段 1 产出 `data/journal/*.jsonl` 并接入 |
| 7 | `Order.created_at: String`，时间类型策略未定（`jiff` / `time` / `chrono`） | 阶段 1 决策；保持 ISO8601 字符串 + 单一转换点 |
| 8 | 资金原语返回 `bool`，丢失了拒单原因 | 阶段 1.2 升级为 `Result<(), RejectReason>` | 
| 10 | **`Snapshot` 硬编码 5 张表**：struct 字段 / load 路径 / check_integrity / save 四处手工同步，表数量增长不可扩展；`#[derive(Default)]` 使「漏注册」不报错 | 阶段 0.5 引入 `tables.rs` 表清单 |

## 四、阶段计划

### ✅ 阶段 0：Mock 数据与内存加载（已完成）

领域结构 + JSON 数据 + 加载即校验 + 不变量测试。

**验收**：`cargo test -p graydb` 6 passed；`cargo clippy -p graydb --all-targets` 无警告（依赖遗留 1–4 清理）。

### 阶段 0.5：表清单与注册中心（**必须先于阶段 1**）

目标：把「一张表 = 四处硬编码」收敛为「一处声明 + 一个 impl」，为多表、订阅、PG 接入解锁。

| 任务 | 说明 |
|---|---|
| 0.5.1 `tables.rs` | `Spec { id, file, kind, policy, pk, fk }` + `TABLES: &[Spec]` 清单；`Kind::{Dict,State}`、`LoadPolicy::{Critical,Optional,Lazy}` |
| 0.5.2 `DataTable` trait | `const ID` 绑定类型与声明；`pk_parts()` 统一主键拼法（取代 `position_key`）；`check_row()` 承载逐行不变量 |
| 0.5.3 通用加载器 | `load_table::<T>()`：读文件 + 「JSON 键 == 主键」机械校验 + `check_row`；漏写 `impl DataTable` 因泛型约束编译失败 |
| 0.5.4 声明式外键校验 | 遍历 `Spec::fk` 的通用检查器，删除 `check_integrity` 里手写的 `for` 循环 |
| 0.5.5 `TableStat` | `{ id, rows, policy, lsn }`：LSN 锚点、内存预算、日终对账的公共底座（现在只填 rows） |
| 0.5.6 分级启动 | `Critical` 失败拒绝启动；`Optional` 失败告警降级；`Lazy` 不进启动清单 |
| 0.5.7 删除 `store/` | `Store<T,K>` + `type_name::<T>()` 方案与 0.5.1 冲突（表名必须显式），见遗留问题 5 |
| 0.5.8 守护测试 | `every_declared_table_is_loaded_and_nonempty_or_marked`、`table_ids_are_unique_and_match_files` |

**验收标准**：
- 新增一张表只需改 `tables.rs`（1 行）+ `domain` 里 1 个 `impl DataTable` + `Snapshot` 1 个字段；
- 守护测试能捕获「声明了没加载」「加载成空表」「主键拼写不符」三类错误；
- 现有 6 个测试语义不变（迁移后仍通过）。

**刻意不做**（避免过度设计）：
- 不引入运行时动态注册 / 插件式表发现 —— 表清单是代码，必须可 grep、可静态核对；
- 不把热路径的 `HashMap<String, Asset>` 改成 `dyn Any` 容器 —— 内核访问必须零开销且类型安全；
- 不做百万行大表的列式/arena 存储 —— 等阶段 4 有真实体量数据再决策（`Kind`/`LoadPolicy` 已预留）。

### 阶段 1：内核写路径（**下一步**）
<!-- ... existing code ... -->
| 1.1 `engine.rs` | 单线程内核：持 `Snapshot` + `seq: u64` + `Journal`，暴露 `place` / `cancel` / `apply_fill`；访问表走 `Snapshot` 强类型字段（零开销） |
<!-- ... existing code ... -->
### 阶段 2：日志与恢复

- 2.1 JSONL 升级为 `(term, seq)` 段文件布局（mmap 追加写）；记录携带 `table: &'static str`（取自 `Spec::id`），回放按表分流；
<!-- ... existing code ... -->
### 阶段 3：订阅分发

- 3.1 在写路径唯一出口挂 `tokio::sync::broadcast<RowChange>`；
- 3.2 协议：`Subscribe { tables, ops, snapshot, columns, filter }`；帧含 `SNAPSHOT_BEGIN / ROW / SNAPSHOT_END` 屏障；
- 3.3 断线按 `seq` 从 ring buffer 补发，补不齐降级为重新下发快照；
- 3.4 慢消费者隔离：broadcast 丢包 → 客户端触发快照重建，不拖累内核主循环；
- 3.5 主题粒度：`table:{spec.id}`、`table:{schema}.*`、`table:*`；`tables` 入参用 `spec_of()` 校验，未知表名直接拒绝订阅；
- 3.6 全量快照下发遍历 `TABLES`，无需为新表改订阅代码。
<!-- ... existing code ... -->
### 阶段 4：接入真实 PG

- 4.1 `tokio-postgres` + `COPY`；`store` 模块此时正式成型（对齐 `Snapshot` 的加载接口）；
- 4.2 日初加载记录 `(lsn, batch_seq)` 锚点，日终校验主数据在窗口内未被改写；锚点落在 `TableStat.lsn`（按表粒度）；
- 4.3 逻辑复制槽（wal2json 验证 → pgoutput 定稿）单向同步主数据变更进内存；PG 表名 = `Spec::id`，订阅槽与清单自动对齐；
- 4.4 日终归档：重放日志 → `COPY` 进 PG，顺带产出 Parquet 冷备；导出 SQL 由 `TABLES` 生成；
- 4.5 替换数据源：`load_table::<T>()` 的文件读取换成 `COPY TO STDOUT` / 流式查询，**表清单与校验逻辑完全复用**，`data/` 退化为测试 fixture。
<!-- ... existing code ... -->
| `SCALE` 不存在数据里，喂错容器 | 同一份 JSON 被按不同口径解读 | 测试已文档化；加载时校验数量级 |
| 表数量增长导致加载/校验/保存三处不同步 | 静默少加载一张表，业务在缺数据的情况下继续跑 | 阶段 0.5 表清单单一事实源 + 守护测试 |
| 表名在 topic / WAL / COPY 三处各写一遍字符串 | 拼写不一致，订阅收不到、归档进错表 | 阶段 0.5：`Spec::id` 作为唯一身份贯穿三处 |
| 所有表同等重要（一张附表坏 = 全停） | 可用性被最弱依赖绑架 | 阶段 0.5.6 分级启动 |
## 四、阶段计划

### ✅ 阶段 0：Mock 数据与内存加载（已完成）

领域结构 + JSON 数据 + 加载即校验 + 不变量测试。

**验收**：`cargo test -p graydb` 6 passed；`cargo clippy -p graydb --all-targets` 无警告（依赖遗留 1–4 清理）。

### 阶段 1：内核写路径（**下一步**）

目标：跑通「下单 → 校验 → 冻结资金 → 模拟成交 → 记账 → 写日志」全链路。

| 任务 | 说明 |
|---|---|
| 1.1 `engine.rs` | 单线程内核：持 `Snapshot` + `seq: u64` + `Journal`，暴露 `place` / `cancel` / `apply_fill` |
| 1.2 `RejectReason` | `InsufficientFunds` / `AccountFrozen` / `UnknownSymbol` / `NotActive` / `BelowLotSize` / `DuplicateOrder`；`try_freeze` 改为 `Result<(), RejectReason>` |
| 1.3 seq 单调 + 空洞检测 | 每笔写占一个 `seq`；跳号即 `panic!`（fail-fast，宁停不脏） |
| 1.4 幂等键 | `order_id` 重复提交 → 返回原单，不重复冻结 |
| 1.5 写路径顺序 | 校验 → **先写 journal** → 改内存 → 回 ACK（将来升级为「+ fsync + 从库确认」） |
| 1.6 持仓更新 | 买入 `quantity += q`、`available_qty` 按 T+1 置 0；`avg_cost` 用 `notional` / `convert` 精确计算 |
| 1.7 事务边界 | 一笔成交引发的多表变更（asset + position + order + trade）打包为同 `seq` 的一组记录 |
| 1.8 清理 | 遗留问题 1–4、6、7 一并解决 |

**验收标准（每条都要有测试）**：

- 全链路后 `available + frozen` 与初始资金守恒；
- 拒单不产生任何状态变化（原子性）；
- 重复 `order_id` 不重复扣款；
- 部分成交（`PartiallyFilled`）下 `filled_qty` 与资金增量一致；
- 撮合规则保持极简：限价单立即全成（价格/时间优先后续迭代）。

### 阶段 2：日志与恢复

- 2.1 JSONL 升级为 `(term, seq)` 段文件布局（mmap 追加写）；
- 2.2 group commit：攒批 `sync_all`，量化 µs 级摊销；
- 2.3 **回放恢复测试**：`replay(journal) == 内存终态`，逐分不差（整个 WAL 设计的守护测试）；
- 2.4 崩溃截断：末条记录不完整时丢弃并停止，不做「猜测式补齐」。

### 阶段 3：订阅分发

- 3.1 在写路径唯一出口挂 `tokio::sync::broadcast<RowChange>`；
- 3.2 协议：`Subscribe { tables, ops, snapshot, columns, filter }`；帧含 `SNAPSHOT_BEGIN / ROW / SNAPSHOT_END` 屏障；
- 3.3 断线按 `seq` 从 ring buffer 补发，补不齐降级为重新下发快照；
- 3.4 慢消费者隔离：broadcast 丢包 → 客户端触发快照重建，不拖累内核主循环；
- 3.5 主题粒度：`table:{schema}.{table}`、`table:{schema}.*`、`table:*`。

### 阶段 4：接入真实 PG

- 4.1 `tokio-postgres` + `COPY`；`store` 模块此时正式成型（对齐 `Snapshot` 的加载接口）；
- 4.2 日初加载记录 `(lsn, batch_seq)` 锚点，日终校验主数据在窗口内未被改写；
- 4.3 逻辑复制槽（wal2json 验证 → pgoutput 定稿）单向同步主数据变更进内存；
- 4.4 日终归档：重放日志 → `COPY` 进 PG，顺带产出 Parquet 冷备；
- 4.5 替换数据源：`Snapshot::load` 由「读文件」变为「查 PG」，`data/` 退化为测试 fixture。

### 阶段 5：高可用

- 5.1 同步复制后才 ACK（同机房 0.1~0.5ms；跨城降为异步，不为容灾牺牲撮合延迟）；
- 5.2 etcd / Consul lease 仲裁 + **fencing**（租约丢失立即自我降级为只读）；
- 5.3 `term` 递增，带旧 term 的请求直接拒绝；
- 5.4 接管演练测试：杀主 → 从接管 → 客户端重试不重复下单；
- 5.5 前置条件：1.4 的幂等键 + 1.3 的 seq 单调，缺这两条不得开始本阶段。

### 阶段 6（可选）：分析出口

- 6.1 日终对账：DataFusion 比对日初/日终快照（`EXCEPT` / `JOIN`）；
- 6.2 盘中只读分析：微批 sink 进 DataFusion 内存表（或 `rusqlite :memory:`）；
- 6.3 归档流水线：日志重放 → DataFusion 转换 → `COPY` / Parquet；
- 6.4 判断准则：点查走内存表（µs 级），聚合/对账才上 DataFusion；若只需简单汇总，单用 `arrow` 不引 `datafusion`（重依赖）。

## 五、常用命令

```powershell
cargo test                      # 全 workspace
cargo test -p account           # 定点数与序列化测试
cargo test -p graydb            # mock 数据 + 内存加载测试
cargo test -p graydb mem::     # 只跑内存镜像测试
cargo clippy --workspace --all-targets
cargo run -p graydb             # GrayDB 应用
cargo run                       # 根 package GrayMarket
```

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
| 日终落库当作唯一持久化 | 崩溃丢失当日全部订单 | 阶段 2 的 WAL + 阶段 5 同步复制 |
| 主从异步复制 | 已 ACK 的订单被抹掉 | 阶段 5.1 |
| 无仲裁的主从 | 脑裂双写，比丢数据更严重 | 阶段 5.2 fencing |
| 双舍入（先中间标度再目标标度） | 差一分钱，清算时才发现 | `convert` 一次完成 + `amount.rs` 测试锁定 |
| `i128 → i64` 静默截断 | 账面值被悄悄改小 | 全 `Option`/`Result`，无饱和路径 |
| 手写 mock JSON 字段/单位错误 | 流程跑一半对不平 | `mock_data_tests` + `mem::tests` |
| `SCALE` 不存在数据里，喂错容器 | 同一份 JSON 被按不同口径解读 | 测试已文档化；加载时校验数量级 |
| seq 空洞（panic 回滚 / 任务丢包） | 日志与内存分叉，无法收敛 | 阶段 1.3 fail-fast |
| 订阅端拖慢内核主循环 | 全市场延迟劣化 | 阶段 3.4 慢消费者隔离 |
| `Rounding` 反序列化退化为默认值 | 静默改变金额 | 已禁 `#[serde(other)]`，未知即 `Err` |