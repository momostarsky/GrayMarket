# GrayMarkset 类型映射与实体建模设计说明

> 目标：为基于 SQLx 的代码生成（codegen）定义一套「物理 DB 类型 ↔ Rust 语义类型」的映射体系，
> 支持 **MySQL 与 PostgreSQL** 双引擎，并将业务约束（长度/范围/可空/写默认）显式化。
> 本文档为设计定稿说明，后续 Rust 骨架 / codegen 均以此为准。

---

## 1. 架构与三阶段流水线

```
Stage1 catalog.yaml       类型真值表：scalars / domains / domain_templates / identity
        ▲ (dict.Type 引用 catalog)
Stage2 entity_dicts.yaml  字段字典：一列的完整语义(类型+写默认+中英文名)，可复用
        ▲ (entity 列引用 dict)
Stage3 entities/*.yaml    实体：只列字段名/引用 dict → codegen 查表展开
        │
        └──codegen──▶ Rust 领域 newtype ──sqlx──▶ 各引擎物理类型 / DDL
```

| 阶段 | 文件 | 职责 |
|---|---|---|
| Stage1 类型目录 | `catalog.yaml` | 逻辑标量/领域类型 → Rust 类型 + MySQL/PG 列类型（唯一真值表） |
| Stage2 字段字典 | `entity_dicts.yaml` | 可复用的「列定义」：`Type` + 写默认 + `CnName`/`EnName`（见 §10） |
| Stage3 实体 DSL | `entities/*.yaml` | 只声明实体与列引用；codegen 查 Stage2/Stage1 逐级展开 |

**核心原则**：Rust 表示与 DB 表示可以不同。`Age` 用 `u8`（18~200）而 DB 存 `INT`，
差异由 SQLx 的 `Encode/Decode` 桥接（泛型 `impl<DB: Database>`，一份代码适配两引擎）。

---

## 2. catalog.yaml（定稿）

命名统一采用业务化大写名，`domains.BaseType` 直接引用，零别名映射。

