//! 单线程内核写路径（阶段 1）：下单 → 校验 → 冻结 → 先写 journal → 改内存 → 回 ACK。
//!
//! 三条铁律落在类型与顺序上，不靠调用方自觉：
//! - **seq 单调 + 空洞检测**（1.3）：每笔写恰好消耗一个序号，已分配与已落盘不一致即停止内核
//!   —— 宁停不脏，日志与内存绝不能分叉；
//! - **幂等键**（1.4）：同 `order_id` 同内容重复提交返回原单序号，不产生任何状态变化；
//!   同键不同内容是真冲突，`DuplicateOrder` 拒绝；
//! - **事务边界**（1.7）：一笔成交牵动的多表变更（order + trade + asset + position）
//!   共用同一个 `seq`，整组先落 journal 再改内存。
//!
//! 拒单原因 [RejectReason] 是热路径的结构化出口（取代原语返回 `bool` 丢信息的旧写法，1.2）。
//!
//! journal 记的是**实体流水**（order / trade 整行），不是 asset/position 的逐字段 delta。
//! 所以恢复（阶段 2）的口径是「按流水重建余额」：重放 order 得出应冻结额、重放 trade
//! 得出实付/实收与持仓，而不是把 journal 里的数字往内存上抹。写失败一律 panic 而不返
//! `Err`，正是为了让「内存已改、日志未落」这个窗口不存在。
//!
//! 阶段 3.1 之后，内核的终点不止 journal：每个事务组在内存变更**全部成功**后，把「这一组
//! 真正改过的行」发布到 [`Broadcast`] 订阅出口。出口是纯内存 append，不等任何消费者。
use std::fmt;
use std::path::Path;

use account::amount::{Amount, MicroAmount, Money, Price, Quantity, Rounding, notional};
use rust_decimal::Decimal;
use serde::Serialize;

use crate::domain::{Asset, Order, OrderStatus, Position, Side, Trade};
use crate::journal::{Entry, GroupWriter, Journal, Record};
use crate::mem::Snapshot;
use crate::pubsub::{Broadcast, CatchUp, DEFAULT_RING_ROWS, Note, Op, RowChange};
use crate::tables::{composite_key_str, DataTable};

/// 拒单原因。计划 1.2 的六个为主，其余是实现全链路时必然要区分的分支（宁可多列，不合并成
/// 一个 `Other` 让排查无从下手）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    /// 账户不存在。
    UnknownAccount,
    /// 账户状态非 Active（Closed 等）。
    NotActive,
    /// 账户被冻结。
    AccountFrozen,
    /// 证券不存在。
    UnknownSymbol,
    /// 数量小于或不是 lot_size 的整数倍。
    BelowLotSize,
    /// 可用资金不足（含溢出）。
    InsufficientFunds,
    /// 可卖数量不足（T+1：当日买入不可卖）。
    InsufficientPosition,
    /// 价格非正。
    InvalidPrice,
    /// 成交价劣于委托价（买入高于限价 / 卖出低于限价）。
    InvalidFillPrice,
    /// 订单不存在。
    UnknownOrder,
    /// 订单已终态（Filled / Cancelled / Rejected），不可再改。
    OrderNotLive,
    /// 成交量超过委托剩余量。
    OverFill,
    /// 同 `order_id` 但内容不同 —— 真冲突，不是幂等重放。
    DuplicateOrder,
    /// 冻结余额被算坏（内核状态不一致，正常路径不可达）。
    FrozenUnderflow,
    /// 成交号重复但内容不同 —— 同一成交号不能代指两笔不同的成交。
    DuplicateFill,
}

impl fmt::Display for RejectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            RejectReason::UnknownAccount => "未知账户",
            RejectReason::NotActive => "账户未激活",
            RejectReason::AccountFrozen => "账户已冻结",
            RejectReason::UnknownSymbol => "未知证券",
            RejectReason::BelowLotSize => "数量不足一手或非整数倍",
            RejectReason::InsufficientFunds => "可用资金不足",
            RejectReason::InsufficientPosition => "可卖数量不足",
            RejectReason::InvalidPrice => "价格必须为正",
            RejectReason::InvalidFillPrice => "成交价劣于委托价",
            RejectReason::UnknownOrder => "订单不存在",
            RejectReason::OrderNotLive => "订单已终态",
            RejectReason::OverFill => "成交量超过委托剩余量",
            RejectReason::DuplicateOrder => "订单号重复但内容不同",
            RejectReason::FrozenUnderflow => "冻结余额不一致",
            RejectReason::DuplicateFill => "成交号重复但内容不同",
        };
        write!(f, "{text}")
    }
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::Reject(reason) => write!(f, "拒单: {reason}"),
            KernelError::State(err) => write!(f, "状态变更失败: {err}"),
        }
    }
}

/// 内核错误：拒单（业务可预期）与状态写坏（实现缺陷 / 不变量崩塌）分流。
/// journal 写失败不走 `Result` —— 那会留下 seq 空洞，按 1.3 直接 panic 停内核。
#[derive(Debug)]
pub enum KernelError {
    Reject(RejectReason),
    State(anyhow::Error),
}

impl From<anyhow::Error> for KernelError {
    fn from(err: anyhow::Error) -> Self {
        KernelError::State(err)
    }
}

/// 资金原语（`mem::try_freeze` 等）直接返回拒单原因，热路径上 `?` 就能落地（1.2）。
impl From<RejectReason> for KernelError {
    fn from(reason: RejectReason) -> Self {
        KernelError::Reject(reason)
    }
}

pub type KernelResult<T> = Result<T, KernelError>;

/// 只在测试里给 [`KernelError`] 补上相等比较：`anyhow::Error` 没有结构相等，
/// 用 `to_string()` 比会假装出一种并不存在的语义 —— 所以下面只承认真实需要的
/// `Reject` 分支比较，断言到 `State` 一律不相等（那种用例本就该靠 panic 暴露）。
#[cfg(test)]
impl PartialEq for KernelError {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (KernelError::Reject(a), KernelError::Reject(b)) if a == b)
    }
}

/// 受理结果。`Duplicate` 是幂等重放的正常出口，不是错误：客户端重试不该重复扣款。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ack {
    Accepted { seq: i64 },
    Duplicate { seq: i64 },
}

impl Ack {
    #[must_use]
    pub fn seq(self) -> i64 {
        match self {
            Ack::Accepted { seq } | Ack::Duplicate { seq } => seq,
        }
    }
}

/// 下单请求。7 个字段挤在一串位置参数里极易调错顺序，故用具名结构。
#[derive(Debug, Clone, Copy)]
pub struct PlaceRequest<'a> {
    pub order_id: &'a str,
    pub account_id: &'a str,
    pub symbol: &'a str,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
    /// ISO8601 字符串（遗留 7 的裁决：模拟阶段时间单一按字符串处理，接入真实时钟时只改一处）。
    pub created_at: &'a str,
}

/// [`Engine::recover`] 的结果：重建好的内核 + 回放统计。
///
/// `dropped_tail` 非空 = 崩溃吞掉了最后一组（日志末尾半行），恢复出来的状态比崩溃前
/// **少一笔**。调用方必须显式处置（停服核对 / 向从库补齐），不能当正常启动继续撮合（2.4）。
pub struct Recovery {
    pub engine: Engine,
    /// 实际回放并采纳的记录数。
    pub replayed: usize,
    pub dropped_tail: Option<String>,
}

/// 单线程内核：独占 [Snapshot] 的写权限 + seq + journal。
///
/// 刻意不 `Send`/不 intern 锁 —— 「单线程内核独占写、多读者读」是既定架构（阶段 0.5 决策），
/// 并发在阶段 3 用 broadcast 出口解决，而不是把内核改成多线程共享可变状态。
pub struct Engine {
    pub snapshot: Snapshot,
    journal: Journal,
    /// 3.1 订阅出口：写路径的终点，有界、无锁、不等消费者。
    broadcast: Broadcast,
    /// 本组已登记、尚未发布的行变更。出口在 [`Engine::publish_group`] 收尾，
    /// 泄漏到下一组即 panic（见 [`Engine::alloc_seq`]）。
    pending: Vec<Note>,
    /// 任期。模拟阶段恒为 0，阶段 5.3 才递增；它随每条 journal 记录落盘，
    /// 恢复时跨 term 的日志一律拒绝混放（带旧 term 的请求/日志不得进新任期）。
    term: i64,
    /// 已分配的最后一个序号。
    seq: i64,
    /// 已落盘的最后一个序号。与 `seq` 不等即有空洞，下次分配前 panic。
    durable: i64,
}

impl Engine {
    pub fn new(snapshot: Snapshot, journal: Journal) -> Self {
        Self::with_ring(snapshot, journal, DEFAULT_RING_ROWS)
    }

    /// 指定补发窗口容量（行数）建内核：3.4 的慢消费者测试要用小环把淘汰跑出来。
    pub fn with_ring(snapshot: Snapshot, journal: Journal, ring_rows: usize) -> Self {
        Self {
            snapshot,
            journal,
            broadcast: Broadcast::new(ring_rows),
            pending: Vec::new(),
            term: 0,
            seq: 0,
            durable: 0,
        }
    }

    /// 当前任期（阶段 5.3 的 `term` 递增在此之上，回放端已在 2.1 就把它写进每条记录）。
    #[must_use]
    pub fn term(&self) -> i64 {
        self.term
    }

    #[must_use]
    pub fn seq(&self) -> i64 {
        self.seq
    }

    /// 已落盘序号。与 [Engine::seq] 相等才说明没有「分了号却没写盘」的空洞（1.3）。
    #[must_use]
    pub fn durable(&self) -> i64 {
        self.durable
    }

    /// 订阅出口本体（只读）：消费者靠游标拉，所以内核不需要知道有几个订阅方。
    #[must_use]
    pub fn broadcast(&self) -> &Broadcast {
        &self.broadcast
    }

    /// 3.3 补发入口：给出 `after_seq` 之后的全部行变更，或判定落后需重建快照。
    /// 只读，且拷出一段有界的 Vec —— 读侧多慢都不会回压到写路径（`CatchUp` 在类型上已 `#[must_use]`）。
    pub fn catch_up(&self, after_seq: i64) -> CatchUp {
        self.broadcast.catch_up(after_seq)
    }

    /// 1.3：每笔写恰好一个序号；上一个分配未落盘就是空洞，宁停不脏。
    fn alloc_seq(&mut self) -> i64 {
        if self.seq != self.durable {
            panic!(
                "seq 空洞: 已分配 {} 但已落盘 {} —— 停止内核，禁止带脏继续（阶段 1.3）",
                self.seq, self.durable
            );
        }
        // 3.1 的出口守卫：上一组改了内存却没发布，变更就会混进下一组（订阅端拿到错的 seq）。
        // 能走到这里只有两种事：入口漏挂 `publish_group`，或落盘之后动内存失败（日志与内存已分叉）。
        // 两者都必须停内核，不允许「抹抹补补」继续跑。
        if !self.pending.is_empty() {
            panic!(
                "seq {} 的组改了内存却没发布到订阅出口（pending {} 条）—— 入口漏挂 publish_group 或落盘后改内存失败，停内核（阶段 3.1）",
                self.durable,
                self.pending.len()
            );
        }
        let next = self
            .seq
            .checked_add(1)
            .expect("seq 溢出：内核序号已用尽，必须停服换 term/段文件");
        self.seq = next;
        next
    }

