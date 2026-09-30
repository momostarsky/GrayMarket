//! GrayDB 代码生成器（阶段 0.7 / 0.8）。
//!
//! 输入两份事实源：
//! - `graydb/sql/*.sql`：有哪些列、叫什么、物理类型 —— 演示表用受控子集 `schema.sql`，
//!   真实表直接吃 `pg_dump --schema-only` 产物（`jzdb_prod_schema.sql` 等，每表用 `source` 指定）；
//! - `graydb/tables.toml`：DDL 推不出的策略与语义映射（文件/级别/主外键/行数、numeric→Amount、text→枚举）。
//!
//! 输出 `graydb/src/generated/*.rs`：每表一个文件（struct + 列枚举 + `ColumnName` impl + `impl DataTable`）
//! 外加 `specs.rs`（`TABLES`）与 `mod.rs`。产物是签入库的普通源文件，只在 DDL/策略变更时重跑：
//! `cargo codegen`。业务不变量不在这里 —— 人写在 `crate::domain` 的 `impl RowValidator`，互不覆盖。
//!
//! 迁移中间态：`register = false` 的表只生成 struct（编译进 graydb），**不**进 `TABLES`、
//! 不参与加载与校验 —— 用于真实库几十张表的渐进接入，接一张登记一张。

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    table: Vec<TableCfg>,
    /// 批量占位：把某个 dump 文件里的全部表展开成 `register=false` 的桩表（已被 `[[table]]` 声明的跳过）。
    #[serde(default)]
    stub: Vec<StubCfg>,
}

/// `[[stub]]`：整份 dump 一键搬表的结构占位规则（不登记、不加载、不校验）。
#[derive(Debug, Deserialize)]
struct StubCfg {
    /// `sql/` 下的 dump 文件名。
    source: String,
    /// 额外排除的表（物理表名，不含 schema 限定）。
    #[serde(default)]
    exclude: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TableCfg {
    id: String,
    #[serde(rename = "struct")]
    struct_name: String,
    /// DDL 事实源文件名（`sql/` 下），默认 `schema.sql`；真实表指向 pg_dump 产物。
    #[serde(default)]
    source: Option<String>,
    /// DDL 里的表名（可带 schema 限定），默认 = `id`；两者不同源于逻辑 id 与物理表名分离。
    #[serde(default)]
    table: Option<String>,
    /// 是否登记进 `TABLES`：`false` = 只生成 struct 的迁移中间态（不加载、不校验）。
    #[serde(default)]
    register: Option<bool>,
    file: String,
    kind: String,
    policy: String,
    pk: Vec<String>,
    #[serde(default)]
    fk: Vec<[String; 3]>,
    expected_rows: Option<usize>,
    /// 列名 → Rust 类型名：numeric→Money/Price/Quantity、text→业务枚举等 DDL 推不出的语义。
    #[serde(default)]
    types: BTreeMap<String, String>,
    /// 整表 numeric 列的默认映射（如 "Money"）；个别列仍可用 `types` 逐列覆盖。
    #[serde(default)]
    numeric_default: Option<String>,
}

impl TableCfg {
    fn source(&self) -> &str {
        self.source.as_deref().unwrap_or("schema.sql")
    }

    fn ddl(&self) -> &str {
        self.table.as_deref().unwrap_or(&self.id)
    }

