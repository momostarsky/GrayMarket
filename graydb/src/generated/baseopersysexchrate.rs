//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_sys_exch_rate`（codegen：字段与列名源自 DDL，`BaseoperSysExchRateColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperSysExchRate {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub rate_from_no: i32,
    pub rate_from_info: String,
    pub crncy_type: i32,
    pub for_crncy_type: i32,
    pub buy_exch_rate: Decimal,
    pub sale_exch_rate: Decimal,
}

/// `tb_baseoper_sys_exch_rate` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperSysExchRateColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    RateFromNo,
    RateFromInfo,
    CrncyType,
    ForCrncyType,
    BuyExchRate,
    SaleExchRate,
}

impl ColumnName for BaseoperSysExchRateColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::RateFromNo, Self::RateFromInfo, Self::CrncyType, Self::ForCrncyType, Self::BuyExchRate, Self::SaleExchRate];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::RateFromNo => "rate_from_no",
            Self::RateFromInfo => "rate_from_info",
            Self::CrncyType => "crncy_type",
            Self::ForCrncyType => "for_crncy_type",
            Self::BuyExchRate => "buy_exch_rate",
            Self::SaleExchRate => "sale_exch_rate",
        }
    }
}

impl DataTable for BaseoperSysExchRate {
    const ID: &'static str = "tb_baseoper_sys_exch_rate";
    type Column = BaseoperSysExchRateColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperSysExchRateColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperSysExchRate {}
