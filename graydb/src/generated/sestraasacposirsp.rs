//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_sestra_asac_posi_rsp`（codegen：字段与列名源自 DDL，`SestraAsacPosiRspColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SestraAsacPosiRsp {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub source_row_id: i64,
    pub last_update_times: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub co_no: i32,
    pub secu_acco: String,
    pub secu_name: String,
    pub forbid_order_dir: String,
    pub buy_mode: i32,
    pub sell_mode: i32,
    pub avail_qty: Decimal,
    pub begin_qty: Decimal,
    pub curr_qty: Decimal,
    pub begin_max_loan_amount: Decimal,
    pub curr_max_loan_amount: Decimal,
    pub begin_shortsell_quota: Decimal,
    pub curr_shortsell_quota: Decimal,
    pub begin_used_loan_qty: Decimal,
    pub curr_used_loan_qty: Decimal,
    pub cost_price: Decimal,
    pub cost_amt: Decimal,
}

/// `tb_sestra_asac_posi_rsp` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SestraAsacPosiRspColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    SourceRowId,
    LastUpdateTimes,
    PdNo,
    AsacNo,
    ExchNo,
    SecuCode,
    CoNo,
    SecuAcco,
    SecuName,
    ForbidOrderDir,
    BuyMode,
    SellMode,
    AvailQty,
    BeginQty,
    CurrQty,
    BeginMaxLoanAmount,
    CurrMaxLoanAmount,
    BeginShortsellQuota,
    CurrShortsellQuota,
    BeginUsedLoanQty,
    CurrUsedLoanQty,
    CostPrice,
    CostAmt,
}

impl ColumnName for SestraAsacPosiRspColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::SourceRowId, Self::LastUpdateTimes, Self::PdNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::CoNo, Self::SecuAcco, Self::SecuName, Self::ForbidOrderDir, Self::BuyMode, Self::SellMode, Self::AvailQty, Self::BeginQty, Self::CurrQty, Self::BeginMaxLoanAmount, Self::CurrMaxLoanAmount, Self::BeginShortsellQuota, Self::CurrShortsellQuota, Self::BeginUsedLoanQty, Self::CurrUsedLoanQty, Self::CostPrice, Self::CostAmt];
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
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::CoNo => "co_no",
            Self::SecuAcco => "secu_acco",
            Self::SecuName => "secu_name",
            Self::ForbidOrderDir => "forbid_order_dir",
            Self::BuyMode => "buy_mode",
            Self::SellMode => "sell_mode",
            Self::AvailQty => "avail_qty",
            Self::BeginQty => "begin_qty",
            Self::CurrQty => "curr_qty",
            Self::BeginMaxLoanAmount => "begin_max_loan_amount",
            Self::CurrMaxLoanAmount => "curr_max_loan_amount",
            Self::BeginShortsellQuota => "begin_shortsell_quota",
            Self::CurrShortsellQuota => "curr_shortsell_quota",
            Self::BeginUsedLoanQty => "begin_used_loan_qty",
            Self::CurrUsedLoanQty => "curr_used_loan_qty",
            Self::CostPrice => "cost_price",
            Self::CostAmt => "cost_amt",
        }
    }
}

impl DataTable for SestraAsacPosiRsp {
    const ID: &'static str = "tb_sestra_asac_posi_rsp";
    type Column = SestraAsacPosiRspColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SestraAsacPosiRspColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SestraAsacPosiRsp {}
