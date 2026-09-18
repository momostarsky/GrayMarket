//! 定点金额 / 价格 / 数量：整数计数 + **编译期标度**（scaled integer newtype）。
//!
//! # 表示法
//!
//! `Amount` 内部只有一个整数，含义是「**最小单位的个数**」：
//!
//! ```text
//! 数值 = units / 10^SCALE
//! ```
//!
//! 「精度」（金额 0.01 元、价格 0.0001 元）**不存在数据里，而是编码在类型参数 `SCALE` 中**。
//! 本系统的口径是 **金额 2 位小数、价格 4 位小数**：
//!
//! | 类型        | `SCALE` | 1 个单位     | 1 元 = 多少个单位 | 语义                         |
//! |-------------|---------|--------------|-------------------|------------------------------|
//! | `Amount<0>` | 0       | 1            | —                 | 数量（整股、整张）           |
//! | `Amount<2>` | 2       | 0.01 元      | 100               | **金额**：资金、成交额、费用 |
//! | `Amount<4>` | 4       | 0.0001 元    | 10_000            | **价格**：委托价、净值       |
//! | `Amount<6>` | 6       | 0.000001 元  | 1_000_000         | 中间精度：费用率、汇率       |
//!
//! ```text
//! Amount::<2>::from_units(1)      // 1 个单位      → 0.01 元
//! Amount::<2>::from_units(100)    // 100 个单位    → 1.00 元
//! Amount::<4>::from_units(1)      // 1 个单位      → 0.0001 元
//! Amount::<4>::from_units(10_000) // 10_000 个单位 → 1.0000 元
//! ```
//!
//! 金额与价格是**不同类型**，编译期就不能相加减 —— 这正是「品种级静态 scale」的价值：
//! 把「元」当「分」加、把价格漏乘数量，这类 bug 被编译器拦住，
//! 而不是在清算差一分钱时才被发现。
//!
//! # 精度（总位数）
//!
//! `SCALE` 只决定小数点在哪儿，不改变总位数；总位数由底层整数 `B` 决定：
//!
//! | `B`    | 十进制位数 | `SCALE` 范围 | `Amount<2>` 金额上限 | `Amount<4>` 价格上限 | `Amount<6>` 中间量上限 |
//! |--------|-----------|--------------|----------------------|----------------------|------------------------|
//! | `i64`  | 19        | 0..=18       | 16 位（≈9.2e16 元）  | 15 位（≈9.2e14 元）  | 13 位（≈9.2e12 元）    |
//! | `i128` | 39        | 0..=38       | 37 位                | 35 位                | 33 位                  |
//!
//! `SCALE` 越小，能装的整数区间越大、小数位越少。**金额口径 `Amount<2>` 有 16 位整数位，
//! 单账户余额、单笔成交额、单日费用用 `i64` 都绰绰有余**；而 `SCALE = 6` 的中间量
//! 上限只有约 9.2 万亿元，所以**汇总、中央账户、长期累计成交额必须换 `i128` 后备**
//! （见 [`WideMicroAmount`]），否则会在跑批时静默溢出。
//!
//! # `SCALE` 的取值范围
//!
//! 上限不是「十进制位数」而是「十进制位数 − 1」，因为 [`Amount::units_per_one`] 要能装下
//! `10^SCALE`：
//!
//! * `i64` → `SCALE ∈ 0..=18`（`10^18 = 1e18 ≤ i64::MAX ≈ 9.22e18`；`10^19` 溢出）
//! * `i128` → `SCALE ∈ 0..=38`（`10^38 ≤ i128::MAX ≈ 1.70e38`；`10^39` 溢出）
//!
//! 上限由 [`Backing::MAX_SCALE`] / [`Amount::max_scale`] 给出。超出上限的类型仍可
//! `from_units` / `units` / `checked_add`（就是普通的整数运算），只有
//! [`Amount::units_per_one`] 与依赖它的 [`Amount::from_whole`] 会返回 `None`；
//! 此时该类型已无实际意义（1 元都表示不出来），属于设计错误而非运行时状况。
//!
//! # 跨标度换算
//!
//! 放大标度是精确的（×10^n），缩小标度必须舍入（÷10^n），策略由 [`Rounding`] 显式给出：
//!
//! ```text
//! let price = Amount::<2>::from_units(1234);                              // 12.34 元
//! let wide  = price.widen::<6>().unwrap();                                // 12_340_000，精确
//! let back  = wide.narrow::<2>(Rounding::MidpointAwayFromZero).unwrap();  // 1234
//! ```
//!
//! **注意**：stable Rust 不允许在两个 `const` 泛型之间做运算（`where { TO >= SCALE }`
//! 需要 nightly 的 `generic_const_exprs`），所以 `widen` / `narrow` 的标度方向只能在
//! **运行期**检查，方向不对返回 `None`；编译期能保证的只有「不同标度是不同类型」。
//! 同理，把命名常量用作 const 泛型实参时必须加大括号：`Amount<{ scale::MONEY }>`。
//!
//! # 中间结果用 `i128`，余额用 `i64`
//!
//! 三个环节，转换点越少越好：
//!
//! 1. **中间计算**：成交额、按比例分摊、费用累加用 `Amount<S, i128>`（见 [`WideMoney`] /
//!    [`WideMicroAmount`]），全程整数、精确、无舍入；
//! 2. **落账（唯一切换点）**：用 [`Amount::convert`] 一次完成「换标度 + 换后备」，
//!    或 [`Amount::settle`] 额外把「舍入后为 0」也判为错误；
//! 3. **余额**：回到 `Amount<S, i64>`。只有 `i64` 有 `AtomicI64`，
//!    无锁 CAS 扣减只能建立在 `i64` 上 —— 这就是余额必须是 `i64` 的根本原因。
//!
//! ```text
//! // 1) 中间结果：标度 6 + i128 累计，全程不舍入
//! let accrued: WideMicroAmount = fees.iter().copied().sum();   // 2.469135 元
//!
//! // 2) 落账：一次缩到「分」（目标类型 Money 已经把 SCALE 定死，不必写泛型实参）
//! let booked: Money = accrued.convert(Rounding::MidpointAwayFromZero)?;  // 2.47 元
//!
//! // 3) 余额：Money（标度 2 的 i64），可无锁 CAS 更新
//! balance = balance.checked_add_wide(booked)?;
//! ```
//!
//! **必须舍入一次**：先缩到中间标度再缩到目标标度会引入双舍入误差。
//! 0.4449 元一次缩到 2 位小数得 `0.44`，先缩到 3 位（`0.445`）再缩到 2 位却得 `0.45`。
//!
//! **i128 → i64 溢出绝不允许静默处理**：没有截断、没有饱和，只有 [`Option`] / [`Result`]。
//! 交易内核里「账面值被悄悄改小」比「下单被拒」严重得多。

use core::fmt;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use rust_decimal::Decimal;

mod sealed {
    pub trait Sealed {}
    impl Sealed for i64 {}
    impl Sealed for i128 {}
}

/// 底层整数后备类型：`i64`（默认）或 `i128`。
///
/// 该 trait 是 sealed 的，外部无法实现。
pub trait Backing:
    sealed::Sealed
    + Copy
    + Default
    + Ord
    + fmt::Debug
    + fmt::Display
    + core::hash::Hash
    + Send
    + Sync
    + 'static
{
    const ZERO: Self;
    const ONE: Self;
    const TEN: Self;
    const MIN: Self;
    const MAX: Self;
    /// 该类型能表示的十进制位数。
    const MAX_DIGITS: u32;
    /// `SCALE` 的上限，即 `MAX_DIGITS - 1`：`10^SCALE` 必须仍在 `Self` 范围内。
    const MAX_SCALE: u32;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_neg(self) -> Option<Self>;
    fn saturating_abs(self) -> Self;
    /// 无损转成 `i128`（`i64` → `i128` 恒成功）。
    fn to_i128(self) -> i128;
    /// 从 `i128` 转换，超出范围返回 `None`。
    fn from_i128(value: i128) -> Option<Self>;
}

macro_rules! impl_backing {
    ($t:ty, $digits:expr) => {
        impl Backing for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TEN: Self = 10;
            const MIN: Self = <$t>::MIN;
            const MAX: Self = <$t>::MAX;
            const MAX_DIGITS: u32 = $digits;
            const MAX_SCALE: u32 = $digits - 1;

            #[inline]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                <$t>::checked_add(self, rhs)
            }

            #[inline]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                <$t>::checked_sub(self, rhs)
            }

            #[inline]
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                <$t>::checked_mul(self, rhs)
            }

            #[inline]
            fn checked_neg(self) -> Option<Self> {
                <$t>::checked_neg(self)
            }

            #[inline]
            fn saturating_abs(self) -> Self {
                <$t>::saturating_abs(self)
            }

            #[inline]
            fn to_i128(self) -> i128 {
                self as i128
            }

            #[inline]
            fn from_i128(value: i128) -> Option<Self> {
                value.try_into().ok()
            }
        }
    };
}

impl_backing!(i64, 19);
impl_backing!(i128, 39);

/// 常用标度常量。用作 const 泛型实参时必须加大括号，如 `Amount<{ scale::MONEY }>`。
pub mod scale {
    /// 数量：整股 / 整张 / 整手。
    pub const QUANTITY: u32 = 0;
    /// 金额：0.01 元（分）。资金、成交额、费用、冻结金额统一用这个口径。
    pub const MONEY: u32 = 2;
    /// 价格：0.0001 元。委托价、成交价、单位净值、摊薄成本价。
    pub const PRICE: u32 = 4;
    /// 中间精度：0.000001 元。费用率、汇率、按比例分摊的中间结果。
    pub const MICRO: u32 = 6;
}

/// 数量：整股 / 整张（`SCALE = 0`）。
pub type Quantity = Amount<{ scale::QUANTITY }>;
/// 金额：0.01 元（`SCALE = 2`）—— 资金、成交额、费用的唯一记账口径。
pub type Money = Amount<{ scale::MONEY }>;
/// 价格：0.0001 元（`SCALE = 4`）—— 委托价、成交价、单位净值。
pub type Price = Amount<{ scale::PRICE }>;
/// 精度更高的金额容器（`SCALE = 6`）：存放费用率等中间量，落账前用 `convert` 一次缩到 [`Money`]。
pub type MicroAmount = Amount<{ scale::MICRO }>;
/// 中间计算用的宽后备金额（`SCALE = 2`，`i128`）：汇总、中央账户累加不会溢出。
pub type WideMoney = Amount<{ scale::MONEY }, i128>;
/// 中间计算用的宽后备中间精度金额（`SCALE = 6`，`i128`）：累加费用、汇率换算。
pub type WideMicroAmount = Amount<{ scale::MICRO }, i128>;

