//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_sestra_pd_unit_posi_trade_rsp`（codegen：字段与列名源自 DDL，`SestraPdUnitPosiTradeRspColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SestraPdUnitPosiTradeRsp {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub last_update_times: i32,
    pub pd_unit_no: i32,
    pub secu_type: i32,
    pub invest_type: i32,
    pub asset_type: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub main_flag: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub co_no: i32,
    pub secu_acco: String,
    pub trade_frozen_qty: Decimal,
    pub trade_unfrozen_qty: Decimal,
    pub net_trade_frozen_qty: Decimal,
    pub trade_net_qty: Decimal,
    pub buy_instr_qty: Decimal,
    pub buy_instr_amt: Decimal,
    pub buy_qty: Decimal,
    pub buy_amt: Decimal,
    pub fina_buy_strike_qty: Decimal,
    pub fina_buy_strike_amt: Decimal,
    pub buy_strike_qty: Decimal,
    pub buy_strike_amt: Decimal,
    pub sell_instr_qty: Decimal,
    pub sell_instr_amt: Decimal,
    pub sell_qty: Decimal,
    pub sell_amt: Decimal,
    pub sell_strike_qty: Decimal,
    pub sell_strike_amt: Decimal,
    pub buy_strike_unfrozen_qty: Decimal,
    pub loan_sell_instr_qty: Decimal,
    pub loan_sell_instr_amt: Decimal,
    pub loan_sell_qty: Decimal,
    pub loan_sell_amt: Decimal,
    pub loan_sell_strike_qty: Decimal,
    pub loan_sell_strike_amt: Decimal,
    pub secuback_amount: Decimal,
    pub used_loan_qty: Decimal,
    pub loan_return_strike_amt: Decimal,
}

/// `tb_sestra_pd_unit_posi_trade_rsp` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SestraPdUnitPosiTradeRspColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    LastUpdateTimes,
    PdUnitNo,
    SecuType,
    InvestType,
    AssetType,
    PdNo,
    AsacNo,
    MainFlag,
    ExchNo,
    SecuCode,
    CoNo,
    SecuAcco,
    TradeFrozenQty,
    TradeUnfrozenQty,
    NetTradeFrozenQty,
    TradeNetQty,
    BuyInstrQty,
    BuyInstrAmt,
    BuyQty,
    BuyAmt,
    FinaBuyStrikeQty,
    FinaBuyStrikeAmt,
    BuyStrikeQty,
    BuyStrikeAmt,
    SellInstrQty,
    SellInstrAmt,
    SellQty,
    SellAmt,
    SellStrikeQty,
    SellStrikeAmt,
    BuyStrikeUnfrozenQty,
    LoanSellInstrQty,
    LoanSellInstrAmt,
    LoanSellQty,
    LoanSellAmt,
    LoanSellStrikeQty,
    LoanSellStrikeAmt,
    SecubackAmount,
    UsedLoanQty,
    LoanReturnStrikeAmt,
}

impl ColumnName for SestraPdUnitPosiTradeRspColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::LastUpdateTimes, Self::PdUnitNo, Self::SecuType, Self::InvestType, Self::AssetType, Self::PdNo, Self::AsacNo, Self::MainFlag, Self::ExchNo, Self::SecuCode, Self::CoNo, Self::SecuAcco, Self::TradeFrozenQty, Self::TradeUnfrozenQty, Self::NetTradeFrozenQty, Self::TradeNetQty, Self::BuyInstrQty, Self::BuyInstrAmt, Self::BuyQty, Self::BuyAmt, Self::FinaBuyStrikeQty, Self::FinaBuyStrikeAmt, Self::BuyStrikeQty, Self::BuyStrikeAmt, Self::SellInstrQty, Self::SellInstrAmt, Self::SellQty, Self::SellAmt, Self::SellStrikeQty, Self::SellStrikeAmt, Self::BuyStrikeUnfrozenQty, Self::LoanSellInstrQty, Self::LoanSellInstrAmt, Self::LoanSellQty, Self::LoanSellAmt, Self::LoanSellStrikeQty, Self::LoanSellStrikeAmt, Self::SecubackAmount, Self::UsedLoanQty, Self::LoanReturnStrikeAmt];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::LastUpdateTimes => "last_update_times",
            Self::PdUnitNo => "pd_unit_no",
            Self::SecuType => "secu_type",
            Self::InvestType => "invest_type",
            Self::AssetType => "asset_type",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::MainFlag => "main_flag",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::CoNo => "co_no",
            Self::SecuAcco => "secu_acco",
            Self::TradeFrozenQty => "trade_frozen_qty",
            Self::TradeUnfrozenQty => "trade_unfrozen_qty",
            Self::NetTradeFrozenQty => "net_trade_frozen_qty",
            Self::TradeNetQty => "trade_net_qty",
            Self::BuyInstrQty => "buy_instr_qty",
            Self::BuyInstrAmt => "buy_instr_amt",
            Self::BuyQty => "buy_qty",
            Self::BuyAmt => "buy_amt",
            Self::FinaBuyStrikeQty => "fina_buy_strike_qty",
            Self::FinaBuyStrikeAmt => "fina_buy_strike_amt",
            Self::BuyStrikeQty => "buy_strike_qty",
            Self::BuyStrikeAmt => "buy_strike_amt",
            Self::SellInstrQty => "sell_instr_qty",
            Self::SellInstrAmt => "sell_instr_amt",
            Self::SellQty => "sell_qty",
            Self::SellAmt => "sell_amt",
            Self::SellStrikeQty => "sell_strike_qty",
            Self::SellStrikeAmt => "sell_strike_amt",
            Self::BuyStrikeUnfrozenQty => "buy_strike_unfrozen_qty",
            Self::LoanSellInstrQty => "loan_sell_instr_qty",
            Self::LoanSellInstrAmt => "loan_sell_instr_amt",
            Self::LoanSellQty => "loan_sell_qty",
            Self::LoanSellAmt => "loan_sell_amt",
            Self::LoanSellStrikeQty => "loan_sell_strike_qty",
            Self::LoanSellStrikeAmt => "loan_sell_strike_amt",
            Self::SecubackAmount => "secuback_amount",
            Self::UsedLoanQty => "used_loan_qty",
            Self::LoanReturnStrikeAmt => "loan_return_strike_amt",
        }
    }
}

impl DataTable for SestraPdUnitPosiTradeRsp {
    const ID: &'static str = "tb_sestra_pd_unit_posi_trade_rsp";
    type Column = SestraPdUnitPosiTradeRspColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SestraPdUnitPosiTradeRspColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SestraPdUnitPosiTradeRsp {}