```yaml
# graymarkset/catalog.yaml  —— SQLx 物理标量目录（仅 MySQL / PostgreSQL）
version: 1
engines: [postgres, mysql]

# ===========================================================================
# 1) scalars：逻辑标量 -> Rust 类型 + 各引擎列类型
#    rust        : SQLx 内建支持的 Rust 类型
#    params      : 参数化标量形参（default 默认 / max 上限）
#    sql.<engine>: 列类型模板，{Key} 由参数填充
# ===========================================================================
scalars:
  INT16:   { rust: i16,  sql: { postgres: SMALLINT, mysql: SMALLINT } }
  INT32:   { rust: i32,  sql: { postgres: INTEGER,  mysql: INT } }
  INT64:   { rust: i64,  sql: { postgres: BIGINT,   mysql: BIGINT } }
  BOOL:    { rust: bool, sql: { postgres: BOOLEAN,  mysql: "TINYINT(1)" } }
  FLOAT:   { rust: f32,  sql: { postgres: REAL,     mysql: FLOAT } }
  DOUBLE:  { rust: f64,  sql: { postgres: "DOUBLE PRECISION", mysql: DOUBLE } }

  DECIMAL:
    rust: "rust_decimal::Decimal"
    params: { Precision: { default: 18, max: 38 }, Scale: { default: 4, max: 30 } }
    sql: { postgres: "NUMERIC({Precision},{Scale})", mysql: "DECIMAL({Precision},{Scale})" }

  STR:
    rust: String
    params: { MaxLength: { default: 255, max: 65535 } }
    sql: { postgres: "VARCHAR({MaxLength})", mysql: "VARCHAR({MaxLength})" }

  TEXT:    { rust: String,  sql: { postgres: TEXT,   mysql: LONGTEXT } }
  BYTES:   { rust: "Vec<u8>", sql: { postgres: BYTEA, mysql: LONGBLOB } }
  DATE:    { rust: "chrono::NaiveDate", sql: { postgres: DATE, mysql: DATE } }

  TIME:
    rust: "chrono::NaiveTime"
    params: { Precision: { default: 6, max: 6 } }
    sql: { postgres: "TIME({Precision})", mysql: "TIME({Precision})" }

  DATETIME:
    rust: "chrono::NaiveDateTime"
    params: { Precision: { default: 6, max: 6 } }
    sql: { postgres: "TIMESTAMP({Precision})", mysql: "DATETIME({Precision})" }

  TIMESTAMPTZ:
    rust: "chrono::DateTime<chrono::Utc>"
    params: { Precision: { default: 6, max: 6 } }
    sql: { postgres: "TIMESTAMPTZ({Precision})", mysql: "DATETIME({Precision})" }

  UUID:    { rust: "uuid::Uuid", sql: { postgres: UUID, mysql: "CHAR(36)" } }
  JSON:    { rust: "serde_json::Value", sql: { postgres: JSONB, mysql: JSON } }

# ===========================================================================
# 2) identity：自增列语法差异（PG GENERATED AS IDENTITY / MySQL AUTO_INCREMENT）
# ===========================================================================
identity:
  INT32: { postgres: "INTEGER GENERATED ALWAYS AS IDENTITY", mysql: "INT AUTO_INCREMENT" }
  INT64: { postgres: "BIGINT GENERATED ALWAYS AS IDENTITY",  mysql: "BIGINT AUTO_INCREMENT" }

# ===========================================================================
# 3) domains：领域 newtype 的离散约束（不含正则；复杂规则见 §5 RowCheck）
#    通用 : BaseType / Nullable / DbDefault / RustType
#    字符串: MaxLength / MinLength
#    数值  : MaxValue / MinValue
#    定点  : Precision / Scale
#    枚举  : Kind(enum|flags) + Variants（enum=互斥单选 / flags=多选位图，见 §8）
#    注：DefValue 更名为 DbDefault，语义明确为「仅 DDL DEFAULT 子句」，不参与 Rust 写路径
# ===========================================================================
domains:
  SecuCode:
    BaseType: STR
    MaxLength: 6
    MinLength: 5
    Nullable: false
    DbDefault: null          # 仅当需要建表 DEFAULT 时填写；写默认见 BeforeWriteDB

  Age:
    BaseType: INT32
    MinValue: 18
    MaxValue: 200
    Nullable: false
    RustType: u8             # Rust 收窄为 u8，DB 仍存 INT
    DbDefault: null

  Amount:
    BaseType: DECIMAL
    Precision: 18
    Scale: 4
    MinValue: 0
    Nullable: false
    DbDefault: null

  # ---- 互斥枚举（Kind: enum）：值必须恰为 Variants 之一；Rust 生成 #[repr(i32)] enum ----
  OrderSide:
    BaseType: INT32          # 存储位宽
    Nullable: false
    Kind: enum               # 互斥单选
    DbDefault: null          # 仅 DDL；写默认见 BeforeWriteDB
    Variants:
      - { name: Buy,    value: 0, label: "买入" }
      - { name: Sell,   value: 1, label: "卖出" }
      - { name: Cancel, value: 9, label: "撤单" }

  # ---- 多选位图（Kind: flags）：任意已定义位的组合；Rust 生成 bitflags! ----
  FeaturePerm:
    BaseType: INT32          # 位图 >32 位改 INT64
    Nullable: false
    Kind: flags              # 多选位掩码
    DbDefault: null
    Variants:
      - { name: Read,  bit: 0, label: "读" }
      - { name: Write, bit: 1, label: "写" }
      - { name: Exec,  bit: 2, label: "执行" }

# ===========================================================================
# 4) domain_templates：参数化领域模板（一个定义派生一族），
#    避免手写 Str4/Str8/Str16。实体引用如 type: "MaxStr(8)"。
#    params 填充实例化；MaxLength 同时喂 DDL VARCHAR({N}) 与 Rust 校验。
# ===========================================================================
domain_templates:
  MaxStr:                    # Rust 侧用 const 泛型 MaxStr<N>，一份 impl 覆盖所有 N
    BaseType: STR
    params: { N: { max: 65535 } }
    MaxLength: "{N}"
    Nullable: false
  ExactStr:                  # 定长：chars().count()==N
    BaseType: STR
    params: { N: { max: 65535 } }
    MaxLength: "{N}"
    MinLength: "{N}"
    Nullable: false
```