/// 缩小标度（÷10^n）时的舍入策略。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Rounding {
    /// 四舍五入，遇 0.5 远离零 —— A 股 / 中国结算的常规口径，也是 [`Default`]。
    #[default]
    MidpointAwayFromZero,
    /// 去尾（截断，向零）。
    TowardZero,
    /// 进位：只要有零头就朝远离零的方向进一位。
    AwayFromZero,
    /// 向下取整（向 -∞）。
    Floor,
    /// 向上取整（向 +∞）。
    Ceil,
}

/// 把中间结果（`i128`）落成 `i64` 余额时的失败原因。
///
/// 三种情况都必须由业务显式处理，绝不能退化为截断或饱和。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettleError {
    /// `decimal_places` 大于该类型的标度 —— 调用错误（不可能舍入出更强的精度）。
    PrecisionTooHigh,
    /// 舍入到目标精度后为 0，但原值不是 0：金额不足最小记账单位。
    ///
    /// 典型场景是手续费 0.003 元舍入到「分」：这笔钱既不能凭空消失，也不该静默记成 0，
    /// 通常按业务规则进位到 0.01 或拒绝该笔指令。
    BelowMinimumUnit,
    /// 超出 `i64` 余额可表示范围（`i128` 中间结果落不回 `i64`）。
    OutOfRange,
}

/// 定点数：`SCALE` 位小数，内部是 [`Backing`] 整数计数。
///
/// 与裸整数互转请用 [`Amount::from_units`] / [`Amount::units`]，**不要**依赖 `From<i64>`：
/// 裸整数看不出单位是元、分还是最小单位，这正是资金代码最容易出错的地方。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Amount<const SCALE: u32, B: Backing = i64>(B);

impl<const SCALE: u32, B: Backing> Amount<SCALE, B> {
    /// 零。
    pub const ZERO: Self = Amount(B::ZERO);

    /// 小数位数（标度）：数值 = `units / 10^SCALE`。
    #[inline]
    #[must_use]
    pub const fn scale() -> u32 {
        SCALE
    }

    /// 该后备类型允许的最大标度：`i64` 为 18，`i128` 为 38。
    ///
    /// 判断某个 `SCALE` 是否可用：`Amount::<S>::scale() <= Amount::<S>::max_scale()`。
    #[inline]
    #[must_use]
    pub const fn max_scale() -> u32 {
        B::MAX_SCALE
    }

    /// 用最小单位构造。
    #[inline]
    #[must_use]
    pub const fn from_units(units: B) -> Self {
        Amount(units)
    }

    /// 取最小单位计数。
    #[inline]
    #[must_use]
    pub const fn units(self) -> B {
        self.0
    }

    /// 1 个整数单位（`SCALE = 4` 时即 1 元）对应多少个最小单位，即 `10^SCALE`。
    ///
    /// `SCALE > Self::max_scale()` 时返回 `None`（`i64` 上是 `SCALE >= 19`，`i128` 上是 `>= 39`）。
    #[must_use]
    pub fn units_per_one() -> Option<B> {
        let mut acc = B::ONE;
        let mut remaining = SCALE;
        while remaining > 0 {
            acc = acc.checked_mul(B::TEN)?;
            remaining -= 1;
        }
        Some(acc)
    }

    /// 用整数单位构造：`SCALE = 4` 时 `from_whole(3)` = 3 元；`SCALE = 0` 时 = 3 股。
    #[inline]
    #[must_use]
    pub fn from_whole(whole: B) -> Option<Self> {
        whole.checked_mul(Self::units_per_one()?).map(Amount)
    }

    /// 换算到另一个标度，需要舍入时按 `rounding` 处理。
    #[inline]
    #[must_use]
    pub fn rescale<const TO: u32>(self, rounding: Rounding) -> Option<Amount<TO, B>> {
        self.convert::<TO, B>(rounding)
    }

    /// **唯一的转换入口**：换标度 + 换后备类型，一次完成。
    ///
    /// 一次调用同时完成舍入与范围检查，避免「先换标度再换类型」引入的中间状态与双舍入。
    /// 缩小时必须给定 `rounding`；放大是精确的，`rounding` 被忽略。
    /// 结果超出 `B2` 可表示范围时返回 `None` —— 绝不截断、绝不饱和。
    #[inline]
    #[must_use]
    pub fn convert<const TO: u32, B2: Backing>(
        self,
        rounding: Rounding,
    ) -> Option<Amount<TO, B2>> {
        let value = self.0.to_i128();
        if TO == SCALE {
            return B2::from_i128(value).map(Amount::<TO, B2>::from_units);
        }
        let units = if TO > SCALE {
            value.checked_mul(pow10_i128(TO - SCALE)?)?
        } else {
            rescale_down(value, pow10_i128(SCALE - TO)?, rounding)?
        };
        B2::from_i128(units).map(Amount::<TO, B2>::from_units)
    }

    /// 只换后备类型（标度不变）：`i128` 中间结果 → `i64` 余额，或 `i64` → `i128` 加宽。
    ///
    /// 超出目标类型范围时返回 `None`。
    #[inline]
    #[must_use]
    pub fn with_backing<B2: Backing>(self) -> Option<Amount<SCALE, B2>> {
        self.convert::<SCALE, B2>(Rounding::TowardZero)
    }

