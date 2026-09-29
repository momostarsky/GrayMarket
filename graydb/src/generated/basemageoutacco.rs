//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_out_acco`（codegen：字段与列名源自 DDL，`BasemageOutAccoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageOutAcco {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub broker_co_id: i32,
    pub channel_no: i32,
    pub out_acco_id: i32,
    pub out_acco_name: String,
    pub out_acco: String,
    pub out_trade_pwd: String,
    pub comm_pwd: String,
    pub out_acco_status: i32,
    pub out_acco_type: i32,
    pub busi_user_no: i32,
    pub busi_oper_mac: String,
    pub strike_deal_type: i32,
    pub cost_calc_type: i32,
    pub dma_mode_str: String,
    pub dma_type_str: String,
    pub undma_type_str: String,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub remark_info: String,
}

/// `tb_basemage_out_acco` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageOutAccoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdNo,
    AsacNo,
    BrokerCoId,
    ChannelNo,
    OutAccoId,
    OutAccoName,
    OutAcco,
    OutTradePwd,
    CommPwd,
    OutAccoStatus,
    OutAccoType,
    BusiUserNo,
    BusiOperMac,
    StrikeDealType,
    CostCalcType,
    DmaModeStr,
    DmaTypeStr,
    UndmaTypeStr,
    BusiCtrlStr,
    EnableCtrlStr,
    RemarkInfo,
}

impl ColumnName for BasemageOutAccoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdNo, Self::AsacNo, Self::BrokerCoId, Self::ChannelNo, Self::OutAccoId, Self::OutAccoName, Self::OutAcco, Self::OutTradePwd, Self::CommPwd, Self::OutAccoStatus, Self::OutAccoType, Self::BusiUserNo, Self::BusiOperMac, Self::StrikeDealType, Self::CostCalcType, Self::DmaModeStr, Self::DmaTypeStr, Self::UndmaTypeStr, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::BrokerCoId => "broker_co_id",
            Self::ChannelNo => "channel_no",
            Self::OutAccoId => "out_acco_id",
            Self::OutAccoName => "out_acco_name",
            Self::OutAcco => "out_acco",
            Self::OutTradePwd => "out_trade_pwd",
            Self::CommPwd => "comm_pwd",
            Self::OutAccoStatus => "out_acco_status",
            Self::OutAccoType => "out_acco_type",
            Self::BusiUserNo => "busi_user_no",
            Self::BusiOperMac => "busi_oper_mac",
            Self::StrikeDealType => "strike_deal_type",
            Self::CostCalcType => "cost_calc_type",
            Self::DmaModeStr => "dma_mode_str",
            Self::DmaTypeStr => "dma_type_str",
            Self::UndmaTypeStr => "undma_type_str",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageOutAcco {
    const ID: &'static str = "tb_basemage_out_acco";
    type Column = BasemageOutAccoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageOutAccoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageOutAcco {}
