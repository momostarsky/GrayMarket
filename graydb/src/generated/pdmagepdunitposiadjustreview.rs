//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdmage_pd_unit_posi_adjust_review`（codegen：字段与列名源自 DDL，`PdmagePdUnitPosiAdjustReviewColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdmagePdUnitPosiAdjustReview {
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
    pub pd_unit_no: i32,
    pub secu_acco: String,
    pub target_pd_no: i32,
    pub target_asac_no: i32,
    pub target_pdunit_no: i32,
    pub target_secu_acco: String,
    pub exch_no: i32,
    pub secu_code: String,
    pub invest_type: i32,
    pub target_invest_type: i32,
    pub busi_flag: i32,
    pub opor_no: i32,
    pub adjust_qty: Decimal,
    pub adjust_cost: Decimal,
    pub adjust_realize_pandl: Decimal,
    pub adjust_sum_realize_pandl: Decimal,
    pub effective_date: i32,
    pub expired_date: i32,
    pub deal_status: i32,
    pub remark_info: String,
    pub review_status: i32,
    pub reviewed_opor_no: i32,
    pub reviewed_date: i32,
    pub reviewed_time: i32,
    pub remark_info2: String,
}

/// `tb_pdmage_pd_unit_posi_adjust_review` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdmagePdUnitPosiAdjustReviewColumn {
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
    PdUnitNo,
    SecuAcco,
    TargetPdNo,
    TargetAsacNo,
    TargetPdunitNo,
    TargetSecuAcco,
    ExchNo,
    SecuCode,
    InvestType,
    TargetInvestType,
    BusiFlag,
    OporNo,
    AdjustQty,
    AdjustCost,
    AdjustRealizePandl,
    AdjustSumRealizePandl,
    EffectiveDate,
    ExpiredDate,
    DealStatus,
    RemarkInfo,
    ReviewStatus,
    ReviewedOporNo,
    ReviewedDate,
    ReviewedTime,
    RemarkInfo2,
}

impl ColumnName for PdmagePdUnitPosiAdjustReviewColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::AdjustJourNo, Self::CoNo, Self::PdNo, Self::AsacNo, Self::PdUnitNo, Self::SecuAcco, Self::TargetPdNo, Self::TargetAsacNo, Self::TargetPdunitNo, Self::TargetSecuAcco, Self::ExchNo, Self::SecuCode, Self::InvestType, Self::TargetInvestType, Self::BusiFlag, Self::OporNo, Self::AdjustQty, Self::AdjustCost, Self::AdjustRealizePandl, Self::AdjustSumRealizePandl, Self::EffectiveDate, Self::ExpiredDate, Self::DealStatus, Self::RemarkInfo, Self::ReviewStatus, Self::ReviewedOporNo, Self::ReviewedDate, Self::ReviewedTime, Self::RemarkInfo2];
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
            Self::PdUnitNo => "pd_unit_no",
            Self::SecuAcco => "secu_acco",
            Self::TargetPdNo => "target_pd_no",
            Self::TargetAsacNo => "target_asac_no",
            Self::TargetPdunitNo => "target_pdunit_no",
            Self::TargetSecuAcco => "target_secu_acco",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::InvestType => "invest_type",
            Self::TargetInvestType => "target_invest_type",
            Self::BusiFlag => "busi_flag",
            Self::OporNo => "opor_no",
            Self::AdjustQty => "adjust_qty",
            Self::AdjustCost => "adjust_cost",
            Self::AdjustRealizePandl => "adjust_realize_pandl",
            Self::AdjustSumRealizePandl => "adjust_sum_realize_pandl",
            Self::EffectiveDate => "effective_date",
            Self::ExpiredDate => "expired_date",
            Self::DealStatus => "deal_status",
            Self::RemarkInfo => "remark_info",
            Self::ReviewStatus => "review_status",
            Self::ReviewedOporNo => "reviewed_opor_no",
            Self::ReviewedDate => "reviewed_date",
            Self::ReviewedTime => "reviewed_time",
            Self::RemarkInfo2 => "remark_info2",
        }
    }
}

impl DataTable for PdmagePdUnitPosiAdjustReview {
    const ID: &'static str = "tb_pdmage_pd_unit_posi_adjust_review";
    type Column = PdmagePdUnitPosiAdjustReviewColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdmagePdUnitPosiAdjustReviewColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdmagePdUnitPosiAdjustReview {}
