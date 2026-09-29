//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seswap_asac_capit_adjust_jour`（codegen：字段与列名源自 DDL，`SeswapAsacCapitAdjustJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeswapAsacCapitAdjustJour {
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
    pub settle_crncy_type: i32,
    pub before_curr_amt: Decimal,
    pub before_frozen_amt: Decimal,
    pub before_unfrozen_amt: Decimal,
    pub before_pre_settle_amt: Decimal,
    pub before_amt: Decimal,
    pub busi_flag: i32,
    pub adjust_amt: Decimal,
    pub deal_status: i32,
    pub curr_amt: Decimal,
    pub frozen_amt: Decimal,
    pub unfrozen_amt: Decimal,
    pub pre_settle_amt: Decimal,
    pub after_amt: Decimal,
    pub remark_info: String,
    pub source_row_id: i64,
}

/// `tb_seswap_asac_capit_adjust_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeswapAsacCapitAdjustJourColumn {
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
    SettleCrncyType,
    BeforeCurrAmt,
    BeforeFrozenAmt,
    BeforeUnfrozenAmt,
    BeforePreSettleAmt,
    BeforeAmt,
    BusiFlag,
    AdjustAmt,
    DealStatus,
    CurrAmt,
    FrozenAmt,
    UnfrozenAmt,
    PreSettleAmt,
    AfterAmt,
    RemarkInfo,
    SourceRowId,
}

impl ColumnName for SeswapAsacCapitAdjustJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::AdjustJourNo, Self::CoNo, Self::PdNo, Self::AsacNo, Self::SettleCrncyType, Self::BeforeCurrAmt, Self::BeforeFrozenAmt, Self::BeforeUnfrozenAmt, Self::BeforePreSettleAmt, Self::BeforeAmt, Self::BusiFlag, Self::AdjustAmt, Self::DealStatus, Self::CurrAmt, Self::FrozenAmt, Self::UnfrozenAmt, Self::PreSettleAmt, Self::AfterAmt, Self::RemarkInfo, Self::SourceRowId];
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
            Self::SettleCrncyType => "settle_crncy_type",
            Self::BeforeCurrAmt => "before_curr_amt",
            Self::BeforeFrozenAmt => "before_frozen_amt",
            Self::BeforeUnfrozenAmt => "before_unfrozen_amt",
            Self::BeforePreSettleAmt => "before_pre_settle_amt",
            Self::BeforeAmt => "before_amt",
            Self::BusiFlag => "busi_flag",
            Self::AdjustAmt => "adjust_amt",
            Self::DealStatus => "deal_status",
            Self::CurrAmt => "curr_amt",
            Self::FrozenAmt => "frozen_amt",
            Self::UnfrozenAmt => "unfrozen_amt",
            Self::PreSettleAmt => "pre_settle_amt",
            Self::AfterAmt => "after_amt",
            Self::RemarkInfo => "remark_info",
            Self::SourceRowId => "source_row_id",
        }
    }
}

impl DataTable for SeswapAsacCapitAdjustJour {
    const ID: &'static str = "tb_seswap_asac_capit_adjust_jour";
    type Column = SeswapAsacCapitAdjustJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeswapAsacCapitAdjustJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeswapAsacCapitAdjustJour {}
