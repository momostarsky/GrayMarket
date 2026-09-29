//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_out_credit_asset`（codegen：字段与列名源自 DDL，`SemageOutCreditAssetColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageOutCreditAsset {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub settle_crncy_type: i32,
    pub total_debit: Decimal,
    pub funddebit: Decimal,
    pub stock_debit: Decimal,
    pub assure_ratio: Decimal,
    pub avail_bail: Decimal,
    pub avail_amt: Decimal,
    pub converted_margin: Decimal,
    pub fina_converted_pandl: Decimal,
    pub loan_converted_pandl: Decimal,
    pub loan_sell_amt: Decimal,
    pub loan_sell_amt_remain_amt: Decimal,
    pub loan_sell_amt_used_amt: Decimal,
    pub fina_capt_margin: Decimal,
    pub fina_order_capt_margin: Decimal,
    pub loan_capt_margin: Decimal,
    pub loan_order_capt_margin: Decimal,
    pub debt_interest: Decimal,
    pub debt_fee: Decimal,
    pub fina_limit_max: Decimal,
    pub finance_quota: Decimal,
    pub loan_limit_max: Decimal,
    pub shortsell_quota: Decimal,
}

/// `tb_semage_out_credit_asset` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageOutCreditAssetColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdNo,
    AsacNo,
    SettleCrncyType,
    TotalDebit,
    Funddebit,
    StockDebit,
    AssureRatio,
    AvailBail,
    AvailAmt,
    ConvertedMargin,
    FinaConvertedPandl,
    LoanConvertedPandl,
    LoanSellAmt,
    LoanSellAmtRemainAmt,
    LoanSellAmtUsedAmt,
    FinaCaptMargin,
    FinaOrderCaptMargin,
    LoanCaptMargin,
    LoanOrderCaptMargin,
    DebtInterest,
    DebtFee,
    FinaLimitMax,
    FinanceQuota,
    LoanLimitMax,
    ShortsellQuota,
}

impl ColumnName for SemageOutCreditAssetColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdNo, Self::AsacNo, Self::SettleCrncyType, Self::TotalDebit, Self::Funddebit, Self::StockDebit, Self::AssureRatio, Self::AvailBail, Self::AvailAmt, Self::ConvertedMargin, Self::FinaConvertedPandl, Self::LoanConvertedPandl, Self::LoanSellAmt, Self::LoanSellAmtRemainAmt, Self::LoanSellAmtUsedAmt, Self::FinaCaptMargin, Self::FinaOrderCaptMargin, Self::LoanCaptMargin, Self::LoanOrderCaptMargin, Self::DebtInterest, Self::DebtFee, Self::FinaLimitMax, Self::FinanceQuota, Self::LoanLimitMax, Self::ShortsellQuota];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::TotalDebit => "total_debit",
            Self::Funddebit => "funddebit",
            Self::StockDebit => "stock_debit",
            Self::AssureRatio => "assure_ratio",
            Self::AvailBail => "avail_bail",
            Self::AvailAmt => "avail_amt",
            Self::ConvertedMargin => "converted_margin",
            Self::FinaConvertedPandl => "fina_converted_pandl",
            Self::LoanConvertedPandl => "loan_converted_pandl",
            Self::LoanSellAmt => "loan_sell_amt",
            Self::LoanSellAmtRemainAmt => "loan_sell_amt_remain_amt",
            Self::LoanSellAmtUsedAmt => "loan_sell_amt_used_amt",
            Self::FinaCaptMargin => "fina_capt_margin",
            Self::FinaOrderCaptMargin => "fina_order_capt_margin",
            Self::LoanCaptMargin => "loan_capt_margin",
            Self::LoanOrderCaptMargin => "loan_order_capt_margin",
            Self::DebtInterest => "debt_interest",
            Self::DebtFee => "debt_fee",
            Self::FinaLimitMax => "fina_limit_max",
            Self::FinanceQuota => "finance_quota",
            Self::LoanLimitMax => "loan_limit_max",
            Self::ShortsellQuota => "shortsell_quota",
        }
    }
}

impl DataTable for SemageOutCreditAsset {
    const ID: &'static str = "tb_semage_out_credit_asset";
    type Column = SemageOutCreditAssetColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageOutCreditAssetColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageOutCreditAsset {}