---

## 3. DefValue 语义：降级为「仅 DDL」

本设计的定稿要点。`DefValue` 重命名为 **`DbDefault`**，职责收窄：

| 关注点 | 由谁负责 | 说明 |
|---|---|---|
| 建表 `DEFAULT` 子句（DB 安全网 / 非应用写入源） | **`DbDefault`** | 只影响 DDL，**不参与 Rust 写路径** |
| 写库时的默认值 / 派生值 / 动态值 | **`BeforeWriteDB` trait** | 唯一的写侧入口，见 §5 |

**被明确否决的两种旧方案**：

- ❌ 「`Nullable:true + DefValue` 时，encode 阶段把 `None` 替换成 `DefValue`」
  —— 会导致「声明可空却永远写不进 NULL」的语义矛盾。
- ❌ 「`None` 时省略该列 → 走 DB 默认」
  —— 依赖动态列剔除，复杂且与 `Option::None = NULL` 撞车。

因此：**`DbDefault` 只喂 DDL；写默认一律在 `BeforeWriteDB` 里显式赋值。**

---

## 4. Nullable ↔ Rust 类型 ↔ DDL 对照

`Nullable` 是**唯一驱动可空性与 Rust 类型**的开关：

| Nullable | Rust 字段 | DDL | 写侧行为（配合 BeforeWriteDB） |
|---|---|---|---|
| `false` | `T` | `NOT NULL` | 必须给值；Rust 类型系统保证非空 |
| `true` | `Option<T>` | 允许 NULL | `None` 默认写 NULL；若 `before_write` 置为 `Some` 则写该值 |

**identity 列例外**：`row_id` 为 `Nullable:false`，但值来自序列（PG identity / MySQL auto_increment），
应用不 bind，不套用「非空必须给值」约束。

---

## 5. 写旁路 Trait：BeforeWriteDB 与 RowCheck

### 5.1 生命周期（固定顺序）

```
构造实体 → before_write()  (可变校正:填默认/派生/规范化)
         → validate()      (RowCheck:校验)
         → bind → execute
```

- **`BeforeWriteDB`：只改值，不校验。**
- **`RowCheck`：只校验，不改值。**
- 二者正交，不可混用。

### 5.2 BeforeWriteDB（写默认唯一入口）

```rust
pub trait BeforeWriteDB {
    /// 写库前就地校正：填默认值、跨字段派生、规范化。
    /// 约定：未置值的 Option 将作为 NULL 写入；需要默认值请显式置 Some。
    fn before_write(&mut self);
}
```

示例（含声明式 `DbDefault` 做不到的动态值与跨字段派生）：

```rust
impl BeforeWriteDB for TraderAccount {
    fn before_write(&mut self) {
        if self.alt_code.is_none() {
            self.alt_code = Some(SecuCode::new("000000").unwrap());
        }
        self.create_time = chrono::Utc::now().naive_utc(); // 动态默认
    }
}
```

### 5.3 RowCheck（实体旁路校验，承载正则/跨字段规则）

domain 只表达单列离散约束；字符集/正则/跨字段比较等移出到此处：

```rust
#[derive(Debug, Clone)]
pub struct CheckViolation { pub field: &'static str, pub rule: &'static str, pub msg: String }

pub trait RowCheck {
    fn check_row(&self) -> Vec<CheckViolation>;
    fn validate(&self) -> Result<(), Vec<CheckViolation>> {
        let v = self.check_row();
        if v.is_empty() { Ok(()) } else { Err(v) }
    }
}

impl RowCheck for TraderAccount {
    fn check_row(&self) -> Vec<CheckViolation> {
        let mut out = Vec::new();
        // SecuCode 的「仅字母数字」规则（原 ^[A-Za-z0-9]{5,6}$ 的字符集部分）
        if !self.secu_code.as_str().chars().all(|c| c.is_ascii_alphanumeric()) {
            out.push(CheckViolation { field: "secu_code", rule: "alnum", msg: "证券代码只能是字母或数字".into() });
        }
        out
    }
}
```

---

## 6. 领域类型（Age 示例：Rust 收窄 / DB 放宽）

