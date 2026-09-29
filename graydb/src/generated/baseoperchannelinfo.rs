//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_channel_info`（codegen：字段与列名源自 DDL，`BaseoperChannelInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperChannelInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub channel_no: i32,
    pub broker_co_id: i32,
    pub comm_pwd: String,
    pub channel_ip: String,
    pub channel_port: i32,
    pub channel_status: i32,
    pub busi_user_no: i32,
    pub offer_name: String,
    pub rep_code_type: i32,
    pub conn_type: i32,
    pub trans_conn_str: String,
    pub trans_conn_user_name: String,
    pub trans_conn_user_pwd: String,
    pub trans_config_str: String,
    pub out_order_replenish_flag: i32,
    pub transif_type: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub remark_info: String,
    pub time_stamp: i64,
}

/// `tb_baseoper_channel_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperChannelInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ChannelNo,
    BrokerCoId,
    CommPwd,
    ChannelIp,
    ChannelPort,
    ChannelStatus,
    BusiUserNo,
    OfferName,
    RepCodeType,
    ConnType,
    TransConnStr,
    TransConnUserName,
    TransConnUserPwd,
    TransConfigStr,
    OutOrderReplenishFlag,
    TransifType,
    BusiCtrlStr,
    EnableCtrlStr,
    RemarkInfo,
    TimeStamp,
}

impl ColumnName for BaseoperChannelInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ChannelNo, Self::BrokerCoId, Self::CommPwd, Self::ChannelIp, Self::ChannelPort, Self::ChannelStatus, Self::BusiUserNo, Self::OfferName, Self::RepCodeType, Self::ConnType, Self::TransConnStr, Self::TransConnUserName, Self::TransConnUserPwd, Self::TransConfigStr, Self::OutOrderReplenishFlag, Self::TransifType, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::RemarkInfo, Self::TimeStamp];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ChannelNo => "channel_no",
            Self::BrokerCoId => "broker_co_id",
            Self::CommPwd => "comm_pwd",
            Self::ChannelIp => "channel_ip",
            Self::ChannelPort => "channel_port",
            Self::ChannelStatus => "channel_status",
            Self::BusiUserNo => "busi_user_no",
            Self::OfferName => "offer_name",
            Self::RepCodeType => "rep_code_type",
            Self::ConnType => "conn_type",
            Self::TransConnStr => "trans_conn_str",
            Self::TransConnUserName => "trans_conn_user_name",
            Self::TransConnUserPwd => "trans_conn_user_pwd",
            Self::TransConfigStr => "trans_config_str",
            Self::OutOrderReplenishFlag => "out_order_replenish_flag",
            Self::TransifType => "transif_type",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::RemarkInfo => "remark_info",
            Self::TimeStamp => "time_stamp",
        }
    }
}

impl DataTable for BaseoperChannelInfo {
    const ID: &'static str = "tb_baseoper_channel_info";
    type Column = BaseoperChannelInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperChannelInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperChannelInfo {}
