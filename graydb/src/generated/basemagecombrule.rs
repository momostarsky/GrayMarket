//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_combrule`（codegen：字段与列名源自 DDL，`BasemageCombruleColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageCombrule {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub broker_co_id: i32,
    pub comb_type: i32,
    pub price_deal: i32,
    pub deci_bit: i32,
    pub amt_calc_type: i32,
    pub remark_info: String,
}

/// `tb_basemage_combrule` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageCombruleColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    BrokerCoId,
    CombType,
    PriceDeal,
    DeciBit,
    AmtCalcType,
    RemarkInfo,
}

impl ColumnName for BasemageCombruleColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::BrokerCoId, Self::CombType, Self::PriceDeal, Self::DeciBit, Self::AmtCalcType, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::BrokerCoId => "broker_co_id",
            Self::CombType => "comb_type",
            Self::PriceDeal => "price_deal",
            Self::DeciBit => "deci_bit",
            Self::AmtCalcType => "amt_calc_type",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageCombrule {
    const ID: &'static str = "tb_basemage_combrule";
    type Column = BasemageCombruleColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageCombruleColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageCombrule {}
