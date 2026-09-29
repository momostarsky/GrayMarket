//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_comp_action`（codegen：字段与列名源自 DDL，`SeoperCompActionColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperCompAction {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub settle_crncy_type: i32,
    pub busi_flag: i32,
    pub dividend_calc_unit: i32,
    pub dividend_amt: Decimal,
    pub dividend_qty: Decimal,
    pub tranaddshare_qty: Decimal,
    pub pla_qty: Decimal,
    pub pla_price: Decimal,
    pub notic_date: i32,
    pub reg_date: i32,
    pub entry_date: i32,
    pub begin_trade_date: i32,
    pub exdividend_date: i32,
    pub remark_info: String,
}

/// `tb_seoper_comp_action` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperCompActionColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    ExchNo,
    SecuCode,
    SecuName,
    SettleCrncyType,
    BusiFlag,
    DividendCalcUnit,
    DividendAmt,
    DividendQty,
    TranaddshareQty,
    PlaQty,
    PlaPrice,
    NoticDate,
    RegDate,
    EntryDate,
    BeginTradeDate,
    ExdividendDate,
    RemarkInfo,
}

impl ColumnName for SeoperCompActionColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::SettleCrncyType, Self::BusiFlag, Self::DividendCalcUnit, Self::DividendAmt, Self::DividendQty, Self::TranaddshareQty, Self::PlaQty, Self::PlaPrice, Self::NoticDate, Self::RegDate, Self::EntryDate, Self::BeginTradeDate, Self::ExdividendDate, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::BusiFlag => "busi_flag",
            Self::DividendCalcUnit => "dividend_calc_unit",
            Self::DividendAmt => "dividend_amt",
            Self::DividendQty => "dividend_qty",
            Self::TranaddshareQty => "tranaddshare_qty",
            Self::PlaQty => "pla_qty",
            Self::PlaPrice => "pla_price",
            Self::NoticDate => "notic_date",
            Self::RegDate => "reg_date",
            Self::EntryDate => "entry_date",
            Self::BeginTradeDate => "begin_trade_date",
            Self::ExdividendDate => "exdividend_date",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperCompAction {
    const ID: &'static str = "tb_seoper_comp_action";
    type Column = SeoperCompActionColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperCompActionColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperCompAction {}
