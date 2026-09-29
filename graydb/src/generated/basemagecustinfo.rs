//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_cust_info`（codegen：字段与列名源自 DDL，`BasemageCustInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageCustInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub asac_no: i32,
    pub asac_name: String,
    pub channel_no: i32,
    pub out_acco: String,
    pub asac_type: i32,
    pub busi_oper_mac: String,
    pub busi_oper_ip: String,
    pub report_num_pre: i32,
    pub report_num_total: i32,
    pub order_withdrw_ratio: Decimal,
    pub remark_info: String,
}

/// `tb_basemage_cust_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageCustInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    AsacNo,
    AsacName,
    ChannelNo,
    OutAcco,
    AsacType,
    BusiOperMac,
    BusiOperIp,
    ReportNumPre,
    ReportNumTotal,
    OrderWithdrwRatio,
    RemarkInfo,
}

impl ColumnName for BasemageCustInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::AsacNo, Self::AsacName, Self::ChannelNo, Self::OutAcco, Self::AsacType, Self::BusiOperMac, Self::BusiOperIp, Self::ReportNumPre, Self::ReportNumTotal, Self::OrderWithdrwRatio, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::AsacNo => "asac_no",
            Self::AsacName => "asac_name",
            Self::ChannelNo => "channel_no",
            Self::OutAcco => "out_acco",
            Self::AsacType => "asac_type",
            Self::BusiOperMac => "busi_oper_mac",
            Self::BusiOperIp => "busi_oper_ip",
            Self::ReportNumPre => "report_num_pre",
            Self::ReportNumTotal => "report_num_total",
            Self::OrderWithdrwRatio => "order_withdrw_ratio",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageCustInfo {
    const ID: &'static str = "tb_basemage_cust_info";
    type Column = BasemageCustInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageCustInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageCustInfo {}
