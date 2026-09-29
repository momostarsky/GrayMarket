//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_new_secu_code_info`（codegen：字段与列名源自 DDL，`SeoperNewSecuCodeInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperNewSecuCodeInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub exch_sub_type: i32,
    pub secu_name: String,
    pub trade_code: String,
    pub target_code: String,
    pub apply_date: i32,
    pub apply_limit: Decimal,
    pub begin_trade_date: i32,
    pub issue_price: Decimal,
    pub apply_pay_date: i32,
    pub remark_info: String,
}

/// `tb_seoper_new_secu_code_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperNewSecuCodeInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    SecuCode,
    ExchSubType,
    SecuName,
    TradeCode,
    TargetCode,
    ApplyDate,
    ApplyLimit,
    BeginTradeDate,
    IssuePrice,
    ApplyPayDate,
    RemarkInfo,
}

impl ColumnName for SeoperNewSecuCodeInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::SecuCode, Self::ExchSubType, Self::SecuName, Self::TradeCode, Self::TargetCode, Self::ApplyDate, Self::ApplyLimit, Self::BeginTradeDate, Self::IssuePrice, Self::ApplyPayDate, Self::RemarkInfo];
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
            Self::ExchSubType => "exch_sub_type",
            Self::SecuName => "secu_name",
            Self::TradeCode => "trade_code",
            Self::TargetCode => "target_code",
            Self::ApplyDate => "apply_date",
            Self::ApplyLimit => "apply_limit",
            Self::BeginTradeDate => "begin_trade_date",
            Self::IssuePrice => "issue_price",
            Self::ApplyPayDate => "apply_pay_date",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperNewSecuCodeInfo {
    const ID: &'static str = "tb_seoper_new_secu_code_info";
    type Column = SeoperNewSecuCodeInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperNewSecuCodeInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperNewSecuCodeInfo {}
