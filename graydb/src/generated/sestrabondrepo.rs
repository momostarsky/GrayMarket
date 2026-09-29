//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_sestra_bondrepo`（codegen：字段与列名源自 DDL，`SestraBondrepoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SestraBondrepo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub exch_rate: Decimal,
    pub exch_no: i32,
    pub secu_code: String,
    pub target_code: String,
    pub instr_no: String,
    pub order_dir: i32,
    pub repo_qty: Decimal,
    pub repo_amt: Decimal,
    pub repo_rate: Decimal,
    pub repo_trade_date: i32,
    pub external_no: String,
    pub out_order_id: String,
    pub strike_no: String,
    pub repo_days: i32,
    pub cash_capt_days: i32,
    pub repo_back_date: i32,
    pub repo_back_amt: Decimal,
    pub repo_back_intrst: Decimal,
    pub repo_back_trade_date: i32,
    pub repo_status: String,
}

/// `tb_sestra_bondrepo` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SestraBondrepoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    CoNo,
    PdNo,
    PdUnitNo,
    AsacNo,
    SettleCrncyType,
    ExchCrncyType,
    ExchRate,
    ExchNo,
    SecuCode,
    TargetCode,
    InstrNo,
    OrderDir,
    RepoQty,
    RepoAmt,
    RepoRate,
    RepoTradeDate,
    ExternalNo,
    OutOrderId,
    StrikeNo,
    RepoDays,
    CashCaptDays,
    RepoBackDate,
    RepoBackAmt,
    RepoBackIntrst,
    RepoBackTradeDate,
    RepoStatus,
}

impl ColumnName for SestraBondrepoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::CoNo, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::SettleCrncyType, Self::ExchCrncyType, Self::ExchRate, Self::ExchNo, Self::SecuCode, Self::TargetCode, Self::InstrNo, Self::OrderDir, Self::RepoQty, Self::RepoAmt, Self::RepoRate, Self::RepoTradeDate, Self::ExternalNo, Self::OutOrderId, Self::StrikeNo, Self::RepoDays, Self::CashCaptDays, Self::RepoBackDate, Self::RepoBackAmt, Self::RepoBackIntrst, Self::RepoBackTradeDate, Self::RepoStatus];
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
            Self::PdNo => "pd_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::ExchRate => "exch_rate",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::TargetCode => "target_code",
            Self::InstrNo => "instr_no",
            Self::OrderDir => "order_dir",
            Self::RepoQty => "repo_qty",
            Self::RepoAmt => "repo_amt",
            Self::RepoRate => "repo_rate",
            Self::RepoTradeDate => "repo_trade_date",
            Self::ExternalNo => "external_no",
            Self::OutOrderId => "out_order_id",
            Self::StrikeNo => "strike_no",
            Self::RepoDays => "repo_days",
            Self::CashCaptDays => "cash_capt_days",
            Self::RepoBackDate => "repo_back_date",
            Self::RepoBackAmt => "repo_back_amt",
            Self::RepoBackIntrst => "repo_back_intrst",
            Self::RepoBackTradeDate => "repo_back_trade_date",
            Self::RepoStatus => "repo_status",
        }
    }
}

impl DataTable for SestraBondrepo {
    const ID: &'static str = "tb_sestra_bondrepo";
    type Column = SestraBondrepoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SestraBondrepoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SestraBondrepo {}