    /// 放大标度（`TO >= SCALE`），精确无舍入；方向不对返回 `None`。
    #[inline]
    #[must_use]
    pub fn widen<const TO: u32>(self) -> Option<Amount<TO, B>> {
        if TO < SCALE {
            return None;
        }
        let widened = self.0.to_i128().checked_mul(pow10_i128(TO - SCALE)?)?;
        B::from_i128(widened).map(Amount::<TO, B>::from_units)
    }

    /// 缩小标度（`TO <= SCALE`），按 `rounding` 舍入；方向不对返回 `None`。
    #[inline]
    #[must_use]
    pub fn narrow<const TO: u32>(self, rounding: Rounding) -> Option<Amount<TO, B>> {
        if TO > SCALE {
            return None;
        }
        let units = if TO == SCALE {
            self.0.to_i128()
        } else {
            rescale_down(self.0.to_i128(), pow10_i128(SCALE - TO)?, rounding)?
        };
        B::from_i128(units).map(Amount::<TO, B>::from_units)
    }

    /// 舍入到指定小数位（`decimal_places <= SCALE`），保持自身标度不变。
    ///
    /// 资金出账到「分」即 `MicroAmount::round_to_dp(2, Rounding::MidpointAwayFromZero)`；
    /// 若目标容器本身就是「分」口径，直接用 [`Amount::convert`] 换成 [`Money`] 更省一步。
    #[inline]
    #[must_use]
    pub fn round_to_dp(self, decimal_places: u32, rounding: Rounding) -> Option<Self> {
        if decimal_places > SCALE {
            return None;
        }
        if decimal_places == SCALE {
            return Some(self);
        }
        let divisor = pow10_i128(SCALE - decimal_places)?;
        let rounded = rescale_down(self.0.to_i128(), divisor, rounding)?.checked_mul(divisor)?;
        B::from_i128(rounded).map(Amount)
    }

    /// 从 `Decimal`（单位：整数单位，如元）载入，超出精度时四舍五入。
    ///
    /// 走 `mantissa` / `scale` 的整数路径，不经过 `Decimal` 运算。
    #[inline]
    #[must_use]
    pub fn from_decimal(value: Decimal) -> Option<Self> {
        Self::from_decimal_with(value, Rounding::MidpointAwayFromZero)
    }

    /// 同 [`Amount::from_decimal`]，但可指定舍入策略。
    #[must_use]
    pub fn from_decimal_with(value: Decimal, rounding: Rounding) -> Option<Self> {
        let mantissa = value.mantissa();
        let from = value.scale();
        let units = if from <= SCALE {
            mantissa.checked_mul(pow10_i128(SCALE - from)?)?
        } else {
            rescale_down(mantissa, pow10_i128(from - SCALE)?, rounding)?
        };
        B::from_i128(units).map(Amount)
    }

    /// 转成 `Decimal`（单位：整数单位，如元），用于落库、对账、日志等冷路径。
    ///
    /// `Decimal` 的系数只有 96 位，超出其可表示范围时返回 `None`。
    #[inline]
    #[must_use]
    pub fn to_decimal(self) -> Option<Decimal> {
        Decimal::try_from_i128_with_scale(self.0.to_i128(), SCALE).ok()
    }

    /// 溢出返回 `None` 的加法（同标度才有意义）。
    #[inline]
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Amount)
    }

    /// 溢出返回 `None` 的减法。
    #[inline]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Amount)
    }

    /// 溢出返回 `None` 的取负。
    #[inline]
    #[must_use]
    pub fn checked_neg(self) -> Option<Self> {
        self.0.checked_neg().map(Amount)
    }

    /// 乘以整数倍率（如股数），标度不变。成交额请用 [`notional`]。
    #[inline]
    #[must_use]
    pub fn checked_mul_int(self, factor: B) -> Option<Self> {
        self.0.checked_mul(factor).map(Amount)
    }

    /// 加上一个宽后备（`i128`）的增量，结果落回 `Self` 的后备类型。
    ///
    /// 用于「`i64` 余额 ± `i128` 中间结果」：运算在 `i128` 中进行，
    /// 只有最终落回 `i64` 时才检查范围，溢出返回 `None` 而不是截断。
    #[inline]
    #[must_use]
    pub fn checked_add_wide<B2: Backing>(self, delta: Amount<SCALE, B2>) -> Option<Self> {
        let sum = self.0.to_i128().checked_add(delta.units().to_i128())?;
        B::from_i128(sum).map(Amount)
    }

    /// 减去一个宽后备（`i128`）的增量，见 [`Amount::checked_add_wide`]。
    #[inline]
    #[must_use]
    pub fn checked_sub_wide<B2: Backing>(self, delta: Amount<SCALE, B2>) -> Option<Self> {
        let difference = self
            .0
            .to_i128()
            .checked_sub(delta.units().to_i128())?;
        B::from_i128(difference).map(Amount)
    }

    /// 饱和加法：用于对账、统计等不希望 panic 的路径。
    #[inline]
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(sum) => sum,
            None if rhs.0 > B::ZERO => Self::max(),
            None => Self::min(),
        }
    }

    /// 饱和减法。
    #[inline]
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        match self.checked_sub(rhs) {
            Some(difference) => difference,
            None if rhs.0 < B::ZERO => Self::max(),
            None => Self::min(),
        }
    }

    /// 该后备类型能表示的最大值。
    #[inline]
    #[must_use]
    pub const fn max() -> Self {
        Amount(B::MAX)
    }

    /// 该后备类型能表示的最小值。
    #[inline]
    #[must_use]
    pub const fn min() -> Self {
        Amount(B::MIN)
    }

    /// 绝对值，最小值边界饱和。
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Amount(self.0.saturating_abs())
    }

    #[inline]
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == B::ZERO
    }

    #[inline]
    #[must_use]
    pub fn is_negative(self) -> bool {
        self.0 < B::ZERO
    }

    #[inline]
    #[must_use]
    pub fn is_positive(self) -> bool {
        self.0 > B::ZERO
    }
}

impl<const SCALE: u32> Amount<SCALE, i128> {
    /// 把 `i128` 中间结果**落账**到 `i64` 余额：一次舍入 + 一次范围检查。
    ///
    /// 与 [`Amount::convert`] 的区别是把「舍入后变成 0」也判为错误
    /// （见 [`SettleError::BelowMinimumUnit`]）：在清算里，钱既不能凭空消失，
    /// 也不该从 0.003 元静默变成 0 元，必须由业务决定进位还是拒单。
    ///
    /// 返回值**保持自身标度**，只是把小数位降到 `decimal_places`：
    /// `WideMicroAmount::settle(2, ..)` → `MicroAmount`（容器仍是 1e-6，但只保留到分）。
    /// 若落账目标是**不同标度**的容器（比如余额就是 [`Money`]），请用 [`Amount::convert`]
    /// 一步完成换标度 + 换后备，避免「先降精度再换容器」的二次舍入。
    #[inline]
    pub fn settle(
        self,
        decimal_places: u32,
        rounding: Rounding,
    ) -> Result<Amount<SCALE, i64>, SettleError> {
        let rounded = self
            .round_to_dp(decimal_places, rounding)
            .ok_or(SettleError::PrecisionTooHigh)?;
        if rounded.is_zero() && !self.is_zero() {
            return Err(SettleError::BelowMinimumUnit);
        }
        rounded.with_backing().ok_or(SettleError::OutOfRange)
    }
}

/// `10^n`，超出 `i128` 返回 `None`。
#[inline]
fn pow10_i128(exponent: u32) -> Option<i128> {
    10i128.checked_pow(exponent)
}

/// `value / divisor`，按 `rounding` 舍入，返回**目标标度下的商**。
fn rescale_down(value: i128, divisor: i128, rounding: Rounding) -> Option<i128> {
    let quotient = value / divisor;
    let remainder = value % divisor;
    if remainder == 0 {
        return Some(quotient);
    }
    let negative = value < 0;
    let magnitude = remainder.abs();
    let bump = match rounding {
        Rounding::TowardZero => false,
        Rounding::AwayFromZero => true,
        Rounding::Floor => negative,
        Rounding::Ceil => !negative,
        // |r| * 2 >= divisor，改写成 divisor - |r| 以避免 |r| * 2 溢出。
        Rounding::MidpointAwayFromZero => magnitude >= divisor - magnitude,
    };
    if !bump {
        return Some(quotient);
    }
    if negative {
        quotient.checked_sub(1)
    } else {
        quotient.checked_add(1)
    }
}

/// 成交金额 = 价格 × 数量，并换算到目标标度。
///
/// 乘积在 `i128` 中计算（价格 `SCALE = 4` 时 `0.0001 元 × 1 亿股` 已接近 `i64` 上限），
/// 再按 `rounding` 落到 `A` 位小数。
pub fn notional<const P: u32, const A: u32, B: Backing>(
    price: Amount<P>,
    quantity: Quantity,
    rounding: Rounding,
) -> Option<Amount<A, B>> {
    let product = price.units().to_i128().checked_mul(quantity.units() as i128)?;
    let scaled = Amount::<P, i128>::from_units(product).rescale::<A>(rounding)?;
    B::from_i128(scaled.units()).map(Amount::<A, B>::from_units)
}

impl<const SCALE: u32, B: Backing> fmt::Display for Amount<SCALE, B> {
    /// 按标度输出，保留 `SCALE` 位小数但至少 2 位、去掉多余的尾零
    /// （`0.0001`、`0.000001`、`3.00`、`0.00`）。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0.to_i128();
        let sign = if value < 0 { "-" } else { "" };
        let magnitude = value.unsigned_abs();
        if SCALE == 0 {
            return write!(f, "{sign}{magnitude}");
        }
        let Some(divisor) = 10u128.checked_pow(SCALE) else {
            return write!(f, "{sign}{magnitude}e-{SCALE}");
        };
        let whole = magnitude / divisor;
        let keep = if SCALE >= 2 { 2 } else { SCALE as usize };
        let mut fraction = format!("{:0width$}", magnitude % divisor, width = SCALE as usize);
        while fraction.len() > keep && fraction.ends_with('0') {
            fraction.pop();
        }
        write!(f, "{sign}{whole}.{fraction}")
    }
}

impl<const SCALE: u32, B: Backing> Add for Amount<SCALE, B> {
    type Output = Self;

    /// 溢出视为程序缺陷，直接 panic 而不是静默回绕；不想 panic 请用 [`Amount::checked_add`]。
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs).expect("Amount addition overflow")
    }
}

impl<const SCALE: u32, B: Backing> Sub for Amount<SCALE, B> {
    type Output = Self;

    /// 见 [`Add`] 的溢出说明。
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs).expect("Amount subtraction overflow")
    }
}

impl<const SCALE: u32, B: Backing> Neg for Amount<SCALE, B> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.checked_neg().expect("Amount negation overflow")
    }
}

