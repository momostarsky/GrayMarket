//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdeva_eva_file_data`（codegen：字段与列名源自 DDL，`PdevaEvaFileDataColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdevaEvaFileData {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub co_no: i32,
    pub account_level: i32,
    pub curr_no: i64,
    pub financial_account_code: String,
    pub financial_account_name: String,
    pub finance_account_balance: Decimal,
    pub posi_flag: i32,
    pub exch_no: i32,
    pub posi_qty: Decimal,
    pub secu_unit_cost: Decimal,
    pub cost_amt: Decimal,
    pub market_last_price: Decimal,
    pub posi_market_value: Decimal,
    pub asset_type: i32,
    pub freeze_flag: i32,
    pub secu_code: String,
    pub valid_flag: i32,
    pub direction: i32,
    pub subject_code: String,
    pub money_type: i32,
    pub declaration_code: String,
    pub invest_type: i32,
    pub circl_type: i32,
    pub busi_date: i32,
    pub exch_rate: Decimal,
    pub local_crncy_finance_account_balance: Decimal,
    pub local_crncy_secu_unit_cost: Decimal,
    pub local_crncy_cost_amt: Decimal,
    pub local_crncy_market_last_price: Decimal,
    pub local_crncy_posi_market_value: Decimal,
    pub pupil_flag: i32,
    pub cost_ratio: Decimal,
    pub market_value_ratio: Decimal,
    pub eva_increment: Decimal,
    pub local_crncy_eva_increment: Decimal,
    pub rights_info: String,
    pub stop_status: i32,
    pub row_local: i32,
}

/// `tb_pdeva_eva_file_data` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdevaEvaFileDataColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    CoNo,
    AccountLevel,
    CurrNo,
    FinancialAccountCode,
    FinancialAccountName,
    FinanceAccountBalance,
    PosiFlag,
    ExchNo,
    PosiQty,
    SecuUnitCost,
    CostAmt,
    MarketLastPrice,
    PosiMarketValue,
    AssetType,
    FreezeFlag,
    SecuCode,
    ValidFlag,
    Direction,
    SubjectCode,
    MoneyType,
    DeclarationCode,
    InvestType,
    CirclType,
    BusiDate,
    ExchRate,
    LocalCrncyFinanceAccountBalance,
    LocalCrncySecuUnitCost,
    LocalCrncyCostAmt,
    LocalCrncyMarketLastPrice,
    LocalCrncyPosiMarketValue,
    PupilFlag,
    CostRatio,
    MarketValueRatio,
    EvaIncrement,
    LocalCrncyEvaIncrement,
    RightsInfo,
    StopStatus,
    RowLocal,
}

impl ColumnName for PdevaEvaFileDataColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::CoNo, Self::AccountLevel, Self::CurrNo, Self::FinancialAccountCode, Self::FinancialAccountName, Self::FinanceAccountBalance, Self::PosiFlag, Self::ExchNo, Self::PosiQty, Self::SecuUnitCost, Self::CostAmt, Self::MarketLastPrice, Self::PosiMarketValue, Self::AssetType, Self::FreezeFlag, Self::SecuCode, Self::ValidFlag, Self::Direction, Self::SubjectCode, Self::MoneyType, Self::DeclarationCode, Self::InvestType, Self::CirclType, Self::BusiDate, Self::ExchRate, Self::LocalCrncyFinanceAccountBalance, Self::LocalCrncySecuUnitCost, Self::LocalCrncyCostAmt, Self::LocalCrncyMarketLastPrice, Self::LocalCrncyPosiMarketValue, Self::PupilFlag, Self::CostRatio, Self::MarketValueRatio, Self::EvaIncrement, Self::LocalCrncyEvaIncrement, Self::RightsInfo, Self::StopStatus, Self::RowLocal];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::CoNo => "co_no",
            Self::AccountLevel => "account_level",
            Self::CurrNo => "curr_no",
            Self::FinancialAccountCode => "financial_account_code",
            Self::FinancialAccountName => "financial_account_name",
            Self::FinanceAccountBalance => "finance_account_balance",
            Self::PosiFlag => "posi_flag",
            Self::ExchNo => "exch_no",
            Self::PosiQty => "posi_qty",
            Self::SecuUnitCost => "secu_unit_cost",
            Self::CostAmt => "cost_amt",
            Self::MarketLastPrice => "market_last_price",
            Self::PosiMarketValue => "posi_market_value",
            Self::AssetType => "asset_type",
            Self::FreezeFlag => "freeze_flag",
            Self::SecuCode => "secu_code",
            Self::ValidFlag => "valid_flag",
            Self::Direction => "direction",
            Self::SubjectCode => "subject_code",
            Self::MoneyType => "money_type",
            Self::DeclarationCode => "declaration_code",
            Self::InvestType => "invest_type",
            Self::CirclType => "circl_type",
            Self::BusiDate => "busi_date",
            Self::ExchRate => "exch_rate",
            Self::LocalCrncyFinanceAccountBalance => "local_crncy_finance_account_balance",
            Self::LocalCrncySecuUnitCost => "local_crncy_secu_unit_cost",
            Self::LocalCrncyCostAmt => "local_crncy_cost_amt",
            Self::LocalCrncyMarketLastPrice => "local_crncy_market_last_price",
            Self::LocalCrncyPosiMarketValue => "local_crncy_posi_market_value",
            Self::PupilFlag => "pupil_flag",
            Self::CostRatio => "cost_ratio",
            Self::MarketValueRatio => "market_value_ratio",
            Self::EvaIncrement => "eva_increment",
            Self::LocalCrncyEvaIncrement => "local_crncy_eva_increment",
            Self::RightsInfo => "rights_info",
            Self::StopStatus => "stop_status",
            Self::RowLocal => "row_local",
        }
    }
}

impl DataTable for PdevaEvaFileData {
    const ID: &'static str = "tb_pdeva_eva_file_data";
    type Column = PdevaEvaFileDataColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdevaEvaFileDataColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdevaEvaFileData {}
