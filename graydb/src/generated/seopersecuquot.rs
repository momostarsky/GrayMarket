//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_secu_quot`（codegen：字段与列名源自 DDL，`SeoperSecuQuotColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSecuQuot {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub up_limit_price: Decimal,
    pub down_limit_price: Decimal,
    pub last_price: Decimal,
    pub pre_close_price: Decimal,
    pub today_open_price: Decimal,
    pub today_close_price: Decimal,
    pub today_max_price: Decimal,
    pub today_min_price: Decimal,
    pub buy_price_1: Decimal,
    pub buy_qty_1: Decimal,
    pub buy_price_2: Decimal,
    pub buy_qty_2: Decimal,
    pub buy_price_3: Decimal,
    pub buy_qty_3: Decimal,
    pub buy_price_4: Decimal,
    pub buy_qty_4: Decimal,
    pub buy_price_5: Decimal,
    pub buy_qty_5: Decimal,
    pub sell_price_1: Decimal,
    pub sell_qty_1: Decimal,
    pub sell_price_2: Decimal,
    pub sell_qty_2: Decimal,
    pub sell_price_3: Decimal,
    pub sell_qty_3: Decimal,
    pub sell_price_4: Decimal,
    pub sell_qty_4: Decimal,
    pub sell_price_5: Decimal,
    pub sell_qty_5: Decimal,
    pub strike_qty: Decimal,
    pub strike_amt: Decimal,
    pub time_stamp: i64,
    pub remark_info: String,
}

/// `tb_seoper_secu_quot` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSecuQuotColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    SecuCode,
    SecuName,
    UpLimitPrice,
    DownLimitPrice,
    LastPrice,
    PreClosePrice,
    TodayOpenPrice,
    TodayClosePrice,
    TodayMaxPrice,
    TodayMinPrice,
    BuyPrice1,
    BuyQty1,
    BuyPrice2,
    BuyQty2,
    BuyPrice3,
    BuyQty3,
    BuyPrice4,
    BuyQty4,
    BuyPrice5,
    BuyQty5,
    SellPrice1,
    SellQty1,
    SellPrice2,
    SellQty2,
    SellPrice3,
    SellQty3,
    SellPrice4,
    SellQty4,
    SellPrice5,
    SellQty5,
    StrikeQty,
    StrikeAmt,
    TimeStamp,
    RemarkInfo,
}

impl ColumnName for SeoperSecuQuotColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::UpLimitPrice, Self::DownLimitPrice, Self::LastPrice, Self::PreClosePrice, Self::TodayOpenPrice, Self::TodayClosePrice, Self::TodayMaxPrice, Self::TodayMinPrice, Self::BuyPrice1, Self::BuyQty1, Self::BuyPrice2, Self::BuyQty2, Self::BuyPrice3, Self::BuyQty3, Self::BuyPrice4, Self::BuyQty4, Self::BuyPrice5, Self::BuyQty5, Self::SellPrice1, Self::SellQty1, Self::SellPrice2, Self::SellQty2, Self::SellPrice3, Self::SellQty3, Self::SellPrice4, Self::SellQty4, Self::SellPrice5, Self::SellQty5, Self::StrikeQty, Self::StrikeAmt, Self::TimeStamp, Self::RemarkInfo];
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
            Self::SecuName => "secu_name",
            Self::UpLimitPrice => "up_limit_price",
            Self::DownLimitPrice => "down_limit_price",
            Self::LastPrice => "last_price",
            Self::PreClosePrice => "pre_close_price",
            Self::TodayOpenPrice => "today_open_price",
            Self::TodayClosePrice => "today_close_price",
            Self::TodayMaxPrice => "today_max_price",
            Self::TodayMinPrice => "today_min_price",
            Self::BuyPrice1 => "buy_price_1",
            Self::BuyQty1 => "buy_qty_1",
            Self::BuyPrice2 => "buy_price_2",
            Self::BuyQty2 => "buy_qty_2",
            Self::BuyPrice3 => "buy_price_3",
            Self::BuyQty3 => "buy_qty_3",
            Self::BuyPrice4 => "buy_price_4",
            Self::BuyQty4 => "buy_qty_4",
            Self::BuyPrice5 => "buy_price_5",
            Self::BuyQty5 => "buy_qty_5",
            Self::SellPrice1 => "sell_price_1",
            Self::SellQty1 => "sell_qty_1",
            Self::SellPrice2 => "sell_price_2",
            Self::SellQty2 => "sell_qty_2",
            Self::SellPrice3 => "sell_price_3",
            Self::SellQty3 => "sell_qty_3",
            Self::SellPrice4 => "sell_price_4",
            Self::SellQty4 => "sell_qty_4",
            Self::SellPrice5 => "sell_price_5",
            Self::SellQty5 => "sell_qty_5",
            Self::StrikeQty => "strike_qty",
            Self::StrikeAmt => "strike_amt",
            Self::TimeStamp => "time_stamp",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperSecuQuot {
    const ID: &'static str = "tb_seoper_secu_quot";
    type Column = SeoperSecuQuotColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSecuQuotColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSecuQuot {}