impl<const SCALE: u32, B: Backing> AddAssign for Amount<SCALE, B> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const SCALE: u32, B: Backing> SubAssign for Amount<SCALE, B> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const SCALE: u32, B: Backing> Sum for Amount<SCALE, B> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a, const SCALE: u32, B: Backing> Sum<&'a Amount<SCALE, B>> for Amount<SCALE, B> {
    fn sum<I: Iterator<Item = &'a Amount<SCALE, B>>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, amount| acc + *amount)
    }
}

#[cfg(test)]
mod tests {
    use super::{scale::*, *};

    #[test]
    fn scale_decides_decimal_places() {
        assert_eq!(Amount::<2>::from_units(1).to_string(), "0.01");
        assert_eq!(Amount::<2>::from_units(100).to_string(), "1.00");
        assert_eq!(Amount::<4>::from_units(1).to_string(), "0.0001");
        assert_eq!(Amount::<4>::from_units(10_000).to_string(), "1.00");
        assert_eq!(Amount::<6>::from_units(1).to_string(), "0.000001");
        assert_eq!(Amount::<6>::from_units(1_200_000).to_string(), "1.20");
        assert_eq!(Amount::<0>::from_units(100).to_string(), "100");

        assert_eq!(Money::from_units(1).to_string(), "0.01");
        assert_eq!(Price::from_units(123_400).to_string(), "12.34");
        assert_eq!(Price::from_units(12_300).to_string(), "1.23");

        assert_eq!(Amount::<0>::units_per_one(), Some(1));
        assert_eq!(Amount::<2>::units_per_one(), Some(100));
        assert_eq!(Amount::<4>::units_per_one(), Some(10_000));
        assert_eq!(Amount::<6>::units_per_one(), Some(1_000_000));
        assert_eq!(Amount::<6, i128>::units_per_one(), Some(1_000_000));
    }

    #[test]
    fn scale_range_is_bounded_by_ten_pow_scale_fitting_the_backing() {
        // i64：十进制 19 位，但 10^19 > i64::MAX，所以 SCALE 上限是 18。
        assert_eq!(Amount::<4>::max_scale(), 18);
        assert_eq!(Amount::<18>::units_per_one(), Some(1_000_000_000_000_000_000));
        assert_eq!(Amount::<19>::units_per_one(), None);
        assert_eq!(Amount::<20>::units_per_one(), None);
        assert_eq!(Amount::<19>::from_whole(1), None);

        // i128：十进制 39 位，10^39 溢出，所以 SCALE 上限是 38。
        assert_eq!(Amount::<4, i128>::max_scale(), 38);
        assert_eq!(
            Amount::<38, i128>::units_per_one(),
            Some(100_000_000_000_000_000_000_000_000_000_000_000_000)
        );
        assert_eq!(Amount::<39, i128>::units_per_one(), None);

        // 超出上限的类型仍可做纯整数运算，只有「整数单位」的换算不可用。
        assert_eq!(Amount::<19>::from_units(7).units(), 7);
        assert_eq!(Amount::<19, i128>::from_units(7).units(), 7);
    }

    #[test]
    fn from_whole_scales_by_ten_pow_scale() {
        assert_eq!(Money::from_whole(3).unwrap().units(), 300);
        assert_eq!(Price::from_whole(3).unwrap().units(), 30_000);
        assert_eq!(MicroAmount::from_whole(3).unwrap().units(), 3_000_000);
        assert_eq!(Quantity::from_whole(300).unwrap().units(), 300);
        assert_eq!(Money::from_whole(i64::MAX), None);
    }

    #[test]
    fn widening_is_exact_and_narrowing_rounds() {
        let price = Money::from_units(1_234); // 12.34
        assert_eq!(price.to_string(), "12.34");

        let wide = price.widen::<6>().unwrap();
        assert_eq!(wide.units(), 12_340_000);
        assert_eq!(wide.to_string(), "12.34");
        assert_eq!(price.widen::<4>().unwrap().units(), 123_400);

        assert_eq!(wide.narrow::<2>(Rounding::MidpointAwayFromZero).unwrap(), price);

        // 0.000001 元缩到 4 位小数 → 0；0.000050 元 → 0.0001（四舍五入）。
        assert_eq!(Amount::<6>::from_units(1).narrow::<4>(Rounding::MidpointAwayFromZero).unwrap().units(), 0);
        assert_eq!(Amount::<6>::from_units(50).narrow::<4>(Rounding::MidpointAwayFromZero).unwrap().units(), 1);
        assert_eq!(Amount::<6>::from_units(49).narrow::<4>(Rounding::MidpointAwayFromZero).unwrap().units(), 0);
        assert_eq!(Amount::<6>::from_units(-50).narrow::<4>(Rounding::MidpointAwayFromZero).unwrap().units(), -1);

        // 标度方向在运行期校验（stable Rust 无法静态比较两个 const 泛型）。
        assert_eq!(price.narrow::<4>(Rounding::TowardZero), None);
        assert_eq!(wide.widen::<2>(), None);
    }

    #[test]
    fn round_to_dp_keeps_scale() {
        let price = Price::from_units(12_345); // 1.2345 元
        assert_eq!(price.round_to_dp(2, Rounding::MidpointAwayFromZero).unwrap().units(), 12_300);
        assert_eq!(price.round_to_dp(2, Rounding::TowardZero).unwrap().units(), 12_300);
        assert_eq!(price.round_to_dp(2, Rounding::Ceil).unwrap().units(), 12_400);
        assert_eq!(Price::from_units(-12_349).round_to_dp(2, Rounding::MidpointAwayFromZero).unwrap().units(), -12_300);
        assert_eq!(Price::from_units(-12_350).round_to_dp(2, Rounding::MidpointAwayFromZero).unwrap().units(), -12_400);
        assert_eq!(price.round_to_dp(4, Rounding::TowardZero), Some(price));
        assert_eq!(price.round_to_dp(5, Rounding::TowardZero), None);

        // 金额口径只有 2 位小数：`round_to_dp(2)` 是恒等，`(3)` 直接判为调用错误。
        assert_eq!(Money::from_units(12_345).round_to_dp(2, Rounding::TowardZero), Some(Money::from_units(12_345)));
        assert_eq!(Money::from_units(12_345).round_to_dp(3, Rounding::TowardZero), None);
        // 舍入到 0 位小数 = 取整到元。
        assert_eq!(Money::from_units(12_345).round_to_dp(0, Rounding::TowardZero).unwrap().units(), 12_300);
    }

    #[test]
    fn rescale_covers_all_rounding_modes() {
        let value = Money::from_units(105); // 1.05 → 1 位小数
        assert_eq!(value.rescale::<1>(Rounding::MidpointAwayFromZero).unwrap().units(), 11);
        assert_eq!(value.rescale::<1>(Rounding::TowardZero).unwrap().units(), 10);
        assert_eq!(value.rescale::<1>(Rounding::AwayFromZero).unwrap().units(), 11);
        assert_eq!(value.rescale::<1>(Rounding::Floor).unwrap().units(), 10);
        assert_eq!(value.rescale::<1>(Rounding::Ceil).unwrap().units(), 11);

        let negative = Money::from_units(-105);
        assert_eq!(negative.rescale::<1>(Rounding::MidpointAwayFromZero).unwrap().units(), -11);
        assert_eq!(negative.rescale::<1>(Rounding::TowardZero).unwrap().units(), -10);
        assert_eq!(negative.rescale::<1>(Rounding::Floor).unwrap().units(), -11);
        assert_eq!(negative.rescale::<1>(Rounding::Ceil).unwrap().units(), -10);

        assert_eq!(value.rescale::<2>(Rounding::TowardZero), Some(value));
        assert_eq!(value.rescale::<1>(Rounding::default()).unwrap().units(), 11);
    }

    #[test]
    fn decimal_interop_uses_mantissa_path() {
        assert_eq!(Price::from_decimal(Decimal::new(3_141, 3)).unwrap().units(), 31_410);
        assert_eq!(Price::from_decimal(Decimal::new(31_415, 4)).unwrap().units(), 31_415);
        assert_eq!(Price::from_decimal(Decimal::new(31_416, 4)).unwrap().units(), 31_416);
        // 4 位小数的 Decimal 落到 2 位小数的金额口径 → 一次舍入。
        assert_eq!(Money::from_decimal(Decimal::new(3_141, 3)).unwrap().units(), 314);
        // Decimal 的 4 位小数在 SCALE = 6 下是精确放大。
        assert_eq!(MicroAmount::from_decimal(Decimal::new(1, 4)).unwrap().units(), 100);
        assert_eq!(MicroAmount::from_decimal(Decimal::new(1, 6)).unwrap().units(), 1);
        assert_eq!(MicroAmount::from_units(12_345).to_decimal(), Some(Decimal::new(12_345, 6)));
        assert_eq!(Money::from_decimal(Decimal::new(i64::MAX, 0)), None);
    }

    #[test]
    fn arithmetic_and_sum() {
        let a = Money::from_units(15_000);
        let b = Money::from_units(25_000);

        assert_eq!((a + b).units(), 40_000);
        assert_eq!((b - a).units(), 10_000);
        assert_eq!((-a).units(), -15_000);
        assert_eq!(Money::from_units(i64::MAX).checked_add(Money::from_units(1)), None);
        assert_eq!(Money::from_units(i64::MIN).checked_sub(Money::from_units(1)), None);
        assert_eq!(Money::from_units(i64::MIN).checked_neg(), None);
        assert_eq!(Money::max().saturating_add(Money::from_units(1)), Money::max());
        assert_eq!(Money::min().saturating_sub(Money::from_units(1)), Money::min());
        assert_eq!(Money::from_units(-15_000).abs().units(), 15_000);

        assert_eq!([a, b].into_iter().sum::<Money>().units(), 40_000);
        assert_eq!([a, b].iter().sum::<Money>().units(), 40_000);

        let mut total = Money::ZERO;
        total += a;
        total -= b;
        assert_eq!(total.units(), -10_000);
        assert!(total.is_negative());
        assert!(a.is_positive());
        assert!(Money::ZERO.is_zero());
    }

    #[test]
    fn notional_converts_price_times_quantity() {
        let price = Price::from_units(123_400); // 12.3400 元
        let quantity = Quantity::from_units(100);

        let amount: Money = notional(price, quantity, Rounding::MidpointAwayFromZero).unwrap();
        assert_eq!(amount.units(), 123_400);
        assert_eq!(amount.to_string(), "1234.00");

        // 10 万分之 1 元的「仙价」× 1 亿股：乘积在 i128 中算，不会被 i64 截断。
        let tiny = Price::from_units(1);
        let huge = Quantity::from_units(100_000_000);
        assert_eq!(notional::<4, 4, i64>(tiny, huge, Rounding::TowardZero).unwrap().units(), 100_000_000);
    }

