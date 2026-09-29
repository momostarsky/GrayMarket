//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_sestra_pd_unit_capit`（codegen：字段与列名源自 DDL，`SestraPdUnitCapitColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SestraPdUnitCapit {
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
    pub settle_crncy_type: i32,
    pub co_no: i32,
    pub begin_amt: Decimal,
    pub curr_amt: Decimal,
    pub avail_amt: Decimal,
    pub fetch_amt: Decimal,
    pub loan_sell_amt: Decimal,
    pub fina_debt: Decimal,
    pub payback_balance: Decimal,
    pub frozen_amt: Decimal,
    pub unfrozen_amt: Decimal,
    pub avail_adjust_amt: Decimal,
    pub instr_avail_amt: Decimal,
    pub hk_avail_amt: Decimal,
    pub hk_instr_avail_amt: Decimal,
    pub avail_bail: Decimal,
    pub instr_avail_margin: Decimal,
    pub bank_balance: Decimal,
    pub futu_bail: Decimal,
    pub futu_bail_capt: Decimal,
    #[serde(rename = "T1_avail_amt")]
    pub t1_avail_amt: Decimal,
    #[serde(rename = "T2_avail_amt")]
    pub t2_avail_amt: Decimal,
    #[serde(rename = "T1_instr_avail_amt")]
    pub t1_instr_avail_amt: Decimal,
    #[serde(rename = "T2_instr_avail_amt")]
    pub t2_instr_avail_amt: Decimal,
}

/// `tb_sestra_pd_unit_capit` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SestraPdUnitCapitColumn {
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
    SettleCrncyType,
    CoNo,
    BeginAmt,
    CurrAmt,
    AvailAmt,
    FetchAmt,
    LoanSellAmt,
    FinaDebt,
    PaybackBalance,
    FrozenAmt,
    UnfrozenAmt,
    AvailAdjustAmt,
    InstrAvailAmt,
    HkAvailAmt,
    HkInstrAvailAmt,
    AvailBail,
    InstrAvailMargin,
    BankBalance,
    FutuBail,
    FutuBailCapt,
    T1AvailAmt,
    T2AvailAmt,
    T1InstrAvailAmt,
    T2InstrAvailAmt,
}

impl ColumnName for SestraPdUnitCapitColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::SourceRowId, Self::LastUpdateTimes, Self::PdUnitNo, Self::PdNo, Self::AsacNo, Self::SettleCrncyType, Self::CoNo, Self::BeginAmt, Self::CurrAmt, Self::AvailAmt, Self::FetchAmt, Self::LoanSellAmt, Self::FinaDebt, Self::PaybackBalance, Self::FrozenAmt, Self::UnfrozenAmt, Self::AvailAdjustAmt, Self::InstrAvailAmt, Self::HkAvailAmt, Self::HkInstrAvailAmt, Self::AvailBail, Self::InstrAvailMargin, Self::BankBalance, Self::FutuBail, Self::FutuBailCapt, Self::T1AvailAmt, Self::T2AvailAmt, Self::T1InstrAvailAmt, Self::T2InstrAvailAmt];
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
            Self::SettleCrncyType => "settle_crncy_type",
            Self::CoNo => "co_no",
            Self::BeginAmt => "begin_amt",
            Self::CurrAmt => "curr_amt",
            Self::AvailAmt => "avail_amt",
            Self::FetchAmt => "fetch_amt",
            Self::LoanSellAmt => "loan_sell_amt",
            Self::FinaDebt => "fina_debt",
            Self::PaybackBalance => "payback_balance",
            Self::FrozenAmt => "frozen_amt",
            Self::UnfrozenAmt => "unfrozen_amt",
            Self::AvailAdjustAmt => "avail_adjust_amt",
            Self::InstrAvailAmt => "instr_avail_amt",
            Self::HkAvailAmt => "hk_avail_amt",
            Self::HkInstrAvailAmt => "hk_instr_avail_amt",
            Self::AvailBail => "avail_bail",
            Self::InstrAvailMargin => "instr_avail_margin",
            Self::BankBalance => "bank_balance",
            Self::FutuBail => "futu_bail",
            Self::FutuBailCapt => "futu_bail_capt",
            Self::T1AvailAmt => "T1_avail_amt",
            Self::T2AvailAmt => "T2_avail_amt",
            Self::T1InstrAvailAmt => "T1_instr_avail_amt",
            Self::T2InstrAvailAmt => "T2_instr_avail_amt",
        }
    }
}

impl DataTable for SestraPdUnitCapit {
    const ID: &'static str = "tb_sestra_pd_unit_capit";
    type Column = SestraPdUnitCapitColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SestraPdUnitCapitColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SestraPdUnitCapit {}
