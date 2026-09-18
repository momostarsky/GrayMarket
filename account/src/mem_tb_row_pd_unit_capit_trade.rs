use crate::amount::Money;

/// 产品单元资金交易记录行（对应 C# 的 MemTBRow_PdUnitCapitTrade）
///
/// 所有 `*_amt` / `*_fee` / `*_commis` 字段都是**金额**，统一用 [`Money`]（`Amount<2>`，
/// 0.01 元）——与数据库列的 2 位小数额度一致，因此内存值与库值之间是**无损映射**。
///
/// 这些字段是**运行累计值**（指令金额 → 委托金额 → 成交金额 逐级累加），
/// 累加器本身不需要更高精度：加数已是「分」，和必然仍是「分」的整数倍。
/// 需要更高精度的中间结果（费用率、按比例分摊）请用
/// [`WideMicroAmount`](crate::amount::WideMicroAmount) 累加，落账时用
/// [`Amount::convert`](crate::amount::Amount::convert) **一次**舍入到本结构。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MemTbRowPdUnitCapitTrade {
    /// 连续记录序号
    pub tb_rec_id: i32,
    /// 记录序号
    pub row_id: i64,
    /// 创建日期
    pub create_date: i32,
    /// 创建时间
    pub create_time: i32,
    /// 更新日期
    pub update_date: i32,
    /// 更新时间
    pub update_time: i32,
    /// 更新次数
    pub update_times: i32,
    /// 上次更新次数
    pub last_update_times: i32,
    /// 主推更新次数
    pub push_update_times: i32,
    /// 初始化日期
    pub init_date: i32,
    /// 产品单元编号
    pub pd_unit_no: i32,
    /// 产品编号
    pub pd_no: i32,
    /// 资产账户编号
    pub asac_no: i32,
    /// 主副标志
    pub main_flag: i32,
    /// 结算币种
    pub settle_crncy_type: i32,
    /// 交易币种
    pub exch_crncy_type: i32,
    /// 机构编号
    pub co_no: i32,
    /// 交易冻结金额
    pub trade_frozen_amt: Money,
    /// 交易解冻金额
    pub trade_unfrozen_amt: Money,
    /// 买入指令金额
    pub buy_instr_amt: Money,
    /// 买入金额
    pub buy_amt: Money,
    /// 买入成交金额
    pub buy_strike_amt: Money,
    /// 卖出指令金额
    pub sell_instr_amt: Money,
    /// 卖出金额
    pub sell_amt: Money,
    /// 卖出成交金额
    pub sell_strike_amt: Money,
    /// 融资买入指令金额
    pub fina_buy_instr_amt: Money,
    /// 融资买入金额
    pub fina_buy_amt: Money,
    /// 融资买入成交金额
    pub fina_buy_strike_amt: Money,
    /// 融券卖出指令金额
    pub loan_sell_instr_amt: Money,
    /// 融券卖出金额
    pub loan_sell_amt: Money,
    /// 融券卖出成交金额
    pub loan_sell_strike_amt: Money,
    /// 融券归还指令金额
    pub loan_return_comm_amt: Money,
    /// 融券归还订单金额
    pub loan_return_order_amt: Money,
    /// 融券归还成交金额
    pub loan_return_strike_amt: Money,
    /// 融资归还指令金额
    pub fina_return_comm_amt: Money,
    /// 融资归还订单金额
    pub fina_return_order_amt: Money,
    /// 融资归还成交金额
    pub fina_return_strike_amt: Money,
    /// 归还成交费用
    pub return_strike_fee: Money,
    /// 负债成交费用
    pub debt_strike_fee: Money,
    /// 全部费用
    pub all_fee: Money,
    /// 印花税
    pub stamp_tax: Money,
    /// 过户费
    pub trans_fee: Money,
    /// 经手费
    pub brkage_fee: Money,
    /// 证管费（原列名 SEC_charges）
    pub sec_charges: Money,
    /// 其他费用
    pub other_fee: Money,
    /// 交易佣金
    pub trade_commis: Money,
    /// 其他佣金
    pub other_commis: Money,
}
