//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_asac_ctrl_config`（codegen：字段与列名源自 DDL，`BasemageAsacCtrlConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageAsacCtrlConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub ctrl_orders_pre_second: i32,
    pub ctrl_exch_orders_pre_second: i32,
    pub ctrl_orders: i32,
    pub ctrl_precond: String,
    pub withdrw_ratio: Decimal,
    pub invalid_order_ratio: Decimal,
    pub strike_ratio: Decimal,
    pub net_buy_amt: Decimal,
}

/// `tb_basemage_asac_ctrl_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageAsacCtrlConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdNo,
    AsacNo,
    CtrlOrdersPreSecond,
    CtrlExchOrdersPreSecond,
    CtrlOrders,
    CtrlPrecond,
    WithdrwRatio,
    InvalidOrderRatio,
    StrikeRatio,
    NetBuyAmt,
}

impl ColumnName for BasemageAsacCtrlConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdNo, Self::AsacNo, Self::CtrlOrdersPreSecond, Self::CtrlExchOrdersPreSecond, Self::CtrlOrders, Self::CtrlPrecond, Self::WithdrwRatio, Self::InvalidOrderRatio, Self::StrikeRatio, Self::NetBuyAmt];
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
            Self::CtrlOrdersPreSecond => "ctrl_orders_pre_second",
            Self::CtrlExchOrdersPreSecond => "ctrl_exch_orders_pre_second",
            Self::CtrlOrders => "ctrl_orders",
            Self::CtrlPrecond => "ctrl_precond",
            Self::WithdrwRatio => "withdrw_ratio",
            Self::InvalidOrderRatio => "invalid_order_ratio",
            Self::StrikeRatio => "strike_ratio",
            Self::NetBuyAmt => "net_buy_amt",
        }
    }
}

impl DataTable for BasemageAsacCtrlConfig {
    const ID: &'static str = "tb_basemage_asac_ctrl_config";
    type Column = BasemageAsacCtrlConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageAsacCtrlConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageAsacCtrlConfig {}