    /// 1.5 的唯一落盘点：整组记录写完后一次 `commit`（flush + `sync_data`），
    /// 成功后才允许改内存。失败即 panic —— 序号已消耗却无日志，恢复时无法与内存对上。
    ///
    /// 组内多条记录共用同一个 `(term, seq)`，落盘由本函数收尾：写句柄 [`GroupWriter`]
    /// 不带 `append` 入口之外的逃逸路径，「忘了 sync」在结构上就写不出来。
    fn commit_journal(&mut self, seq: i64, write: impl FnOnce(&mut GroupWriter<'_>) -> std::io::Result<()>) {
        let term = self.term;
        let outcome = write(&mut GroupWriter::new(&mut self.journal, term, seq))
            .and_then(|()| self.journal.commit());
        if let Err(err) = outcome {
            panic!(
                "journal 写入失败（seq={seq}）: {err} —— 序号已消耗，宁停不脏（阶段 1.3）"
            );
        }
        self.durable = seq;
    }

    /// 3.1：把一行变更登记进当前事务组。表名取自类型（`T::ID` == `Spec::id`），
    /// 调用方不写字符串字面量 —— 与 WAL / 订阅主题 / PG 表名是同一个身份。
    ///
    /// 只登记键，不在这里取值：同一组里资金行会被结算改一次、再被市值重算改一次，
    /// 登记时取到的是中间态。行内容统一由 [`Engine::publish_group`] 在组收尾时现读。
    /// 同组重复登记合并成一条（后面的 `op` 覆盖前面的），所以「先 upsert 后 delete」
    /// 最终只发一条 Delete，订阅端不会先收到一行再删一行。
    fn note_row<T: DataTable>(&mut self, key: &str, op: Op) {
        if let Some(existing) = self
            .pending
            .iter_mut()
            .find(|note| note.table == T::ID && note.key == key)
        {
            existing.op = op;
            return;
        }
        self.pending.push(Note {
            table: T::ID,
            key: key.to_string(),
            op,
        });
    }

    /// 事务组的出口：内存变更全部成功后由三个入口调用。把 pending 翻译成 [`RowChange`]
    /// （行内容仍从内存表现取 = 本组终态），盖上 `(term, seq)` 后进环。
    fn publish_group(&mut self, seq: i64) {
        let notes = std::mem::take(&mut self.pending);
        if notes.is_empty() {
            return; // 纯校验没改行（当前入口不会走到）：不推 published，也不报错
        }
        let mut changes = Vec::with_capacity(notes.len());
        for note in notes {
            let row = match note.op {
                Op::Upsert => Some(row_json(&self.snapshot, note.table, &note.key).unwrap_or_else(|| {
                    panic!(
                        "出口登记的 {} 行 {} 在表里找不到 —— 登记与实际写入不一致（阶段 3.1）",
                        note.table, note.key
                    )
                })),
                Op::Delete => None,
            };
            changes.push(RowChange {
                term: self.term,
                seq,
                table: note.table,
                key: note.key,
                op: note.op,
                row,
            });
        }
        self.broadcast.publish_group(changes);
    }

    /// 下单：全部校验通过后才冻结资金，再落 journal，最后改内存。
    /// 任何拒单发生在写盘之前 —— 拒单零状态变化（验收标准）。
    pub fn place(&mut self, req: PlaceRequest<'_>) -> KernelResult<Ack> {
        // `anyhow::ensure!` 不能直接用在这里：它 `return Err(anyhow::Error)`，不经过 `From`，
        // 而本层的错误类型是 `KernelError`。空 `order_id` 是调用方契约违反，走 `State`。
        if req.order_id.is_empty() {
            return Err(KernelError::State(anyhow::anyhow!("order_id 不能为空（幂等键）")));
        }

        // 1.4 幂等：同键同内容 → 返回原单序号，不重复冻结、不重复写盘。
        if let Some(existing) = self.snapshot.orders.get(req.order_id) {
            let same_content = existing.account_id == req.account_id
                && existing.symbol == req.symbol
                && existing.side == req.side
                && existing.price == req.price
                && existing.quantity == req.quantity;
            if !same_content {
                return Err(KernelError::Reject(RejectReason::DuplicateOrder));
            }
            return Ok(Ack::Duplicate {
                seq: existing.seq,
            });
        }

        let account = self
            .snapshot
            .accounts
            .get(req.account_id)
            .ok_or(KernelError::Reject(RejectReason::UnknownAccount))?;
        match account.status {
            crate::domain::AccountStatus::Active => {}
            crate::domain::AccountStatus::Frozen => {
                return Err(KernelError::Reject(RejectReason::AccountFrozen));
            }
            crate::domain::AccountStatus::Closed => {
                return Err(KernelError::Reject(RejectReason::NotActive));
            }
        }

        let security = self
            .snapshot
            .security(req.symbol)
            .ok_or(KernelError::Reject(RejectReason::UnknownSymbol))?;
        if !req.price.is_positive() {
            return Err(KernelError::Reject(RejectReason::InvalidPrice));
        }
        if !req.quantity.is_positive() {
            return Err(KernelError::Reject(RejectReason::BelowLotSize));
        }
        // 整手校验直接拿 `units()` 取余：`Quantity` 刻意没开 `checked_rem_int`，
        // 不为这一个校验泛化金额层，序号层面的整除性用原始整数最直白。
        let lot = security.lot_size.units();
        if lot <= 0 || req.quantity.units() % lot != 0 {
            return Err(KernelError::Reject(RejectReason::BelowLotSize));
        }

        // 买入试算冻结额、卖出试算可卖量：真正的扣减在 `apply_place_effect`，
        // 这里只保证「日志落下去的这组，内存一定变得动」，且拒单发生在写盘之前。
        match req.side {
            Side::Buy => {
                let amount: Money = notional(req.price, req.quantity, Rounding::MidpointAwayFromZero)
                    .ok_or_else(|| anyhow::anyhow!("买入冻结额计算溢出"))
                    .map_err(KernelError::State)?;
                let asset = self
                    .snapshot
                    .asset(req.account_id)
                    .ok_or(KernelError::Reject(RejectReason::UnknownAccount))?;
                if asset.available < amount {
                    return Err(KernelError::Reject(RejectReason::InsufficientFunds));
                }
            }
            Side::Sell => {
                let position = self
                    .snapshot
                    .position(req.account_id, req.symbol)
                    .ok_or(KernelError::Reject(RejectReason::InsufficientPosition))?;
                if position.available_qty < req.quantity {
                    return Err(KernelError::Reject(RejectReason::InsufficientPosition));
                }
            }
        }

        let seq = self.alloc_seq();
        let order = Order {
            order_id: req.order_id.to_string(),
            account_id: req.account_id.to_string(),
            symbol: req.symbol.to_string(),
            side: req.side,
            price: req.price,
            quantity: req.quantity,
            filled_qty: Quantity::from_units(0),
            status: OrderStatus::New,
            created_at: req.created_at.to_string(),
            seq,
        };
        order
            .check_row()
            .map_err(|err| anyhow::anyhow!("订单不合法: {err}"))
            .map_err(KernelError::State)?;

        self.commit_journal(seq, |group| group.order(&order));

        self.apply_place_effect(&order)?;
        self.publish_group(seq);
        Ok(Ack::Accepted { seq })
    }

    /// 下单的内存效应：冻结现金 / 锁定可卖量，再把订单行入表。
    ///
    /// live 路径（[`Engine::place`]）与回放路径（[`Engine::replay_record`]）共用同一个函数 ——
    /// 「重放结果 == 崩溃前终态」是**构造保证**，不是两套算法碰巧算得一样。
    /// 先动资金再入行：冻结失败时订单行不该已经躺在表里（与「拒单零状态变化」同一口径）。
    fn apply_place_effect(&mut self, order: &Order) -> KernelResult<()> {
        match order.side {
            Side::Buy => {
                let amount: Money =
                    notional(order.price, order.quantity, Rounding::MidpointAwayFromZero)
                        .ok_or_else(|| anyhow::anyhow!("买入冻结额计算溢出"))
                        .map_err(KernelError::State)?;
                let key = order.account_id.clone();
                let asset = self
                    .snapshot
                    .assets
                    .rows_mut()
                    .get_mut(&key)
                    .ok_or(KernelError::Reject(RejectReason::UnknownAccount))?;
                crate::mem::try_freeze(asset, amount)?;
                self.note_row::<Asset>(&key, Op::Upsert);
            }
            Side::Sell => {
                self.lock_sell_quantity(&order.account_id, &order.symbol, order.quantity)?;
            }
        }
        self.snapshot.orders.upsert(order.clone())?;
        self.note_row::<Order>(&order.order_id, Op::Upsert);
        Ok(())
    }

    /// 极简撮合（阶段 1 规则）：限价单立即全成，价格 = 委托价。
    /// 价格/时间优先的真实撮合在后续迭代，不影响本层的资金与状态机正确性。
    pub fn place_and_fill(&mut self, req: PlaceRequest<'_>) -> KernelResult<Ack> {
        let ack = self.place(req)?;
        if matches!(ack, Ack::Accepted { .. }) {
            // 立即全成只有一笔成交，成交号由订单号衍接序号生成（外部成交号属于真实交易所接入时的入参）。
            let trade_id = format!("{}-1", req.order_id);
            return self.apply_fill(&trade_id, req.order_id, req.quantity, req.price);
        }
        Ok(ack)
    }

    /// 成交回报：同一 `seq` 覆盖 order + trade + asset + position 四表变更（1.7 事务边界）。
    ///
    /// `trade_id` 是成交侧的幂等键（对应交易所的 exec id）：由内核自己用“订单号+已成交量”拼
    /// 并不能拦住同一回报重放两次（第二次会拼出另一个号、静默重复扣款），所以必须外部传入。
    pub fn apply_fill(
        &mut self,
        trade_id: &str,
        order_id: &str,
        fill_qty: Quantity,
        fill_price: Price,
    ) -> KernelResult<Ack> {
        // 1.4 幂等（成交侧）：同成交号同内容 → 返回原序号，不重复扣款、不重复落盘。
        if let Some(existing) = self.snapshot.trades.get(trade_id) {
            let same_content = existing.order_id == order_id
                && existing.quantity == fill_qty
                && existing.price == fill_price;
            if !same_content {
                return Err(KernelError::Reject(RejectReason::DuplicateFill));
            }
            return Ok(Ack::Duplicate {
                seq: existing.seq,
            });
        }

        let mut order = self
            .snapshot
            .orders
            .get(order_id)
            .cloned()
            .ok_or(KernelError::Reject(RejectReason::UnknownOrder))?;
        if !matches!(order.status, OrderStatus::New | OrderStatus::PartiallyFilled) {
            return Err(KernelError::Reject(RejectReason::OrderNotLive));
        }
        if !fill_qty.is_positive() || !fill_price.is_positive() {
            return Err(KernelError::Reject(RejectReason::InvalidPrice));
        }
        let remaining = sub_or_negative(order.quantity, order.filled_qty)
            .ok_or_else(|| anyhow::anyhow!("订单 {order_id} 剩余量为负：已成交量超过委托量"))
            .map_err(KernelError::State)?;
        if fill_qty > remaining {
            return Err(KernelError::Reject(RejectReason::OverFill));
        }
        let fully_filled = fill_qty.units() == remaining.units();

        // 卖出先校验持仓总量：可卖额度在 `place` 已锁定，这里只确认成交不吃负。
        if order.side == Side::Sell {
            let position = self
                .snapshot
                .position(&order.account_id, &order.symbol)
                .ok_or(KernelError::Reject(RejectReason::InsufficientPosition))?;
            let consumed = order
                .filled_qty
                .checked_add(fill_qty)
                .ok_or(KernelError::Reject(RejectReason::OverFill))?;
            if position.quantity < consumed {
                return Err(KernelError::Reject(RejectReason::InsufficientPosition));
            }
        }

        let amount: Money = notional(fill_price, fill_qty, Rounding::MidpointAwayFromZero)
            .ok_or_else(|| anyhow::anyhow!("成交金额计算溢出"))
            .map_err(KernelError::State)?;

        // 限价约束：成交价只能优于或等于委托价（同量同舍入下 `notional` 单调，
        // 所以这道校验同时保证“按限价的冻结额 ≥ 实扣额”，差额不会算成负数）。
        match order.side {
            Side::Buy if fill_price > order.price => {
                return Err(KernelError::Reject(RejectReason::InvalidFillPrice));
            }
            Side::Sell if fill_price < order.price => {
                return Err(KernelError::Reject(RejectReason::InvalidFillPrice));
            }
            _ => {}
        }

        // 买入先校冻结账目：冻结额必须盖得住本笔按限价的应扣额，回补差额才不会减成负。
        // 真正的结算在 `settle_fill`（它自己按同一算式重算 owed，不依赖这里的临时量），
        // 多一次乘法换「live 与回放只有一份落账算法」。
        if order.side == Side::Buy {
            let owed: Money = notional(order.price, fill_qty, Rounding::MidpointAwayFromZero)
                .ok_or_else(|| anyhow::anyhow!("本笔成交的限价冻结额计算溢出"))
                .map_err(KernelError::State)?;
            let asset = self
                .snapshot
                .asset(&order.account_id)
                .ok_or(KernelError::Reject(RejectReason::UnknownAccount))?;
            if asset.frozen < owed {
                return Err(KernelError::Reject(RejectReason::FrozenUnderflow));
            }
        }

        let seq = self.alloc_seq();
        order.filled_qty = order
            .filled_qty
            .checked_add(fill_qty)
            .ok_or(KernelError::Reject(RejectReason::OverFill))?;
        order.status = if fully_filled {
            OrderStatus::Filled
        } else {
            OrderStatus::PartiallyFilled
        };
        order.seq = seq;
        order.check_row().map_err(KernelError::State)?;

        let trade = Trade {
            trade_id: trade_id.to_string(),
            order_id: order.order_id.clone(),
            account_id: order.account_id.clone(),
            symbol: order.symbol.clone(),
            side: order.side,
            price: fill_price,
            quantity: fill_qty,
            amount,
            seq,
        };
        trade.check_row().map_err(KernelError::State)?;

        // 整组（trade + 更新后的 order）先落盘，之后才动内存。
        let order_for_journal = order.clone();
        let trade_for_journal = trade.clone();
        self.commit_journal(seq, move |group| {
            group.trade(&trade_for_journal)?;
            group.order(&order_for_journal)
        });

        // 落账：资金 + 持仓 + 市值对账（回放走同一个 `settle_fill`）。
        self.settle_fill(&order, &trade)?;

        // 成交必须写回 orders 表，并把这笔成交落进 trades 表（同一 seq 已落盘）。
        let order_id = order.order_id.clone();
        self.snapshot.orders.replace(&order_id, order)?;
        self.note_row::<Order>(&order_id, Op::Upsert);
        let trade_id = trade.trade_id.clone();
        self.snapshot.trades.upsert(trade)?;
        self.note_row::<Trade>(&trade_id, Op::Upsert);

        self.publish_group(seq);
        Ok(Ack::Accepted { seq })
    }

    /// 成交的落账效应：资金结算（含逐笔回补限价差额）+ 持仓更新 + 市值重算与对账。
    ///
    /// live（[`Engine::apply_fill`]）与回放（[`Engine::replay_record`]）共用。两个入参分职责：
    /// `order` 提供**委托价**（限价差额按它算），`trade` 提供成交额 —— 直接用
    /// `trade.amount` 而不重算，日志里的数字与内存里的数字因此是同一个来源。
    /// 不回写 orders/trades 表：那是调用方的事（回放时 order 行就在下一条记录里）。
    fn settle_fill(&mut self, order: &Order, trade: &Trade) -> KernelResult<()> {
        let account_id = order.account_id.clone();
        {
            let asset = self
                .snapshot
                .assets
                .rows_mut()
                .get_mut(&account_id)
                .ok_or(KernelError::Reject(RejectReason::UnknownAccount))?;
            match order.side {
                Side::Buy => {
                    crate::mem::settle_frozen_out(asset, trade.amount)?;
                    // 逐笔回补而不是一次性退差：部分成交同样存在成交价优于限价的情况，
                    // 不回补就会把现金锁死在冻结里。
                    let owed_at_limit: Money = notional(
                        order.price,
                        trade.quantity,
                        Rounding::MidpointAwayFromZero,
                    )
                    .ok_or_else(|| anyhow::anyhow!("本笔成交的限价冻结额计算溢出"))
                    .map_err(KernelError::State)?;
                    let rebate = owed_at_limit.checked_sub(trade.amount).ok_or_else(|| {
                        anyhow::anyhow!("限价差额为负：成交价劣于限价却通过了校验")
                    })
                    .map_err(KernelError::State)?;
                    if !rebate.is_zero() {
                        crate::mem::unfreeze(asset, rebate)?;
                    }
                }
                Side::Sell => {
                    asset.available = asset
                        .available
                        .checked_add(trade.amount)
                        .ok_or_else(|| anyhow::anyhow!("卖出入账溢出"))
                        .map_err(KernelError::State)?;
                }
            }
        }
        self.update_position(order, trade.quantity, trade.price)?;
        // 聚合不变量：市值由持仓现算，落账后立刻与资产表对齐（差一分即停）。
        self.recalc_market_value(&account_id)?;
        self.snapshot.check_valuation().map_err(KernelError::State)?;
        Ok(())
    }

    /// 撤单：解冻残余资金 / 释放锁定的可卖数量，状态置 Cancelled。
    pub fn cancel(&mut self, order_id: &str) -> KernelResult<Ack> {
        let mut order = self
            .snapshot
            .orders
            .get(order_id)
            .cloned()
            .ok_or(KernelError::Reject(RejectReason::UnknownOrder))?;
        if !matches!(order.status, OrderStatus::New | OrderStatus::PartiallyFilled) {
            return Err(KernelError::Reject(RejectReason::OrderNotLive));
        }

        // 试算解冻额：撤单落盘前必须确认冻结账目盖得住，否则 `apply_cancel_effect`
        // 会在日志已落之后才报 FrozenUnderflow（内存与日志就分叉了）。
        if order.side == Side::Buy {
            let unfilled = sub_or_negative(order.quantity, order.filled_qty)
                .ok_or_else(|| anyhow::anyhow!("撤单时已成交量超过委托量"))
                .map_err(KernelError::State)?;
            let residual: Money = notional(order.price, unfilled, Rounding::MidpointAwayFromZero)
                .ok_or_else(|| anyhow::anyhow!("撤单解冻额计算溢出"))
                .map_err(KernelError::State)?;
            let asset = self
                .snapshot
                .asset(&order.account_id)
                .ok_or(KernelError::Reject(RejectReason::UnknownAccount))?;
            if asset.frozen < residual {
                return Err(KernelError::Reject(RejectReason::FrozenUnderflow));
            }
        }

        let seq = self.alloc_seq();
        order.status = OrderStatus::Cancelled;
        order.seq = seq;
        order.check_row().map_err(KernelError::State)?;

        self.commit_journal(seq, |group| group.order(&order));

        self.apply_cancel_effect(&order)?;
        // 先拿主键再移入：`replace(&order.order_id, order)` 会在同一表达式里既借又移。
        let order_id = order.order_id.clone();
        self.snapshot.orders.replace(&order_id, order)?;
        self.note_row::<Order>(&order_id, Op::Upsert);
        self.publish_group(seq);
        Ok(Ack::Accepted { seq })
    }

    /// 撤单的内存效应：解冻未成交部分 / 释放锁定的可卖量。live 与回放共用。
    /// 残余额从订单行现算（`(quantity - filled_qty) × 委托价`），与日志里那一行严格对应。
    fn apply_cancel_effect(&mut self, order: &Order) -> KernelResult<()> {
        match order.side {
            Side::Buy => {
                let unfilled = sub_or_negative(order.quantity, order.filled_qty)
                    .ok_or_else(|| anyhow::anyhow!("撤单时已成交量超过委托量"))
                    .map_err(KernelError::State)?;
                let residual: Money =
                    notional(order.price, unfilled, Rounding::MidpointAwayFromZero)
                        .ok_or_else(|| anyhow::anyhow!("撤单解冻额计算溢出"))
                        .map_err(KernelError::State)?;
                if residual.is_zero() {
                    return Ok(());
                }
                let key = order.account_id.clone();
                let asset = self
                    .snapshot
                    .assets
                    .rows_mut()
                    .get_mut(&key)
                    .ok_or(KernelError::Reject(RejectReason::UnknownAccount))?;
                crate::mem::unfreeze(asset, residual)?;
                self.note_row::<Asset>(&key, Op::Upsert);
            }
            Side::Sell => self.release_sell_quantity(order)?,
        }
        Ok(())
    }

    /// 卖单锁定可卖数量：下单即从 available_qty 扣，撤单/成交再释放或结转。
    fn lock_sell_quantity(&mut self, account_id: &str, symbol: &str, quantity: Quantity) -> KernelResult<()> {
        let key = composite_key_str(&[account_id, symbol]);
        let mut position = self
            .snapshot
            .positions
            .get(&key)
            .cloned()
            .ok_or(KernelError::Reject(RejectReason::InsufficientPosition))?;
        position.available_qty = sub_or_negative(position.available_qty, quantity)
            .ok_or(KernelError::Reject(RejectReason::InsufficientPosition))?;
        self.snapshot.positions.replace(&key, position)?;
        self.note_row::<Position>(&key, Op::Upsert);
        Ok(())
    }

    /// 撤单释放锁定的可卖数量（未成交部分）。
    fn release_sell_quantity(&mut self, order: &Order) -> KernelResult<()> {
        let unfilled = sub_or_negative(order.quantity, order.filled_qty)
            .ok_or_else(|| anyhow::anyhow!("已成交量超过委托量"))
            .map_err(KernelError::State)?;
        if unfilled.is_zero() {
            return Ok(());
        }
        let key = composite_key_str(&[&order.account_id, &order.symbol]);
        let mut position = self
            .snapshot
            .positions
            .get(&key)
            .ok_or(KernelError::Reject(RejectReason::InsufficientPosition))?
            .clone();
        position.available_qty = position
            .available_qty
            .checked_add(unfilled)
            .ok_or_else(|| anyhow::anyhow!("可卖数量回补溢出"))
            .map_err(KernelError::State)?;
        self.snapshot.positions.replace(&key, position)?;
        self.note_row::<Position>(&key, Op::Upsert);
        Ok(())
    }

    /// 1.6 持仓更新：买入加 quantity、当日不可卖（`available_qty` 不变，新建仓位为 0）；
    /// 卖出减 quantity 并结转已锁定可卖量；清仓走 `delete` 不留零行。
    fn update_position(
        &mut self,
        order: &Order,
        fill_qty: Quantity,
        fill_price: Price,
    ) -> KernelResult<()> {
        let key = composite_key_str(&[&order.account_id, &order.symbol]);
        let cost: MicroAmount = notional(fill_price, fill_qty, Rounding::MidpointAwayFromZero)
            .ok_or_else(|| anyhow::anyhow!("成交成本（中间量 6 位）溢出"))
            .map_err(KernelError::State)?;

        match order.side {
            Side::Buy => {
                let updated = match self.snapshot.positions.get(&key).cloned() {
                    Some(mut old) => {
                        let new_qty = old
                            .quantity
                            .checked_add(fill_qty)
                            .ok_or_else(|| anyhow::anyhow!("持仓数量溢出"))
                            .map_err(KernelError::State)?;
                        // 平均成本先于数量变更计算（要用旧量旧价当权重）。
                        old.avg_cost =
                            weighted_avg_price(old.quantity, old.avg_cost, cost, fill_qty)?;
                        old.quantity = new_qty;
                        old
                    }
                    None => {
                        // 新建仓位：当日买入不可卖（T+1），`available_qty` 从 0 起。
                        Position {
                            account_id: order.account_id.clone(),
                            symbol: order.symbol.clone(),
                            quantity: fill_qty,
                            available_qty: Quantity::ZERO,
                            avg_cost: weighted_avg_price(Quantity::ZERO, Price::ZERO, cost, fill_qty)?,
                        }
                    }
                };
                // 已有仓位走 `replace`（显式旧键），首次建仓走 `upsert`（纯插入）——
                // 两者的区分是机制拦截的，不能拿 upsert 当“插入或更新”用。
                if self.snapshot.positions.contains_key(&key) {
                    self.snapshot.positions.replace(&key, updated)?;
                } else {
                    self.snapshot.positions.upsert(updated)?;
                }
                self.note_row::<Position>(&key, Op::Upsert);
            }
            Side::Sell => {
                let mut position = self
                    .snapshot
                    .positions
                    .get(&key)
                    .cloned()
                    .ok_or(KernelError::Reject(RejectReason::InsufficientPosition))?;
                // 卖出消耗的是下单时已锁定的可卖额度：只减总数量，`available_qty` 已扣。
                position.quantity = sub_or_negative(position.quantity, fill_qty)
                    .ok_or(KernelError::Reject(RejectReason::InsufficientPosition))?;
                if position.quantity.is_zero() {
                    // 清仓即删行：市值聚合与持仓遍历都不该再看见零行。
                    // 注意不能按 `fully_filled` 删 —— 全成但账上还剩货的情形必须保留。
                    self.snapshot.positions.delete(&key);
                    self.note_row::<Position>(&key, Op::Delete);
                } else {
                    self.snapshot.positions.replace(&key, position)?;
                    self.note_row::<Position>(&key, Op::Upsert);
                }
            }
        }
        Ok(())
    }

    /// 市值由持仓现算后写回资产表 —— 增量累加会漂移，每次全量重算才与 `check_valuation` 同口径。
    fn recalc_market_value(&mut self, account_id: &str) -> KernelResult<()> {
        let total = market_value_of(&self.snapshot, account_id)?;
        let asset = self
            .snapshot
            .assets
            .rows_mut()
            .get_mut(account_id)
            .ok_or(KernelError::Reject(RejectReason::UnknownAccount))?;
        asset.total_market_value = total;
        self.note_row::<Asset>(account_id, Op::Upsert);
        Ok(())
    }

    /// 2.3 从「日初快照 + journal」重放，重建崩溃前的终态。
    ///
    /// 起点必须是**日初**镜像（orders/trades 为空、`available` 未被任何当日订单冻结）：
    /// 日志记的是实体流水，资金/持仓由流水推导（项目定位表里的「恢复方式 = 日志回放」）。
    /// 拿当日 `save()` 出的脏镜像再重一遍，冻结会被重复计算 —— 所以这里显式核对首条 seq 必为 1。
    ///
    /// 返回的 [`Recovery::dropped_tail`] 不是可有可无的装饰：末组被崩溃吞掉时，恢复出的状态比崩溃前少一笔，
    /// 上层需据此停服或向从库对齐，而不是默认“已经接上了”。
    pub fn recover(day_open: Snapshot, journal_path: &Path) -> anyhow::Result<Recovery> {
        let log = Journal::read(journal_path)?;
        if let Some(first) = log.records.first()
            && first.seq != 1
        {
            return Err(anyhow::anyhow!(
                "日初快照重放必须从 seq=1 开始，实得 {} —— 日志不完整或起点镜像选错",
                first.seq
            ));
        }
        // 恢复后继续接单：同一文件 append，序号接在已落盘那条后面。
        let mut engine = Engine::new(day_open, Journal::open(journal_path)?);
        engine.term = log.term;
        for record in &log.records {
            engine.replay_record(record).map_err(|err| {
                anyhow::anyhow!(
                    "回放 seq={} 的 {} 记录失败: {err}",
                    record.seq,
                    record.entry.table_id()
                )
            })?;
        }
        let last = log.last_seq();
        engine.seq = last;
        engine.durable = last;
        // 回放只重建状态，不对外发布：出口只给「这个进程里真正发生过的写」用。
        // 刚恢复的内核 ring 为空，新接上的消费者只能拿快照重建（catch_up 会判 Lagged），
        // 而不是拿到一份跨进程混放的「历史」。
        engine.pending.clear();
        Ok(Recovery {
            engine,
            replayed: log.records.len(),
            dropped_tail: log.dropped_tail,
        })
    }

    /// 重放一条记录：只跑落账效应，不校验、不写盘（那些在日志产生之前就已发生过）。
    ///
    /// 同一 `order_id` 会出现多条记录，语义由行状态区分：
    /// - 表里还没有这个单 → 本条是**下单流水**，重新冻结/锁定并入表；
    /// - 表里已有 → 不再重复冻结；只有「活单 → Cancelled」这一跳才施加解冻效应；
    ///   成交引起的资金/持仓变动由同组的 trade 记录负责，本条只搬行。
    ///
    /// trade 记录在日志里排在其 order 记录之前（见 `apply_fill` 的写入顺序），
    /// 所以回放成交时表里还是**成交前**那行，委托价/可卖量与当时完全一致。
    fn replay_record(&mut self, record: &Record) -> KernelResult<()> {
        match &record.entry {
            Entry::Orders(order) => {
                let stored = self.snapshot.orders.get(order.order_id.as_str()).cloned();
                match stored {
                    None => self.apply_place_effect(order),
                    Some(previous) => {
                        if matches!(
                            previous.status,
                            OrderStatus::New | OrderStatus::PartiallyFilled
                        ) && order.status == OrderStatus::Cancelled
                        {
                            self.apply_cancel_effect(order)?;
                        }
                        let order_id = order.order_id.clone();
                        self.snapshot
                            .orders
                            .replace(&order_id, order.clone())
                            .map_err(KernelError::State)
                    }
                }
            }
            Entry::Trades(trade) => {
                let order = self
                    .snapshot
                    .orders
                    .get(trade.order_id.as_str())
                    .cloned()
                    .ok_or_else(|| {
                        KernelError::State(anyhow::anyhow!(
                            "回放成交 {} 时找不到订单 {} —— 日志与起点镜像对不上",
                            trade.trade_id,
                            trade.order_id
                        ))
                    })?;
                self.settle_fill(&order, trade)?;
                self.snapshot
                    .trades
                    .upsert(trade.clone())
                    .map_err(KernelError::State)
            }
        }
    }
}

/// 出口取行：把表身份（`Spec::id`）翻译成具体行的 JSON。新可变表进内核时必须在这里加一臂；
/// 漏加的后果是发布时 `panic`（登记了却取不到行），而不是静默丢一个变更。
/// `row_json_dispatches_every_mutable_table` 测试钉住四张表都能取到。
fn row_json(snapshot: &Snapshot, table: &str, key: &str) -> Option<serde_json::Value> {
    match table {
        Asset::ID => snapshot.assets.get(key).map(encode_row),
        Position::ID => snapshot.positions.get(key).map(encode_row),
        Order::ID => snapshot.orders.get(key).map(encode_row),
        Trade::ID => snapshot.trades.get(key).map(encode_row),
        other => panic!("表 {other:?} 未接入订阅出口（row_json 缺分派臂）"),
    }
}

/// 行→JSON 只走 serde 一条路：与 journal 落盘、快照文件同一份序列化实现，
/// 订阅端拿到的行与日志里的行不可能不一致。
fn encode_row<T: Serialize>(row: &T) -> serde_json::Value {
    serde_json::to_value(row).expect("行应可序列化")
}

/// 扣减专用：`Amount::checked_sub` 只挡算术溢出，减成负数是 `Some(负值)` —— 金额层允许负
/// （盈亏需要），而余额/持仓的语义不允许。拿它当「不够减」的判断会漏判，所有扣减统一走这里。
#[must_use]
fn sub_or_negative<const S: u32>(minuend: Amount<S>, subtrahend: Amount<S>) -> Option<Amount<S>> {
    minuend
        .checked_sub(subtrahend)
        .filter(|value| !value.is_negative())
}

/// 逐账户市值：`Σ quantity × avg_cost`，与 `Snapshot::check_valuation` 完全同一条算式。
fn market_value_of(snapshot: &Snapshot, account_id: &str) -> anyhow::Result<Money> {
    let mut total = Money::ZERO;
    for position in snapshot.positions.rows().values() {
        if position.account_id != account_id {
            continue;
        }
        let value: Money = notional(
            position.avg_cost,
            position.quantity,
            Rounding::MidpointAwayFromZero,
        )
        .ok_or_else(|| anyhow::anyhow!("持仓市值计算溢出"))?;
        total = total
            .checked_add(value)
            .ok_or_else(|| anyhow::anyhow!("账户 {account_id} 市值累加溢出"))?;
    }
    Ok(total)
}

/// 加权平均成本：`旧总成本(6 位) + 本次成本(6 位) / 新总股数`，只在最后一步舍入到 [`Price`]。
///
/// 中间量走 `MicroAmount`（6 位）再除数量，避免「先落 Money(2) 再换 Price(4)」的双舍入
/// —— 与 `amount.rs` 的 `convert` 一次完成原则一致。
fn weighted_avg_price(
    old_qty: Quantity,
    old_avg: Price,
    add_cost: MicroAmount,
    add_qty: Quantity,
) -> KernelResult<Price> {
    let old_cost: MicroAmount = if old_qty.is_zero() {
        MicroAmount::ZERO
    } else {
        notional(old_avg, old_qty, Rounding::MidpointAwayFromZero)
            .ok_or_else(|| anyhow::anyhow!("旧持仓成本（中间量）溢出"))?
    };
    let total_cost = old_cost
        .checked_add(add_cost)
        .ok_or_else(|| anyhow::anyhow!("总成本（中间量）溢出"))?;
    let total_qty = old_qty
        .checked_add(add_qty)
        .ok_or_else(|| anyhow::anyhow!("总数量溢出"))?;
    if total_qty.is_zero() {
        return Err(KernelError::State(anyhow::anyhow!("加权平均成本的分母为 0")));
    }
    let denominator = Decimal::from(total_qty.units());
    let per_share = total_cost
        .to_decimal()
        .ok_or_else(|| anyhow::anyhow!("总成本转 Decimal 失败"))?
        / denominator;
    Price::from_decimal_with(per_share, Rounding::MidpointAwayFromZero)
        .ok_or_else(|| anyhow::anyhow!("平均成本换算为 Price 失败"))
        .map_err(KernelError::State)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn snapshot() -> Snapshot {
        Snapshot::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("data")).unwrap()
    }

