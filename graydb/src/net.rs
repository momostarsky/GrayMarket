//! 网络发送侧（阶段 3.4）：把内核的拉取式出口接上真 socket，同时让「慢」只发生在连接自己身上。
//!
//! 铁律 5 在这一层的形状很具体：**持有 [`Engine`] 与 [`Hub`] 的那个线程一次 `await` 都不许有**。
//! 落成三条做法，都不是风格选择：
//! - 扇出用 `try_send`；队列满就**不提交游标**。那段历史留在环里，下一轮再投一次 ——
//!   既不丢数据也不阻塞内核。唯一的失败模式是「这个连接一直不消费 → 环把它淘汰掉 →
//!   判它落后」，代价全在它自己头上（见 [`Hub::fan_out`] 与 `batches_stalled`）。
//! - 新连接握手经一条 `try_recv` 的单向通道：内核不主动等谁，连上都要等下一轮扇出才被 attach。
//! - 快照帧不进有界队列，attach 时整块交给连接自己的任务 —— 否则「大表快照」会把
//!   队列容量变成协议限制（分批下发属后续项，见文末边界）。
//!
//! 反过来为什么不算「推送式广播回魂」（3.1 否决它是因为它会阻塞内核）：socket 读写、字节编码、
//! flush 全在 runtime 的任务里，跨线程传过去的只有**已拥有的 `String`**；内核类型
//! （`&'static str` 表名、`Subscriber` 游标、`RowChange`）从不离开内核线程。所以 `Engine` 不需要
//! `Send`，也不需要 `Arc<Mutex<..>>` —— 一旦给内核加读写锁，读侧就重新获得了把写路径卡死的能力，
//! 那正是当初拒绝 `tokio::sync::broadcast` 的理由。
//!
//! 已知边界（有意留白）：
//! - 快照一次性下发，未按表分批：一张 10 万行的表会在 attach 时产出一串帧、并在任务里攒成一块
//!   写出。真需要时改的是 [`Launch::snapshot`] 的生产方式，扇出与游标语义不动。
//! - 队列容量按**批**计不按字节计：一帧多大都占一格。真吞吐测试要换成字节预算。
//! - attach 的扇出轮次之间没有公平性策略：连接多时按注册顺序轮询，饿死的可能留到压测再治。

use std::fmt;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};

use crate::engine::Engine;
use crate::pubsub::{CatchUp, Frame, Subscribe, SubscribeError};
use crate::wire::{self, Mirror, WireFrame};

/// 投给某连接的一批 NDJSON 行。一批 = 一帧 `Delta`（或一条 `RebuildRequired`）。
pub type Batch = Vec<String>;

/// 握手与读写的兜底时限：宁可有界失败，不要无限期挂着一个任务。
const IO_TIMEOUT: Duration = Duration::from_secs(5);

/// 默认的连接发送队列容量，单位是**批**。
///
/// 1 批 = 1 帧 `Delta`，所以 64 表示「这个连接可以慢内核 64 组」。超出之后不再往里塞，
/// 游标原地不动，由环的淘汰来决定它是补发还是判落后。
pub const DEFAULT_QUEUE_BATCHES: usize = 64;

/// 内核线程与接入任务之间的握手请求。
pub struct Command {
    pub spec: Subscribe,
    /// attach 结果送回连接任务：成功带来 [`Launch`]，失败带回结构化拒订原因。
    pub reply: oneshot::Sender<Result<Launch, NetError>>,
}

/// attach 交给连接任务的两样东西：先写的快照，和之后收增量的队列。
#[derive(Debug)]
pub struct Launch {
    pub id: u64,
    /// 快照帧编好的行（整块写出，不占发送队列的格子）。
    pub snapshot: Vec<String>,
    /// 增量批队列。任务退出即 drop，内核在下一轮扇出时据此摘除连接。
    pub rx: mpsc::Receiver<Batch>,
}

/// 这一层的错误。`Reject` 原样携带 [`SubscribeError`]：拒订原因是协议内容，
/// 客户端该看到「表 orders 的主键首列不是账户列」，而不是一个 `400`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetError {
    /// 请求被协议层拒绝（表名、列名、过滤越界……）。
    Reject(SubscribeError),
    /// 请求行读不出来：JSON 非法、字段拼错、或超过单帧上限。
    BadRequest(String),
    /// 对端或内核已不再服务这条连接。
    Closed,
    /// 在 [`IO_TIMEOUT`] 内没等到该到的东西。
    Timeout(&'static str),
    /// 服务端把拒订原因写在握手层回话里（拿到的是「为什么」，不是一个 EOF）。
    Refused(String),
    /// 收到畸形帧（含 `Mirror` 判定为协议腐化而停下）。
    Wire(String),
    /// socket 读写失败（对端断开、超时之后的 IO 错）。
    Io(String),
}

impl From<wire::WireError> for NetError {
    fn from(err: wire::WireError) -> Self {
        NetError::Wire(err.to_string())
    }
}

impl From<mpsc::error::SendError<Batch>> for NetError {
    fn from(_: mpsc::error::SendError<Batch>) -> Self {
        NetError::Closed
    }
}

impl From<std::io::Error> for NetError {
    fn from(err: std::io::Error) -> Self {
        NetError::Io(err.to_string())
    }
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetError::Reject(reason) => write!(f, "订阅请求被拒：{reason}"),
            NetError::BadRequest(detail) => write!(f, "订阅请求读不出来：{detail}"),
            NetError::Closed => write!(f, "连接已不再被服务"),
            NetError::Timeout(what) => write!(f, "等 {what} 超时"),
            NetError::Refused(reason) => write!(f, "服务端拒订：{reason}"),
            NetError::Wire(detail) => write!(f, "帧不合规：{detail}"),
            NetError::Io(detail) => write!(f, "网络 IO 失败：{detail}"),
        }
    }
}

impl std::error::Error for NetError {}

/// 一个连接在内核侧的登记：游标 + 它的发送队列 + 它自己的账。
#[derive(Debug)]
struct Conn {
    id: u64,
    /// 服务端替它持有的游标（过滤与裁列口径都在这份 spec 里）。
    subscriber: crate::pubsub::Subscriber,
    tx: mpsc::Sender<Batch>,
    frames: u64,
    /// `try_send` 因队列满而失败的次数 —— 慢连接的**唯一**证据，内核不会替它等。
    stalled: u64,
    /// 已发过 `REBUILD_REQUIRED`：本地副本不可信，在内核看来这个连接到此为止，
    /// 不再给它增量（继续发只会让它以为自己能续上）。它要数据就得出站重连、重新 attach。
    paused: bool,
    dead: bool,
}

/// 扇出结果：一次轮次干了什么。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FanOut {
    pub frames: usize,
    pub stalled: usize,
    pub lagged: usize,
    pub detached: usize,
}

/// 累计统计（打印与断言都够用，不必逐个连接翻）。
#[derive(Debug, Default, Clone, Copy)]
pub struct HubStats {
    pub fan_outs: u64,
    pub frames_sent: u64,
    pub batches_stalled: u64,
    pub lagged: u64,
    pub detached: u64,
    /// 握手中被协议层拒掉的请求数。发送侧的拒绝也是个可观察量，
    /// 否则调用方只能靠「连接数没涨」反推，而那个信号与「请求还在路上」分不开。
    pub refused: u64,
}

impl fmt::Display for HubStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "扇出 {} 轮，发出 {} 帧，队列满而退让 {} 批，判落后 {} 次，摘除 {} 条，拒订 {} 条",
            self.fan_outs,
            self.frames_sent,
            self.batches_stalled,
            self.lagged,
            self.detached,
            self.refused
        )
    }
}

/// 内核线程独占的发送侧登记表。
///
/// 它**不**在 runtime 里跑：`Hub` 的方法全是同步的 `try_*`。runtime 任务只拿到
/// `mpsc::Receiver<Batch>`，因此没有任何一条路径能从异步侧摸回内核状态。
#[derive(Debug)]
pub struct Hub {
    conns: Vec<Conn>,
    next_id: u64,
    queue_batches: usize,
    commands: mpsc::UnboundedReceiver<Command>,
    stats: HubStats,
}

impl Hub {
    /// 建登记表与握手通道。通道交给 [`serve`]；登记表留在内核线程，**永不 `Send` 出去**。
    #[must_use]
    pub fn new(queue_batches: usize) -> (mpsc::UnboundedSender<Command>, Hub) {
        let (tx, rx) = mpsc::unbounded_channel();
        (
            tx,
            Hub {
                conns: Vec::new(),
                next_id: 1,
                queue_batches: queue_batches.max(1),
                commands: rx,
                stats: HubStats::default(),
            },
        )
    }

    #[must_use]
    pub fn connections(&self) -> usize {
        self.conns.len()
    }

    #[must_use]
    pub fn stats(&self) -> HubStats {
        self.stats
    }

    /// 某连接当前游标（诊断与测试断言用）。
    #[must_use]
    pub fn cursor_of(&self, id: u64) -> Option<i64> {
        self.conns.iter().find(|conn| conn.id == id).map(|conn| conn.subscriber.cursor())
    }

    /// 所有连接的 `(id, 游标, 已降级, 它自己的退让次数)`。连接 id 由内核发号，调用方不
    /// 该猜它是几；而「谁在退让」这件事必须能从服务端直接读出来，否则只能靠反推。
    #[must_use]
    pub fn cursors(&self) -> Vec<(u64, i64, bool, u64)> {
        self.conns
            .iter()
            .map(|conn| (conn.id, conn.subscriber.cursor(), conn.paused, conn.stalled))
            .collect()
    }

    /// 处理一条已到达的握手请求：校验 + attach + 把快照编成行整块交给连接任务。
    ///
    /// 快照的取行发生在**被调用的那一刻**（与 3.2 的 `subscribe` 同一份代码路径），
    /// 所以它的成本是 O(订阅表行数)，与对端网速无关。
    pub fn attach(&mut self, engine: &Engine, spec: Subscribe) -> Result<Launch, SubscribeError> {
        self.attach_with(engine, spec, self.queue_batches)
    }

    /// 与 [`Hub::attach`] 同，但由服务端（不是客户端）指定这条连接的发送队列容量。
    ///
    /// 每个订阅者缓冲不同是真实的（像 socket 的 SNDBUF）；而它**不能**从请求里拿：
    /// 那等于让客户端决定内核侧占多少内存。容量小只会让自己更容易退让与落后，影不到别人。
    pub fn attach_with(
        &mut self,
        engine: &Engine,
        spec: Subscribe,
        queue_batches: usize,
    ) -> Result<Launch, SubscribeError> {
        let (subscriber, frames) = engine.subscribe(spec)?;
        let id = self.next_id;
        self.next_id += 1;
        let snapshot = encode(&frames);
        let (tx, rx) = mpsc::channel(queue_batches.max(1));
        self.conns.push(Conn {
            id,
            subscriber,
            tx,
            frames: frames.len() as u64,
            stalled: 0,
            paused: false,
            dead: false,
        });
        Ok(Launch { id, snapshot, rx })
    }

    /// 收干握手队列：把已到达的请求逐个 attach 并把结果送回连接任务。
    ///
    /// 只在**有东西可取**时干活（`try_recv`），取不到立刻返回 —— 内核循环里绝不 park 等连接。
    pub fn drain_commands(&mut self, engine: &Engine) -> usize {
        let mut attached = 0;
        while let Ok(Command { spec, reply }) = self.commands.try_recv() {
            match self.attach(engine, spec) {
                Ok(launch) => {
                    attached += 1;
                    let id = launch.id;
                    // 任务可能已超时退出：送不出去就按普通断连处理，不留登记半成品。
                    if reply.send(Ok(launch)).is_err() {
                        self.drop_conn(id);
                    }
                }
                Err(reason) => {
                    self.stats.refused += 1;
                    let _ = reply.send(Err(NetError::Reject(reason)));
                }
            }
        }
        attached
    }

    /// 一轮扇出：给每个连接试着投它还没确认的那一截。
    ///
    /// 「先投出去、后提交游标」是这个方法的全部要点：[`Subscriber::peek_batch`] 只过滤裁列，
    /// 投成功才 [`Subscriber::commit`]。反过来（先推进再投）会在队列满的那一刻把那段历史
    /// 对这个连接永久抹掉 —— 它既收不到帧，也不会被判落后，只能靠对账发现少数据。
    pub fn fan_out(&mut self, engine: &Engine) -> FanOut {
        self.stats.fan_outs += 1;
        let mut out = FanOut::default();
        for conn in &mut self.conns {
            if conn.dead {
                continue;
            }
            // 探活：`try_reserve` 不占队列格子也能确定性地发现「对端已收摊」（它不像
            // `is_closed()` 那样是一个需要一次失败发送才会竖起来的事后标志）。不探这一步，
            // 写不出帧的死连接就永远没机会被发现（没新组 → 不 try_send → 不看错）。
            match conn.tx.try_reserve() {
                // 当场 drop：位置原样还回去，真要发帧时再重新要。
                Ok(reserved) => drop(reserved),
                // 满 = 正常背压，不是断连：两者混为一谈会把还在认真收数据的连接掉线。
                Err(mpsc::error::TrySendError::Full(_)) => {}
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    conn.dead = true;
                    out.detached += 1;
                    self.stats.detached += 1;
                    continue;
                }
            }
            if conn.paused {
                continue;
            }
            let cursor = conn.subscriber.cursor();
            match engine.catch_up(cursor) {
                CatchUp::Lagged { lost_through, .. } => {
                    let batch = encode(&[Frame::RebuildRequired { lost_through }]);
                    if conn.tx.try_send(batch).is_ok() {
                        conn.paused = true;
                        conn.frames += 1;
                        out.frames += 1;
                        out.lagged += 1;
                        self.stats.lagged += 1;
                        self.stats.frames_sent += 1;
                    } else {
                        // 投不出去同样是退让：不记下这一笔，统计上就会把「降级没送达」
                        // 看成「本轮什么都没发生」。游标不动，下一轮再试（这条判断幂等）。
                        conn.stalled += 1;
                        out.stalled += 1;
                        self.stats.batches_stalled += 1;
                    }
                }
                CatchUp::Delta { through, changes, .. } => {
                    if through <= cursor {
                        continue;
                    }
                    let picked = conn.subscriber.peek_batch(&changes);
                    let batch = encode(&[Frame::Delta { through, changes: picked }]);
                    match conn.tx.try_send(batch) {
                        Ok(()) => {
                            conn.subscriber.commit(through);
                            conn.frames += 1;
                            out.frames += 1;
                            self.stats.frames_sent += 1;
                        }
                        // 队列满：游标停在原地，历史继续在环里等他 —— 内核一分钱成本没替他垫。
                        Err(mpsc::error::TrySendError::Full(_)) => {
                            conn.stalled += 1;
                            out.stalled += 1;
                            self.stats.batches_stalled += 1;
                        }
                        Err(mpsc::error::TrySendError::Closed(_)) => {
                            conn.dead = true;
                            out.detached += 1;
                            self.stats.detached += 1;
                        }
                    }
                }
            }
        }
        self.conns.retain(|conn| !conn.dead);
        out
    }

    fn drop_conn(&mut self, id: u64) {
        if let Some(conn) = self.conns.iter_mut().find(|conn| conn.id == id) {
            conn.dead = true;
        }
        self.stats.detached += 1;
        self.conns.retain(|conn| !conn.dead);
    }
}

fn encode(frames: &[Frame]) -> Batch {
    wire::encode_lines(frames).expect("帧编码失败只能是编码器 bug，不是运行期状况")
}