泛型 `impl<DB>` 一份代码同时适配 MySQL 与 PG；约束在 `Decode` 阶段校验（脏数据读出即失败）：

```rust
use sqlx::{Database, Encode, Decode, Type, encode::IsNull, error::BoxDynError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Age(u8);

impl Age {
    pub const MIN: u8 = 18;
    pub const MAX: u8 = 200;
    pub fn new(v: i64) -> Result<Self, AgeError> {
        if !(Self::MIN as i64..=Self::MAX as i64).contains(&v) { return Err(AgeError::OutOfRange(v)); }
        Ok(Self(v as u8))
    }
    pub const fn value(&self) -> u8 { self.0 }
}

impl<DB> Type<DB> for Age where DB: Database, i32: Type<DB> {
    fn type_info(t: &DB::TypeInfo) -> bool { <i32 as Type<DB>>::type_info(t) }
}
impl<'q, DB> Encode<'q, DB> for Age where DB: Database, i32: Encode<'q, DB> + Type<DB> {
    fn encode_by_ref(&self, buf: &mut DB::ArgumentBuffer<'q>) -> Result<IsNull, BoxDynError> {
        (self.0 as i32).encode_by_ref(buf)
    }
}
impl<'r, DB> Decode<'r, DB> for Age where DB: Database, i32: Decode<'r, DB> + Type<DB> {
    fn decode(val: DB::Value<'r>) -> Result<Self, BoxDynError> {
        Ok(Age::new(<i32 as Decode<DB>>::decode(val)? as i64)?)
    }
}
```

---

## 7. 引擎差异备忘（MySQL vs PostgreSQL）

| 主题 | PostgreSQL | MySQL | 备注 |
|---|---|---|---|
| 布尔 | `BOOLEAN` | `TINYINT(1)` | |
| 浮点 | `DOUBLE PRECISION` | `DOUBLE` | |
| 时间默认精度 | 默认即微秒 | 裸 `DATETIME` **截断秒** | 故 catalog 固定 `DATETIME(6)` |
| 带时区 | `TIMESTAMPTZ` 原生 | 无，映射 `DATETIME(6)` | 需应用层约定存 UTC |
| 文本/二进制 | `TEXT` / `BYTEA` | `LONGTEXT` / `LONGBLOB` | |
| UUID | 原生 `UUID` | `CHAR(36)` | |
| JSON | `JSONB` | `JSON` | |
| 自增 | `GENERATED ALWAYS AS IDENTITY` | `AUTO_INCREMENT` | 见 catalog `identity` |

---

## 8. 枚举（互斥）与位图（多选）领域类型

`domains` 用 `Kind` 字段区分两个子类，基础类型恒为 `INT32`/`INT64`：

| 子类型 | `Kind` | 语义 | 存储 | Rust 生成 | `Decode` 约束 |
|---|---|---|---|---|---|
| **互斥枚举** | `enum` | 恰好取一个 | 单个 `INT` | `#[repr(i32)] enum` | `value ∈ Variants`，否则 `Err` |
| **多选位图** | `flags` | 任意已定义位的组合 | 位掩码 `INT32`/`INT64` | `bitflags!` | 不含未定义位 `(v & !MASK)==0` |

### 8.1 互斥枚举 → Rust 原生 enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum OrderSide { Buy = 0, Sell = 1, Cancel = 9 }

impl OrderSide {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v { 0 => Some(Self::Buy), 1 => Some(Self::Sell), 9 => Some(Self::Cancel), _ => None }
    }
    pub const fn as_i32(self) -> i32 { self as i32 }
}