    /// 每个用例一份独立 journal（临时目录 + 用例名 + 进程号），并行跑也不串写。
    fn journal_path(tag: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("graydb-engine-{tag}-{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&path);
        path
    }

    fn engine(tag: &str) -> Engine {
        Engine::new(
            snapshot(),
            Journal::open(&journal_path(tag)).expect("测试 journal 打开失败"),
        )
    }

    fn req<'a>(
        order_id: &'a str,
        account_id: &'a str,
        symbol: &'a str,
        side: Side,
        price_units: i64,
        qty_units: i64,
    ) -> PlaceRequest<'a> {
        PlaceRequest {
            order_id,
            account_id,
            symbol,
            side,
            price: Price::from_units(price_units),
            quantity: Quantity::from_units(qty_units),
            created_at: "2026-09-29T09:30:00Z",
        }
    }

    fn cash(engine: &Engine, account_id: &str) -> (Money, Money) {
        let asset = engine.snapshot.asset(account_id).expect("应有资产行");
        (asset.available, asset.frozen)
    }

    fn spend(price_units: i64, qty_units: i64) -> Money {
        notional(
            Price::from_units(price_units),
            Quantity::from_units(qty_units),
            Rounding::MidpointAwayFromZero,
        )
        .expect("演示规模不会溢出")
    }

    /// 买入全成的守恒链：现金只减实付额、冻结不残留、持仓加 quantity 不加可卖、
    /// 四表共用同一 seq、外键与市值不变量均过。
    #[test]
    fn buy_fill_conserves_cash_position_and_seq() {
        let mut eng = engine("buy-fill");
        let (available_before, frozen_before) = cash(&eng, "A001");
        let total_before = available_before.checked_add(frozen_before).unwrap();

        let ack = eng
            .place_and_fill(req("O1", "A001", "09018", Side::Buy, 90_025, 300))
            .unwrap();
        assert_eq!(ack, Ack::Accepted { seq: 2 });
        assert_eq!(eng.seq(), eng.durable(), "已分配与已落盘必须齐平（1.3）");

        let spent = spend(90_025, 300);
        let (available, frozen) = cash(&eng, "A001");
        assert_eq!(
            available.checked_add(frozen).unwrap(),
            total_before.checked_sub(spent).unwrap(),
            "available + frozen 只应减掉实付额"
        );
        assert!(frozen.is_zero(), "全成后不得残留冻结");

        let position = eng.snapshot.position("A001", "09018").unwrap();
        assert_eq!(position.quantity, Quantity::from_units(1_100));
        assert_eq!(
            position.available_qty,
            Quantity::from_units(800),
            "T+1：当日买入的 300 股不可卖（1.6）"
        );
        assert_eq!(position.avg_cost, Price::from_units(90_025), "同价加仓均价不变");

        let order = eng.snapshot.orders.get("O1").unwrap();
        assert_eq!(order.status, OrderStatus::Filled);
        assert_eq!(order.filled_qty, order.quantity);
        assert_eq!(order.seq, 2);
        let trade = eng.snapshot.trades.get("O1-1").unwrap();
        assert_eq!(trade.seq, order.seq, "一笔成交牵动的多表变更共用同一 seq（1.7）");
        assert_eq!(trade.amount, spent);

        eng.snapshot.check_integrity().expect("orders/trades 外键应成立");
        eng.snapshot.check_valuation().expect("市值应由持仓全量重算后平");
    }

    /// 拒单零状态变化：所有校验失败都发生在 `alloc_seq` 与写盘之前，
    /// 不消耗序号、不落 journal、不改内存。
    #[test]
    fn rejection_leaves_zero_state_delta() {
        let mut eng = engine("reject");
        let before = cash(&eng, "A001");
        let seq_before = eng.seq();

        let cases = [
            (
                req("O-R1", "A001", "09018", Side::Buy, 9_990_000, 1_000),
                RejectReason::InsufficientFunds,
            ),
            (
                req("O-R2", "A404", "09018", Side::Buy, 90_025, 100),
                RejectReason::UnknownAccount,
            ),
            (
                req("O-R3", "A003", "09018", Side::Buy, 90_025, 100),
                RejectReason::AccountFrozen,
            ),
            (
                req("O-R4", "A001", "999999", Side::Buy, 90_025, 100),
                RejectReason::UnknownSymbol,
            ),
            (
                req("O-R5", "A001", "09018", Side::Buy, 90_025, 150),
                RejectReason::BelowLotSize,
            ),
            (
                req("O-R6", "A001", "09018", Side::Buy, 0, 100),
                RejectReason::InvalidPrice,
            ),
            (
                req("O-R7", "A001", "09018", Side::Sell, 90_025, 900),
                RejectReason::InsufficientPosition,
            ),
        ];
        for (request, expected) in cases {
            assert_eq!(
                eng.place(request),
                Err(KernelError::Reject(expected)),
                "订单 {} 应以 {expected} 被拒",
                request.order_id
            );
        }

        assert_eq!(cash(&eng, "A001"), before, "拒单不得动一分资金");
        assert_eq!(eng.seq(), seq_before, "拒单不得消耗序号");
        assert_eq!(eng.durable(), seq_before, "拒单不得写盘");
        assert!(eng.snapshot.orders.is_empty() && eng.snapshot.trades.is_empty());
        assert_eq!(
            eng.snapshot.position("A001", "09018").unwrap().available_qty,
            Quantity::from_units(800),
            "被拒的卖单不得锁定可卖数量"
        );
    }

    /// 下单幂等：同 order_id 同内容重放 → `Duplicate` 且零副作用；同键不同内容 → 拒。
    #[test]
    fn duplicate_order_id_is_idempotent_not_double_frozen() {
        let mut eng = engine("idempotent");
        let request = req("O2", "A001", "600000", Side::Buy, 60_030, 200);

        let first = eng.place(request).unwrap();
        assert_eq!(first, Ack::Accepted { seq: 1 });
        let frozen_once = cash(&eng, "A001").1;
        assert_eq!(frozen_once, spend(60_030, 200));

        assert_eq!(eng.place(request).unwrap(), Ack::Duplicate { seq: 1 });
        assert_eq!(cash(&eng, "A001").1, frozen_once, "重放不得重复冻结");
        assert_eq!(eng.seq(), 1, "重放不消耗序号");
        assert_eq!(eng.snapshot.orders.len(), 1);

        // 同键不同内容是真冲突，不是重试
        assert_eq!(
            eng.place(req("O2", "A001", "600000", Side::Buy, 60_031, 200)),
            Err(KernelError::Reject(RejectReason::DuplicateOrder))
        );
    }

    /// 成交幂等：同 trade_id 同内容 → `Duplicate`；同 id 不同内容 → `DuplicateFill`。
    #[test]
    fn duplicate_trade_id_is_idempotent_not_double_settled() {
        let mut eng = engine("fill-idempotent");
        eng.place(req("O9", "A001", "09018", Side::Buy, 90_100, 400))
            .unwrap();
        let seq = eng
            .apply_fill("T1", "O9", Quantity::from_units(100), Price::from_units(90_025))
            .unwrap()
            .seq();
        let state = (cash(&eng, "A001"), eng.seq());

        assert_eq!(
            eng.apply_fill("T1", "O9", Quantity::from_units(100), Price::from_units(90_025))
                .unwrap(),
            Ack::Duplicate { seq },
            "同一成交号重放应回到原序号"
        );
        assert_eq!(cash(&eng, "A001"), state.0, "重放不得重复扣款");
        assert_eq!(eng.seq(), state.1, "重放不消耗序号");
        assert_eq!(eng.snapshot.trades.len(), 1);
        assert_eq!(
            eng.snapshot.orders.get("O9").unwrap().filled_qty,
            Quantity::from_units(100)
        );

        assert_eq!(
            eng.apply_fill("T1", "O9", Quantity::from_units(200), Price::from_units(90_025)),
            Err(KernelError::Reject(RejectReason::DuplicateFill)),
            "同一成交号不能代指两笔不同的成交"
        );
    }

    /// 部分成交 + 撤单：逐笔回补限价差额，撤单只释放未成交部分的冻结，
    /// 全部加起来恰好等于实付额。
    #[test]
    fn partial_fill_then_cancel_settles_exactly() {
        let mut eng = engine("partial");
        let (available_before, _) = cash(&eng, "A001");

        // 委托 400 股 @9.0100（限价），先成 100 股 @9.0025（成交价优于限价）
        eng.place(req("O3", "A001", "09018", Side::Buy, 90_100, 400))
            .unwrap();
        eng.apply_fill("T1", "O3", Quantity::from_units(100), Price::from_units(90_025))
            .unwrap();

        let (available, frozen) = cash(&eng, "A001");
        assert_eq!(frozen, spend(90_100, 300), "残余 300 股仍按限价冻结");
        assert_eq!(
            available.checked_add(frozen).unwrap(),
            available_before.checked_sub(spend(90_025, 100)).unwrap(),
            "总额只减实付，限价差额 0.75 已即时回到可用"
        );
        let order = eng.snapshot.orders.get("O3").unwrap();
        assert_eq!(order.status, OrderStatus::PartiallyFilled);
        assert_eq!(order.filled_qty, Quantity::from_units(100));

        // 撤单：未成交的 300 股按限价算的冻结全额回到可用
        let ack = eng.cancel("O3").unwrap();
        assert_eq!(ack.seq(), 3);
        let (available, frozen) = cash(&eng, "A001");
        assert!(frozen.is_zero(), "撤单后不得残留冻结");
        assert_eq!(
            available,
            available_before.checked_sub(spend(90_025, 100)).unwrap(),
            "可用 = 原额 - 实付"
        );
        assert_eq!(eng.snapshot.orders.get("O3").unwrap().status, OrderStatus::Cancelled);
        assert_eq!(
            eng.snapshot.position("A001", "09018").unwrap().quantity,
            Quantity::from_units(900)
        );
        assert_eq!(
            eng.cancel("O3"),
            Err(KernelError::Reject(RejectReason::OrderNotLive)),
            "已终态的订单不可再撤"
        );
        eng.snapshot.check_valuation().unwrap();
    }

    /// T+1：当日买入不可卖，日初可卖为 0 的仓位也不得透支。
    #[test]
    fn t_plus_one_blocks_same_day_buy_from_selling() {
        let mut eng = engine("tplus1");
        eng.place_and_fill(req("O4", "A001", "09018", Side::Buy, 90_025, 100))
            .unwrap();
        assert_eq!(
            eng.place(req("O5", "A001", "09018", Side::Sell, 90_025, 900)),
            Err(KernelError::Reject(RejectReason::InsufficientPosition)),
            "原有 800 可卖 + 当日买入 100，下 900 股的卖单必须被拒"
        );
        assert_eq!(
            eng.place(req("O6", "A002", "600000", Side::Sell, 60_030, 100)),
            Err(KernelError::Reject(RejectReason::InsufficientPosition)),
            "A002 持有 2000 股但可卖为 0（未解禁）"
        );
        assert_eq!(
            eng.snapshot.position("A001", "09018").unwrap().available_qty,
            Quantity::from_units(800),
            "被拒的卖单不得预先锁定可卖数量"
        );
    }

    /// 卖出全链路：下单锁可卖不减总量 → 成交减总量并入账 → 撤单释放锁定。
    #[test]
    fn sell_fill_settles_lock_and_credit() {
        let mut eng = engine("sell");
        let (available_before, _) = cash(&eng, "A001");

        eng.place(req("O7", "A001", "09018", Side::Sell, 90_000, 300))
            .unwrap();
        let locked = eng.snapshot.position("A001", "09018").unwrap().clone();
        assert_eq!(locked.available_qty, Quantity::from_units(500), "下单即锁 300 股可卖");
        assert_eq!(locked.quantity, Quantity::from_units(800), "锁定不减总量");

        eng.apply_fill("T-S1", "O7", Quantity::from_units(300), Price::from_units(90_025))
            .unwrap();
        let position = eng.snapshot.position("A001", "09018").unwrap();
        assert_eq!(position.quantity, Quantity::from_units(500));
        assert_eq!(
            position.available_qty, position.quantity,
            "被成交消耗的锁定已结转，剩余额度可卖"
        );
        assert_eq!(position.avg_cost, Price::from_units(90_025), "卖出不改平均成本");
        let (available, frozen) = cash(&eng, "A001");
        assert_eq!(
            available,
            available_before.checked_add(spend(90_025, 300)).unwrap(),
            "卖出全款入可用"
        );
        assert!(frozen.is_zero());

        // 再下一笔卖 200 井撤销：锁定必须全额回补
        eng.place(req("O8", "A001", "09018", Side::Sell, 90_000, 200))
            .unwrap();
        assert_eq!(
            eng.snapshot.position("A001", "09018").unwrap().available_qty,
            Quantity::from_units(300)
        );
        eng.cancel("O8").unwrap();
        let position = eng.snapshot.position("A001", "09018").unwrap();
        assert_eq!(
            position.available_qty,
            Quantity::from_units(500),
            "撤单释放未成交部分的可卖锁定"
        );
        assert_eq!(position.quantity, Quantity::from_units(500), "撤单不动总量");
        assert_eq!(cash(&eng, "A001").0, available);
        eng.snapshot.check_integrity().unwrap();
        eng.snapshot.check_valuation().unwrap();
    }

    /// 成交回报的入参校验：劣价 / 超量 / 未知订单 / 终态单均被拒且零副作用。
    #[test]
    fn fill_validation_rejects_bad_price_and_overfill() {
        let mut eng = engine("fill-validation");
        eng.place(req("O10", "A001", "09018", Side::Buy, 90_000, 200))
            .unwrap();
        let state = (cash(&eng, "A001"), eng.seq(), eng.snapshot.trades.len());

        let bad = [
            (
                "T-B1",
                Quantity::from_units(100),
                Price::from_units(90_100),
                RejectReason::InvalidFillPrice,
            ),
            (
                "T-B2",
                Quantity::from_units(300),
                Price::from_units(90_000),
                RejectReason::OverFill,
            ),
            (
                "T-B3",
                Quantity::from_units(0),
                Price::from_units(90_000),
                RejectReason::InvalidPrice,
            ),
        ];
        for (trade_id, qty, price, expected) in bad {
            assert_eq!(
                eng.apply_fill(trade_id, "O10", qty, price),
                Err(KernelError::Reject(expected)),
                "成交 {trade_id} 应以 {expected} 被拒"
            );
        }
        assert_eq!(
            eng.apply_fill("T-B4", "O-404", Quantity::from_units(100), Price::from_units(90_000)),
            Err(KernelError::Reject(RejectReason::UnknownOrder))
        );
        assert_eq!(cash(&eng, "A001"), state.0, "被拒的成交不得动资金");
        assert_eq!(eng.seq(), state.1, "被拒的成交不得消耗序号（未写盘）");
        assert_eq!(eng.snapshot.trades.len(), state.2);

        // 正常成交后进入终态，再回报即 `OrderNotLive`
        eng.apply_fill("T-B5", "O10", Quantity::from_units(200), Price::from_units(90_000))
            .unwrap();
        assert_eq!(
            eng.apply_fill("T-B6", "O10", Quantity::from_units(100), Price::from_units(90_000)),
            Err(KernelError::Reject(RejectReason::OrderNotLive))
        );
        assert_eq!(eng.snapshot.orders.get("O10").unwrap().status, OrderStatus::Filled);
    }

    /// 卖单逐笔成交：部分成交后剩余锁定仍在，撤单只回补未成交部分。
    #[test]
    fn partial_sell_fill_keeps_remaining_lock() {
        let mut eng = engine("partial-sell");
        // 可卖 800，挂卖 500 → 锁定后剩 300 可卖；成 200 后总量 600、锁定仍在 300
        eng.place(req("O11", "A001", "09018", Side::Sell, 90_000, 500))
            .unwrap();
        eng.apply_fill("T-P1", "O11", Quantity::from_units(200), Price::from_units(90_000))
            .unwrap();
        let position = eng.snapshot.position("A001", "09018").unwrap().clone();
        assert_eq!(position.quantity, Quantity::from_units(600));
        assert_eq!(
            position.available_qty,
            Quantity::from_units(300),
            "未成交的 300 股仍处于锁定，不可被另下一笔卖单透支"
        );
        assert_eq!(
            eng.place(req("O12", "A001", "09018", Side::Sell, 90_000, 400)),
            Err(KernelError::Reject(RejectReason::InsufficientPosition))
        );

        eng.cancel("O11").unwrap();
        let position = eng.snapshot.position("A001", "09018").unwrap();
        assert_eq!(position.quantity, Quantity::from_units(600));
        assert_eq!(
            position.available_qty, position.quantity,
            "撤单释放未成交的 300 股锁定"
        );
        // 卖出不改均价，市值按剩余持仓全量重算
        assert_eq!(position.avg_cost, Price::from_units(90_025));
        eng.snapshot.check_valuation().unwrap();
    }

    /// 序号单调且无空洞：place / fill 各消耗一个，内存与落盘始终齐平。
    #[test]
    fn seq_is_monotonic_and_gapless() {
        let mut eng = engine("seq");
        let mut expected = 0;
        for step in 0..5u32 {
            let order_id = format!("SQ-{step}");
            let trade_id = format!("SQ-T{step}");
            expected += 1;
            assert_eq!(
                eng.place(req(&order_id, "A001", "09018", Side::Buy, 90_025, 100))
                    .unwrap()
                    .seq(),
                expected
            );
            expected += 1;
            assert_eq!(
                eng.apply_fill(&trade_id, &order_id, Quantity::from_units(100), Price::from_units(90_025))
                    .unwrap()
                    .seq(),
                expected
            );
            assert_eq!(eng.seq(), expected);
            assert_eq!(eng.seq(), eng.durable(), "每步都无空洞");
        }
        // 成交后订单已终态：被拒的撤单不得消耗序号
        assert_eq!(
            eng.cancel("SQ-0"),
            Err(KernelError::Reject(RejectReason::OrderNotLive))
        );
        assert_eq!(eng.seq(), expected);
        // 五笔买入累加：数量 800+500，均价逐笔单舍入，市值与持仓全量对齐
        let position = eng.snapshot.position("A001", "09018").unwrap();
        assert_eq!(position.quantity, Quantity::from_units(1_300));
        assert_eq!(position.available_qty, Quantity::from_units(800));
        eng.snapshot.check_valuation().unwrap();
    }

    /// 单舍入铁律：中间量留在 `MicroAmount`（6 位），只在最后一步落 [`Price`]。
    /// 若中途先把总成本压成 `Money`（2 位），这组的平均成本会被算成 0。
    #[test]
    fn weighted_avg_price_rounds_once_at_the_end() {
        let cost: MicroAmount = notional(
            Price::from_units(2),
            Quantity::from_units(2),
            Rounding::MidpointAwayFromZero,
        )
        .unwrap();
        let avg = weighted_avg_price(
            Quantity::from_units(1),
            Price::from_units(1),
            cost,
            Quantity::from_units(2),
        )
        .unwrap();
        assert_eq!(avg, Price::from_units(2), "(0.0001 + 0.0004) / 3 = 0.000166… → 0.0002");
    }

    /// 整表逐字段比对：`Amount` 序列化为 `{"units":N}`（纯整数、绝不经浮点），
    /// 所以「两个 Value 相等」就是每一行每个金额/数量字段的整数完全一致 ——
    /// 阶段 2.3 要的就是这个「逐分不差」，不是看起来差不多。
    fn state_fingerprint(engine: &Engine) -> serde_json::Value {
        serde_json::to_value((
            engine.snapshot.assets.rows(),
            engine.snapshot.positions.rows(),
            engine.snapshot.orders.rows(),
            engine.snapshot.trades.rows(),
        ))
        .expect("状态应可序列化")
    }

    /// journal 侧的取证（1.5 + 1.7 + 2.1）：写盘顺序与内存一致，一笔成交的
    /// trade + 更新后 order 落在同一个 `(term, seq)` 信封下，表身份取自注册中心。
    #[test]
    fn journal_records_the_whole_group_under_one_seq() {
        let path = journal_path("journal");
        let mut eng = Engine::new(snapshot(), Journal::open(&path).expect("测试 journal 打开失败"));

        eng.place(req("OJ", "A001", "09018", Side::Buy, 90_025, 200))
            .unwrap();
        eng.apply_fill("TJ", "OJ", Quantity::from_units(100), Price::from_units(90_025))
            .unwrap();

        let log = Journal::read(&path).expect("journal 应可读且序号稠密");
        assert_eq!(log.dropped_tail, None, "正常收尾不该有被丢弃的尾");
        assert_eq!(log.term, 0);
        assert_eq!(log.records.len(), 3, "下单 1 条（orders）+ 成交 2 条（trades → orders）");
        let seqs: Vec<i64> = log.records.iter().map(|record| record.seq).collect();
        assert_eq!(seqs, vec![1, 2, 2], "同组记录共用 seq=2（1.7）");
        let tables: Vec<&str> = log
            .records
            .iter()
            .map(|record| record.entry.table_id())
            .collect();
        assert_eq!(tables, vec![Order::ID, Trade::ID, Order::ID]);
        assert_eq!(log.records[1].entry.row_key(), "TJ", "第二条是成交");
        assert_eq!(log.records[2].entry.row_key(), "OJ", "第三条是更新后的订单");
        assert!(
            log.records
                .iter()
                .all(|record| record.entry.row_seq() == record.seq),
            "行内序号必须与信封一致"
        );
        match &log.records[2].entry {
            Entry::Orders(order) => {
                assert_eq!(
                    order.status,
                    OrderStatus::PartiallyFilled,
                    "订单行已反映部分成交状态"
                );
                assert_eq!(order.filled_qty, Quantity::from_units(100));
            }
            other => panic!("第三条应是订单，实得 {other:?}"),
        }
        assert_eq!(eng.seq(), eng.durable());
    }

    /// 2.3 守护测试：`replay(journal) == 内存终态`，逐分不差。
    ///
    /// 脚本特意凑齐四类效应 —— 买入部分成交（限价差额逐笔回补）、买入全成、
    /// 卖出锁量后成交、撤单解冻 —— 再加两次幂等重放（不该产生新记录）。
    /// 恢复端从**日初**镜像起，靠与 live 同一条 `apply_place_effect` / `settle_fill` /
    /// `apply_cancel_effect` 把每个数字重算一遍；能算错就说明日志与内存路径分叉了。
    #[test]
    fn replay_reconstructs_the_exact_final_state() {
        let path = journal_path("replay");
        let mut live = Engine::new(snapshot(), Journal::open(&path).expect("打开 journal 失败"));

        // 买入部分成交：残余 300 股一直冻着，恢复必须重建这笔冻结
        live.place(req("R1", "A001", "09018", Side::Buy, 90_100, 400))
            .unwrap();
        live.apply_fill("RT1", "R1", Quantity::from_units(100), Price::from_units(90_025))
            .unwrap();
        // 买入全成（新建仓位）
        live.place(req("R2", "A001", "600000", Side::Buy, 60_030, 200))
            .unwrap();
        live.apply_fill("RT2", "R2", Quantity::from_units(200), Price::from_units(60_030))
            .unwrap();
        // 卖出：下单锁可卖 → 成交减总量并入账
        live.place(req("R3", "A001", "09018", Side::Sell, 90_000, 300))
            .unwrap();
        live.apply_fill("RT3", "R3", Quantity::from_units(300), Price::from_units(90_025))
            .unwrap();
        // 下单后撤单：冻结原地回补，内存净变化为零
        live.place(req("R4", "A001", "600000", Side::Buy, 60_030, 100))
            .unwrap();
        live.cancel("R4").unwrap();
        // 幂等重放：两条都不该落新记录
        live.place(req("R1", "A001", "09018", Side::Buy, 90_100, 400))
            .unwrap();
        live.apply_fill("RT1", "R1", Quantity::from_units(100), Price::from_units(90_025))
            .unwrap();

        let mut recovered = Engine::recover(snapshot(), &path).expect("回放应能重建终态");
        assert_eq!(recovered.dropped_tail, None, "日志完整，不该丢尾");
        // 8 个事务组 = 11 条记录（三个成交组各 2 条，其余各 1 条）；两次幂等重放不入库
        assert_eq!(recovered.replayed, 11);
        assert_eq!(recovered.engine.seq(), live.seq(), "序号接到崩溃前的位置");
        assert_eq!(
            recovered.engine.durable(),
            recovered.engine.seq(),
            "恢复后无空洞，可继续接单"
        );
        assert_eq!(recovered.engine.term(), live.term());
        assert_eq!(
            state_fingerprint(&recovered.engine),
            state_fingerprint(&live),
            "回放结果必须与崩溃前的内存逐分不差"
        );

        // 恢复出的内核能接着写：序号衔接、下一个组照常落盘
        let ack = recovered
            .engine
            .place(req("R5", "A001", "600000", Side::Buy, 60_030, 100))
            .expect("恢复后应可继续接单");
        assert_eq!(ack.seq(), live.seq() + 1);
        recovered.engine.snapshot.check_valuation().unwrap();
    }

    /// 2.4 崩溃截断：末条只有半行（写入中途被 kill）→ 丢弃并停止。
    /// 恢复出来的是「截至上一条完整记录」的状态，且 `dropped_tail` 必须报给上层 ——
    /// 少了一笔是可见事实，不能被当成正常启动。
    #[test]
    fn replay_drops_partial_tail_and_stops_at_the_last_complete_record() {
        use std::io::Write as _;

        let path = journal_path("truncated");
        {
            let mut eng = Engine::new(snapshot(), Journal::open(&path).unwrap());
            eng.place(req("C1", "A001", "09018", Side::Buy, 90_025, 100))
                .unwrap();
            eng.apply_fill("CT1", "C1", Quantity::from_units(100), Price::from_units(90_025))
                .unwrap();
            eng.place(req("C2", "A001", "09018", Side::Buy, 90_025, 100))
                .unwrap();
        } // 作用域结束才释放文件句柄（Windows 上否则下一次 append 会被锁）

        // 模拟崩溃：第 4 条只写了半行就断
        std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .expect("追加半行失败")
            .write_all(br#"{"term":0,"seq":4,"entry":{"trades":{"trade_id":"CT2""#)
            .expect("写入半行失败");

        let recovered = Engine::recover(snapshot(), &path)
            .expect("末条截断只应丢弃该条，不该拒绝整个恢复");
        assert!(
            recovered.dropped_tail.is_some(),
            "被丢弃的尾必须上报，上层据此停服/对从库"
        );
        assert_eq!(recovered.replayed, 4, "采纳 seq=1,2,2,3 四条完整记录");
        assert_eq!(recovered.engine.seq(), 3, "序号停在最后一条完整记录");
        assert_eq!(recovered.engine.seq(), recovered.engine.durable());
        // C2 只到「下单冻结」，那笔成交随尾一起丢了：成交量为 0、冻结仍在
        let order = recovered.engine.snapshot.orders.get("C2").unwrap();
        assert_eq!(order.filled_qty, Quantity::from_units(0));
        assert_eq!(order.status, OrderStatus::New);
        assert_eq!(recovered.engine.snapshot.trades.len(), 1);
        assert_eq!(
            cash(&recovered.engine, "A001").1,
            spend(90_025, 100),
            "C2 的冻结照旧重建"
        );
        recovered.engine.snapshot.check_valuation().unwrap();
    }

    /// 中间行坏 / seq 有洞 / 起点不是 seq=1 —— 三种「日志与内存对不上」都得拒绝恢复，
    /// 而不是跳过一条继续算（跳过的代价是账面静默不平）。
    #[test]
    fn replay_refuses_interior_corruption_and_seq_gap() {
        let path = journal_path("corrupt");
        {
            let mut eng = Engine::new(snapshot(), Journal::open(&path).unwrap());
            eng.place(req("X1", "A001", "09018", Side::Buy, 90_025, 100))
                .unwrap();
            eng.place(req("X2", "A001", "09018", Side::Buy, 90_025, 100))
                .unwrap();
            eng.place(req("X3", "A001", "09018", Side::Buy, 90_025, 100))
                .unwrap();
        }
        let lines: Vec<String> = std::fs::read_to_string(&path)
            .expect("读 journal 失败")
            .lines()
            .map(str::to_string)
            .collect();
        assert_eq!(lines.len(), 3);
        let rewrite = |tag: &str, kept: &[&str]| -> std::path::PathBuf {
            let target = journal_path(tag);
            std::fs::write(&target, kept.join("\n") + "\n").expect("改写测试 journal 失败");
            target
        };

        // ① 中间行不是合法 JSON → 不能当截断处理
        let corrupt = rewrite("corrupt-line", &[&lines[0], "### 中间行被插坏了 ###", &lines[2]]);
        let err = Engine::recover(snapshot(), &corrupt)
            .err()
            .expect("内部行损坏必须拒绝恢复");
        assert!(err.to_string().contains("损坏"), "报错要指认问题: {err}");

        // ② 中间行整条丢失 = seq 不稠密
        let gap = rewrite("corrupt-gap", &[&lines[0], &lines[2]]);
        let err = Engine::recover(snapshot(), &gap)
            .err()
            .expect("seq 空洞必须拒绝恢复");
        assert!(err.to_string().contains("稠密"), "报错要指认问题: {err}");

        // ③ 开头被截掉（首条不是 seq=1）：配日初快照重放会少算
        let head = rewrite("corrupt-head", &[&lines[1], &lines[2]]);
        let err = Engine::recover(snapshot(), &head)
            .err()
            .expect("起点不是 seq=1 必须拒绝");
        assert!(err.to_string().contains("seq=1"), "报错要指认问题: {err}");
    }

    // ── 阶段 3.1 / 3.3：订阅出口与按游标补发 ──────────────────────

    /// 内核会改的四张表。出口发布、差异算式、取行分派都以这份清单为准。
    const MUTABLE_TABLES: [&str; 4] = [Asset::ID, Position::ID, Order::ID, Trade::ID];

    /// 某张表当前的全部行 `(键, JSON)`，按键排序。取行故意在测试里另写一套，
    /// 不与生产的 `row_json` 共用代码 —— 否则漏一张表永远测不出来。
    fn rows_of(table: &str, snapshot: &Snapshot) -> Vec<(String, serde_json::Value)> {
        let mut rows: Vec<(String, serde_json::Value)> = match table {
            Asset::ID => snapshot
                .assets
                .rows()
                .iter()
                .map(|(key, row)| (key.clone(), serde_json::to_value(row).unwrap()))
                .collect(),
            Position::ID => snapshot
                .positions
                .rows()
                .iter()
                .map(|(key, row)| (key.clone(), serde_json::to_value(row).unwrap()))
                .collect(),
            Order::ID => snapshot
                .orders
                .rows()
                .iter()
                .map(|(key, row)| (key.clone(), serde_json::to_value(row).unwrap()))
                .collect(),
            Trade::ID => snapshot
                .trades
                .rows()
                .iter()
                .map(|(key, row)| (key.clone(), serde_json::to_value(row).unwrap()))
                .collect(),
            other => panic!("测试未覆盖表 {other}"),
        };
        rows.sort_by(|left, right| left.0.cmp(&right.0));
        rows
    }

    /// 两份镜像之间的**真实**行差异（含删除）：出口该发什么，由这里独立算一遍。
    fn changed_rows(before: &Snapshot, after: &Snapshot) -> Vec<(String, String, Op)> {
        let mut out = Vec::new();
        for table in MUTABLE_TABLES {
            let old = rows_of(table, before);
            let new = rows_of(table, after);
            for (key, row) in &new {
                let dirty = match old.iter().find(|(old_key, _)| old_key == key) {
                    None => true,
                    Some((_, old_row)) => old_row != row,
                };
                if dirty {
                    out.push((table.to_string(), key.clone(), Op::Upsert));
                }
            }
            for (key, _) in &old {
                if !new.iter().any(|(new_key, _)| new_key == key) {
                    out.push((table.to_string(), key.clone(), Op::Delete));
                }
            }
        }
        out.sort();
        out
    }

    /// 环里属于 `seq` 这一组的变更集合。
    fn published_group(engine: &Engine, seq: i64) -> Vec<(String, String, Op)> {
        let mut rows: Vec<(String, String, Op)> = engine
            .broadcast()
            .retained()
            .filter(|change| change.seq == seq)
            .map(|change| (change.table.to_string(), change.key.clone(), change.op))
            .collect();
        rows.sort();
        rows
    }

    /// 跑一步写路径，并把「这一步发布的组」与「前后镜像的真实差异」逐行对齐。
    /// 这是 3.1 的守护核心：漏登记任何一个变更点（包括 `delete` 与走 `rows_mut` 的原地改）都会红。
    ///
    /// 游标 `cursor` 之后只有两种合法表现：窗口还罩得住就是连续 Delta（每行与表里现读一致）；
    /// 罩不住只允许 `Lagged`，且 `oldest_seq` 必须已滑过 cursor 之后那一组 —— 绝不返回缺段的 Delta。
    fn step(engine: &mut Engine, cursor: i64, write: impl FnOnce(&mut Engine)) -> i64 {
        let before = engine.snapshot.clone();
        write(engine);
        let seq = engine.seq();
        assert_eq!(engine.durable(), seq, "写路径必须已落盘（1.3）");
        assert_eq!(
            published_group(engine, seq),
            changed_rows(&before, &engine.snapshot),
            "seq {seq} 发布的行集合应恰好等于内存真实差异"
        );
        let pulled = engine.catch_up(cursor);
        match pulled {
            CatchUp::Delta { from, through, ref changes } => {
                assert_eq!((from, through), (cursor + 1, seq));
                for change in changes {
                    let live = rows_of(change.table, &engine.snapshot)
                        .into_iter()
                        .find(|(key, _)| key == &change.key)
                        .map(|(_, row)| row);
                    match (change.op, &change.row, live) {
                        (Op::Upsert, Some(published), Some(row)) => {
                            assert_eq!(published, &row, "出口里的行应等于表里现读那一行")
                        }
                        (Op::Delete, None, None) => {}
                        (op, published, live) => panic!(
                            "出口记录与内存不符：op={op:?} 带行={:?} 表里有={:?}",
                            published.is_some(),
                            live.is_some()
                        ),
                    }
                }
            }
            CatchUp::Lagged { oldest_seq, .. } => assert!(
                oldest_seq.is_some_and(|oldest| oldest > cursor + 1),
                "只有整组被淘汰出窗口才允许降级：{}",
                engine.catch_up(cursor).summary()
            ),
        }
        seq
    }

    /// 卖（含清仓 `delete`）/ 买（含新建仓位）/ 部分成交 / 撤单 每一步，发布的行集合
    /// 都恰好等于内存差异；一笔成交把四张表的变更打在同一个 seq 上。
    #[test]
    fn published_groups_match_the_real_row_diff() {
        let mut eng = engine("pub-diff");
        let mut cursor = 0;

        // 卖单下单：只锁可卖量（position）+ 订单入表，资金行不该动。
        cursor = step(&mut eng, cursor, |e| {
            e.place(req("OS", "A001", "09018", Side::Sell, 90_025, 800))
                .unwrap();
        });
        // 全成：现金 + 市值 + 订单 + 成交，且持仓被卖空 → `delete`（最容易漏登记的一处）。
        cursor = step(&mut eng, cursor, |e| {
            e.apply_fill(
                "OS-1",
                "OS",
                Quantity::from_units(800),
                Price::from_units(90_025),
            )
            .unwrap();
        });
        assert_eq!(
            published_group(&eng, cursor),
            vec![
                ("account_asset".to_string(), "A001".to_string(), Op::Upsert),
                ("orders".to_string(), "OS".to_string(), Op::Upsert),
                (
                    "position".to_string(),
                    composite_key_str(&["A001", "09018"]),
                    Op::Delete
                ),
                ("trades".to_string(), "OS-1".to_string(), Op::Upsert),
            ],
            "成交应一次发出四张表的变更（1.7）"
        );
        // 买回：冻结资金 + 订单入表；此时仓位还不该动。
        cursor = step(&mut eng, cursor, |e| {
            e.place(req("OB", "A001", "09018", Side::Buy, 90_025, 300))
                .unwrap();
        });
        // 部分成交：新建仓位走 `upsert`（与上面的 `replace`/`delete` 分岔全部跑过）。
        cursor = step(&mut eng, cursor, |e| {
            e.apply_fill(
                "OB-1",
                "OB",
                Quantity::from_units(100),
                Price::from_units(90_025),
            )
            .unwrap();
        });
        assert_eq!(
            eng.snapshot.orders.get("OB").unwrap().status,
            OrderStatus::PartiallyFilled
        );
        cursor = step(&mut eng, cursor, |e| {
            e.apply_fill(
                "OB-2",
                "OB",
                Quantity::from_units(200),
                Price::from_units(90_025),
            )
            .unwrap();
        });
        // 撤单：解冻残额 + 订单置终态。
        cursor = step(&mut eng, cursor, |e| {
            e.place(req("OC", "A001", "09018", Side::Buy, 90_025, 100))
                .unwrap();
        });
        cursor = step(&mut eng, cursor, |e| {
            e.cancel("OC").unwrap();
        });

        // 幂等重放：不消耗序号、不改内存，因此也不该往出口塞一个空组。
        let before = eng.snapshot.clone();
        let (published_before, groups_before) = (eng.broadcast().published(), eng.broadcast().groups());
        let ack = eng
            .place(req("OB", "A001", "09018", Side::Buy, 90_025, 300))
            .unwrap();
        assert!(matches!(ack, Ack::Duplicate { .. }), "同键同内容应走幂等重放");
        assert_eq!(eng.broadcast().published(), published_before, "重放不得发布新组");
        assert_eq!(eng.broadcast().groups(), groups_before, "重放不得发布空组");
        assert_eq!(changed_rows(&before, &eng.snapshot), Vec::new(), "内存一分没动");
        assert!(eng.pending.is_empty(), "出口不该残留未发布的登记");
        assert_eq!(cursor, eng.seq(), "序号未推，最后一步就是撤单");
    }

    /// 每个消费者一份独立游标：各自拿到自己那段连续历史，已跟上的拿到空 Delta 而不是 Lagged。
    #[test]
    fn each_subscriber_pulls_its_own_contiguous_slice() {
        let mut eng = engine("pub-cursor");
        eng.place_and_fill(req("O1", "A001", "09018", Side::Buy, 90_025, 300))
            .unwrap();
        eng.place(req("O2", "A001", "09018", Side::Buy, 90_025, 100))
            .unwrap();
        assert_eq!(eng.seq(), 3);

        // 从头一次补齐：三个组依次相连，没被拆散。
        let CatchUp::Delta { changes, through, .. } = eng.catch_up(0) else {
            panic!("新消费者应能一次补到当前位：{}", eng.catch_up(0).summary());
        };
        assert_eq!(through, 3);
        let seqs: Vec<i64> = {
            let mut seen: Vec<i64> = Vec::new();
            for change in &changes {
                if seen.last() != Some(&change.seq) {
                    seen.push(change.seq);
                }
            }
            seen
        };
        assert_eq!(seqs, vec![1, 2, 3], "补发必须按组连续，不重不漏");

        // 中途接入（游标 2）只看得到第 3 组。
        let CatchUp::Delta { changes, from, .. } = eng.catch_up(2) else {
            panic!("游标 2 仍在留存窗口内");
        };
        assert_eq!(from, 3);
        assert!(changes.iter().all(|change| change.seq == 3));

        // 已跟上的消费者：空 Delta，不是降级。
        assert!(matches!(eng.catch_up(3), CatchUp::Delta { changes, .. } if changes.is_empty()));

        // 推游标后再写一组：只拿到新增的那一组，不漏不重。
        eng.cancel("O2").unwrap();
        let CatchUp::Delta { changes, from, through } = eng.catch_up(3) else {
            panic!("游标 3 应在留存窗口内");
        };
        assert_eq!((from, through), (4, 4));
        assert!(changes.iter().all(|change| change.seq == 4));
        assert!(!changes.is_empty());
    }

    /// 环小到只装得下一组时，落后的消费者被判 Lagged，而内核照常写 ——
    /// 读侧的速度永远不会变成写侧的债（铁律 5）。
    #[test]
    fn a_lagging_subscriber_is_dropped_without_touching_the_write_path() {
        let journal = Journal::open(&journal_path("pub-lag")).expect("测试 journal 打开失败");
        let mut eng = Engine::with_ring(snapshot(), journal, 1);

        for index in 0..4 {
            let order_id = format!("OL{index}");
            let cursor = if index == 0 { 0 } else { eng.seq() - 1 };
            let seq = step(&mut eng, cursor, |e| {
                e.place(req(&order_id, "A001", "09018", Side::Buy, 90_025, 100))
                    .unwrap();
            });
            assert_eq!(seq, index + 1);
        }

        assert_eq!(eng.seq(), 4);
        assert_eq!(eng.durable(), 4, "淘汰历史不影响落盘序号");
        assert!(eng.broadcast().dropped_rows() > 0, "落后组应被计入淘汰");
        let CatchUp::Lagged { oldest_seq, .. } = eng.catch_up(0) else {
            panic!("游标 0 早已跌出窗口：{}", eng.catch_up(0).summary());
        };
        assert_eq!(oldest_seq, Some(4), "只留最新一组");
        // 跟得上的消费者仍能拿到完整的一组变更。
        let CatchUp::Delta { changes, .. } = eng.catch_up(3) else {
            panic!("游标 3 仍在窗口内");
        };
        assert!(changes.iter().all(|change| change.seq == 4) && !changes.is_empty());
    }

    /// 回放只重建状态、不对外发布：恢复出来的内核 ring 从空开始，
    /// 而跨进程的历史（崩溃前那两组）一律不补发 —— 拿不到比拿错安全。
    #[test]
    fn replay_rebuilds_state_without_publishing() {
        let path = journal_path("pub-replay");
        {
            let journal = Journal::open(&path).expect("测试 journal 打开失败");
            let mut eng = Engine::new(snapshot(), journal);
            eng.place_and_fill(req("O1", "A001", "09018", Side::Buy, 90_025, 300))
                .unwrap();
            assert_eq!(eng.broadcast().published(), eng.durable(), "静置时出口与落盘齐平");
        }

        let mut recovered = Engine::recover(snapshot(), &path).expect("日初镜像 + journal 应能恢复");
        assert_eq!(recovered.engine.seq(), 2);
        assert!(recovered.engine.broadcast().is_empty(), "回放不得对外发布");
        assert!(
            recovered.engine.pending.is_empty(),
            "回放的登记必须清空（不然下一组就被守卫生卡住）"
        );
        // 恢复完、还没写新单：出口是空的 —— 这个进程还没有自己的历史。
        assert!(matches!(recovered.engine.catch_up(0), CatchUp::Delta { changes, .. } if changes.is_empty()));

        // 恢复后的第一笔写：序号接得上，出口也从这一组重新开始。
        recovered
            .engine
            .place(req("O2", "A001", "09018", Side::Buy, 90_025, 100))
            .expect("恢复后的内核应能继续接单");
        let CatchUp::Delta { changes, from, through } = recovered.engine.catch_up(2) else {
            panic!("游标 2 就是恢复点：{}", recovered.engine.catch_up(2).summary());
        };
        assert_eq!((from, through), (3, 3));
        assert!(changes.iter().all(|change| change.seq == 3));
        // 崩溃前的 1..2 不补发：游标 0 直接判降级。
        assert!(
            matches!(
                recovered.engine.catch_up(0),
                CatchUp::Lagged { lost_through: 0, oldest_seq: Some(3) }
            ),
            "{}",
            recovered.engine.catch_up(0).summary()
        );
    }

    /// 取行分派必须覆盖四张可变表：新表进内核而忘在 `row_json` 加臂，这里先红。
    #[test]
    fn row_json_dispatches_every_mutable_table() {
        let mut eng = engine("pub-rowjson");
        eng.place_and_fill(req("O1", "A001", "09018", Side::Buy, 90_025, 300))
            .unwrap();
        assert_eq!(
            MUTABLE_TABLES,
            [Asset::ID, Position::ID, Order::ID, Trade::ID],
            "内核可变表清单变了，取行分派与这份测试都要跟进"
        );
        for table in MUTABLE_TABLES {
            let key = match table {
                Asset::ID => "A001".to_string(),
                Position::ID => composite_key_str(&["A001", "09018"]),
                Order::ID => "O1".to_string(),
                Trade::ID => "O1-1".to_string(),
                other => panic!("内核新增可变表 {other}，测试未跟进"),
            };
            assert!(
                row_json(&eng.snapshot, table, &key).is_some(),
                "{table} 行 {key} 应能从出口取到"
            );
        }
    }

    /// 未接入出口的表不得静默丢变更，而是直接 panic（宁停不脏）。
    #[test]
    #[should_panic(expected = "未接入订阅出口")]
    fn unregistered_table_panics_instead_of_being_dropped() {
        let eng = engine("pub-unknown");
        let _ = row_json(&eng.snapshot, "user_info", "U001");
    }

    /// 入口漏挂 `publish_group`（或落盘后改内存失败）时，变更不得混进下一组 —— 下一次写即 panic。
    #[test]
    #[should_panic(expected = "没发布到订阅出口")]
    fn unpublished_changes_stop_the_next_group() {
        let mut eng = engine("pub-leak");
        eng.place(req("O1", "A001", "09018", Side::Buy, 90_025, 100))
            .unwrap();
        assert!(eng.pending.is_empty(), "正常出口应当已清空登记");
        // 模拟“改了内存却忘了发布”。
        eng.note_row::<Asset>("A001", Op::Upsert);
        let _ = eng.place(req("O2", "A001", "09018", Side::Buy, 90_025, 100));
    }
}