// ── runtime 侧 ───────────────────────────────────────────────────

/// 接受循环：每个连接一个任务，握手与写出都在任务里，内核不参与等待。
pub async fn serve(listener: TcpListener, commands: mpsc::UnboundedSender<Command>) {
    loop {
        let (stream, peer) = match listener.accept().await {
            Ok(accepted) => accepted,
            Err(err) => {
                eprintln!("accept 失败，继续监听：{err}");
                continue;
            }
        };
        let commands = commands.clone();
        tokio::spawn(async move {
            if let Err(err) = conn_task(stream, commands).await {
                eprintln!("连接 {peer} 结束：{err}");
            }
        });
    }
}

/// 一条连接的完整生命周期：读一行请求 → 等 attach → 写快照 → 转发增量直到队列关闭。
pub async fn conn_task(
    stream: TcpStream,
    commands: mpsc::UnboundedSender<Command>,
) -> Result<(), NetError> {
    let _ = stream.set_nodelay(true);
    let (read_half, write_half) = stream.into_split();
    let spec = read_request(read_half).await?;

    let (reply_tx, reply_rx) = oneshot::channel();
    commands
        .send(Command { spec, reply: reply_tx })
        .map_err(|_| NetError::Closed)?;
    // 三层结果各有一件不同的事：等超时、内核循环已退出、内核拒订（原因原样上抛）。
    let reply = with_timeout("attach", reply_rx).await?;
    match reply.map_err(|_| NetError::Closed)? {
        Ok(launch) => write_launch(launch, BufWriter::new(write_half)).await,
        // 拒订：先把原因写成一行控制回话再收摊。只关连接的话，客户端只能从 EOF 反推，
        // 而 EOF 与「网络掉了」分不开。写不进也不影响这次拒绝已成立（原因只是送得漂亮点）。
        Err(reason @ NetError::Reject(_)) => {
            let mut writer = BufWriter::new(write_half);
            let line = wire::reject_line(&reason.to_string());
            let _ = with_timeout("拒订回话", async {
                writer.write_all(line.as_bytes()).await?;
                writer.flush().await
            })
            .await;
            Err(reason)
        }
        Err(other) => Err(other),
    }
}

async fn read_request(read_half: OwnedReadHalf) -> Result<Subscribe, NetError> {
    let mut lines = BufReader::new(read_half).lines();
    let Some(line) = with_timeout("订阅请求", lines.next_line()).await?? else {
        return Err(NetError::Closed); // 对端关掉写半，一个请求都没发就走
    };
    wire::decode_request(&line).map_err(|err| NetError::BadRequest(err.to_string()))
}

async fn write_launch(
    launch: Launch,
    mut writer: BufWriter<OwnedWriteHalf>,
) -> Result<(), NetError> {
    let Launch { id: _, snapshot, mut rx } = launch;
    if !snapshot.is_empty() {
        // 快照整块写出：大表快照的字节数不该受发送队列容量约束（分批下发是另一件事）。
        let block = format!("{}\n", snapshot.join("\n"));
        with_timeout("快照写出", writer.write_all(block.as_bytes())).await??;
        with_timeout("快照 flush", writer.flush()).await??;
    }
    while let Some(batch) = rx.recv().await {
        // 这里不等超时：写不出去 = 对端不收数据，5 秒后直接掉线（由 Hub 下一轮摘除）。
        // 这是第二道背压：它掉的是这一条连接，不是内核。
        let body = format!("{}\n", batch.join("\n"));
        with_timeout("增量写出", writer.write_all(body.as_bytes())).await??;
        with_timeout("增量 flush", writer.flush()).await??;
    }
    // rx 到这里已关闭（内核摘除连接或 Hub 被丢弃）：正常收摊。
    Ok(())
}

async fn with_timeout<T>(what: &'static str, fut: impl std::future::Future<Output = T>) -> Result<T, NetError> {
    tokio::time::timeout(IO_TIMEOUT, fut).await.map_err(|_| NetError::Timeout(what))
}

// ── 回环客户端 ───────────────────────────────────────────────────
//
// 演示与测试需要一个「只会按协议读」的一端，它刻意不与内核共用任何判定：
// 请求手写 JSON 形状、行由 [`wire`] 解、镜像由 [`Mirror`] 重建 —— 服务端 bug 骗不过它。

/// 读端：把帧序列重建成 [`Mirror`]。
pub struct Client {
    lines: tokio::io::Lines<BufReader<OwnedReadHalf>>,
    writer: OwnedWriteHalf,
    mirror: Mirror,
    frames: u64,
    ends: usize,
    tables: usize,
    /// 第一行才需验是不是拒订回话（它只可能出现在握手位）。
    handed: bool,
}

