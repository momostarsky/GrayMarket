//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_out_capit_rsp`（codegen：字段与列名源自 DDL，`SemageOutCapitRspColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageOutCapitRsp {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub asac_no: i32,
    pub broker_co_id: i32,
    pub out_acco: String,
    pub settle_crncy_type: i32,
    pub begin_amt: Decimal,
    pub curr_amt: Decimal,
    pub avail_amt: Decimal,
    pub fetch_amt: Decimal,
    pub stock_balance: Decimal,
    pub nav_asset: Decimal,
    pub busi_msg_id: String,
    pub if_error_no: i32,
    pub if_error_info: String,
    pub busi_error_info: String,
    pub busi_msg_content: String,
}

/// `tb_semage_out_capit_rsp` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageOutCapitRspColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    AsacNo,
    BrokerCoId,
    OutAcco,
    SettleCrncyType,
    BeginAmt,
    CurrAmt,
    AvailAmt,
    FetchAmt,
    StockBalance,
    NavAsset,
    BusiMsgId,
    IfErrorNo,
    IfErrorInfo,
    BusiErrorInfo,
    BusiMsgContent,
}

impl ColumnName for SemageOutCapitRspColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::AsacNo, Self::BrokerCoId, Self::OutAcco, Self::SettleCrncyType, Self::BeginAmt, Self::CurrAmt, Self::AvailAmt, Self::FetchAmt, Self::StockBalance, Self::NavAsset, Self::BusiMsgId, Self::IfErrorNo, Self::IfErrorInfo, Self::BusiErrorInfo, Self::BusiMsgContent];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::AsacNo => "asac_no",
            Self::BrokerCoId => "broker_co_id",
            Self::OutAcco => "out_acco",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::BeginAmt => "begin_amt",
            Self::CurrAmt => "curr_amt",
            Self::AvailAmt => "avail_amt",
            Self::FetchAmt => "fetch_amt",
            Self::StockBalance => "stock_balance",
            Self::NavAsset => "nav_asset",
            Self::BusiMsgId => "busi_msg_id",
            Self::IfErrorNo => "if_error_no",
            Self::IfErrorInfo => "if_error_info",
            Self::BusiErrorInfo => "busi_error_info",
            Self::BusiMsgContent => "busi_msg_content",
        }
    }
}

impl DataTable for SemageOutCapitRsp {
    const ID: &'static str = "tb_semage_out_capit_rsp";
    type Column = SemageOutCapitRspColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageOutCapitRspColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageOutCapitRsp {}
