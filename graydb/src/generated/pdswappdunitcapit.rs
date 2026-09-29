//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdswap_pd_unit_capit`（codegen：字段与列名源自 DDL，`PdswapPdUnitCapitColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdswapPdUnitCapit {
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
    pub bail_amt: Decimal,
    pub broker_bail_amt: Decimal,
    pub capt_margin: Decimal,
    pub bank_balance: Decimal,
    pub futu_bail: Decimal,
    pub futu_bail_capt: Decimal,
    pub pre_settle_amt: Decimal,
    pub remark_info: String,
}

/// `tb_pdswap_pd_unit_capit` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdswapPdUnitCapitColumn {
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
    BailAmt,
    BrokerBailAmt,
    CaptMargin,
    BankBalance,
    FutuBail,
    FutuBailCapt,
    PreSettleAmt,
    RemarkInfo,
}

impl ColumnName for PdswapPdUnitCapitColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdUnitNo, Self::PdUnitName, Self::MainFlag, Self::PdNo, Self::AsacNo, Self::SettleCrncyType, Self::BeginAmt, Self::CurrAmt, Self::AvailAmt, Self::FetchAmt, Self::LoanSellAmt, Self::FinaDebt, Self::PaybackBalance, Self::FrozenAmt, Self::UnfrozenAmt, Self::AvailAdjustAmt, Self::BailAmt, Self::BrokerBailAmt, Self::CaptMargin, Self::BankBalance, Self::FutuBail, Self::FutuBailCapt, Self::PreSettleAmt, Self::RemarkInfo];
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
            Self::BailAmt => "bail_amt",
            Self::BrokerBailAmt => "broker_bail_amt",
            Self::CaptMargin => "capt_margin",
            Self::BankBalance => "bank_balance",
            Self::FutuBail => "futu_bail",
            Self::FutuBailCapt => "futu_bail_capt",
            Self::PreSettleAmt => "pre_settle_amt",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for PdswapPdUnitCapit {
    const ID: &'static str = "tb_pdswap_pd_unit_capit";
    type Column = PdswapPdUnitCapitColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdswapPdUnitCapitColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdswapPdUnitCapit {}