// Type/Encode/Decode 桥接 i32（泛型 impl<DB>，同 Age 模式）；Decode 用 from_i32，None 即 Err
// impl<DB> Type<DB> for OrderSide ... <i32 as Type<DB>>
// Encode: self.as_i32().encode_by_ref(buf)
// Decode: OrderSide::from_i32(<i32 as Decode<DB>>::decode(val)?).ok_or(...)
```

### 8.2 多选位图 → bitflags

```rust
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FeaturePerm: i32 {
        const Read  = 1 << 0;
        const Write = 1 << 1;
        const Exec  = 1 << 2;
    }
}
// 已知位掩码 MASK = Read|Write|Exec；Decode 校验 (raw & !Self::all().bits())==0，否则 Err
// Encode: self.bits().encode_by_ref(buf)（i32；>32 位用 i64 + BaseType INT64）
```

### 8.3 设计要点

- **位宽**：`flags` 变体数 ≤32 用 `INT32`(i32)，>32 用 `INT64`(i64)；`enum` 一般 `INT32` 足够。
- **可空性**：`Nullable:true` → `Option<OrderSide>` / `Option<FeaturePerm>`，仍遵守 §4 单轴 `Nullable` 规则。
- **写默认**：初始状态（如 `OrderSide::Buy`、权限 `Read`）由 `BeforeWriteDB` 显式赋值；`DbDefault` 只生成 DDL `DEFAULT`。
- **字典联动（可选）**：`Variants.label` 可与业务字典表（如 BRSK 的 `tb_baseoper_dictionary`）对齐，供展示层翻译，但不参与类型校验。
- **`enum` 稀疏值**：允许 `value` 非连续（如 Cancel=9），Rust `#[repr]` 与 `from_i32` 均支持。

---

## 9. 参数化 / 可复用领域类型

除逐个命名的 domain 外，提供**一个定义派生一族**的参数化模板（如 `MaxStr<N>`），避免手写 `Str4/Str8/Str16`。

### 9.1 const 泛型实现（机制 A：一份 impl 覆盖所有 N）

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxStr<const N: usize>(String);   // N = 最大长度