impl Client {
    pub async fn connect(addr: std::net::SocketAddr, spec: &Subscribe) -> Result<Self, NetError> {
        let stream = TcpStream::connect(addr).await.map_err(|err| NetError::Io(err.to_string()))?;
        let _ = stream.set_nodelay(true);
        let (read_half, mut writer) = stream.into_split();
        let request = serde_json::to_string(spec).map_err(|err| NetError::BadRequest(err.to_string()))?;
        writer.write_all(request.as_bytes()).await.map_err(|err| NetError::Io(err.to_string()))?;
        writer.write_all(b"\n").await.map_err(|err| NetError::Io(err.to_string()))?;
        writer.flush().await.map_err(|err| NetError::Io(err.to_string()))?;
        let tables = match spec.snapshot {
            // 屏障数按**展开后的表集**算，不按主题串数：`table:*` 一条就代表若干张表。
            crate::pubsub::SnapshotMode::Full => {
                match crate::pubsub::expand_topics(&spec.topics, crate::tables::TABLES) {
                    Ok(tables) => tables.len(),
                    // 展开不开只有一种情况：这条请求本身会被服务端拒。那就一路读到那句拒因为止 ——
                    // 客户端有权把一个会被拒的请求真发出去，不在这里抢跑判定（拒订通道靠这条路才能被验到）。
                    Err(_) => usize::MAX,
                }
            }
            crate::pubsub::SnapshotMode::DeltaOnly => 0,
        };
        Ok(Client {
            lines: BufReader::new(read_half).lines(),
            writer,
            mirror: Mirror::new(),
            frames: 0,
            ends: 0,
            tables,
            handed: false,
        })
    }

    #[must_use]
    pub fn mirror(&self) -> &Mirror {
        &self.mirror
    }

    #[must_use]
    pub fn frames(&self) -> u64 {
        self.frames
    }

    /// 读并应用一帧。服务端收摊报 [`NetError::Closed`]，被拒报 [`NetError::Refused`]。
    pub async fn next_frame(&mut self) -> Result<WireFrame, NetError> {
        let Some(line) = with_timeout("下一帧", self.lines.next_line()).await?? else {
            return Err(NetError::Closed);
        };
        // 拒订回话只可能占握手位；不在每一行上都试，免得每帧多解一次 JSON。
        if !self.handed {
            self.handed = true;
            if let Some(reason) = wire::as_reject(&line) {
                return Err(NetError::Refused(reason));
            }
        }
        let frame = wire::decode_line(&line)?;
        self.mirror.apply(&frame).map_err(|err| NetError::Wire(err.to_string()))?;
        self.frames += 1;
        if matches!(frame, WireFrame::SnapshotEnd { .. }) {
            self.ends += 1;
        }
        Ok(frame)
    }

    /// 读到全表快照收尾（`SNAPSHOT_END` 数够表数）。
    pub async fn finish_snapshot(&mut self) -> Result<(), NetError> {
        while self.ends < self.tables {
            self.next_frame().await?;
        }
        Ok(())
    }