    fn register(&self) -> bool {
        self.register.unwrap_or(true)
    }
}

/// 从 DDL 解析出的一列：`name` 是标识符文本（去引号），`sql_type` 是原始物理类型。
#[derive(Debug, Clone)]
struct SqlColumn {
    name: String,
    sql_type: String,
}

/// 一列解析出的生成信息：字段名、Rust 类型、是否需要 serde rename、`column()` 取值表达式。
#[derive(Debug)]
struct Field {
    ident: String, // Rust 字段名（snake_case）
    ty: String,    // Rust 类型
    ddl_name: String, // DDL/JSON 列名（as_str / serde rename 用）
    rename: bool,  // ident != ddl_name 时需要 #[serde(rename)]
    variant: String, // 列枚举变体（PascalCase）
    colval: Option<String>, // 若为可索引列，`column()` 里 Some(...) 的右值
}

fn main() -> Result<()> {
    let graydb = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("codegen 必须位于 workspace 根下的 codegen/ 目录")?
        .join("graydb");
    let sql_dir = graydb.join("sql");
    let config_path = graydb.join("tables.toml");
    let out_dir = graydb.join("src").join("generated");

    let config_toml = fs::read_to_string(&config_path)
        .with_context(|| format!("读取策略失败: {}", config_path.display()))?;
    let mut config: Config =
        toml::from_str(&config_toml).context("解析 tables.toml 失败")?;
    ensure!(!config.table.is_empty(), "tables.toml 未声明任何表");

    // 按源文件缓存：同一个 sql 只解析一次（演示表 schema.sql，真实表各 pg_dump 产物）。
    let mut schemas: BTreeMap<String, BTreeMap<String, Vec<SqlColumn>>> = BTreeMap::new();
    let mut sources: Vec<&str> = config.table.iter().map(TableCfg::source).collect();
    sources.extend(config.stub.iter().map(|s| s.source.as_str()));
    for source in sources {
        if schemas.contains_key(source) {
            continue;
        }
        let path = sql_dir.join(source);
        let sql = fs::read_to_string(&path)
            .with_context(|| format!("读取 DDL 失败: {}", path.display()))?;
        let parsed =
            parse_schema(&sql).with_context(|| format!("解析 DDL 失败: {source}"))?;
        println!("解析 {source}：{} 张表", parsed.len());
        schemas.insert(source.to_string(), parsed);
    }

    // 展开 `[[stub]]`：dump 里每张未被 `[[table]]` 接管的表 → 占位 cfg（机器裁决已定：
    // pk=row_id、numeric→Decimal 无损占位、register=false；转正时人写 [[table]] 覆盖）。
    let mut stub_cfgs = Vec::new();
    for stub in &config.stub {
        let tables = schemas
            .get(&stub.source)
            .with_context(|| format!("stub source {} 未解析", stub.source))?;
        let declared: BTreeSet<String> = config
            .table
            .iter()
            .filter(|t| t.source() == stub.source)
            .map(|t| bare_name(t.ddl()))
            .collect();
        for (key, cols) in tables {
            let bare = bare_name(key);
            if declared.contains(&bare) || stub.exclude.contains(&bare) {
                continue;
            }
            stub_cfgs.push(stub_to_table_cfg(&stub.source, &bare, cols)?);
        }
    }
    if !stub_cfgs.is_empty() {
        println!("stub 展开：{} 张占位表", stub_cfgs.len());
        config.table.extend(stub_cfgs);
    }

    fs::create_dir_all(&out_dir)
        .with_context(|| format!("创建输出目录失败: {}", out_dir.display()))?;

    let mut module_stems = Vec::new();
    let mut seen_stems: BTreeSet<String> = BTreeSet::new();
    for cfg in &config.table {
        let tables = schemas
            .get(cfg.source())
            .unwrap_or_else(|| panic!("源文件 {} 未解析", cfg.source()));
        let columns = find_table_columns(cfg.ddl(), tables).with_context(|| {
            format!("tables.toml 声明的表 {}（source = {}）", cfg.id, cfg.source())
        })?;
        let fields = build_fields(cfg, columns, &config)?;
        let stem = cfg.struct_name.to_ascii_lowercase();
        ensure!(seen_stems.insert(stem.clone()), "struct 名冲突：{stem}");
        write_table_file(&out_dir, &stem, cfg, &fields)?;
        module_stems.push(stem);
    }

    write_specs_file(&out_dir, &config.table)?;
    write_mod_file(&out_dir, &module_stems)?;

    let registered = config.table.iter().filter(|t| t.register()).count();
    println!(
        "codegen 完成：{} 张表（{} 张登记进 TABLES）→ {}",
        config.table.len(),
        registered,
        out_dir.display()
    );
    Ok(())
}

/// 去 schema 限定：`jzdb_prod.tb_x` → `tb_x`。
fn bare_name(ddl: &str) -> String {
    ddl.rsplit('.').next().unwrap_or(ddl).to_string()
}

/// 一表一键占位的固定裁决（见 tables.toml `[[stub]]` 注释）：
/// id = 物理表名；struct = 去 `tb_` 后的全表名 Pascal（保留模块段，跨模块同名表不撞车）；
/// pk = row_id；numeric → Decimal 无损占位；register=false 不进 TABLES。
fn stub_to_table_cfg(source: &str, bare: &str, cols: &[SqlColumn]) -> Result<TableCfg> {
    ensure!(
        cols.iter().any(|c| c.name == "row_id"),
        "stub 表 {bare}（{source}）没有 row_id 列：请进 tables.toml 用 [[table]] 显式声明主键，或加入 exclude"
    );
    let body = bare.strip_prefix("tb_").unwrap_or(bare);
    Ok(TableCfg {
        id: bare.to_string(),
        struct_name: to_pascal(body),
        source: Some(source.to_string()),
        table: Some(bare.to_string()),
        register: Some(false),
        file: format!("state/{bare}.json"),
        kind: "State".to_string(),
        policy: "Lazy".to_string(),
        pk: vec!["row_id".to_string()],
        fk: Vec::new(),
        expected_rows: None,
        types: BTreeMap::new(),
        numeric_default: Some("Decimal".to_string()),
    })
}

/// 按 DDL 名找表：先全名精确，再后缀匹配（配置写 `tb_x` 可命中 `jzdb_prod.tb_x`）。
fn find_table_columns<'a>(
    want: &str,
    tables: &'a BTreeMap<String, Vec<SqlColumn>>,
) -> Result<&'a [SqlColumn]> {
    if let Some(cols) = tables.get(want) {
        return Ok(cols);
    }
    let dotted = format!(".{want}");
    let mut hits = tables.iter().filter(|(k, _)| k.ends_with(&dotted));
    if let Some((_, cols)) = hits.next() {
        ensure!(
            hits.next().is_none(),
            "DDL 名 {want} 匹配到多张表（不同 schema 同名），须写全限定名"
        );
        return Ok(cols);
    }
    let available = tables.keys().cloned().collect::<Vec<_>>().join(", ");
    bail!("源里不存在表 {want}；可用表: [{available}]")
}

