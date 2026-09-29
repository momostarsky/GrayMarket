//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdmage_pd_asset`（codegen：字段与列名源自 DDL，`PdmagePdAssetColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdmagePdAsset {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub settle_crncy_type: i32,
    pub begin_evalu_nav_asset: Decimal,
    pub curr_evalu_nav_asset: Decimal,
    pub begin_nav_asset: Decimal,
    pub curr_nav_asset: Decimal,
    pub cash_asset: Decimal,
    pub secu_asset: Decimal,
    pub hk_thrgh_secu_asset: Decimal,
    pub fund_asset: Decimal,
    pub bond_asset: Decimal,
    pub futu_asset: Decimal,
    pub repo_asset: Decimal,
    pub other_asset: Decimal,
    pub out_nav_asset: Decimal,
    pub secu_cash_asset: Decimal,
    pub futu_cash_asset: Decimal,
    pub sh_asecu_asset: Decimal,
    pub sz_asecu_asset: Decimal,
    pub bj_asecu_asset: Decimal,
    pub sh_hk_secu_asset: Decimal,
    pub sz_hk_secu_asset: Decimal,
    pub money_fund_asset: Decimal,
    pub not_money_fund_asset: Decimal,
    pub fina_debt: Decimal,
    pub loan_debt: Decimal,
    pub futu_long_market_value: Decimal,
    pub futu_short_market_value: Decimal,
}

/// `tb_pdmage_pd_asset` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdmagePdAssetColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdNo,
    SettleCrncyType,
    BeginEvaluNavAsset,
    CurrEvaluNavAsset,
    BeginNavAsset,
    CurrNavAsset,
    CashAsset,
    SecuAsset,
    HkThrghSecuAsset,
    FundAsset,
    BondAsset,
    FutuAsset,
    RepoAsset,
    OtherAsset,
    OutNavAsset,
    SecuCashAsset,
    FutuCashAsset,
    ShAsecuAsset,
    SzAsecuAsset,
    BjAsecuAsset,
    ShHkSecuAsset,
    SzHkSecuAsset,
    MoneyFundAsset,
    NotMoneyFundAsset,
    FinaDebt,
    LoanDebt,
    FutuLongMarketValue,
    FutuShortMarketValue,
}

impl ColumnName for PdmagePdAssetColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdNo, Self::SettleCrncyType, Self::BeginEvaluNavAsset, Self::CurrEvaluNavAsset, Self::BeginNavAsset, Self::CurrNavAsset, Self::CashAsset, Self::SecuAsset, Self::HkThrghSecuAsset, Self::FundAsset, Self::BondAsset, Self::FutuAsset, Self::RepoAsset, Self::OtherAsset, Self::OutNavAsset, Self::SecuCashAsset, Self::FutuCashAsset, Self::ShAsecuAsset, Self::SzAsecuAsset, Self::BjAsecuAsset, Self::ShHkSecuAsset, Self::SzHkSecuAsset, Self::MoneyFundAsset, Self::NotMoneyFundAsset, Self::FinaDebt, Self::LoanDebt, Self::FutuLongMarketValue, Self::FutuShortMarketValue];
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
            Self::SettleCrncyType => "settle_crncy_type",
            Self::BeginEvaluNavAsset => "begin_evalu_nav_asset",
            Self::CurrEvaluNavAsset => "curr_evalu_nav_asset",
            Self::BeginNavAsset => "begin_nav_asset",
            Self::CurrNavAsset => "curr_nav_asset",
            Self::CashAsset => "cash_asset",
            Self::SecuAsset => "secu_asset",
            Self::HkThrghSecuAsset => "hk_thrgh_secu_asset",
            Self::FundAsset => "fund_asset",
            Self::BondAsset => "bond_asset",
            Self::FutuAsset => "futu_asset",
            Self::RepoAsset => "repo_asset",
            Self::OtherAsset => "other_asset",
            Self::OutNavAsset => "out_nav_asset",
            Self::SecuCashAsset => "secu_cash_asset",
            Self::FutuCashAsset => "futu_cash_asset",
            Self::ShAsecuAsset => "sh_asecu_asset",
            Self::SzAsecuAsset => "sz_asecu_asset",
            Self::BjAsecuAsset => "bj_asecu_asset",
            Self::ShHkSecuAsset => "sh_hk_secu_asset",
            Self::SzHkSecuAsset => "sz_hk_secu_asset",
            Self::MoneyFundAsset => "money_fund_asset",
            Self::NotMoneyFundAsset => "not_money_fund_asset",
            Self::FinaDebt => "fina_debt",
            Self::LoanDebt => "loan_debt",
            Self::FutuLongMarketValue => "futu_long_market_value",
            Self::FutuShortMarketValue => "futu_short_market_value",
        }
    }
}

impl DataTable for PdmagePdAsset {
    const ID: &'static str = "tb_pdmage_pd_asset";
    type Column = PdmagePdAssetColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdmagePdAssetColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdmagePdAsset {}
