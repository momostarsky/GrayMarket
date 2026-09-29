//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_sestra_pd_unit_posi`（codegen：字段与列名源自 DDL，`SestraPdUnitPosiColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SestraPdUnitPosi {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub last_update_times: i32,
    pub pd_unit_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub co_no: i32,
    pub secu_type: i32,
    pub invest_type: i32,
    pub asset_type: i32,
    pub secu_acco: String,
    pub forbid_order_dir: String,
    pub buy_mode: i32,
    pub sell_mode: i32,
    pub avail_qty: Decimal,
    pub begin_qty: Decimal,
    pub curr_qty: Decimal,
    pub cost_price: Decimal,
    pub cost_amt: Decimal,
    pub begin_max_loan_amount: Decimal,
    pub curr_max_loan_amount: Decimal,
    pub begin_shortsell_quota: Decimal,
    pub curr_shortsell_quota: Decimal,
    pub begin_used_loan_qty: Decimal,
    pub curr_used_loan_qty: Decimal,
    pub frozen_qty: Decimal,
    pub unfrozen_qty: Decimal,
    pub avail_adjust_qty: Decimal,
    pub lock_secu_qty: Decimal,
    pub pupil_flag: i32,
    pub online_new_share_wait_qty: Decimal,
    pub offline_new_share_wait_qty: Decimal,
    pub dividend_qty: Decimal,
    pub pla_qty: Decimal,
    pub impawn_qty: Decimal,
    pub realize_pandl: Decimal,
    pub sum_realize_pandl: Decimal,
    #[serde(rename = "T1_avail_qty")]
    pub t1_avail_qty: Decimal,
    pub instr_avail_qty: Decimal,
    #[serde(rename = "T1_instr_avail_qty")]
    pub t1_instr_avail_qty: Decimal,
    pub intrst_cost_amt: Decimal,
    pub buy_fee: Decimal,
    pub sell_fee: Decimal,
}

/// `tb_sestra_pd_unit_posi` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SestraPdUnitPosiColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    LastUpdateTimes,
    PdUnitNo,
    PdNo,
    AsacNo,
    ExchNo,
    SecuCode,
    SecuName,
    CoNo,
    SecuType,
    InvestType,
    AssetType,
    SecuAcco,
    ForbidOrderDir,
    BuyMode,
    SellMode,
    AvailQty,
    BeginQty,
    CurrQty,
    CostPrice,
    CostAmt,
    BeginMaxLoanAmount,
    CurrMaxLoanAmount,
    BeginShortsellQuota,
    CurrShortsellQuota,
    BeginUsedLoanQty,
    CurrUsedLoanQty,
    FrozenQty,
    UnfrozenQty,
    AvailAdjustQty,
    LockSecuQty,
    PupilFlag,
    OnlineNewShareWaitQty,
    OfflineNewShareWaitQty,
    DividendQty,
    PlaQty,
    ImpawnQty,
    RealizePandl,
    SumRealizePandl,
    T1AvailQty,
    InstrAvailQty,
    T1InstrAvailQty,
    IntrstCostAmt,
    BuyFee,
    SellFee,
}

impl ColumnName for SestraPdUnitPosiColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::LastUpdateTimes, Self::PdUnitNo, Self::PdNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::CoNo, Self::SecuType, Self::InvestType, Self::AssetType, Self::SecuAcco, Self::ForbidOrderDir, Self::BuyMode, Self::SellMode, Self::AvailQty, Self::BeginQty, Self::CurrQty, Self::CostPrice, Self::CostAmt, Self::BeginMaxLoanAmount, Self::CurrMaxLoanAmount, Self::BeginShortsellQuota, Self::CurrShortsellQuota, Self::BeginUsedLoanQty, Self::CurrUsedLoanQty, Self::FrozenQty, Self::UnfrozenQty, Self::AvailAdjustQty, Self::LockSecuQty, Self::PupilFlag, Self::OnlineNewShareWaitQty, Self::OfflineNewShareWaitQty, Self::DividendQty, Self::PlaQty, Self::ImpawnQty, Self::RealizePandl, Self::SumRealizePandl, Self::T1AvailQty, Self::InstrAvailQty, Self::T1InstrAvailQty, Self::IntrstCostAmt, Self::BuyFee, Self::SellFee];
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
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::CoNo => "co_no",
            Self::SecuType => "secu_type",
            Self::InvestType => "invest_type",
            Self::AssetType => "asset_type",
            Self::SecuAcco => "secu_acco",
            Self::ForbidOrderDir => "forbid_order_dir",
            Self::BuyMode => "buy_mode",
            Self::SellMode => "sell_mode",
            Self::AvailQty => "avail_qty",
            Self::BeginQty => "begin_qty",
            Self::CurrQty => "curr_qty",
            Self::CostPrice => "cost_price",
            Self::CostAmt => "cost_amt",
            Self::BeginMaxLoanAmount => "begin_max_loan_amount",
            Self::CurrMaxLoanAmount => "curr_max_loan_amount",
            Self::BeginShortsellQuota => "begin_shortsell_quota",
            Self::CurrShortsellQuota => "curr_shortsell_quota",
            Self::BeginUsedLoanQty => "begin_used_loan_qty",
            Self::CurrUsedLoanQty => "curr_used_loan_qty",
            Self::FrozenQty => "frozen_qty",
            Self::UnfrozenQty => "unfrozen_qty",
            Self::AvailAdjustQty => "avail_adjust_qty",
            Self::LockSecuQty => "lock_secu_qty",
            Self::PupilFlag => "pupil_flag",
            Self::OnlineNewShareWaitQty => "online_new_share_wait_qty",
            Self::OfflineNewShareWaitQty => "offline_new_share_wait_qty",
            Self::DividendQty => "dividend_qty",
            Self::PlaQty => "pla_qty",
            Self::ImpawnQty => "impawn_qty",
            Self::RealizePandl => "realize_pandl",
            Self::SumRealizePandl => "sum_realize_pandl",
            Self::T1AvailQty => "T1_avail_qty",
            Self::InstrAvailQty => "instr_avail_qty",
            Self::T1InstrAvailQty => "T1_instr_avail_qty",
            Self::IntrstCostAmt => "intrst_cost_amt",
            Self::BuyFee => "buy_fee",
            Self::SellFee => "sell_fee",
        }
    }
}

impl DataTable for SestraPdUnitPosi {
    const ID: &'static str = "tb_sestra_pd_unit_posi";
    type Column = SestraPdUnitPosiColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SestraPdUnitPosiColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SestraPdUnitPosi {}