/// 结合 DDL 列与 tables.toml 映射，产出每列的字段名 / Rust 类型 / `column()` 取值式。
fn build_fields(cfg: &TableCfg, columns: &[SqlColumn], config: &Config) -> Result<Vec<Field>> {
    // 需要 `column()` 返回 Some 的列 = 本表主键 ∪ 本表外键源列 ∪ 指向本表的外键目标列。
    let mut indexed: BTreeSet<String> = BTreeSet::new();
    for pk in &cfg.pk {
        indexed.insert(pk.clone());
    }
    for [src, _, _] in &cfg.fk {
        indexed.insert(src.clone());
    }
    for other in &config.table {
        for [_src_col, target_id, target_col] in &other.fk {
            if target_id == &cfg.id {
                indexed.insert(target_col.clone());
            }
        }
    }

    let mut fields = Vec::new();
    for col in columns {
        let ty = resolve_rust_type(col, cfg)?;
        let ident = to_snake(&col.name);
        let rename = ident != col.name;
        let variant = to_pascal(&col.name);
        let colval = if indexed.contains(&col.name) {
            Some(colval_expr(&ident, &ty, col, cfg)?)
        } else {
            None
        };
        fields.push(Field {
            ident,
            ty,
            ddl_name: col.name.clone(),
            rename,
            variant,
            colval,
        });
    }
    Ok(fields)
}

/// 可索引列 → `ColVal` 右值；不可表达为 `ColVal` 的类型（业务枚举等）若被判为索引列则报错。
fn colval_expr(ident: &str, ty: &str, col: &SqlColumn, cfg: &TableCfg) -> Result<String> {
    Ok(match ty {
        "String" => format!("Some(ColVal::Text(&self.{ident}))"),
        "i64" => format!("Some(ColVal::Int(self.{ident}))"),
        "i32" | "i16" | "u64" | "u32" => format!("Some(ColVal::Int(self.{ident} as i64))"),
        "Money" | "Price" | "Quantity" | "MicroAmount" => {
            format!("Some(ColVal::Int(self.{ident}.units()))")
        }
        other => bail!(
            "表 {} 的列 {}（{}）被声明为主键/外键，但类型 `{other}` 无法表达为 ColVal",
            cfg.id,
            col.name,
            col.sql_type
        ),
    })
}

