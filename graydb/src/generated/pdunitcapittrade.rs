//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use account::amount::Money;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdmage_pd_unit_capit_trade`（codegen：字段与列名源自 DDL，`PdUnitCapitTradeColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdUnitCapitTrade {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_unit_no: i32,
    pub pd_unit_name: String,
    pub main_flag: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub trade_frozen_amt: Money,
    pub trade_unfrozen_amt: Money,
    pub buy_instr_amt: Money,
    pub buy_amt: Money,
    pub buy_strike_amt: Money,
    pub sell_instr_amt: Money,
    pub sell_amt: Money,
    pub sell_strike_amt: Money,
    pub fina_buy_instr_amt: Money,
    pub fina_buy_amt: Money,
    pub fina_buy_strike_amt: Money,
    pub loan_sell_instr_amt: Money,
    pub loan_sell_amt: Money,
    pub loan_sell_strike_amt: Money,
    pub loan_return_comm_amt: Money,
    pub loan_return_order_amt: Money,
    pub loan_return_strike_amt: Money,
    pub fina_return_comm_amt: Money,
    pub fina_return_order_amt: Money,
    pub fina_return_strike_amt: Money,
    pub return_strike_fee: Money,
    pub debt_strike_fee: Money,
    pub all_fee: Money,
    pub stamp_tax: Money,
    pub trans_fee: Money,
    pub brkage_fee: Money,
    #[serde(rename = "SEC_charges")]
    pub sec_charges: Money,
    pub other_fee: Money,
    pub trade_commis: Money,
    pub other_commis: Money,
    pub remark_info: String,
}

/// `tb_pdmage_pd_unit_capit_trade` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdUnitCapitTradeColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdUnitNo,
    PdUnitName,
    MainFlag,
    PdNo,
    AsacNo,
    SettleCrncyType,
    ExchCrncyType,
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
    RemarkInfo,
}

impl ColumnName for PdUnitCapitTradeColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdUnitNo, Self::PdUnitName, Self::MainFlag, Self::PdNo, Self::AsacNo, Self::SettleCrncyType, Self::ExchCrncyType, Self::TradeFrozenAmt, Self::TradeUnfrozenAmt, Self::BuyInstrAmt, Self::BuyAmt, Self::BuyStrikeAmt, Self::SellInstrAmt, Self::SellAmt, Self::SellStrikeAmt, Self::FinaBuyInstrAmt, Self::FinaBuyAmt, Self::FinaBuyStrikeAmt, Self::LoanSellInstrAmt, Self::LoanSellAmt, Self::LoanSellStrikeAmt, Self::LoanReturnCommAmt, Self::LoanReturnOrderAmt, Self::LoanReturnStrikeAmt, Self::FinaReturnCommAmt, Self::FinaReturnOrderAmt, Self::FinaReturnStrikeAmt, Self::ReturnStrikeFee, Self::DebtStrikeFee, Self::AllFee, Self::StampTax, Self::TransFee, Self::BrkageFee, Self::SECCharges, Self::OtherFee, Self::TradeCommis, Self::OtherCommis, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::PdUnitName => "pd_unit_name",
            Self::MainFlag => "main_flag",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
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
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for PdUnitCapitTrade {
    const ID: &'static str = "tb_pdmage_pd_unit_capit_trade";
    type Column = PdUnitCapitTradeColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdUnitCapitTradeColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdUnitCapitTrade {}
