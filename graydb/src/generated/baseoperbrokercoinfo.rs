//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_broker_co_info`（codegen：字段与列名源自 DDL，`BaseoperBrokerCoInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperBrokerCoInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub broker_co_id: i32,
    pub broker_co_name: String,
    pub broker_co_key: String,
    pub broker_co_type: i32,
    pub broker_co_acco_flag: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub remark_info: String,
}

/// `tb_baseoper_broker_co_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperBrokerCoInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    BrokerCoId,
    BrokerCoName,
    BrokerCoKey,
    BrokerCoType,
    BrokerCoAccoFlag,
    BusiCtrlStr,
    EnableCtrlStr,
    RemarkInfo,
}

impl ColumnName for BaseoperBrokerCoInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::BrokerCoId, Self::BrokerCoName, Self::BrokerCoKey, Self::BrokerCoType, Self::BrokerCoAccoFlag, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::BrokerCoId => "broker_co_id",
            Self::BrokerCoName => "broker_co_name",
            Self::BrokerCoKey => "broker_co_key",
            Self::BrokerCoType => "broker_co_type",
            Self::BrokerCoAccoFlag => "broker_co_acco_flag",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BaseoperBrokerCoInfo {
    const ID: &'static str = "tb_baseoper_broker_co_info";
    type Column = BaseoperBrokerCoInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperBrokerCoInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperBrokerCoInfo {}