/// 解析物理类型；`numeric` / 浮点 / 未知类型必须由 tables.toml 显式映射，绝不猜。
fn resolve_rust_type(col: &SqlColumn, cfg: &TableCfg) -> Result<String> {
    if let Some(t) = cfg.types.get(&col.name) {
        return Ok(t.clone());
    }
    let base = col.sql_type.split('(').next().unwrap_or("").trim();
    let first = base.split_whitespace().next().unwrap_or("").to_ascii_lowercase();
    let rust = match first.as_str() {
        "text" | "varchar" | "character" | "citext" | "name" | "uuid" => "String",
        "bigint" | "int8" => "i64",
        "integer" | "int" | "int4" | "serial" | "serial4" => "i32",
        "smallint" | "int2" => "i16",
        "boolean" | "bool" => "bool",
        "timestamp" | "timestamptz" | "date" | "time" => "String",
        "numeric" | "decimal" | "money" | "real" | "float" | "double" | "float4" | "float8" => {
            if let Some(default) = &cfg.numeric_default {
                return Ok(default.clone());
            }
            bail!(
                "列 {}.{} 是 {}：numeric/浮点的 Rust 语义类型必须在 tables.toml 显式声明（[table.types] 逐列或 numeric_default 整表），不猜、不静默落浮点",
                cfg.id,
                col.name,
                col.sql_type
            );
        }
        other => bail!("列 {}.{} 未知 SQL 类型 `{other}`", cfg.id, col.name),
    };
    Ok(rust.to_string())
}

fn write_table_file(out_dir: &Path, stem: &str, cfg: &TableCfg, fields: &[Field]) -> Result<()> {
    // 收集导入：Amount 类型来自 account::amount，Decimal 来自 rust_decimal，
    // 其余非原始类型视为 crate::domain 的业务枚举。
    let primitives: BTreeSet<&str> =
        ["String", "i64", "i32", "i16", "u64", "u32", "bool"].into();
    let mut amount_imports: BTreeSet<String> = BTreeSet::new();
    let mut domain_imports: BTreeSet<String> = BTreeSet::new();
    let mut decimal_used = false;
    for f in fields {
        if matches!(f.ty.as_str(), "Money" | "Price" | "Quantity" | "MicroAmount") {
            amount_imports.insert(f.ty.clone());
        } else if f.ty == "Decimal" {
            decimal_used = true;
        } else if !primitives.contains(f.ty.as_str()) {
            domain_imports.insert(f.ty.clone());
        }
    }

    let enum_name = format!("{}Column", cfg.struct_name);
    let mut out = String::new();
    out.push_str(&format!(
        "//! 由 `codegen` 从 `sql/{source}` + `tables.toml` 生成 —— 请勿手改。\n\
         //! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。\n\
         #![allow(dead_code)]\n\n",
        source = cfg.source(),
    ));

    if !amount_imports.is_empty() {
        out.push_str(&format!(
            "use account::amount::{};\n",
            import_tree(&amount_imports)
        ));
    }
    if decimal_used {
        out.push_str("use rust_decimal::Decimal;\n");
    }
    if !domain_imports.is_empty() {
        out.push_str(&format!(
            "use crate::domain::{};\n",
            import_tree(&domain_imports)
        ));
    }
    // 迁移中间态（register=false）的表整个文件归机器独占，空 RowValidator 也生成；
    // 登记进 TABLES 的表校验归人（写在 domain 的 impl RowValidator），超界由编译器把关。
    let unregistered = !cfg.register();
    if unregistered {
        out.push_str("use crate::tables::RowValidator;\n");
    }
    out.push_str("use crate::tables::{ColVal, ColumnName, DataTable};\n\n");

    // struct
    out.push_str(&format!(
        "/// `{}`（codegen：字段与列名源自 DDL，`{enum_name}::as_str` 与本结构体字段同源，不可能写错）。\n\
         #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]\n\
         pub struct {} {{\n",
        cfg.id, cfg.struct_name
    ));
    for f in fields {
        if f.rename {
            out.push_str(&format!("    #[serde(rename = \"{}\")]\n", f.ddl_name));
        }
        out.push_str(&format!("    pub {}: {},\n", f.ident, f.ty));
    }
    out.push_str("}\n\n");

    // column enum
    out.push_str(&format!(
        "/// `{}` 列枚举（codegen）。\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
         pub enum {enum_name} {{\n",
        cfg.id
    ));
    for f in fields {
        out.push_str(&format!("    {},\n", f.variant));
    }
    out.push_str("}\n\n");

    out.push_str(&format!("impl ColumnName for {enum_name} {{\n"));
    out.push_str(&format!(
        "    const ALL: &'static [Self] = &[{}];\n",
        fields
            .iter()
            .map(|f| format!("Self::{}", f.variant))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str("    fn as_str(self) -> &'static str {\n        match self {\n");
    for f in fields {
        out.push_str(&format!(
            "            Self::{} => \"{}\",\n",
            f.variant, f.ddl_name
        ));
    }
    out.push_str("        }\n    }\n}\n\n");

    // impl DataTable（不含 check_row：默认委托 RowValidator；校验留人在 domain）
    out.push_str(&format!(
        "impl DataTable for {} {{\n    const ID: &'static str = \"{}\";\n    type Column = {enum_name};\n\n",
        cfg.struct_name, cfg.id
    ));
    out.push_str("    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {\n        match col {\n");
    for f in fields.iter().filter(|f| f.colval.is_some()) {
        out.push_str(&format!(
            "            {enum_name}::{} => {},\n",
            f.variant,
            f.colval.as_ref().unwrap()
        ));
    }
    out.push_str("            _ => None,\n        }\n    }\n}\n");

    if unregistered {
        out.push_str(&format!(
            "\nimpl RowValidator for {} {{}}\n",
            cfg.struct_name
        ));
    }

    fs::write(out_dir.join(format!("{stem}.rs")), out)
        .with_context(|| format!("写 {} 失败", cfg.id))?;
    Ok(())
}

fn write_specs_file(out_dir: &Path, tables: &[TableCfg]) -> Result<()> {
    let mut out = String::new();
    out.push_str(
        "//! 由 `codegen` 从 `tables.toml` 生成的表清单 —— 请勿手改。\n\
         //! 改策略请编辑 `tables.toml` 后 `cargo codegen`。\n\n\
         use crate::tables::{Kind, LoadPolicy, Spec};\n\n\
         /// 表清单：`codegen` 产出（仅 `register = true` 的表），`tables.rs` 以 `pub use` 重导出为唯一事实源。\n\
         pub const TABLES: &[Spec] = &[\n",
    );
    for cfg in tables.iter().filter(|t| t.register()) {
        let kind = kind_variant(&cfg.kind, &cfg.id)?;
        let policy = policy_variant(&cfg.policy, &cfg.id)?;
        let pk = cfg
            .pk
            .iter()
            .map(|c| format!("\"{c}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let fk = cfg
            .fk
            .iter()
            .map(|[a, b, c]| format!("(\"{a}\", \"{b}\", \"{c}\")"))
            .collect::<Vec<_>>()
            .join(", ");
        let expected = match cfg.expected_rows {
            Some(n) => format!("Some({n})"),
            None => "None".to_string(),
        };
        // schema 名从 source 文件名机械剥出：`jzdb_prod_schema.sql` → `jzdb_prod`。
        // 手写的 `schema.sql` 没有 schema 段，如实留 None —— 主题 `table:{schema}.*` 不猜前缀。
        let schema = match cfg.source().strip_suffix("_schema.sql") {
            Some(stem) if !stem.is_empty() => format!("Some({stem:?})"),
            _ => "None".to_string(),
        };
        out.push_str(&format!(
            "    Spec {{\n        id: \"{}\",\n        schema: {schema},\n        file: \"{}\",\n        \
             kind: {kind},\n        policy: {policy},\n        pk: &[{pk}],\n        fk: &[{fk}],\n        \
             expected_rows: {expected},\n    }},\n",
            cfg.id, cfg.file
        ));
    }
    out.push_str("];\n");
    fs::write(out_dir.join("specs.rs"), out).context("写 specs.rs 失败")?;
    Ok(())
}

fn write_mod_file(out_dir: &Path, stems: &[String]) -> Result<()> {
    let mut out = String::new();
    out.push_str(
        "//! codegen 产物聚合模块。请勿手改，改 DDL/策略后 `cargo codegen` 重生成。\n\n",
    );
    out.push_str("mod specs;\n");
    let mut sorted: Vec<&String> = stems.iter().collect();
    sorted.sort();
    for stem in &sorted {
        out.push_str(&format!("mod {stem};\n"));
    }
    out.push_str("\npub use specs::TABLES;\n");
    for stem in &sorted {
        out.push_str(&format!("pub use {stem}::*;\n"));
    }
    fs::write(out_dir.join("mod.rs"), out).context("写 mod.rs 失败")?;
    Ok(())
}

/// 把导入项拼成 `Money` 或 `{Money, Price}`（单项不加花括号）。
fn import_tree(items: &BTreeSet<String>) -> String {
    let joined = items.iter().cloned().collect::<Vec<_>>().join(", ");
    if items.len() == 1 {
        joined
    } else {
        format!("{{{joined}}}")
    }
}

fn kind_variant(s: &str, id: &str) -> Result<String> {
    match s {
        "Dict" => Ok("Kind::Dict".into()),
        "State" => Ok("Kind::State".into()),
        other => bail!("表 {id} kind = \"{other}\" 非法（应为 Dict / State）"),
    }
}

fn policy_variant(s: &str, id: &str) -> Result<String> {
    match s {
        "Critical" => Ok("LoadPolicy::Critical".into()),
        "Optional" => Ok("LoadPolicy::Optional".into()),
        "Lazy" => Ok("LoadPolicy::Lazy".into()),
        other => bail!("表 {id} policy = \"{other}\" 非法（应为 Critical / Optional / Lazy）"),
    }
}

// ---------------------------------------------------------------------------
// DDL 解析（受控子集）
// ---------------------------------------------------------------------------

/// 解析 `CREATE TABLE <id> ( <col> <type> [约束], ... );`，返回 id → 列（按 DDL 顺序）。
fn parse_schema(sql: &str) -> Result<BTreeMap<String, Vec<SqlColumn>>> {
    let stripped = strip_line_comments(sql);
    let lower = stripped.to_ascii_lowercase();
    let mut tables = BTreeMap::new();
    let mut cursor = 0usize;
    while let Some(rel) = lower[cursor..].find("create table") {
        let after_kw = cursor + rel + "create table".len();
        let rest = &stripped[after_kw..];
        let paren = rest.find('(').context("CREATE TABLE 后缺少 '('")?;
        let raw_name = rest[..paren].trim();
        let table_name = raw_name.trim_end_matches(';').trim();
        let table_name = normalize_table_name(strip_if_not_exists(table_name));

        let body_start = after_kw + paren + 1;
        let (body, body_end) = extract_balanced_parens(&stripped, body_start)
            .with_context(|| format!("表 {table_name} 的列定义括号不匹配"))?;
        cursor = body_end;

        let columns = parse_column_defs(&table_name, &body)?;
        tables.insert(table_name, columns);
    }
    Ok(tables)
}

fn strip_if_not_exists(name: &str) -> &str {
    let lower = name.to_ascii_lowercase();
    if lower.starts_with("if not exists") {
        name["if not exists".len()..].trim()
    } else {
        name
    }
}

/// 从 `open`（'(' 之后一位）开始扫描到配对的 ')'，返回内部文本与其后一位下标。
fn extract_balanced_parens(s: &str, open: usize) -> Result<(String, usize)> {
    let bytes: Vec<char> = s.chars().collect();
    let mut depth = 1usize;
    let mut in_str = false; // 双引号字符串内
    let mut i = open;
    let start = open;
    while i < bytes.len() {
        match bytes[i] {
            '"' => in_str = !in_str,
            '(' if !in_str => depth += 1,
            ')' if !in_str => {
                depth -= 1;
                if depth == 0 {
                    let inner: String = bytes[start..i].iter().collect();
                    return Ok((inner, i + 1));
                }
            }
            _ => {}
        }
        i += 1;
    }
    bail!("括号未配对")
}

/// 按顶层逗号切分列定义，逐条解析列名与物理类型，跳过表级约束。
fn parse_column_defs(table: &str, body: &str) -> Result<Vec<SqlColumn>> {
    let mut cols = Vec::new();
    for def in split_top_level_commas(body) {
        let def = def.trim();
        if def.is_empty() {
            continue;
        }
        let (name, consumed) = read_ident(def)
            .with_context(|| format!("表 {table} 列定义无法解析: {def}"))?;
        let first_kw = name.to_ascii_lowercase();
        if matches!(
            first_kw.as_str(),
            "primary" | "unique" | "foreign" | "constraint" | "check" | "exclude" | "index"
        ) {
            continue; // 表级约束：主键/外键以 tables.toml 为准
        }
        let rest = def[consumed..].trim_start();
        let sql_type = extract_type(rest);
        ensure!(
            !sql_type.is_empty(),
            "表 {table} 的列 {name} 缺少类型定义"
        );
        cols.push(SqlColumn { name, sql_type });
    }
    ensure!(
        !cols.is_empty(),
        "表 {table} 未解析到任何列"
    );
    Ok(cols)
}

/// 归一 `CREATE TABLE` 名：逐段去引号去空白，保留 schema 限定（`jzdb_prod.tb_x`）。
fn normalize_table_name(raw: &str) -> String {
    raw.split('.')
        .map(|part| part.trim().trim_matches('"').to_string())
        .collect::<Vec<_>>()
        .join(".")
}

/// 约束关键字：物理类型到此为止（pg_dump 列定义典型形态 `col type NOT NULL / DEFAULT ... / CONSTRAINT ...`）。
const TYPE_STOP_WORDS: &[&str] = &[
    "not", "null", "default", "constraint", "primary", "unique", "check", "references",
    "generated", "collate", "storage", "compression",
];

/// 提取物理类型：吃多词类型（`character varying(32)`）与括号精度（`numeric(18,4)`），遇约束词停下。
fn extract_type(rest: &str) -> String {
    let chars: Vec<char> = rest.chars().collect();
    let mut out = String::new();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        let start = i;
        while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '(' {
            i += 1;
        }
        let word: String = chars[start..i].iter().collect();
        if TYPE_STOP_WORDS.contains(&word.to_ascii_lowercase().as_str()) {
            break;
        }
        if !word.is_empty() {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&word);
        }
        // 紧贴的词后括号属于类型本身
        if i < chars.len() && chars[i] == '(' {
            let close = matching_paren_end(&chars, i).unwrap_or(chars.len());
            out.extend(chars[i..close].iter());
            i = close;
        }
    }
    out
}

/// 从 `open`（'(' 下标）扫到配对 ')'，返回其后一位下标。引号内不计深度。
fn matching_paren_end(chars: &[char], open: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut in_str = false;
    for (idx, &c) in chars.iter().enumerate().skip(open) {
        match c {
            '"' => in_str = !in_str,
            '(' if !in_str => depth += 1,
            ')' if !in_str => {
                depth -= 1;
                if depth == 0 {
                    return Some(idx + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// 读取起始标识符（可能是 `"带引号 名字"`），返回（去引号后的名字, 消耗字符数）。
fn read_ident(s: &str) -> Option<(String, usize)> {
    let mut it = s.char_indices().peekable();
    let first = it.peek()?.1;
    if first == '"' {
        let mut buf = String::new();
        it.next(); // 跳过开引号
        for (idx, c) in it {
            if c == '"' {
                return Some((buf, idx + c.len_utf8()));
            }
            buf.push(c);
        }
        None // 引号未闭合
    } else {
        for (idx, c) in it {
            if c.is_whitespace() || c == '(' || c == ',' {
                return Some((s[..idx].to_string(), idx));
            }
        }
        let trimmed = s.trim_end();
        Some((trimmed.to_string(), trimmed.len()))
    }
}

/// 在顶层（括号深度 0、引号外）按逗号切分。
fn split_top_level_commas(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    let mut in_str = false;
    for c in s.chars() {
        match c {
            '"' => {
                in_str = !in_str;
                cur.push(c);
            }
            '(' if !in_str => {
                depth += 1;
                cur.push(c);
            }
            ')' if !in_str => {
                depth -= 1;
                cur.push(c);
            }
            ',' if !in_str && depth == 0 => {
                parts.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        parts.push(cur);
    }
    parts
}

/// 去掉 `--` 行注释（保留其余文本原样）。
fn strip_line_comments(sql: &str) -> String {
    let mut out = String::with_capacity(sql.len());
    for line in sql.lines() {
        match line.find("--") {
            Some(pos) => out.push_str(&line[..pos]),
            None => out.push_str(line),
        }
        out.push('\n');
    }
    out
}

/// snake_case → PascalCase（`total_market_value` → `TotalMarketValue`）。
fn to_pascal(name: &str) -> String {
    name.split('_')
        .filter(|seg| !seg.is_empty())
        .map(|seg| {
            let mut chars = seg.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// 标识符 → Rust 字段名：缩略语感知的小写蛇形（`SEC_charges` → `sec_charges`、`Symbol` → `symbol`、
/// `total_market_value` 原样；连续大写不算词边界，避免 `S_E_C` 式碎片）。
fn to_snake(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() + 4);
    for (idx, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() {
            let prev_lower = idx > 0
                && (chars[idx - 1].is_ascii_lowercase() || chars[idx - 1].is_ascii_digit());
            let acronym_end = idx + 1 < chars.len()
                && idx > 0
                && chars[idx - 1].is_ascii_uppercase()
                && chars[idx + 1].is_ascii_lowercase();
            if prev_lower || acronym_end {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else if c == '-' {
            out.push('_');
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graydb_sql(name: &str) -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("graydb")
            .join("sql")
            .join(name);
        fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("读 {} 失败: {e}", path.display()))
    }

    /// 真实 pg_dump 产物必须整体解析得动：45 张表、多词类型、引号列名、内联约束、非建表语句全跳过。
    #[test]
    fn parses_real_pg_dump() {
        let tables = parse_schema(&graydb_sql("jzdb_prod_schema.sql")).unwrap();
        assert_eq!(tables.len(), 45, "jzdb_prod 应有 45 张表");

        let cols = &tables["jzdb_prod.tb_pdmage_pd_unit_capit_trade"];
        assert_eq!(cols.len(), 45, "capit_trade 应有 45 列");
        assert_eq!(cols[0].name, "row_id");
        assert_eq!(cols[0].sql_type, "bigint");

        // 多词类型带精度完整保留（不是只取首词 "character"）
        let name = cols.iter().find(|c| c.name == "pd_unit_name").unwrap();
        assert_eq!(name.sql_type, "character varying(64)");

        // 引号混合大小写列名原样保留（去引号不去 casing）
        let sec = cols.iter().find(|c| c.name == "SEC_charges").unwrap();
        assert_eq!(sec.sql_type, "numeric(18,2)");

        // 内联命名约束不污染类型（credit_asset 的 CONSTRAINT ... NOT NULL 列）
        let tables2 = parse_schema(&graydb_sql("jzdb_prod_schema.sql")).unwrap();
        let credit = &tables2["jzdb_prod.tb_pdmage_pd_unit_credit_asset"];
        let c1 = credit
            .iter()
            .find(|c| c.name == "loan_sell_amt_remain_amt")
            .unwrap();
        assert_eq!(c1.sql_type, "numeric(18,4)");
    }

    /// 演示 schema 与升级后的解析器行为兼容（提取类型不含约束词）。
    /// 先逐张断言「声明过的都在」，再校总数 —— 新增表时只需往清单里加一个名字。
    #[test]
    fn parses_demo_schema() {
        let tables = parse_schema(&graydb_sql("schema.sql")).unwrap();
        for id in [
            "user_info",
            "account_info",
            "dict_security",
            "account_asset",
            "position",
            "orders",
            "trades",
        ] {
            assert!(tables.contains_key(id), "演示 schema 缺表 {id}");
        }
        assert_eq!(tables.len(), 7, "除声明的 7 张外不该解析出别的东西");
        let asset = &tables["account_asset"];
        assert!(asset.iter().any(|c| c.name == "frozen"));
        // 阶段 1 的流水表：numeric 标度按 DDL 原样带出，映射表才能分派到 Quantity/Price
        let filled = tables["orders"]
            .iter()
            .find(|c| c.name == "filled_qty")
            .expect("orders 缺少 filled_qty 列");
        assert_eq!(filled.sql_type, "numeric(20,0)");
    }

    /// 整表 stub 搬表（[[stub]]）：去 tb_ 全名 Pascal 不撞车；无 row_id 即报错；Decimal 占位。
    #[test]
    fn stub_expansion_rules_hold() {
        assert_eq!(to_pascal("pdmage_pd_unit_capit"), "PdmagePdUnitCapit");
        assert_eq!(to_pascal("pdswap_pd_unit_capit"), "PdswapPdUnitCapit");
        assert_eq!(bare_name("jzdb_prod.tb_x"), "tb_x");
        assert_eq!(bare_name("tb_x"), "tb_x");

        let tables = parse_schema(&graydb_sql("jzdb_prod_schema.sql")).unwrap();
        let cols = &tables["jzdb_prod.tb_pdmage_pd_unit_capit"];
        let cfg = stub_to_table_cfg("jzdb_prod_schema.sql", "tb_pdmage_pd_unit_capit", cols).unwrap();
        assert_eq!(cfg.struct_name, "PdmagePdUnitCapit");
        assert_eq!(cfg.pk, ["row_id".to_string()]);
        assert!(!cfg.register());
        assert_eq!(cfg.numeric_default.as_deref(), Some("Decimal"));

        // 无 row_id 的列集必须报错（指向 [[table]]/exclude 出路）
        let no_rowid = vec![SqlColumn { name: "foo".into(), sql_type: "bigint".into() }];
        assert!(stub_to_table_cfg("x.sql", "tb_no_rowid", &no_rowid).is_err());
    }

    #[test]
    fn snake_case_handles_acronyms() {
        assert_eq!(to_snake("SEC_charges"), "sec_charges");
        assert_eq!(to_snake("Symbol"), "symbol");
        assert_eq!(to_snake("total_market_value"), "total_market_value");
        assert_eq!(to_snake("row_id"), "row_id");
    }
}
