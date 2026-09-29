//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_asac_posi_adjust_jour`（codegen：字段与列名源自 DDL，`SemageAsacPosiAdjustJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageAsacPosiAdjustJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub adjust_jour_no: i64,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub secu_acco: String,
    pub invest_type: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub before_curr_qty: Decimal,
    pub before_frozen_qty: Decimal,
    pub before_unfrozen_qty: Decimal,
    pub before_pre_settle_qty: Decimal,
    pub before_qty: Decimal,
    pub before_cost_amt: Decimal,
    pub before_realize_pandl: Decimal,
    pub before_sum_realize_pandl: Decimal,
    pub busi_flag: i32,
    pub adjust_qty: Decimal,
    pub deal_status: i32,
    pub curr_qty: Decimal,
    pub frozen_qty: Decimal,
    pub unfrozen_qty: Decimal,
    pub pre_settle_qty: Decimal,
    pub after_qty: Decimal,
    pub cost_amt: Decimal,
    pub realize_pandl: Decimal,
    pub sum_realize_pandl: Decimal,
    pub remark_info: String,
    pub source_row_id: i64,
}

/// `tb_semage_asac_posi_adjust_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageAsacPosiAdjustJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    AdjustJourNo,
    CoNo,
    PdNo,
    AsacNo,
    SecuAcco,
    InvestType,
    ExchNo,
    SecuCode,
    BeforeCurrQty,
    BeforeFrozenQty,
    BeforeUnfrozenQty,
    BeforePreSettleQty,
    BeforeQty,
    BeforeCostAmt,
    BeforeRealizePandl,
    BeforeSumRealizePandl,
    BusiFlag,
    AdjustQty,
    DealStatus,
    CurrQty,
    FrozenQty,
    UnfrozenQty,
    PreSettleQty,
    AfterQty,
    CostAmt,
    RealizePandl,
    SumRealizePandl,
    RemarkInfo,
    SourceRowId,
}

impl ColumnName for SemageAsacPosiAdjustJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::AdjustJourNo, Self::CoNo, Self::PdNo, Self::AsacNo, Self::SecuAcco, Self::InvestType, Self::ExchNo, Self::SecuCode, Self::BeforeCurrQty, Self::BeforeFrozenQty, Self::BeforeUnfrozenQty, Self::BeforePreSettleQty, Self::BeforeQty, Self::BeforeCostAmt, Self::BeforeRealizePandl, Self::BeforeSumRealizePandl, Self::BusiFlag, Self::AdjustQty, Self::DealStatus, Self::CurrQty, Self::FrozenQty, Self::UnfrozenQty, Self::PreSettleQty, Self::AfterQty, Self::CostAmt, Self::RealizePandl, Self::SumRealizePandl, Self::RemarkInfo, Self::SourceRowId];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::AdjustJourNo => "adjust_jour_no",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::SecuAcco => "secu_acco",
            Self::InvestType => "invest_type",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::BeforeCurrQty => "before_curr_qty",
            Self::BeforeFrozenQty => "before_frozen_qty",
            Self::BeforeUnfrozenQty => "before_unfrozen_qty",
            Self::BeforePreSettleQty => "before_pre_settle_qty",
            Self::BeforeQty => "before_qty",
            Self::BeforeCostAmt => "before_cost_amt",
            Self::BeforeRealizePandl => "before_realize_pandl",
            Self::BeforeSumRealizePandl => "before_sum_realize_pandl",
            Self::BusiFlag => "busi_flag",
            Self::AdjustQty => "adjust_qty",
            Self::DealStatus => "deal_status",
            Self::CurrQty => "curr_qty",
            Self::FrozenQty => "frozen_qty",
            Self::UnfrozenQty => "unfrozen_qty",
            Self::PreSettleQty => "pre_settle_qty",
            Self::AfterQty => "after_qty",
            Self::CostAmt => "cost_amt",
            Self::RealizePandl => "realize_pandl",
            Self::SumRealizePandl => "sum_realize_pandl",
            Self::RemarkInfo => "remark_info",
            Self::SourceRowId => "source_row_id",
        }
    }
}

impl DataTable for SemageAsacPosiAdjustJour {
    const ID: &'static str = "tb_semage_asac_posi_adjust_jour";
    type Column = SemageAsacPosiAdjustJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageAsacPosiAdjustJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageAsacPosiAdjustJour {}
