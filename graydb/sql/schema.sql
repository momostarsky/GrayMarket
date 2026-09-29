-- GrayDB 主数据 schema —— codegen 的唯一列名/类型事实源。
--
-- 生产环境由 `pg_dump --schema-only` 维护；此处手写以贴合当前 mock 表。
-- 约定：
--   * 列名（含引号大小写，如未来 "Symbol"）由本文件决定，生成器据此产出
--     结构体字段 + 列枚举 `as_str`，二者同源，不可能写错。
--   * DDL 只描述物理类型（text / bigint / numeric(p,s) …）；
--     `numeric` → `Money` / `Price` / `Quantity` 的语义映射在 tables.toml 声明，
--     DDL 推不出来（例如 `frozen` 物理上是 decimal，业务上是 Money，不是反过来）。
--   * 生成器只解析受控子集：`CREATE TABLE <id> ( <col> <type> [约束], ... );`，
--     约束（PRIMARY KEY / NOT NULL）仅供人读，主键/外键以 tables.toml 为准。

CREATE TABLE user_info (
    user_id     text        PRIMARY KEY,
    username    text        NOT NULL,
    phone       text,
    id_type     text,                       -- 业务枚举 IdType（tables.toml 映射）
    id_number   text,
    status      text,                        -- 业务枚举 UserStatus
    opened_at   timestamptz                  -- 模拟阶段按 ISO8601 String 处理
);

CREATE TABLE account_info (
    account_id    text   PRIMARY KEY,
    user_id       text   NOT NULL,
    account_name  text,
    status        text                       -- 业务枚举 AccountStatus
);

CREATE TABLE dict_security (
    symbol      text          PRIMARY KEY,
    name        text          NOT NULL,
    currency    text,
    lot_size    numeric(20,0) NOT NULL,      -- → Quantity (SCALE 0)
    price_tick  numeric(20,4) NOT NULL,      -- → Price    (SCALE 4)
    market      text
);

CREATE TABLE account_asset (
    account_id          text         PRIMARY KEY,
    currency            text,
    available           numeric(28,2) NOT NULL,  -- → Money (SCALE 2)；物理是 decimal
    frozen              numeric(28,2) NOT NULL,  -- → Money；DDL 不携带此语义，靠 tables.toml
    total_market_value  numeric(28,2) NOT NULL   -- → Money
);

CREATE TABLE position (
    account_id     text          NOT NULL,     -- 复合主键 (account_id, symbol)
    symbol         text          NOT NULL,
    quantity       numeric(20,0) NOT NULL,     -- → Quantity
    available_qty  numeric(20,0) NOT NULL,     -- → Quantity
    avg_cost       numeric(20,4) NOT NULL,     -- → Price
    PRIMARY KEY (account_id, symbol)
);

-- 内核写路径（阶段 1.1）：orders / trades 是 Kind::State，内核独占写。
-- order_id 同时是幂等键（阶段 1.4）：重复提交返回原单，不重复冻结。
CREATE TABLE orders (
    order_id    text          PRIMARY KEY,
    account_id  text          NOT NULL,
    symbol      text          NOT NULL,
    side        text          NOT NULL,         -- 业务枚举 Side
    price       numeric(20,4) NOT NULL,         -- → Price
    quantity    numeric(20,0) NOT NULL,         -- → Quantity
    filled_qty  numeric(20,0) NOT NULL,         -- → Quantity
    status      text          NOT NULL,         -- 业务枚举 OrderStatus
    created_at  timestamptz,                    -- ISO8601 String（遗留 7：时间策略先定字符串）
    seq         bigint        NOT NULL          -- 内核全局序号（阶段 1.3）
);

CREATE TABLE trades (
    trade_id   text          PRIMARY KEY,
    order_id   text          NOT NULL,          -- 外键 → orders.order_id
    account_id text          NOT NULL,
    symbol     text          NOT NULL,
    side       text          NOT NULL,          -- 业务枚举 Side
    price      numeric(20,4) NOT NULL,          -- → Price
    quantity   numeric(20,0) NOT NULL,          -- → Quantity
    amount     numeric(28,2) NOT NULL,          -- → Money = notional(price, qty)
    seq        bigint        NOT NULL
);
