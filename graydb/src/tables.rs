//! 表清单与注册中心 —— 「一张表 = 一处声明 + 一个 impl」。
//!
//! 声明与类型通过 `DataTable::ID` 互相锚定：
//! - 漏写 `impl DataTable` → 泛型约束不满足，编译失败；
//! - 漏登记 `Spec` → 加载期 `unknown table` 报错 + 守护测试失败；
//! - 主键列名写错 / `column` 未覆盖声明列 → 加载期按 `Spec::pk` 逐列取值拼键即暴露。

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::Path;

use serde::de::DeserializeOwned;

/// 数据来源与可变性：`Dict` 只读镜像（PG → CDC），`State` 内核独占写。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Dict,
    State,
}

/// 分级启动策略。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadPolicy {
    /// 失败即拒绝启动（默认：带缺数据开盘比不启动危险）。
    Critical,
    /// 失败告警并降级为空表，允许启动。
    Optional,
    /// 不进启动清单，按需延迟加载。
    Lazy,
}

/// 一张表的完整声明。
#[derive(Debug, Clone, Copy)]
pub struct Spec {
    /// 唯一身份，贯穿内存表 / WAL 记录 / 订阅主题 / PG 表名 / COPY 目标。
    pub id: &'static str,
    /// 相对 `data/` 的文件路径（阶段 4 起由 `COPY TO STDOUT` 取代）。
    pub file: &'static str,
    pub kind: Kind,
    pub policy: LoadPolicy,
    /// 主键列，顺序即复合主键拼法。
    pub pk: &'static [&'static str],
    /// 外键声明：`(本表列, 目标表 id, 目标表列)`。
    ///
    /// 校验发生在全部表载入之后（见 `FkIndex`），因此允许互相引用；
    /// 目标列必须是目标表的主键列，由 `check_registry_shape` 静态核对。
    pub fk: &'static [(&'static str, &'static str, &'static str)],
    /// 期望行数。mock 阶段作为数据回归哨兵；接真实库后改 `None`，由日终对账接管。
    pub expected_rows: Option<usize>,
}

/// 表清单 —— 唯一事实源，由 `codegen` 从 `sql/schema.sql` + `tables.toml` 生成（见 `crate::generated::TABLES`）。
///
/// 新增一张表：改 DDL / `tables.toml` 后跑 `cargo codegen`，再给 `Snapshot` 加一个强类型字段，
/// 并在 `domain` 写一行 `impl RowValidator`（热路径零开销，见计划「刻意不做」）。
/// `Spec` / `Kind` / `LoadPolicy` 的类型定义仍在上方，`check_registry_shape` 继续守护 id / 文件唯一性与外键目标合法性。
pub use crate::generated::TABLES;

#[must_use]
pub fn spec_of(id: &str) -> Option<&'static Spec> {
    TABLES.iter().find(|spec| spec.id == id)
}

/// 列取值：文本列借用（零拷贝），整型列内联 `Copy`。
///
/// 这是「列值」在冷路径上的唯一载体，取代旧的 `&str`-only 约束：
/// 整型 / `Amount`（取 `.units()`）主键因此可进注册中心，而读侧仍不分配。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColVal<'a> {
    /// 文本列取值，借用自行内 `String`/`&str`，零拷贝。
    Text(&'a str),
    /// 整型列取值（含 `Amount::units()`），内联 `Copy`，零拷贝。
    Int(i64),
}

impl fmt::Display for ColVal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColVal::Text(s) => write!(f, "{s}"),
            ColVal::Int(n) => write!(f, "{n}"),
        }
    }
}

/// 复合主键的唯一拼法（[ColVal] 段），取代散落各处的 `format!("{a}:{b}")` / `position_key`。
#[must_use]
pub fn composite_key(parts: &[ColVal<'_>]) -> String {
    let mut key = String::new();
    for (idx, part) in parts.iter().enumerate() {
        if idx > 0 {
            key.push(':');
        }
        // 文本段直接 push_str 零额外分配；整型段本就必须转十进制，仅此一次分配。
        match part {
            ColVal::Text(s) => key.push_str(s),
            ColVal::Int(n) => key.push_str(&n.to_string()),
        }
    }
    key
}

/// 纯文本主键的便捷入口：把 `&str` 段包成 [ColVal::Text] 后转 [composite_key]，
/// 让字面量调用点（演示 / 测试）不强制手写枚举。
#[must_use]
pub fn composite_key_str(parts: &[&str]) -> String {
    composite_key(&parts.iter().copied().map(ColVal::Text).collect::<Vec<_>>())
}

/// 每表的列枚举需实现的桥接：在「`Spec`/`FkIndex` 以列名为键」与「类型侧编译期可检查的枚举」之间往返。
///
/// 注册中心必须以列名字符串做跨表 join（外键指向别的表），故 `Spec::pk`/`Spec::fk` 仍是 `&'static str`；
/// 本 trait 在加载边界把字符串翻译成枚举，`DataTable::column` 因此匹配枚举变体而非字面量。
///
/// 实现只需提供 `ALL` 与 `as_str`：列名字面量在全表仅出现一次（`as_str`），
/// `parse` 由它反向派生，杜绝「`as_str` 与 `parse` 两处字面量各自写错」的不一致。
/// `as_str`/`ALL` 的机械映射正是后续过程宏要生成的部分。
pub trait ColumnName: Copy + PartialEq + Eq + core::fmt::Debug + 'static {
    /// 全部列变体，供 [`ColumnName::parse`] 线性反查（列数有限，冷路径）。
    const ALL: &'static [Self];

    /// 枚举变体 → `Spec`/PG 列名（列名字面量的唯一来源）。
    fn as_str(self) -> &'static str;

    /// 列名 → 枚举变体；`Spec` 出现枚举未定义的列名时返回 `None`，加载期即暴露「声明与枚举不同步」。
    fn parse(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|col| col.as_str() == name)
    }
}

/// 行不变量的业务钩子：codegen 生成的 `impl DataTable` 不携带手写校验，
/// `check_row` 默认转调本 trait；人把单行不变量写在 `impl RowValidator for Xxx` 里。
///
/// 这样「样板（枚举/列名/主键）归生成器、不变量归人」各占一个 `impl` 块，互不抢占。
#[allow(unused_variables)]
pub trait RowValidator {
    /// 默认无附加不变量；有跨列约束的类型覆写此方法。
    fn validate_row(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

/// 行与声明的绑定：类型侧只需回答「我是哪张表、取哪一列的值是什么、我这一行合法吗」。
/// 行键不再手写，由 `Spec::pk` × `column` 在加载期现算。
pub trait DataTable: RowValidator {
    /// 必须等于某个 `Spec::id`。
    const ID: &'static str;

    /// 本表列枚举：把「取哪一列」从字符串升级为编译期可检查的类型。
    type Column: ColumnName;

    /// 按列枚举取值。必须覆盖本表 `Spec::pk` 的列，以及被别表 `fk` 引用为目标的列；
    /// 其余列（金额 / 数量等非 `ColVal` 可表达的列）返回 `None` 即可。
    ///
    /// 文本列返回 [`ColVal::Text`]（借用零拷贝），整型 / `Amount` 主键返回 [`ColVal::Int`]。
    fn column(&self, col: Self::Column) -> Option<ColVal<'_>>;

    /// 单行不变量（不跨表）：默认委托给 [`RowValidator::validate_row`]，跨表不变量由 `Spec::fk` + `FkIndex` 承担。
    /// 手写表覆写此方法即可；生成表把逻辑写在 [`RowValidator`] 里，两者语义等价。
    fn check_row(&self) -> anyhow::Result<()> {
        self.validate_row()
    }
}

/// 一列的取值索引：`values` 做成员判定，`rows` 保留 `(行主键, 取值)` 让外键报错能指认到行。
#[derive(Debug, Default, Clone)]
pub struct ColumnIndex {
    values: HashSet<String>,
    rows: Vec<(String, String)>,
}

impl ColumnIndex {
    #[must_use]
    pub fn contains(&self, value: &str) -> bool {
        self.values.contains(value)
    }

    fn push(&mut self, row_key: &str, value: ColVal<'_>) {
        let value = value.to_string();
        self.rows.push((row_key.to_string(), value.clone()));
        self.values.insert(value);
    }

    /// 抹掉某行在本列的全部取值对，并重建成员集 —— `upsert`/`delete` 的防线①（阶段 1.9）：
    /// 同行二次写入改已索引列后，旧值不得再被 `has` 命中（外键假阳性）。
    /// 冷路径写入用 `retain` + 重建，规模与行数同阶，百万行表将来换 `HashMap<key, Vec<value>>` 侧索引。
    fn remove_row(&mut self, row_key: &str) {
        let before = self.rows.len();
        self.rows.retain(|(key, _)| key != row_key);
        if self.rows.len() != before {
            self.values = self.rows.iter().map(|(_, value)| value.clone()).collect();
        }
    }

    /// 第一个「本表出现、目标表不存在」的取值及其行主键。
    fn first_missing_in(&self, target: &Self) -> Option<&(String, String)> {
        self.rows.iter().find(|(_, value)| !target.contains(value))
    }
}

/// 已加载的一张表：强类型行 + 只覆盖 `Spec` 声明列的取值索引。
///
/// 索引只在 `build_columns` 与 `upsert` 时维护，全表仅此一份。
/// `DataTable` 约束只加在需要它的 `impl` 上，不往结构体上挂，以免每个使用点重复堆 bound。
#[derive(Debug, Clone)]
pub struct Table<T> {
    pub spec: &'static Spec,
    rows: HashMap<String, T>,
    /// 已索引列名单：`upsert` 靠它知道要同步维护哪些列。
    indexed: Vec<&'static str>,
    columns: HashMap<&'static str, ColumnIndex>,
    /// `true` = `Optional` 表加载失败后降级成的空表（`Critical` 永远 `false`）。
    pub degraded: bool,
}

impl<T: DataTable> Table<T> {
    #[must_use]
    pub fn rows(&self) -> &HashMap<String, T> {
        &self.rows
    }

    /// 原地改非索引列（金额、数量等）用这个；改主键或增删行必须走 `upsert`，否则索引与数据分叉。
    #[must_use]
    pub fn rows_mut(&mut self) -> &mut HashMap<String, T> {
        &mut self.rows
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&T> {
        self.rows.get(key)
    }

    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.rows.contains_key(key)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// 某列是否出现过这个取值（冷路径：外键校验、将来的订阅过滤）。
    #[must_use]
    pub fn has(&self, column: &str, value: &str) -> bool {
        self.columns
            .get(column)
            .is_some_and(|index| index.contains(value))
    }

    #[must_use]
    pub fn stat(&self) -> TableStat {
        TableStat {
            id: self.spec.id,
            rows: self.rows.len(),
            policy: self.spec.policy,
            lsn: None,
        }
    }

    /// 列名字符串 → 本表列枚举；`Spec` 出现枚举未定义的列名即报错（声明与枚举不同步的捕获点）。
    fn parse_column(&self, name: &str) -> anyhow::Result<T::Column> {
        T::Column::parse(name).ok_or_else(|| {
            anyhow::anyhow!(
                "表 {} 的列 {name} 未在其 Column 枚举定义（Spec 与枚举不同步）",
                self.spec.id
            )
        })
    }

    /// 内核写入唯一入口（阶段 1）：主键由 `Spec::pk` × `column` 现算，行与列索引一起更新。
    ///
    /// 契约（阶段 1.9 / 1.10，均为机制拦截非注释约定）：
    /// - 入口强制 `check_row`：非法行在任何变更（行图 / 索引）之前即 `Err`，表不被触碰；
    /// - 先按行键抹旧索引对再插新值（对 replace 的键不变重写同样成立）：改已索引列不残留假阳性；
    /// - 行键已被占用即拒绝（静默双行 / 覆盖都不允许）—— 本入口语义 = **纯插入**，
    ///   重写已有行（含改非索引列、改已索引列、改主键）一律走 [`Table::replace`]；
    /// - 改主键（行级迁移）走 [`Table::replace`]；非索引列原地改走 `rows_mut()`；
    /// - 增删行、改已索引列一律走本入口族，不绕过。
    pub fn upsert(&mut self, row: T) -> anyhow::Result<()> {
        self.write_row(row, None)
    }

    /// 以 `old_key` 为身份重写整行：行图与全部列索引从 `old_key` 迁移到新主键拼出的行键。
    /// 调用方（内核）知道「改的是哪一行」，旧键显式传入、不靠取值猜测；
    /// `old_key` 不存在于行图时拒绝 —— 防拿错键静默新建重复行。
    pub fn replace(&mut self, old_key: &str, row: T) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.rows.contains_key(old_key),
            "表 {} replace：旧行键 {old_key} 不存在，拒绝静默新建",
            self.spec.id
        );
        self.write_row(row, Some(old_key.to_string()))
    }

    fn write_row(&mut self, row: T, old_key: Option<String>) -> anyhow::Result<()> {
        // 防线②：校验先于一切变更 —— 失败时 rows 与 columns 均未被触碰。
        row.check_row()?;

        let mut parts: Vec<ColVal<'_>> = Vec::with_capacity(self.spec.pk.len());
        for column in self.spec.pk {
            let col = self.parse_column(column)?;
            let value = row.column(col).ok_or_else(|| {
                anyhow::anyhow!("表 {} 缺少主键列 {column}", self.spec.id)
            })?;
            parts.push(value);
        }
        let row_key = composite_key(&parts);

        let mut fresh: Vec<(&'static str, String)> = Vec::with_capacity(self.indexed.len());
        for column in self.indexed.iter().copied() {
            let col = self.parse_column(column)?;
            let value = row.column(col).ok_or_else(|| {
                anyhow::anyhow!("表 {} 写入的行缺少已索引列 {column}", self.spec.id)
            })?;
            fresh.push((column, value.to_string()));
        }

        // 碰撞拒绝：新键已属于别的行（old_key 自替换、同键重写除外）。
        anyhow::ensure!(
            old_key.is_some() || !self.rows.contains_key(&row_key),
            "表 {} 行键 {row_key} 已被另一行占用，upsert 拒绝跨行覆盖；改主键请走 replace",
            self.spec.id
        );

        // 改主键迁移（replace 专用）：旧行条目连同其全部索引一起消失。
        if let Some(stale) = &old_key
            && stale != &row_key
        {
            self.rows.remove(stale);
            for index in self.columns.values_mut() {
                index.remove_row(stale);
            }
        }

        // 防线①：先清旧后插新 —— 同一行改已索引列不残留假阳性。
        for column in self.indexed.iter().copied() {
            if let Some(index) = self.columns.get_mut(column) {
                index.remove_row(&row_key);
            }
        }
        for (column, value) in &fresh {
            if let Some(index) = self.columns.get_mut(*column) {
                index.push(&row_key, ColVal::Text(value.as_str()));
            }
        }
        self.rows.insert(row_key, row);
        Ok(())
    }

    /// 内核删除入口：行与已索引取值一起消失，不留悬空索引（外键假阳性的另一半来源）。
    /// 行不存在时返回 `false`，但索引清理照常完成（幂等）。
    pub fn delete(&mut self, row_key: &str) -> bool {
        for column in self.indexed.iter().copied() {
            if let Some(index) = self.columns.get_mut(column) {
                index.remove_row(row_key);
            }
        }
        self.rows.remove(row_key).is_some()
    }

    /// 为 `Spec` 声明涉及的全部列建索引：本表 pk + 本表外键列 + 被别表引用的目标列。
    /// 任一列取不到值立即 `Err` —— 这就是「`impl DataTable` 与声明不同步」的捕获点。
    fn build_columns(&mut self, target_columns: &[&'static str]) -> anyhow::Result<()> {
        let mut needed: Vec<&'static str> = self.spec.pk.to_vec();
        needed.extend(self.spec.fk.iter().map(|(column, _, _)| *column));
        needed.extend_from_slice(target_columns);
        needed.sort_unstable();
        needed.dedup();

        for column in needed.iter().copied() {
            let col = self.parse_column(column)?;
            let mut index = ColumnIndex::default();
            for (row_key, row) in &self.rows {
                let value = row.column(col).ok_or_else(|| {
                    anyhow::anyhow!(
                        "表 {} 未在 DataTable::column 暴露声明所需列 {column}",
                        self.spec.id
                    )
                })?;
                index.push(row_key, value);
            }
            self.columns.insert(column, index);
        }
        self.indexed = needed;
        Ok(())
    }
}

/// 每表运行期统计：LSN 锚点 / 内存预算 / 日终对账的公共底座。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableStat {
    pub id: &'static str,
    pub rows: usize,
    pub policy: LoadPolicy,
    /// 阶段 4.2 接 PG 后填真实 LSN，现在恒 `None`。
    pub lsn: Option<u64>,
}

/// 通用加载器：按类型定位声明后转给 `load_specified_table`。
pub fn load_table<T>(root: &Path) -> anyhow::Result<Table<T>>
where
    T: DataTable + DeserializeOwned,
{
    let spec = spec_of(T::ID)
        .ok_or_else(|| anyhow::anyhow!("类型 {} 未在 tables::TABLES 登记", T::ID))?;
    load_specified_table::<T>(root, spec)
}

/// 显式给定声明的底层入口：分级启动、阶段 4 的数据源替换（读文件 → `COPY TO STDOUT`）
/// 与测试都走这里，校验逻辑完全复用。
pub fn load_specified_table<T>(root: &Path, spec: &'static Spec) -> anyhow::Result<Table<T>>
where
    T: DataTable + DeserializeOwned,
{
    anyhow::ensure!(
        spec.id == T::ID,
        "Spec {} 与类型 {} 不匹配（DataTable::ID 写错）",
        spec.id,
        T::ID
    );

    let (rows, degraded) = read_rows::<T>(spec, &root.join(spec.file))?;
    let mut table = Table {
        spec,
        rows,
        indexed: Vec::new(),
        columns: HashMap::new(),
        degraded,
    };

    table.build_columns(&target_columns_of(spec.id))?;
    verify_primary_keys(&table)?;
    verify_expected_rows(&table)?;
    Ok(table)
}

/// 收集「把本表当外键目标」的所有目标列：被引用的列必须可取值，否则校验无从下手。
fn target_columns_of(table_id: &str) -> Vec<&'static str> {
    let mut columns: Vec<&'static str> = Vec::new();
    for other in TABLES {
        for (_, target_id, target_col) in other.fk {
            if *target_id == table_id {
                columns.push(*target_col);
            }
        }
    }
    columns.sort_unstable();
    columns.dedup();
    columns
}

/// 分级启动的唯一分支点：`Critical` 失败上抛，`Optional` 失败降级空表，
/// `Lazy` 根本不去读文件（不进启动清单，而非「读取失败」）。返回值第二位 = 是否降级。
fn read_rows<T: DeserializeOwned>(
    spec: &'static Spec,
    path: &Path,
) -> anyhow::Result<(HashMap<String, T>, bool)> {
    if matches!(spec.policy, LoadPolicy::Lazy) {
        eprintln!("提示: 表 {} 策略 Lazy，启动期跳过加载", spec.id);
        return Ok((HashMap::new(), false));
    }

    let raw = match std::fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(err) => return degrade(spec, &format!("读取 {}", path.display()), &err),
    };
    match serde_json::from_str::<HashMap<String, T>>(&raw) {
        Ok(rows) => Ok((rows, false)),
        Err(err) => degrade(spec, &format!("解析 {}", path.display()), &err),
    }
}

fn degrade<T>(
    spec: &'static Spec,
    action: &str,
    err: &dyn std::fmt::Display,
) -> anyhow::Result<(HashMap<String, T>, bool)> {
    match spec.policy {
        LoadPolicy::Critical => Err(anyhow::anyhow!(
            "{action} 失败: {err}（表 {} 为 Critical，拒绝启动）",
            spec.id
        )),
        LoadPolicy::Optional => {
            eprintln!(
                "警告: {action} 失败: {err}（表 {} 为 Optional，降级为空表）",
                spec.id
            );
            Ok((HashMap::new(), true))
        }
        LoadPolicy::Lazy => Ok((HashMap::new(), false)),
    }
}

/// 机械校验两件事：`按 Spec::pk 逐列 column() 取值拼键` 成功，且 == `JSON 外层键`，
/// 顺带跑一遍 `check_row`。主键列名写错、`column` 未暴露声明列在此当场失败。
fn verify_primary_keys<T: DataTable>(table: &Table<T>) -> anyhow::Result<()> {
    for (json_key, row) in &table.rows {
        let mut parts: Vec<ColVal<'_>> = Vec::with_capacity(table.spec.pk.len());
        for column in table.spec.pk {
            let col = T::Column::parse(column).ok_or_else(|| {
                anyhow::anyhow!(
                    "表 {} 的列 {column} 未在其 Column 枚举定义（Spec 与枚举不同步）",
                    table.spec.id
                )
            })?;
            let value = row.column(col).ok_or_else(|| {
                anyhow::anyhow!("表 {} 缺少主键列 {column}", table.spec.id)
            })?;
            parts.push(value);
        }
        let declared = composite_key(&parts);
        anyhow::ensure!(
            json_key == &declared,
            "表 {} 的 JSON 键 {json_key} 与复合主键 {declared} 不一致",
            table.spec.id
        );
        row.check_row()?;
    }
    Ok(())
}

/// 行数哨兵：mock 阶段拿它当数据回归护栏，防「改数据结构却忘了改 fixture」。
/// 降级表与 `Lazy` 表本来就没数据，不能参与行数断言。
fn verify_expected_rows<T: DataTable>(table: &Table<T>) -> anyhow::Result<()> {
    let Some(expected) = table.spec.expected_rows else {
        return Ok(());
    };
    if table.degraded || matches!(table.spec.policy, LoadPolicy::Lazy) {
        return Ok(());
    }
    anyhow::ensure!(
        table.rows.len() == expected,
        "表 {} 行数 {} != 期望 {expected}",
        table.spec.id,
        table.rows.len()
    );
    Ok(())
}

/// 日终 dump：路径取自 `Spec::file`，与加载共用同一份声明，不再第二处硬编码。
pub fn save_table<T: serde::Serialize>(table: &Table<T>, root: &Path) -> anyhow::Result<()> {
    let path = root.join(table.spec.file);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let raw = serde_json::to_string_pretty(&table.rows)?;
    std::fs::write(&path, raw)?;
    Ok(())
}

/// 「表 id → 列 → 取值索引」的冷路径快照：在全部表载入后从强类型 `Table` 抄一份。
///
/// 刻意不做类型擦除的行容器：校验阶段只需要列值集合，而热路径仍然是
/// `Snapshot` 上的 `Table<T>` 强类型字段，两者互不拖累。
#[derive(Default)]
pub struct FkIndex {
    tables: HashMap<&'static str, TableDomain>,
}

#[derive(Default)]
struct TableDomain {
    rows: usize,
    degraded: bool,
    columns: HashMap<&'static str, ColumnIndex>,
}

impl FkIndex {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 登记一张已加载表。新表在 `Snapshot` 里多一个字段后，这里多一行 `register` 即可。
    pub fn register<T: DataTable>(&mut self, table: &Table<T>) {
        self.tables.insert(
            table.spec.id,
            TableDomain {
                rows: table.rows.len(),
                degraded: table.degraded,
                columns: table.columns.clone(),
            },
        );
    }

