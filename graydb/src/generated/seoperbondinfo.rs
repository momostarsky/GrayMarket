//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_bond_info`（codegen：字段与列名源自 DDL，`SeoperBondInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperBondInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub trade_code: String,
    pub target_code: String,
    pub secu_name: String,
    pub issue_date: i32,
    pub end_date: i32,
    pub value_date: i32,
    pub next_value_date: i32,
    pub begin_trade_date: i32,
    pub bond_limit: Decimal,
    pub issue_price: Decimal,
    pub par_value: Decimal,
    pub intrst_ratio: Decimal,
    pub intrst_days: i32,
    pub pay_inteval: i32,
    pub bond_accr_intrst: Decimal,
    pub bond_rate_type: i32,
    pub inteval_days: i32,
    pub net_price_flag: i32,
    pub last_trade_date: i32,
    pub rights_type: i32,
    pub trans_begin_date: i32,
    pub trans_end_date: i32,
    pub exec_begin_date: i32,
    pub exec_end_date: i32,
    pub impawn_ratio: Decimal,
    pub pay_intrst_flag: i32,
    pub remark_info: String,
    pub time_stamp: i64,
}

/// `tb_seoper_bond_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperBondInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    SecuCode,
    TradeCode,
    TargetCode,
    SecuName,
    IssueDate,
    EndDate,
    ValueDate,
    NextValueDate,
    BeginTradeDate,
    BondLimit,
    IssuePrice,
    ParValue,
    IntrstRatio,
    IntrstDays,
    PayInteval,
    BondAccrIntrst,
    BondRateType,
    IntevalDays,
    NetPriceFlag,
    LastTradeDate,
    RightsType,
    TransBeginDate,
    TransEndDate,
    ExecBeginDate,
    ExecEndDate,
    ImpawnRatio,
    PayIntrstFlag,
    RemarkInfo,
    TimeStamp,
}

impl ColumnName for SeoperBondInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::SecuCode, Self::TradeCode, Self::TargetCode, Self::SecuName, Self::IssueDate, Self::EndDate, Self::ValueDate, Self::NextValueDate, Self::BeginTradeDate, Self::BondLimit, Self::IssuePrice, Self::ParValue, Self::IntrstRatio, Self::IntrstDays, Self::PayInteval, Self::BondAccrIntrst, Self::BondRateType, Self::IntevalDays, Self::NetPriceFlag, Self::LastTradeDate, Self::RightsType, Self::TransBeginDate, Self::TransEndDate, Self::ExecBeginDate, Self::ExecEndDate, Self::ImpawnRatio, Self::PayIntrstFlag, Self::RemarkInfo, Self::TimeStamp];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::TradeCode => "trade_code",
            Self::TargetCode => "target_code",
            Self::SecuName => "secu_name",
            Self::IssueDate => "issue_date",
            Self::EndDate => "end_date",
            Self::ValueDate => "value_date",
            Self::NextValueDate => "next_value_date",
            Self::BeginTradeDate => "begin_trade_date",
            Self::BondLimit => "bond_limit",
            Self::IssuePrice => "issue_price",
            Self::ParValue => "par_value",
            Self::IntrstRatio => "intrst_ratio",
            Self::IntrstDays => "intrst_days",
            Self::PayInteval => "pay_inteval",
            Self::BondAccrIntrst => "bond_accr_intrst",
            Self::BondRateType => "bond_rate_type",
            Self::IntevalDays => "inteval_days",
            Self::NetPriceFlag => "net_price_flag",
            Self::LastTradeDate => "last_trade_date",
            Self::RightsType => "rights_type",
            Self::TransBeginDate => "trans_begin_date",
            Self::TransEndDate => "trans_end_date",
            Self::ExecBeginDate => "exec_begin_date",
            Self::ExecEndDate => "exec_end_date",
            Self::ImpawnRatio => "impawn_ratio",
            Self::PayIntrstFlag => "pay_intrst_flag",
            Self::RemarkInfo => "remark_info",
            Self::TimeStamp => "time_stamp",
        }
    }
}

impl DataTable for SeoperBondInfo {
    const ID: &'static str = "tb_seoper_bond_info";
    type Column = SeoperBondInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperBondInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperBondInfo {}
