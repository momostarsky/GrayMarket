//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_secu_type_busi_arg`（codegen：字段与列名源自 DDL，`SeoperSecuTypeBusiArgColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSecuTypeBusiArg {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub exch_sub_type: i32,
    pub secu_type: i32,
    pub order_dir: i32,
    pub cash_frozen_type: i32,
    pub order_split_flag: i32,
    pub min_unit: i32,
    pub max_qty: Decimal,
    pub min_qty: Decimal,
    pub time_stamp: i64,
    pub remark_info: String,
}

/// `tb_seoper_secu_type_busi_arg` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSecuTypeBusiArgColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    ExchSubType,
    SecuType,
    OrderDir,
    CashFrozenType,
    OrderSplitFlag,
    MinUnit,
    MaxQty,
    MinQty,
    TimeStamp,
    RemarkInfo,
}

impl ColumnName for SeoperSecuTypeBusiArgColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::ExchSubType, Self::SecuType, Self::OrderDir, Self::CashFrozenType, Self::OrderSplitFlag, Self::MinUnit, Self::MaxQty, Self::MinQty, Self::TimeStamp, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::ExchSubType => "exch_sub_type",
            Self::SecuType => "secu_type",
            Self::OrderDir => "order_dir",
            Self::CashFrozenType => "cash_frozen_type",
            Self::OrderSplitFlag => "order_split_flag",
            Self::MinUnit => "min_unit",
            Self::MaxQty => "max_qty",
            Self::MinQty => "min_qty",
            Self::TimeStamp => "time_stamp",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperSecuTypeBusiArg {
    const ID: &'static str = "tb_seoper_secu_type_busi_arg";
    type Column = SeoperSecuTypeBusiArgColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSecuTypeBusiArgColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSecuTypeBusiArg {}