impl<const N: usize> MaxStr<N> {
    pub fn new(s: impl Into<String>) -> Result<Self, StrError> {
        let s = s.into();
        if s.chars().count() > N { return Err(StrError::TooLong { max: N, got: s.chars().count() }); }
        Ok(Self(s))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

// 两个正交泛型维度：const N + impl<DB>；桥接 String，一份实现适用所有 N 与两引擎
// impl<const N, DB> Type<DB> / Encode<'q,DB> / Decode<'r,DB> for MaxStr<N> ...（同 Age 模式）
```

于是实体里直接写 `MaxStr<8>`、`MaxStr<16>`，`Str4/Str8/Str16` 自动成立，零枚举成本。

### 9.2 catalog 侧（机制 B：domain_templates）

已在 catalog `domain_templates` 定义 `MaxStr` / `ExactStr`；实体写 `type: "MaxStr(8)"`
→ 复用 `MaxStr<8>`，DDL 出 `VARCHAR(8)`。详见 §2 catalog。

### 9.3 匿名类型 vs 命名 domain（关键区分）

| | `MaxStr<8>` / `Str8` | `SecuCode` |
|---|---|---|
| 类型身份 | **结构型**：任何「长 8」列可互换 | **名义型**：独立类型，别的字段塞不进 |
| 可读性 | 签名只说「8 长的串」 | 签名即业务含义 |
| 附加规则 | 仅长度 | 长度 + 字符集(RowCheck) + 可扩展 |

**选型规则**：
1. 纯长度、语义弱、可互换无所谓 → `MaxStr<N>`（缩写/短码/标志位）。
2. 真正业务键 / 需名义隔离 / 有额外规则 → 命名 domain（哪怕底层就是 `Str(5..6)` 的 `SecuCode`）。
3. 既要长度快捷又要名义隔离 → 命名 domain 包一层：`struct PostCode(MaxStr<6>)`。
   （注：`type PostCode = MaxStr<6>` 只是别名，不产生新类型，仍可互赋值；需严格隔离就用 `struct` 包。）

### 9.4 数值同理（收益较低，暂缓）

理论上可泛型化 `Range<const LO, const HI>`；但数值约束通常语义强、位宽多变，泛型收益不如字符串明显，**建议先只做 `MaxStr<N>` / `ExactStr<N>`**。

---

## 10. Stage2 字段字典 entity_dicts

在「类型」与「实体」之间插入一层**可复用列定义**，把一列的完整语义定义一次、多表引用。
对同名列（`secu_code`/`asac_no`/`create_date`）在几百张 `tb_*` 表重复的 schema 尤其收益大。

### 10.1 entity_dicts.yaml

```yaml
# graymarkset/entity_dicts.yaml —— 可复用字段字典（Stage2）
version: 1
dicts:
  secu_code:
    Type: SecuCode            # 引用 catalog：domain 名 / "MaxStr(6)" / 标量 INT32…
    Nullable: false           # 可覆盖 domain 默认（不填则继承）
    Default: "00918"          # 只喂 BeforeWriteDB（写侧默认）；不进入 DbDefault
    CnName: 证券代码
    EnName: SecuCode
    Remark: 交易所证券代码，5 或 6 位字母数字

  asac_no:
    Type: INT64
    Nullable: false
    CnName: 账户编号
    EnName: AssetAccountNo

  remark_info:
    Type: "MaxStr(255)"       # 结构型模板：纯长度、语义弱
    Nullable: true
    Default: ""
    CnName: 备注
    EnName: Remark
```

### 10.2 实体的默认值只进 BeforeWriteDB（定稿）

字典的 `Default` **只有一个语义**：写侧默认，只生成 `BeforeWriteDB` 体；
**不产生 DDL `DEFAULT`**。DDL 默认若需，仅在 catalog 的 `domains.DbDefault` 表达，与字段字典正交：

| 来源 | 喂给 | 语义 |
|---|---|---|
| `entity_dicts.Default` | **`BeforeWriteDB`** | 应用写库时，未赋值则填此默认 |
| `catalog.domains.DbDefault` | **DDL `DEFAULT`** | DB 安全网，不参与 Rust 写路径 |

生成示例（可空列的写默认）：

```rust
impl BeforeWriteDB for AccountInfo {
    fn before_write(&mut self) {
        if self.remark_info.is_none() {
            self.remark_info = Some(MaxStr::<255>::new("").unwrap());
        }
        // secu_code 为 NOT NULL，构造已必填，通常无需在此填
    }
}
```

### 10.3 实体引用与 override

```yaml
entities:
  - name: AccountInfo
    table: tb_account_info
    columns:
      - { ref: row_id, identity: true }
      - secu_code                          # 纯引用，完全继承 dict
      - { ref: asac_no }
      - { ref: secu_code, as: alt_secu_code, Nullable: true }  # 改列名 + 覆盖可空
```

解析优先级（就近覆盖）：`实体 override > entity_dicts 条目 > catalog domain/scalar`。

### 10.4 codegen 展开算法

1. 实体列→若为 `ref`，查 `entity_dicts[key]`；未命中且名称合法则当临时标量/domain 处理。
2. 合并：dict 默认值/可空，实体 override 覆盖。
3. `Type` →查 catalog（§2）得 Rust 类型 + DDL 模板。
4. 输出：实体字段 + DDL 列 + `BeforeWriteDB`（从 `Default`）+ `RowCheck` 骨架。

### 10.5 生成期 fail-fast 校验

- **`Default` 必须满足 `Type` 约束**：如 `SecuCode::new("00918")` → 长 5∈[5,6]、字母数字 ✓；不满足则生成期报错。
- **引用完整性**：实体 `ref`→dict、dict `Type`→catalog 条目均必须存在，断链即失败。

### 10.6 可选：字典做成真正的 DB 元数据表

若希望运行时可查/UI 编辑，可将 Stage2 落地为元数据表（如 `tb_meta_entity_dict(key, type, nullable, default, cn_name, en_name, remark)`），
codegen 从表读取。两种载体共存：YAML 为准、导入表供 UI。

### 10.7 规模化：字典≠列映射，与格式选型（待拍板）

当字典膨胀到**上万行**时，首先应拆概念——上万行说明把「每表每列的映射」也塞进了字典：

| 概念 | 规模 | 性质 |
|---|---|---|
| **entity_dict**（列概念字典） | 几百行 | 可复用真值，稳定 |
| **entity_column**（实体↔字典映射 + override） | 上万行 | 每表每列一条，bulk 数据 |

真正上万的是后者，它是纯二维关系数据。

**按角色的格式选型**：

| 载体 | 适合 | 不适合 |
|---|---|---|
| YAML/Git | 类型目录 `catalog`（几十行、代码级、需 review/diff） | 上万行映射（缩进易错、diff 噪音、无约束、慢） |
| CSV/TSV | 批量编辑 + 导入导出 + Excel 互通 | 表达 override/嵌套、类型校验弱 |
| DB 元数据表 | 上万行、需完整性/查询/多人并发 | 纯文本 diff |
| Excel(.xlsx) | 业务方录入/查看的**前端** | 作真值源（二进制、无 diff、合并冲突） |

**按规模的选型阈值**：<~200 行 YAML 即可；数百～低千行 → CSV/拆多文件；万级/多人/需完整性 → DB 元数据表（+ Excel/CSV 作录入前端）。

> ✅ **真值源载体已定板：继续 YAML 拆分**（见 §10.8）。`entity_dict` 用 YAML（≤6000 行，按模块拆多文件）；实体一类一文件。超阈值或高频多人编辑时再评估 CSV/DB。
> 恒定的不变量：
> - catalog.yaml 始终留 Git；
> - `Default` 仅喂 `BeforeWriteDB`、`DbDefault` 仅在 catalog（与 §10.2 一致）；
> - 入库/生成前的 fail-fast 校验（引用完整性 + Default 满足 Type）不变。

### 10.8 文件布局与载体定稿

```
graymarkset/
  catalog.yaml                 # Stage1 类型（留 Git）
  entity_dicts/                # Stage2 字典：按模块拆多文件，合并为一个 dict map
    base.yaml                  #   公共/账户类列概念
    secu.yaml
    prod.yaml
  entities/                    # Stage3 实体：一类一 YAML 文件
    tb_account_info.yaml
    tb_seconv_fixorder.yaml
    ...
```

**entity_dict（Stage2）**：
- 按模块拆为 `entity_dicts/*.yaml`，codegen 合并为一个 map；**重复 key = 报错**（fail-fast）。
- 条目用 **flow 单行式**，一 entry 一行，git diff/排序干净：
  ```yaml
  dicts:
    - { key: secu_code,   Type: SecuCode,     Nullable: false, Default: "00918", CnName: 证券代码, EnName: SecuCode }
    - { key: asac_no,     Type: INT64,        Nullable: false, CnName: 账户编号, EnName: AssetAccountNo }
    - { key: remark_info, Type: "MaxStr(255)", Nullable: true,  Default: "",     CnName: 备注, EnName: Remark }
  ```
- **阈值**：≤ 6000 行单行式 YAML 可接受；再涨就是迁 CSV/DB 的信号（回看 §10.7）。

**entities（Stage3）**：**一个实体类一个 YAML 文件**（定为标准）。理由：与产物 `.rs` 1:1、diff/冲突最小、可增量生成。实体文件只写 `ref` + 少量 override，不重复列定义。

分工：**字典 = 库（几个模块文件，集中复用）**，**实体 = 消费方（很多小文件，只引用）**。

> 注：YAML 跨文件不支持 anchor 引用，实体靠字符串 key 查字典（而非 YAML alias）。

---

## 11. 待落地清单（下一步）

- [ ] 建 `graymarkset/` Rust 工程：`Cargo.toml`（feature `postgres` + `mysql` + `chrono` + `rust_decimal` + `sqlx::FromRow` + `bitflags`）
- [ ] `domain_scalar!` 宏：由离散约束生成 `Type/Encode/Decode`
- [ ] `Age` / `SecuCode` / `Amount` 三个领域类型 + 单测
- [ ] `enum`/`flags` 两类领域类型生成（`OrderSide` 原生 enum + `FeaturePerm` bitflags）+ 单测
- [ ] 参数化模板 `MaxStr<N>` / `ExactStr<N>`（const 泛型，一份 impl 覆盖所有 N）+ 单测
- [ ] `BeforeWriteDB` / `RowCheck` trait 定义
- [ ] Stage2 `entity_dicts.yaml` 字典解析 + 实体 `ref` 查表展开（`Default`→`BeforeWriteDB`）
- [ ] codegen：读 catalog + entity.yaml → 输出 PG/MySQL 两份 DDL + Rust 实体（含两个旁路 impl 骨架）
- [ ] `cargo build` 验证泛型 impl 在两 feature 下均编译通过
