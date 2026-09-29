//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_ctmstrike_allocation`（codegen：字段与列名源自 DDL，`SemageCtmstrikeAllocationColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageCtmstrikeAllocation {
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
    pub out_acco_id: i32,
    pub broker_co_id: i32,
    pub channel_no: i32,
    pub secu_acco: String,
    pub exor_no: i32,
    pub dma: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub comm_batch_no: i64,
    pub instr_no: String,
    pub external_no: String,
    pub order_oper_way: i32,
    pub order_date: i32,
    pub order_time: i32,
    pub order_batch_no: i64,
    pub order_id: i64,
    pub order_dir: i32,
    pub order_price: Decimal,
    pub order_qty: Decimal,
    pub report_date: i32,
    pub report_no: String,
    pub strike_date: i32,
    pub strike_time: i32,
    pub strike_no: String,
    pub strike_qty: Decimal,
    pub strike_price: Decimal,
    pub strike_amt: Decimal,
    pub occur_amt: Decimal,
    pub all_fee: Decimal,
    pub stamp_tax: Decimal,
    pub trans_fee: Decimal,
    pub brkage_fee: Decimal,
    #[serde(rename = "SEC_charges")]
    pub sec_charges: Decimal,
    pub other_fee: Decimal,
    pub trade_commis: Decimal,
    pub other_commis: Decimal,
    pub remark_info: String,
}

/// `tb_semage_ctmstrike_allocation` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageCtmstrikeAllocationColumn {
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
    OutAccoId,
    BrokerCoId,
    ChannelNo,
    SecuAcco,
    ExorNo,
    Dma,
    ExchNo,
    SecuCode,
    SecuName,
    CommBatchNo,
    InstrNo,
    ExternalNo,
    OrderOperWay,
    OrderDate,
    OrderTime,
    OrderBatchNo,
    OrderId,
    OrderDir,
    OrderPrice,
    OrderQty,
    ReportDate,
    ReportNo,
    StrikeDate,
    StrikeTime,
    StrikeNo,
    StrikeQty,
    StrikePrice,
    StrikeAmt,
    OccurAmt,
    AllFee,
    StampTax,
    TransFee,
    BrkageFee,
    SECCharges,
    OtherFee,
    TradeCommis,
    OtherCommis,
    RemarkInfo,
}

impl ColumnName for SemageCtmstrikeAllocationColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::CoNo, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::OutAccoId, Self::BrokerCoId, Self::ChannelNo, Self::SecuAcco, Self::ExorNo, Self::Dma, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::CommBatchNo, Self::InstrNo, Self::ExternalNo, Self::OrderOperWay, Self::OrderDate, Self::OrderTime, Self::OrderBatchNo, Self::OrderId, Self::OrderDir, Self::OrderPrice, Self::OrderQty, Self::ReportDate, Self::ReportNo, Self::StrikeDate, Self::StrikeTime, Self::StrikeNo, Self::StrikeQty, Self::StrikePrice, Self::StrikeAmt, Self::OccurAmt, Self::AllFee, Self::StampTax, Self::TransFee, Self::BrkageFee, Self::SECCharges, Self::OtherFee, Self::TradeCommis, Self::OtherCommis, Self::RemarkInfo];
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
            Self::OutAccoId => "out_acco_id",
            Self::BrokerCoId => "broker_co_id",
            Self::ChannelNo => "channel_no",
            Self::SecuAcco => "secu_acco",
            Self::ExorNo => "exor_no",
            Self::Dma => "dma",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::CommBatchNo => "comm_batch_no",
            Self::InstrNo => "instr_no",
            Self::ExternalNo => "external_no",
            Self::OrderOperWay => "order_oper_way",
            Self::OrderDate => "order_date",
            Self::OrderTime => "order_time",
            Self::OrderBatchNo => "order_batch_no",
            Self::OrderId => "order_id",
            Self::OrderDir => "order_dir",
            Self::OrderPrice => "order_price",
            Self::OrderQty => "order_qty",
            Self::ReportDate => "report_date",
            Self::ReportNo => "report_no",
            Self::StrikeDate => "strike_date",
            Self::StrikeTime => "strike_time",
            Self::StrikeNo => "strike_no",
            Self::StrikeQty => "strike_qty",
            Self::StrikePrice => "strike_price",
            Self::StrikeAmt => "strike_amt",
            Self::OccurAmt => "occur_amt",
            Self::AllFee => "all_fee",
            Self::StampTax => "stamp_tax",
            Self::TransFee => "trans_fee",
            Self::BrkageFee => "brkage_fee",
            Self::SECCharges => "SEC_charges",
            Self::OtherFee => "other_fee",
            Self::TradeCommis => "trade_commis",
            Self::OtherCommis => "other_commis",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageCtmstrikeAllocation {
    const ID: &'static str = "tb_semage_ctmstrike_allocation";
    type Column = SemageCtmstrikeAllocationColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageCtmstrikeAllocationColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageCtmstrikeAllocation {}
