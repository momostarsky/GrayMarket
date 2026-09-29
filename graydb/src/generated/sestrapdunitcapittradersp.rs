//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_sestra_pd_unit_capit_trade_rsp`（codegen：字段与列名源自 DDL，`SestraPdUnitCapitTradeRspColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SestraPdUnitCapitTradeRsp {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub source_row_id: i64,
    pub last_update_times: i32,
    pub pd_unit_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub main_flag: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub co_no: i32,
    pub trade_frozen_amt: Decimal,
    pub trade_unfrozen_amt: Decimal,
    pub buy_instr_amt: Decimal,
    pub buy_amt: Decimal,
    pub buy_strike_amt: Decimal,
    pub sell_instr_amt: Decimal,
    pub sell_amt: Decimal,
    pub sell_strike_amt: Decimal,
    pub fina_buy_instr_amt: Decimal,
    pub fina_buy_amt: Decimal,
    pub fina_buy_strike_amt: Decimal,
    pub loan_sell_instr_amt: Decimal,
    pub loan_sell_amt: Decimal,
    pub loan_sell_strike_amt: Decimal,
    pub loan_return_comm_amt: Decimal,
    pub loan_return_order_amt: Decimal,
    pub loan_return_strike_amt: Decimal,
    pub fina_return_comm_amt: Decimal,
    pub fina_return_order_amt: Decimal,
    pub fina_return_strike_amt: Decimal,
    pub return_strike_fee: Decimal,
    pub debt_strike_fee: Decimal,
    pub all_fee: Decimal,
    pub stamp_tax: Decimal,
    pub trans_fee: Decimal,
    pub brkage_fee: Decimal,
    #[serde(rename = "SEC_charges")]
    pub sec_charges: Decimal,
    pub other_fee: Decimal,
    pub trade_commis: Decimal,
    pub other_commis: Decimal,
}

/// `tb_sestra_pd_unit_capit_trade_rsp` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SestraPdUnitCapitTradeRspColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    SourceRowId,
    LastUpdateTimes,
    PdUnitNo,
    PdNo,
    AsacNo,
    MainFlag,
    SettleCrncyType,
    ExchCrncyType,
    CoNo,
    TradeFrozenAmt,
    TradeUnfrozenAmt,
    BuyInstrAmt,
    BuyAmt,
    BuyStrikeAmt,
    SellInstrAmt,
    SellAmt,
    SellStrikeAmt,
    FinaBuyInstrAmt,
    FinaBuyAmt,
    FinaBuyStrikeAmt,
    LoanSellInstrAmt,
    LoanSellAmt,
    LoanSellStrikeAmt,
    LoanReturnCommAmt,
    LoanReturnOrderAmt,
    LoanReturnStrikeAmt,
    FinaReturnCommAmt,
    FinaReturnOrderAmt,
    FinaReturnStrikeAmt,
    ReturnStrikeFee,
    DebtStrikeFee,
    AllFee,
    StampTax,
    TransFee,
    BrkageFee,
    SECCharges,
    OtherFee,
    TradeCommis,
    OtherCommis,
}

impl ColumnName for SestraPdUnitCapitTradeRspColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::SourceRowId, Self::LastUpdateTimes, Self::PdUnitNo, Self::PdNo, Self::AsacNo, Self::MainFlag, Self::SettleCrncyType, Self::ExchCrncyType, Self::CoNo, Self::TradeFrozenAmt, Self::TradeUnfrozenAmt, Self::BuyInstrAmt, Self::BuyAmt, Self::BuyStrikeAmt, Self::SellInstrAmt, Self::SellAmt, Self::SellStrikeAmt, Self::FinaBuyInstrAmt, Self::FinaBuyAmt, Self::FinaBuyStrikeAmt, Self::LoanSellInstrAmt, Self::LoanSellAmt, Self::LoanSellStrikeAmt, Self::LoanReturnCommAmt, Self::LoanReturnOrderAmt, Self::LoanReturnStrikeAmt, Self::FinaReturnCommAmt, Self::FinaReturnOrderAmt, Self::FinaReturnStrikeAmt, Self::ReturnStrikeFee, Self::DebtStrikeFee, Self::AllFee, Self::StampTax, Self::TransFee, Self::BrkageFee, Self::SECCharges, Self::OtherFee, Self::TradeCommis, Self::OtherCommis];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::SourceRowId => "source_row_id",
            Self::LastUpdateTimes => "last_update_times",
            Self::PdUnitNo => "pd_unit_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::MainFlag => "main_flag",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::CoNo => "co_no",
            Self::TradeFrozenAmt => "trade_frozen_amt",
            Self::TradeUnfrozenAmt => "trade_unfrozen_amt",
            Self::BuyInstrAmt => "buy_instr_amt",
            Self::BuyAmt => "buy_amt",
            Self::BuyStrikeAmt => "buy_strike_amt",
            Self::SellInstrAmt => "sell_instr_amt",
            Self::SellAmt => "sell_amt",
            Self::SellStrikeAmt => "sell_strike_amt",
            Self::FinaBuyInstrAmt => "fina_buy_instr_amt",
            Self::FinaBuyAmt => "fina_buy_amt",
            Self::FinaBuyStrikeAmt => "fina_buy_strike_amt",
            Self::LoanSellInstrAmt => "loan_sell_instr_amt",
            Self::LoanSellAmt => "loan_sell_amt",
            Self::LoanSellStrikeAmt => "loan_sell_strike_amt",
            Self::LoanReturnCommAmt => "loan_return_comm_amt",
            Self::LoanReturnOrderAmt => "loan_return_order_amt",
            Self::LoanReturnStrikeAmt => "loan_return_strike_amt",
            Self::FinaReturnCommAmt => "fina_return_comm_amt",
            Self::FinaReturnOrderAmt => "fina_return_order_amt",
            Self::FinaReturnStrikeAmt => "fina_return_strike_amt",
            Self::ReturnStrikeFee => "return_strike_fee",
            Self::DebtStrikeFee => "debt_strike_fee",
            Self::AllFee => "all_fee",
            Self::StampTax => "stamp_tax",
            Self::TransFee => "trans_fee",
            Self::BrkageFee => "brkage_fee",
            Self::SECCharges => "SEC_charges",
            Self::OtherFee => "other_fee",
            Self::TradeCommis => "trade_commis",
            Self::OtherCommis => "other_commis",
        }
    }
}

impl DataTable for SestraPdUnitCapitTradeRsp {
    const ID: &'static str = "tb_sestra_pd_unit_capit_trade_rsp";
    type Column = SestraPdUnitCapitTradeRspColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SestraPdUnitCapitTradeRspColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SestraPdUnitCapitTradeRsp {}