    #[test]
    fn wide_intermediate_converts_to_i64_balance_in_one_step() {
        // 1) 中间结果：微元标度累计，不舍入（0.4449 元）。
        let accrued = WideMicroAmount::from_units(444_900);
        assert_eq!(accrued.units(), 444_900);

        // 2) 落账：直接落到「分」（Money 口径），只舍入一次 → 0.44。
        //    目标类型已把 SCALE 与后备类型定死，泛型实参可以全省。
        let one_step: Money = accrued.convert(Rounding::MidpointAwayFromZero).unwrap();
        assert_eq!(one_step.units(), 44);
        assert_eq!(one_step.to_string(), "0.44");

        // 双舍入陷阱：先缩到 3 位（0.445）再缩到 2 位 → 0.45，差一分钱。
        let two_steps: Money = accrued
            .convert::<3, i64>(Rounding::MidpointAwayFromZero)
            .unwrap()
            .convert::<2, i64>(Rounding::MidpointAwayFromZero)
            .unwrap();
        assert_eq!(two_steps.units(), 45);
        assert_ne!(one_step, two_steps);

        // i64 → i128 加宽永远成功；i128 → i64 超出范围返回 None。
        assert_eq!(Money::from_units(1).with_backing::<i128>().unwrap().units(), 1);
        assert_eq!(WideMoney::from_units(i64::MAX as i128).with_backing::<i64>().unwrap().units(), i64::MAX);
        assert_eq!(WideMoney::from_units(i64::MAX as i128 + 1).with_backing::<i64>(), None);
        assert_eq!(WideMoney::from_units(i64::MIN as i128 - 1).with_backing::<i64>(), None);
    }

    #[test]
    fn settle_reports_why_the_wide_value_cannot_become_a_balance() {
        // 正常落账：标度 6 的 1.2345 元降到 2 位小数 = 1.23 元（容器仍是 1e-6）。
        assert_eq!(
            WideMicroAmount::from_units(1_234_500).settle(2, Rounding::MidpointAwayFromZero).unwrap().units(),
            1_230_000
        );
        // 1.2350 元走「中间值远离零」→ 1.24 元。
        assert_eq!(
            WideMicroAmount::from_units(1_235_000).settle(2, Rounding::MidpointAwayFromZero).unwrap().units(),
            1_240_000
        );
        // 舍入后为 0：0.003 元出账到「分」——必须显式处理，不能静默归零。
        assert_eq!(
            WideMicroAmount::from_units(3_000).settle(2, Rounding::MidpointAwayFromZero),
            Err(SettleError::BelowMinimumUnit)
        );
        // 真的为 0 则不算错误。
        assert_eq!(WideMicroAmount::ZERO.settle(2, Rounding::TowardZero).unwrap().units(), 0);
        // 超出 i64 余额范围：不缩精度时无法落回 i64。
        assert_eq!(
            WideMoney::from_units(i64::MAX as i128 + 1).settle(2, Rounding::TowardZero),
            Err(SettleError::OutOfRange)
        );
        // decimal_places 超过容器标度属调用错误（金额口径只有 2 位小数）。
        assert_eq!(
            WideMoney::from_units(1).settle(3, Rounding::TowardZero),
            Err(SettleError::PrecisionTooHigh)
        );
    }

    #[test]
    fn balance_updates_accept_wide_deltas_without_truncation() {
        let balance = Money::from_units(1_000_000); // 100 元
        let delta = WideMoney::from_units(50_000); // +5 元

        assert_eq!(balance.checked_add_wide(delta).unwrap().units(), 1_050_000);
        assert_eq!(balance.checked_sub_wide(delta).unwrap().units(), 950_000);

        // 增量本身超出 i64 时不会截断成垃圾值。
        let too_large = WideMoney::from_units(i64::MAX as i128 + 1);
        assert_eq!(balance.checked_add_wide(too_large), None);
        let too_small = WideMoney::from_units(i64::MIN as i128 - 1);
        assert_eq!(balance.checked_sub_wide(too_small), None);
    }

    #[test]
    fn backings_are_zero_cost() {
        assert_eq!(core::mem::size_of::<Amount<4>>(), 8);
        assert_eq!(core::mem::size_of::<Amount<0>>(), 8);
        assert_eq!(core::mem::size_of::<Amount<6, i128>>(), 16);
        assert_eq!(i64::MAX_DIGITS, 19);
        assert_eq!(i128::MAX_DIGITS, 39);
    }

    #[test]
    fn named_scale_constants_work_as_const_arguments() {
        let _: Amount<{ QUANTITY }> = Amount::from_units(1);
        let _: Amount<{ MONEY }> = Amount::from_units(1);
        let _: Amount<{ PRICE }> = Amount::from_units(1);
        let _: Amount<{ MICRO }> = Amount::from_units(1);
        let _: Money = Amount::from_units(1);
        let _: Price = Amount::from_units(1);

        // 锁死本系统的口径：金额 2 位小数、价格 4 位小数。
        assert_eq!(QUANTITY, 0);
        assert_eq!(MONEY, 2);
        assert_eq!(PRICE, 4);
        assert_eq!(MICRO, 6);
        assert_eq!(Money::scale(), 2);
        assert_eq!(Price::scale(), 4);
        assert_eq!(WideMoney::scale(), 2);
        assert_eq!(WideMicroAmount::scale(), 6);
        assert_eq!(core::mem::size_of::<Money>(), core::mem::size_of::<Price>());
    }
}