    /// 声明式外键校验：逐条 `Spec::fk` 做集合包含判定。
    ///
    /// 新增一条外键 = 多写一行声明，不写一行校验代码；报错能定位到表 / 行 / 列 / 坏值。
    pub fn check(&self) -> anyhow::Result<()> {
        for spec in TABLES {
            let Some(source) = self.tables.get(spec.id) else {
                continue; // 未登记（Lazy / 本次不加载）的表不参与校验
            };
            for (column, target_id, target_col) in spec.fk {
                let target_spec = spec_of(target_id).ok_or_else(|| {
                    anyhow::anyhow!("表 {}.{} 的外键指向未声明的表 {target_id}", spec.id, column)
                })?;
                anyhow::ensure!(
                    target_spec.pk.contains(target_col),
                    "表 {}.{} 的外键目标 {target_id}.{target_col} 不是主键列",
                    spec.id,
                    column
                );

                let Some(target) = self.tables.get(*target_id) else {
                    eprintln!("警告: 表 {} 的外键 {column} → {target_id} 未加载，跳过该边校验", spec.id);
                    continue;
                };
                if target.degraded || source.degraded {
                    eprintln!(
                        "警告: 表 {} 与 {target_id} 之间存在降级，外键 {column} 校验跳过",
                        spec.id
                    );
                    continue;
                }

                let target_values = target.columns.get(*target_col).ok_or_else(|| {
                    anyhow::anyhow!("表 {target_id} 未索引被引用列 {target_col}（impl DataTable 与声明不同步）")
                })?;
                let source_values = source.columns.get(*column).ok_or_else(|| {
                    anyhow::anyhow!("表 {} 未索引外键列 {column}（impl DataTable 与声明不同步）", spec.id)
                })?;
                if let Some((row_key, value)) = source_values.first_missing_in(target_values) {
                    anyhow::bail!(
                        "外键失败: 表 {} 行 {row_key} 的 {column}={value} 在 {target_id}.{target_col} 中不存在",
                        spec.id
                    );
                }
            }
        }
        Ok(())
    }

