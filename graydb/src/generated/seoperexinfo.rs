//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_ex_info`（codegen：字段与列名源自 DDL，`SeoperExInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperExInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub exch_sub_type: i32,
    pub exch_type: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub exch_status: i32,
    pub distric: i32,
    pub time_lag: i32,
    pub no_exch_date_str: String,
    pub no_settle_date_str: String,
    pub posi_settle_days: i32,
    pub capit_settle_days: i32,
    pub settle_days: i32,
    pub mou_flag: i32,
    pub ex_init_date: i32,
    pub summer_time: i32,
    pub winter_time: i32,
    pub summer_time_begindate: i32,
    pub winter_time_begindate: i32,
    pub price_type_str: String,
    pub remark_info: String,
}

/// `tb_seoper_ex_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperExInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    ExchSubType,
    ExchType,
    SettleCrncyType,
    ExchCrncyType,
    ExchStatus,
    Distric,
    TimeLag,
    NoExchDateStr,
    NoSettleDateStr,
    PosiSettleDays,
    CapitSettleDays,
    SettleDays,
    MouFlag,
    ExInitDate,
    SummerTime,
    WinterTime,
    SummerTimeBegindate,
    WinterTimeBegindate,
    PriceTypeStr,
    RemarkInfo,
}

impl ColumnName for SeoperExInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::ExchSubType, Self::ExchType, Self::SettleCrncyType, Self::ExchCrncyType, Self::ExchStatus, Self::Distric, Self::TimeLag, Self::NoExchDateStr, Self::NoSettleDateStr, Self::PosiSettleDays, Self::CapitSettleDays, Self::SettleDays, Self::MouFlag, Self::ExInitDate, Self::SummerTime, Self::WinterTime, Self::SummerTimeBegindate, Self::WinterTimeBegindate, Self::PriceTypeStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::ExchSubType => "exch_sub_type",
            Self::ExchType => "exch_type",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::ExchStatus => "exch_status",
            Self::Distric => "distric",
            Self::TimeLag => "time_lag",
            Self::NoExchDateStr => "no_exch_date_str",
            Self::NoSettleDateStr => "no_settle_date_str",
            Self::PosiSettleDays => "posi_settle_days",
            Self::CapitSettleDays => "capit_settle_days",
            Self::SettleDays => "settle_days",
            Self::MouFlag => "mou_flag",
            Self::ExInitDate => "ex_init_date",
            Self::SummerTime => "summer_time",
            Self::WinterTime => "winter_time",
            Self::SummerTimeBegindate => "summer_time_begindate",
            Self::WinterTimeBegindate => "winter_time_begindate",
            Self::PriceTypeStr => "price_type_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperExInfo {
    const ID: &'static str = "tb_seoper_ex_info";
    type Column = SeoperExInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperExInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperExInfo {}