    /// 读到镜像水位 >= `through`。中途遇到 `REBUILD_REQUIRED` 直接返回，
    /// 让调用方能在断言里点名「这条连接被降级了」而不是靠超时。
    pub async fn advance_to(&mut self, through: i64) -> Result<(), NetError> {
        while self.mirror.last_through() < through {
            if matches!(self.next_frame().await?, WireFrame::RebuildRequired { .. }) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// 主动关掉写半：服务端据此结束转发（演示断连用）。
    pub async fn shutdown(&mut self) -> Result<(), NetError> {
        self.writer.shutdown().await.map_err(|err| NetError::Io(err.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Side;
    use crate::engine::PlaceRequest;
    use crate::journal::Journal;
    use crate::mem::Snapshot;
    use crate::pubsub::Op;
    use account::amount::{Price, Quantity};
    use std::collections::BTreeMap;
    use std::path::Path;

    const TABLES: &[&str] = &["account_asset", "orders", "trades", "position"];

    fn snapshot() -> Snapshot {
        Snapshot::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("data")).expect("测试主数据加载失败")
    }

    fn engine(tag: &str) -> Engine {
        Engine::new(snapshot(), journal(tag))
    }

    fn engine_with_ring(tag: &str, rows: usize) -> Engine {
        Engine::with_ring(snapshot(), journal(tag), rows)
    }

    fn journal(tag: &str) -> Journal {
        let mut path = std::env::temp_dir();
        path.push(format!("graydb-net-{tag}-{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&path);
        Journal::open(&path).expect("测试 journal 打开失败")
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
            created_at: "2026-09-30T09:30:00Z",
        }
    }

    fn spec() -> Subscribe {
        Subscribe::new(TABLES)
    }

    /// 把队列里已有的批全部取走（**不等**），返回所有行。
    fn drain(rx: &mut mpsc::Receiver<Batch>) -> Vec<String> {
        let mut lines = Vec::new();
        while let Ok(batch) = rx.try_recv() {
            lines.extend(batch);
        }
        lines
    }

    fn apply(lines: &[String], mirror: &mut Mirror) {
        for line in lines {
            let frame = wire::decode_line(line).expect("发送侧发出的行必须解得开");
            mirror.apply(&frame).expect("发送侧发出的帧必须能重建");
        }
    }

    /// 拿发送侧发出去的行与内核当前镜像逐行对账（表内容相等，不只看行数）。
    fn assert_matches_kernel(mirror: &Mirror, engine: &Engine, tables: &[&str]) {
        for &table in tables {
            let kernel: BTreeMap<String, serde_json::Value> =
                engine.snapshot.rows_json(table).into_iter().collect();
            let consumer: BTreeMap<String, serde_json::Value> =
                mirror.table(table).cloned().unwrap_or_default();
            assert_eq!(consumer, kernel, "表 {table} 的消费者副本与内核不一致");
        }
    }

    #[test]
    fn fan_out_commits_the_cursor_only_after_the_batch_is_accepted() {
        // 这条是本阶段的命门：游标推进必须发生在「投出去」之后。反过来做会把那段
        // 历史对这个连接永久抹掉 —— 它既收不到帧，也不会被判落后。
        let (_commands, mut hub) = Hub::new(1);
        let mut eng = engine("commit-last");
        let mut launch = hub.attach(&eng, spec()).expect("默认请求该被接受");
        assert!(!launch.snapshot.is_empty(), "全量快照该随 attach 一起给出");
        assert_eq!(hub.connections(), 1);
        let mut lines = launch.snapshot.clone();

        eng.place_and_fill(req("N1", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        let first_group = eng.durable();
        let out = hub.fan_out(&eng);
        assert_eq!(out.frames, 1, "一帧一批");
        assert_eq!(hub.cursor_of(launch.id), Some(first_group));

        eng.place_and_fill(req("N2", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        let out = hub.fan_out(&eng);
        assert_eq!(out.stalled, 1, "队列已满，该退让而不是推进");
        assert_eq!(out.frames, 0);
        assert_eq!(hub.cursor_of(launch.id), Some(first_group), "投不出去的批不能提交水位");

        // 消费者取走一批 → 下一轮补得上，而且一段都没丢（批都得留着参与对账）。
        lines.extend(drain(&mut launch.rx));
        let out = hub.fan_out(&eng);
        assert_eq!(out.frames, 1);
        assert_eq!(hub.cursor_of(launch.id), Some(eng.durable()));
        lines.extend(drain(&mut launch.rx));

        let mut mirror = Mirror::new();
        apply(&lines, &mut mirror);
        assert_matches_kernel(&mirror, &eng, TABLES);
    }

    #[test]
    fn a_stalled_connection_costs_the_kernel_nothing() {
        // 铁律 5 的正面断言：一条从不消费的连接，既挡不住另一条连接收全量，
        // 也不让内核的序号停下 —— 它的代价全部记在它自己的账上。
        let (_commands, mut hub) = Hub::new(1);
        let mut eng = engine("slow-neighbor");
        let quick = hub.attach(&eng, spec()).unwrap();
        let lagging = hub.attach(&eng, spec()).unwrap();
        let mut quick_rx = quick.rx;
        let mut lag_rx = lagging.rx;

        let mut quick_lines = quick.snapshot.clone();
        for index in 0..6 {
            let order = format!("S{index}");
            eng.place_and_fill(req(&order, "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
            hub.fan_out(&eng);
            quick_lines.extend(drain(&mut quick_rx));
        }

        assert_eq!(eng.seq(), eng.durable(), "内核写完就该落盘齐平，与连接快慢无关");
        assert_eq!(hub.cursor_of(quick.id), Some(eng.durable()), "及时消费的连接该一路跟上");
        let stalled_cursor = hub.cursor_of(lagging.id).expect("落后连接仍在登记里");
        assert!(stalled_cursor < eng.durable(), "落后者的游标该钉在自己的位置，而不是被推平");
        assert_eq!(drain(&mut lag_rx).len(), 1, "它只留得住队列容量那么多批");
        let stats = hub.stats();
        assert!(stats.batches_stalled > 0, "队列满是它的账：{stats}");

        // 发出去的帧必须能重建出与内核逐行一致的副本 —— 不退让、不丢行。
        let mut mirror = Mirror::new();
        apply(&quick_lines, &mut mirror);
        assert_matches_kernel(&mirror, &eng, TABLES);
    }

    #[test]
    fn a_smaller_queue_hurts_only_the_connection_that_has_it() {
        // [`Hub::attach_with`] 存在的理由就两条，都得钉住：容量是**服务端**给的策略（不能来自
        // 请求，否则客户端决定内核侧占多少内存），而把它调小只坑自己 —— 邻居的帧照发、水位照推。
        let (_commands, mut hub) = Hub::new(64);
        let mut eng = engine("per-conn-queue");
        let tight = hub.attach_with(&eng, spec(), 1).expect("小队列登记该被接受");
        let wide = hub.attach(&eng, spec()).expect("默认容量登记该被接受");
        let mut tight_rx = tight.rx;
        let mut wide_rx = wide.rx;
        assert_eq!(hub.connections(), 2);

        eng.place(req("Q1", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        hub.fan_out(&eng);
        // 两边都拿到第一批：容量 1 在这一刻还看不出差别，差别从它填满那轮开始。
        assert_eq!(drain(&mut tight_rx).len(), 1, "小队列也该拿到自己那一批");
        assert_eq!(drain(&mut wide_rx).len(), 1);

        // 接着连下三组而谁都不再读：小队列第二轮就满，后面的轮次只能退让。
        for index in 2..=4 {
            let order = format!("Q{index}");
            eng.place(req(&order, "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
            hub.fan_out(&eng);
        }
        let stats = hub.stats();
        assert!(stats.batches_stalled > 0, "小队列该已替自己退让：{stats}");

        // 差别只落在它自己的账上，而且能直接读出来（不必靠「连接数没涨」这类反推）。
        let cursors = hub.cursors();
        let &(_, tight_cursor, tight_paused, tight_stalled) =
            cursors.iter().find(|(id, ..)| *id == tight.id).expect("小队列连接仍在登记里");
        let &(_, wide_cursor, wide_paused, wide_stalled) =
            cursors.iter().find(|(id, ..)| *id == wide.id).expect("默认容量连接仍在登记里");
        assert_eq!(wide_cursor, eng.durable(), "容量够的连接该一路提交到当前位，不受邻居影响");
        assert_eq!(wide_stalled, 0, "退让不该记到别人头上");
        assert!(tight_cursor < eng.durable(), "小队列的游标该钉在它投得出去的那一截：{tight_cursor}");
        assert!(tight_stalled > 0, "退让只记在容量小的那条：{tight_stalled}");
        assert!(!tight_paused && !wide_paused, "默认环兜得住，两条都不该被判落后");

        // 它自己把攒着的取走之后，下一轮就续上：那段历史只是迟了，没被抹掉。
        drain(&mut tight_rx);
        eng.place(req("Q5", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        hub.fan_out(&eng);
        assert_eq!(hub.cursor_of(tight.id), Some(eng.durable()), "腾出位置后该续到当前位");
    }

    #[test]
    fn a_lagging_connection_is_told_to_rebuild_and_then_left_alone() {
        // 环只留 4 行（一次下单 2 行）→ 不消费的连接迟早把自己那段历史挤掉。
        // 要点有两个：降级必须送达才算降级（投不出去就继续等，不能假装说过）；
        // 以及降级之后内核不再给它增量（继续发只会让它以为自己能续上）。
        let (_commands, mut hub) = Hub::new(1);
        let mut eng = engine_with_ring("lagged-rebuild", 4);
        let mut launch = hub.attach(&eng, spec()).unwrap();

        eng.place(req("G1", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        assert_eq!(hub.fan_out(&eng).frames, 1, "第一批该投进队列");
        let cursor_before = hub.cursor_of(launch.id).unwrap();
        assert_eq!(cursor_before, eng.durable());

        // 之后每轮都投不进去：只退让，不推进游标。
        for index in 2..=4 {
            let order = format!("G{index}");
            eng.place(req(&order, "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
            let out = hub.fan_out(&eng);
            assert_eq!(out.stalled, 1, "第 {index} 组该退让");
            assert_eq!(out.lagged, 0, "降级帧也没投出去时不算已降级");
            assert_eq!(hub.cursor_of(launch.id), Some(cursor_before), "退让不推水位");
        }

        // 腾出位置后，降级才能送达；而它自己那段未确认的历史已被环淘汰。
        assert_eq!(drain(&mut launch.rx).len(), 1, "先取走排着的那一批");
        let out = hub.fan_out(&eng);
        assert_eq!(out.lagged, 1, "该发出 rebuild_required");
        let lines = drain(&mut launch.rx);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with(r#"{"rebuild_required"#), "实得：{}", lines[0]);
        assert_eq!(hub.cursor_of(launch.id), Some(cursor_before), "降级不推进游标（3.2 语义）");

        // 已降级的连接不再收增量。
        eng.place(req("G5", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        assert_eq!(hub.fan_out(&eng).frames, 0, "paused 之后该只当他不存在");
        assert_eq!(drain(&mut launch.rx).len(), 0);

        // 而新连接照常服务：落后是个体的事，不是全局的惩罚。
        let fresh = hub.attach(&eng, spec()).unwrap();
        assert!(!fresh.snapshot.is_empty());
        assert_eq!(hub.cursor_of(fresh.id), Some(eng.durable()));
    }

    #[test]
    fn a_dead_writer_is_detached_on_the_next_fan_out() {
        // 连接任务退出 = rx 被 drop → 下一轮扇出把它从登记里摘掉，不留僵尸。
        let (_commands, mut hub) = Hub::new(4);
        let eng = engine("dead-writer");
        let launch = hub.attach(&eng, spec()).unwrap();
        assert_eq!(hub.connections(), 1);
        drop(launch.rx);
        let out = hub.fan_out(&eng);
        assert_eq!(out.detached, 1);
        assert_eq!(hub.connections(), 0);
        assert_eq!(hub.stats().detached, 1);
    }

    #[test]
    fn an_out_of_scope_request_is_refused_over_the_channel_with_its_reason() {
        // 拒订原因走通道回到连接任务：客户端该看到「表 orders 的主键首列不是账户列」，
        // 而不是一个笼统的「请求不合法」。
        let (commands, mut hub) = Hub::new(4);
        let eng = engine("refused");
        let (reply_tx, mut reply_rx) = oneshot::channel();
        commands
            .send(Command {
                spec: Subscribe::new(&["orders"]).accounts(&["A001"]),
                reply: reply_tx,
            })
            .expect("内核循环还活着");

        assert_eq!(hub.drain_commands(&eng), 0, "被拒的请求不该登记连接");
        assert_eq!(hub.connections(), 0);
        let err = reply_rx
            .try_recv()
            .expect("回话该已送达")
            .expect_err("这条请求必须被拒");
        assert!(
            matches!(
                err,
                NetError::Reject(SubscribeError::FilterNotSupported { table: ref got }) if got == "orders"
            ),
            "拒订原因要走协议层回话，实得：{err}"
        );
    }

    #[test]
    fn an_idle_round_says_nothing_but_a_watermark_round_says_the_watermark() {
        // 静置时扇出不该发出任何帧，也不该碰队列；而水位一旦推进，哪怕本订阅者
        // 一行都没得（只订 delete），也必须把「≤ through 已确认无你的行」说出口。
        let (_commands, mut hub) = Hub::new(4);
        let mut eng = engine("idle-round");
        let mut launch = hub
            .attach(&eng, Subscribe::new(TABLES).ops(&[Op::Delete]).delta_only())
            .expect("delta_only + 只订 delete 是合法请求");
        assert!(launch.snapshot.is_empty(), "delta_only 不该下发快照");

        let out = hub.fan_out(&eng);
        assert_eq!(out, FanOut::default(), "静置时扇出不该发出任何帧");
        assert_eq!(drain(&mut launch.rx).len(), 0);

        // 下单只动 orders 与资产（全是 upsert），delete-only 的连接一行都不要。
        eng.place(req("I1", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        let out = hub.fan_out(&eng);
        assert_eq!(out.frames, 1, "水位推了就要发一帧，即使批是空的");
        let lines = drain(&mut launch.rx);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains(r#""changes":[]"#), "空批也得看得见：{}", lines[0]);
        assert_eq!(hub.cursor_of(launch.id), Some(eng.durable()), "游标该跟上水位");
    }

    /// 3.5：一条 `table:*` 过真 socket。客户端按展开后的表数等屏障，服务端按同一份展开下发 ——
    /// 两端各自算却拿到同一张表集，顺便把 `Client::connect` 里的展开口径也验到。
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_wildcard_topic_over_a_real_connection_delivers_every_table() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind 失败");
        let addr = listener.local_addr().expect("该拿到端口");
        let (commands, mut hub) = Hub::new(DEFAULT_QUEUE_BATCHES);
        tokio::spawn(serve(listener, commands.clone()));

        let mut eng = engine("loopback-star");
        eng.place_and_fill(req("S0", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        let mut client = Client::connect(addr, &Subscribe::of_topics(&["table:*"]))
            .await
            .expect("全库主题的连接与请求写出应成功");
        for _ in 0..100 {
            if hub.drain_commands(&eng) > 0 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        // 屏障数不等于主题串数：一条 `table:*` 背后是注册中心的全部登记表。
        let declared: Vec<&str> = crate::tables::TABLES.iter().map(|spec| spec.id).collect();
        client.finish_snapshot().await.expect("全库快照该读完");
        assert!(declared.len() > 1, "这条验的是通配，单张表证不了");
        assert_matches_kernel(client.mirror(), &eng, &declared);

        // 增量口径不因通配而变：同一个水位、同一批行。
        eng.place_and_fill(req("S1", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        hub.fan_out(&eng);
        client.advance_to(eng.durable()).await.expect("增量该读到");
        assert_matches_kernel(client.mirror(), &eng, &declared);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_real_loopback_connection_rebuilds_the_kernel_state() {
        // 回环上跑完整一跑：真 socket、真 NDJSON、真异步任务。
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind 失败");
        let addr = listener.local_addr().expect("该拿到端口");
        let (commands, mut hub) = Hub::new(DEFAULT_QUEUE_BATCHES);
        tokio::spawn(serve(listener, commands.clone()));

        let mut eng = engine("loopback");
        eng.place_and_fill(req("L0", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        let mut client = Client::connect(addr, &spec()).await.expect("连接与请求写出应成功");

        // 新连接的建立取决于下一次握手轮次（已知边界：内核不等谁）。
        let mut attached = 0;
        for _ in 0..100 {
            attached += hub.drain_commands(&eng);
            if attached > 0 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert_eq!(attached, 1, "握手请求该被 attach");

        client.finish_snapshot().await.expect("快照该能读完");
        assert_matches_kernel(client.mirror(), &eng, TABLES);
        let watermark = client.mirror().last_through();

        eng.place_and_fill(req("L1", "A001", "600000", Side::Buy, 60_600, 100)).unwrap();
        hub.fan_out(&eng);
        client.advance_to(eng.durable()).await.expect("增量该能读到");
        assert_eq!(client.mirror().last_through(), eng.durable());
        assert!(watermark < eng.durable(), "写完就该比快照水位更新");
        assert_matches_kernel(client.mirror(), &eng, TABLES);

        // 拒订也要走真连接：连接本身能建立，被拒的是请求 —— 服务端说完原因就收摊。
        let mut rejected = Client::connect(addr, &Subscribe::new(&["orders"]).accounts(&["A001"])).await
            .expect("连接能建立");
        for _ in 0..100 {
            hub.drain_commands(&eng);
            if hub.stats().refused > 0 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert_eq!(hub.stats().refused, 1, "越界请求该被协议层拒掉");
        assert_eq!(hub.connections(), 1, "被拒的请求不该登记第二条连接");
        let err = rejected
            .finish_snapshot()
            .await
            .expect_err("越界请求不该读到快照");
        // 拿到的不是 EOF，而是服务端那句人话。
        assert!(matches!(err, NetError::Refused(_)), "实得：{err}");
        assert!(err.to_string().contains("账户列"), "原因要能读：{err}");
    }
}
