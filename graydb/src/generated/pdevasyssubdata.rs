//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdeva_sys_sub_data`（codegen：字段与列名源自 DDL，`PdevaSysSubDataColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdevaSysSubData {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub account_level: i32,
    pub curr_no: i64,
    pub subject_code: String,
    pub money_type: i32,
    pub sys_account_balance: Decimal,
    pub frozen_amt: Decimal,
    pub fixed_withdrawal_amt: Decimal,
    pub freezing_start_days: i32,
    pub freezing_start_days_type: i32,
    pub freezing_days_type: i32,
    pub freezing_days: i32,
    pub estimate_rate: Decimal,
    pub freeze_flag: i32,
    pub freeze_due_date: i32,
}

/// `tb_pdeva_sys_sub_data` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdevaSysSubDataColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    AccountLevel,
    CurrNo,
    SubjectCode,
    MoneyType,
    SysAccountBalance,
    FrozenAmt,
    FixedWithdrawalAmt,
    FreezingStartDays,
    FreezingStartDaysType,
    FreezingDaysType,
    FreezingDays,
    EstimateRate,
    FreezeFlag,
    FreezeDueDate,
}

impl ColumnName for PdevaSysSubDataColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::AccountLevel, Self::CurrNo, Self::SubjectCode, Self::MoneyType, Self::SysAccountBalance, Self::FrozenAmt, Self::FixedWithdrawalAmt, Self::FreezingStartDays, Self::FreezingStartDaysType, Self::FreezingDaysType, Self::FreezingDays, Self::EstimateRate, Self::FreezeFlag, Self::FreezeDueDate];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::AccountLevel => "account_level",
            Self::CurrNo => "curr_no",
            Self::SubjectCode => "subject_code",
            Self::MoneyType => "money_type",
            Self::SysAccountBalance => "sys_account_balance",
            Self::FrozenAmt => "frozen_amt",
            Self::FixedWithdrawalAmt => "fixed_withdrawal_amt",
            Self::FreezingStartDays => "freezing_start_days",
            Self::FreezingStartDaysType => "freezing_start_days_type",
            Self::FreezingDaysType => "freezing_days_type",
            Self::FreezingDays => "freezing_days",
            Self::EstimateRate => "estimate_rate",
            Self::FreezeFlag => "freeze_flag",
            Self::FreezeDueDate => "freeze_due_date",
        }
    }
}

impl DataTable for PdevaSysSubData {
    const ID: &'static str = "tb_pdeva_sys_sub_data";
    type Column = PdevaSysSubDataColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdevaSysSubDataColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdevaSysSubData {}
