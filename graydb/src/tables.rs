//! 表清单与注册中心 —— 「一张表 = 一处声明 + 一个 impl」。
//!
//! 声明与类型通过 `DataTable::ID` 互相锚定：
//! - 漏写 `impl DataTable` → 泛型约束不满足，编译失败；
//! - 漏登记 `Spec` → 加载期 `unknown table` 报错 + 守护测试失败；
//! - 主键顺序 / 列名写错 → 加载期机械比对 `pk_parts()` 与 `Spec::pk` 即暴露。

use std::collections::{HashMap, HashSet};
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

/// 表清单 —— 唯一事实源。新增一张表 = 这里加一行 + `domain` 加一个 `impl DataTable`
/// + `Snapshot` 加一个强类型字段（热路径零开销，见计划「刻意不做」）。
///
/// **顺序不构成硬约束**（外键允许成环），这里只按可读性排列；
/// id / 文件唯一性与外键目标合法性由 `check_registry_shape` 守护。
pub const TABLES: &[Spec] = &[
    Spec {
        id: "user_info",
        file: "dict/user.json",
        kind: Kind::Dict,
        policy: LoadPolicy::Critical,
        pk: &["user_id"],
        fk: &[],
        expected_rows: Some(3),
    },
    Spec {
        id: "account_info",
        file: "state/account.json",
        kind: Kind::Dict,
        policy: LoadPolicy::Critical,
        pk: &["account_id"],
        // 原先 `check_integrity` 里两个手写 for 循环，现在由声明表达：
        // 账户必须属于一个存在的用户，且必须有一行资金记录。
        fk: &[
            ("user_id", "user_info", "user_id"),
            ("account_id", "account_asset", "account_id"),
        ],
        expected_rows: Some(3),
    },
    Spec {
        id: "dict_security",
        file: "dict/security.json",
        kind: Kind::Dict,
        policy: LoadPolicy::Critical,
        pk: &["symbol"],
        fk: &[],
        expected_rows: Some(2),
    },
    Spec {
        id: "account_asset",
        file: "state/asset.json",
        kind: Kind::State,
        policy: LoadPolicy::Critical,
        pk: &["account_id"],
        fk: &[("account_id", "account_info", "account_id")],
        expected_rows: Some(3),
    },
    Spec {
        id: "position",
        file: "state/position.json",
        kind: Kind::State,
        policy: LoadPolicy::Critical,
        pk: &["account_id", "symbol"],
        fk: &[
            ("account_id", "account_info", "account_id"),
            ("symbol", "dict_security", "symbol"),
        ],
        expected_rows: Some(2),
    },
];

#[must_use]
pub fn spec_of(id: &str) -> Option<&'static Spec> {
    TABLES.iter().find(|spec| spec.id == id)
}

/// 复合主键的唯一拼法，取代散落各处的 `format!("{a}:{b}")` / `position_key`。
#[must_use]
pub fn composite_key(parts: &[&str]) -> String {
    let mut key = String::new();
    for (idx, part) in parts.iter().enumerate() {
        if idx > 0 {
            key.push(':');
        }
        key.push_str(part);
    }
    key
}

/// 行与声明的绑定：类型侧只需回答「我是谁、我的键在哪、我这一行合法吗」。
pub trait DataTable {
    /// 必须等于某个 `Spec::id`。
    const ID: &'static str;

    /// 按列名取值。必须覆盖本表 `Spec::pk` 的列，以及被别表 `fk` 引用为目标的列；
    /// 其余列返回 `None` 即可（冷路径，不进撮合）。
    fn column(&self, name: &'static str) -> Option<&str>;

    /// 主键各列取值，顺序必须与 `Spec::pk` 完全一致。
    fn pk_parts(&self) -> Vec<&str>;

    /// 单行不变量（不跨表）。跨表不变量由 `Spec::fk` 声明 + `FkIndex` 承担。
    fn check_row(&self) -> anyhow::Result<()> {
        Ok(())
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

    fn push(&mut self, row_key: &str, value: &str) {
        self.values.insert(value.to_string());
        self.rows.push((row_key.to_string(), value.to_string()));
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

    /// 内核写入唯一入口（阶段 1）：主键由 `pk_parts` 现算，行与列索引一起更新。
    pub fn upsert(&mut self, row: T) -> anyhow::Result<()> {
        let parts = row.pk_parts();
        anyhow::ensure!(
            parts.len() == self.spec.pk.len(),
            "表 {} 主键段数不符: 声明 {:?}，实现 {} 段",
            self.spec.id,
            self.spec.pk,
            parts.len()
        );
        let row_key = composite_key(&parts);

        let mut fresh: Vec<(&'static str, String)> = Vec::with_capacity(self.indexed.len());
        for column in self.indexed.iter().copied() {
            let value = row.column(column).ok_or_else(|| {
                anyhow::anyhow!("表 {} 写入的行缺少已索引列 {column}", self.spec.id)
            })?;
            fresh.push((column, value.to_string()));
        }
        for (column, value) in &fresh {
            if let Some(index) = self.columns.get_mut(*column) {
                index.push(&row_key, value);
            }
        }
        self.rows.insert(row_key, row);
        Ok(())
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
            let mut index = ColumnIndex::default();
            for (row_key, row) in &self.rows {
                let value = row.column(column).ok_or_else(|| {
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

/// 机械校验三件事：`JSON 外层键` == `按 Spec::pk 取值拼键` == `pk_parts() 拼键`，
/// 顺带跑一遍 `check_row`。主键列名写错、顺序写反、手拼键串在此全部当场失败。
fn verify_primary_keys<T: DataTable>(table: &Table<T>) -> anyhow::Result<()> {
    for (json_key, row) in &table.rows {
        let parts = row.pk_parts();
        anyhow::ensure!(
            parts.len() == table.spec.pk.len(),
            "表 {} 主键段数不符: 声明 {:?}，实现 {} 段",
            table.spec.id,
            table.spec.pk,
            parts.len()
        );

        let mut declared_parts: Vec<&str> = Vec::with_capacity(table.spec.pk.len());
        for column in table.spec.pk {
            let value = row.column(column).ok_or_else(|| {
                anyhow::anyhow!("表 {} 缺少主键列 {column}", table.spec.id)
            })?;
            declared_parts.push(value);
        }

        let declared = composite_key(&declared_parts);
        let computed = composite_key(&parts);
        anyhow::ensure!(
            declared == computed,
            "表 {} 主键实现与声明不符: 声明 {:?} → {declared}，实现 → {computed}",
            table.spec.id,
            table.spec.pk
        );
        anyhow::ensure!(
            json_key == &computed,
            "表 {} 的 JSON 键 {json_key} 与复合主键 {computed} 不一致",
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
    use crate::domain::{Account, Asset, Position, Security, User};
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

        let implemented = [User::ID, Account::ID, Security::ID, Asset::ID, Position::ID];
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
}