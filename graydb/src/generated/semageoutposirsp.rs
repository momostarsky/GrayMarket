//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_out_posi_rsp`（codegen：字段与列名源自 DDL，`SemageOutPosiRspColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageOutPosiRsp {
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
    pub exch_no: i32,
    pub secu_acco: String,
    pub secu_code: String,
    pub secu_name: String,
    pub begin_qty: Decimal,
    pub curr_qty: Decimal,
    pub avail_qty: Decimal,
    pub cost_price: Decimal,
    pub intrst_cost_amt: Decimal,
    pub frozen_qty: Decimal,
    pub unfrozen_qty: Decimal,
    pub busi_msg_id: String,
    pub if_error_no: i32,
    pub if_error_info: String,
    pub busi_error_info: String,
    pub busi_msg_content: String,
}

/// `tb_semage_out_posi_rsp` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageOutPosiRspColumn {
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
    ExchNo,
    SecuAcco,
    SecuCode,
    SecuName,
    BeginQty,
    CurrQty,
    AvailQty,
    CostPrice,
    IntrstCostAmt,
    FrozenQty,
    UnfrozenQty,
    BusiMsgId,
    IfErrorNo,
    IfErrorInfo,
    BusiErrorInfo,
    BusiMsgContent,
}

impl ColumnName for SemageOutPosiRspColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::AsacNo, Self::BrokerCoId, Self::OutAcco, Self::ExchNo, Self::SecuAcco, Self::SecuCode, Self::SecuName, Self::BeginQty, Self::CurrQty, Self::AvailQty, Self::CostPrice, Self::IntrstCostAmt, Self::FrozenQty, Self::UnfrozenQty, Self::BusiMsgId, Self::IfErrorNo, Self::IfErrorInfo, Self::BusiErrorInfo, Self::BusiMsgContent];
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
            Self::ExchNo => "exch_no",
            Self::SecuAcco => "secu_acco",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::BeginQty => "begin_qty",
            Self::CurrQty => "curr_qty",
            Self::AvailQty => "avail_qty",
            Self::CostPrice => "cost_price",
            Self::IntrstCostAmt => "intrst_cost_amt",
            Self::FrozenQty => "frozen_qty",
            Self::UnfrozenQty => "unfrozen_qty",
            Self::BusiMsgId => "busi_msg_id",
            Self::IfErrorNo => "if_error_no",
            Self::IfErrorInfo => "if_error_info",
            Self::BusiErrorInfo => "busi_error_info",
            Self::BusiMsgContent => "busi_msg_content",
        }
    }
}

impl DataTable for SemageOutPosiRsp {
    const ID: &'static str = "tb_semage_out_posi_rsp";
    type Column = SemageOutPosiRspColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageOutPosiRspColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageOutPosiRsp {}