    /// 某表的某列是否出现过该取值（阶段 3 订阅入参校验可复用）。
    #[must_use]
    pub fn exists(&self, table_id: &str, column: &str, value: &str) -> bool {
        self.tables
            .get(table_id)
            .and_then(|domain| domain.columns.get(column))
            .is_some_and(|index| index.contains(value))
    }

    /// 0.5.5 的出口：按 `TABLES` 声明顺序给出每张表的行数与策略，
    /// 启动报告 / 日终对账 / 内存预算共用这一份。
    #[must_use]
    pub fn stats(&self) -> Vec<TableStat> {
        TABLES.iter()
            .filter_map(|spec| {
                self.tables.get(spec.id).map(|domain| TableStat {
                    id: spec.id,
                    rows: domain.rows,
                    policy: spec.policy,
                    lsn: None,
                })
            })
            .collect()
    }
}

/// 声明自检：一行数据都不读，只核对 `TABLES` 本身是否自相矛盾。
///
/// 新表刚加进来就把「id 重复 / 没主键 / 外键指向不存在的表或非主键列」挡住，
/// 比等到加载时才从 `Err` 文本里发现更早。启动流程首步调用。
pub fn check_registry_shape() -> anyhow::Result<()> {
    let mut ids = HashSet::new();
    let mut files = HashSet::new();

    for spec in TABLES {
        anyhow::ensure!(!spec.id.is_empty(), "存在空表 id（文件 {}）", spec.file);
        anyhow::ensure!(!spec.file.is_empty(), "表 {} 未声明文件", spec.id);
        anyhow::ensure!(
            ids.insert(spec.id),
            "表 id 重复: {}",
            spec.id
        );
        anyhow::ensure!(
            files.insert(spec.file),
            "文件重复: {} 被多张表使用",
            spec.file
        );
        anyhow::ensure!(!spec.pk.is_empty(), "表 {} 未声明主键", spec.id);
        for pk_column in spec.pk {
            anyhow::ensure!(!pk_column.is_empty(), "表 {} 存在空主键列名", spec.id);
        }

        for (column, target_id, target_col) in spec.fk {
            anyhow::ensure!(!column.is_empty(), "表 {} 的外键列名不能为空", spec.id);
            let target = spec_of(target_id)
                .ok_or_else(|| anyhow::anyhow!("表 {}.{} 的外键指向未声明的表 {target_id}", spec.id, column))?;
            anyhow::ensure!(
                target.pk.contains(target_col),
                "表 {}.{} 的外键目标 {target_id}.{target_col} 不是主键列",
                spec.id,
                column
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use account::amount::{Price, Quantity};
    use crate::domain::{Account, Asset, Order, Position, Security, Trade, User};
    use crate::mem::Snapshot;

    fn fixture_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("data")
    }

    /// 每个测试一个独立临时目录，避免互相污染，也避免碰仓内 fixture。
    fn temp_dir(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("graydb-tables-{tag}-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// 临时改一份声明（只改 file / policy / expected_rows），用来测试分级启动。
    fn leaked_spec(patch: impl FnOnce(&mut Spec)) -> &'static Spec {
        let mut spec = *spec_of(User::ID).unwrap();
        spec.file = "user.json";
        spec.expected_rows = None;
        patch(&mut spec);
        Box::leak(Box::new(spec))
    }

    /// 0.5.8 守护之一：清单自身合法，且声明与领域类型一一对应（漏任一半都在这拦）。
    #[test]
    fn table_ids_are_unique_and_match_files() {
        check_registry_shape().unwrap();

        let implemented = [
            User::ID,
            Account::ID,
            Security::ID,
            Asset::ID,
            Position::ID,
            Order::ID,
            Trade::ID,
        ];
        let declared: Vec<&'static str> = TABLES.iter().map(|spec| spec.id).collect();
        for id in implemented {
            assert!(declared.contains(&id), "{id} 实现了 DataTable 但未在 TABLES 登记");
        }
        for id in declared {
            assert!(implemented.contains(&id), "TABLES 登记了 {id} 但没有对应领域类型");
        }
    }

    /// 0.5.8 守护之二：每张声明表都被真正加载且非空，否则必须被显式标记为可空。
    #[test]
    fn every_declared_table_is_loaded_and_nonempty_or_marked() {
        let snap = Snapshot::load(fixture_root()).unwrap();
        let stats = snap.stats();

        for spec in TABLES {
            let stat = stats
                .iter()
                .find(|stat| stat.id == spec.id)
                .unwrap_or_else(|| panic!("表 {} 未被加载（Snapshot 缺字段或缺 register）", spec.id));
            assert_eq!(stat.policy, spec.policy, "表 {} 策略与声明不符", spec.id);
            match spec.expected_rows {
                Some(expected) => assert_eq!(stat.rows, expected, "表 {} 行数与声明不符", spec.id),
                None => assert!(
                    stat.rows > 0 || !matches!(spec.policy, LoadPolicy::Critical),
                    "表 {} 是空表且未标记为可缺",
                    spec.id
                ),
            }
        }
    }

    /// 0.5.6：`Critical` 缺文件 = 拒绝启动，且报错要能指认表与策略。
    #[test]
    fn critical_table_missing_refuses_startup() {
        let root = temp_dir("critical-missing");
        let err = load_table::<User>(&root).unwrap_err().to_string();
        assert!(
            err.contains("user_info") && err.contains("Critical"),
            "报错应指认表与策略: {err}"
        );
    }

    /// 0.5.6：`Optional` 失败只告警降级，行数哨兵不参与，否则降级会被自己卡死。
    #[test]
    fn optional_table_degrades_to_empty_and_is_marked() {
        let root = temp_dir("optional-missing");
        let spec = leaked_spec(|patch| {
            patch.file = "absent_user.json";
            patch.policy = LoadPolicy::Optional;
        });
        let table = load_specified_table::<User>(&root, spec).unwrap();

        assert!(table.degraded, "Optional 失败应标记降级");
        assert!(table.is_empty(), "降级表应为空");
        assert_eq!(table.stat().policy, LoadPolicy::Optional);
    }

    /// 0.5.4：外键只靠声明驱动 —— 新增坏行后 `check` 必须拦住，同时验证 `upsert` 不弄脏索引。
    #[test]
    fn fk_violation_is_caught_from_declaration_only() {
        let root = fixture_root();
        let users = load_table::<User>(&root).unwrap();
        let accounts = load_table::<Account>(&root).unwrap();
        let securities = load_table::<Security>(&root).unwrap();
        let assets = load_table::<Asset>(&root).unwrap();
        let mut positions = load_table::<Position>(&root).unwrap();

        let bogus = Position {
            account_id: "A001".to_string(),
            symbol: "999999".to_string(),
            quantity: Quantity::from_units(100),
            available_qty: Quantity::from_units(0),
            avg_cost: Price::from_units(10_000),
        };
        positions.upsert(bogus).unwrap();

        let mut index = FkIndex::new();
        index.register(&users);
        index.register(&accounts);
        index.register(&securities);
        index.register(&assets);
        index.register(&positions);

        let err = index.check().unwrap_err().to_string();
        assert!(
            err.contains("position") && err.contains("999999"),
            "外键报错应指认表与坏值: {err}"
        );
        assert!(index.exists("dict_security", "symbol", "09018"), "exists 应能查字典");
    }

    /// 0.5.3：「JSON 外层键 == 复合主键」是机械校验，手写键串在此当场失败。
    #[test]
    fn primary_key_must_match_json_outer_key() {
        let root = temp_dir("bad-key");
        std::fs::write(
            root.join("user.json"),
            r#"{"WRONG":{"user_id":"U1001","username":"张三","phone":"13800000001","id_type":"id_card","id_number":"110101199001011234","status":"active","opened_at":"2026-01-15T09:30:00+08:00"}}"#,
        )
        .unwrap();

        let err = load_specified_table::<User>(&root, leaked_spec(|_| {}))
            .unwrap_err()
            .to_string();
        assert!(err.contains("复合主键"), "应报主键不符: {err}");
    }

    /// 契约修订证明：`composite_key` 混合段（文本 + 整型）拼接，`Int` 段十进制稳定。
    #[test]
    fn composite_key_mixed_segments_render_stably() {
        assert_eq!(composite_key(&[ColVal::Text("A001"), ColVal::Int(7)]), "A001:7");
        assert_eq!(ColVal::Int(-42).to_string(), "-42");
        assert_eq!(ColVal::Text("09018").to_string(), "09018");
    }

    /// `as_str` ↔ `parse` 必须一一对应，未知列名返回 `None`——这是「Spec 与枚举不同步」
    /// 会在加载期被 `parse_column` 当场拦截的底层保证。
    #[test]
    fn column_name_round_trips_and_rejects_unknown() {
        use crate::domain::AssetColumn;
        for col in AssetColumn::ALL {
            assert_eq!(AssetColumn::parse(col.as_str()), Some(*col), "往返不一致: {col:?}");
        }
        assert_eq!(AssetColumn::parse("no_such_column"), None);
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct IntDict {
        id: i64,
        label: String,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum IntDictColumn {
        Id,
        Label,
    }

    impl ColumnName for IntDictColumn {
        const ALL: &'static [Self] = &[Self::Id, Self::Label];
        fn as_str(self) -> &'static str {
            match self {
                Self::Id => "id",
                Self::Label => "label",
            }
        }
    }

    impl DataTable for IntDict {
        const ID: &'static str = "int_dict";
        type Column = IntDictColumn;
        fn column(&self, col: IntDictColumn) -> Option<ColVal<'_>> {
            match col {
                IntDictColumn::Id => Some(ColVal::Int(self.id)),
                IntDictColumn::Label => None,
            }
        }
    }

    impl RowValidator for IntDict {}

    /// 契约修订核心：`i64` 主键能走 `column → ColVal::Int → 行键/索引` 全链路，
    /// 且读侧不经 `&str` —— 证明「主键只能文本」的天花板已解除。
    #[test]
    fn int_primary_key_is_supported_end_to_end() {
        let root = temp_dir("int-pk");
        std::fs::write(
            root.join("int_dict.json"),
            r#"{"1001":{"id":1001,"label":"A"},"1002":{"id":1002,"label":"B"}}"#,
        )
        .unwrap();

        let spec = leaked_spec(|patch| {
            patch.id = "int_dict";
            patch.file = "int_dict.json";
            patch.pk = &["id"];
            patch.fk = &[];
            patch.policy = LoadPolicy::Critical;
            patch.expected_rows = Some(2);
        });
        let table = load_specified_table::<IntDict>(&root, spec).unwrap();

        assert!(table.contains_key("1001"), "整型主键应拼成十进制行键");
        assert!(table.has("id", "1002"), "列索引应命中整型取值");
        assert!(!table.has("id", "9999"), "不存在的整型取值应判否");
    }

    /// 0.7.6 防漂移：锁定 codegen 产物内部的契约 —— `tables.toml` 声明的每列（主键 + 本表外键源列）
    /// 必须能在生成的 `XxxColumn` 里 `parse` 出来，且列枚举 `as_str` 全局唯一。
    /// specs 与 enum 同源于一次 codegen，本测试把这条不变量固化，防止任一侧被手改漂移。
    #[test]
    fn generated_column_enums_cover_declared_columns() {
        fn assert_covered<T: DataTable>(spec_id: &str) {
            let spec = spec_of(spec_id).expect("表未在 TABLES 声明");
            let mut cols: Vec<&str> = spec.pk.to_vec();
            cols.extend(spec.fk.iter().map(|(src, _, _)| *src));
            for col in cols {
                assert!(
                    T::Column::parse(col).is_some(),
                    "表 {spec_id} 声明列 {col} 不在其 Column 枚举（specs↔enum 漂移）"
                );
            }
            let mut seen = HashSet::new();
            for variant in T::Column::ALL {
                assert!(
                    seen.insert(variant.as_str()),
                    "表 {spec_id} 列枚举 as_str 重复: {}",
                    variant.as_str()
                );
            }
        }
        assert_covered::<User>("user_info");
        assert_covered::<Account>("account_info");
        assert_covered::<Security>("dict_security");
        assert_covered::<Asset>("account_asset");
        assert_covered::<Position>("position");
        assert_covered::<Order>("orders");
        assert_covered::<Trade>("trades");
    }

    /// 1.9 防线①：同一行二次写入改已索引列后，旧值不得再被 `has` 命中，
    /// 索引 `(key, value)` 对数与实际行数一致（append-only 时代会双双失真）。
    #[test]
    fn upsert_clears_old_index_values_on_reindex_column() {
        let mut positions = load_table::<Position>(&fixture_root()).unwrap();
        let key = composite_key_str(&["A001", "09018"]);
        let original = positions.get(&key).unwrap().clone();
        let before_rows = positions.len();

        // 改主键列 symbol 走 replace：行键迁移，旧值 09018 从索引消失。
        let renamed = Position {
            symbol: "600000".to_string(),
            ..original.clone()
        };
        positions.replace(&key, renamed).unwrap();
        assert!(!positions.has("symbol", "09018"), "旧索引值应被清除");
        assert!(positions.has("symbol", "600000"), "新索引值应命中");
        assert!(!positions.contains_key(&key), "行键随主键列迁移");
        assert_eq!(positions.len(), before_rows, "迁移是移动不是复制，行数不变");

        // 改回原样仍用 replace（当前键 → 原键）。
        positions
            .replace(&composite_key_str(&["A001", "600000"]), original)
            .unwrap();
        assert!(positions.has("symbol", "09018"));
        assert_eq!(positions.stat().rows, positions.len());

        // upsert 撞已有行键（别的身份）即拒绝，行与索引不被触碰 —— 杜绝静默覆盖。
        let squatter = Position {
            account_id: "A001".to_string(),
            symbol: "09018".to_string(),
            quantity: Quantity::from_units(1),
            available_qty: Quantity::from_units(0),
            avg_cost: Price::from_units(1),
        };
        assert!(positions.upsert(squatter).is_err(), "upsert 应拒绝跨行覆盖");
        assert_eq!(positions.len(), before_rows);
        assert!(positions.get(&key).unwrap().quantity.units() > 1, "被占行的数据不得被改");

        // delete 同步清行清索引，不留悬空对。
        assert!(positions.delete(&key));
        assert!(!positions.has("symbol", "09018"), "删除后索引不应残留");
        assert!(!positions.delete(&key), "二次删除返回 false");
    }

    /// 1.10 防线②：违反 `check_row` 的行经 `upsert` 写入必 `Err`，
    /// 且表内容与索引均未被触碰（校验先于一切变更）。
    #[test]
    fn upsert_enforces_check_row_before_any_mutation() {
        let mut positions = load_table::<Position>(&fixture_root()).unwrap();
        let before_rows = positions.len();

        // Position 的不变量：quantity >= available_qty。
        let bad = Position {
            account_id: "A001".to_string(),
            symbol: "999999".to_string(),
            quantity: Quantity::from_units(100),
            available_qty: Quantity::from_units(200),
            avg_cost: Price::from_units(10_000),
        };
        assert!(positions.upsert(bad).is_err(), "非法行应被 check_row 拒绝");
        assert_eq!(positions.len(), before_rows, "失败不得碰行图");
        assert!(!positions.has("symbol", "999999"), "失败不得碰索引");
    }
}