//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_fee_section_config`（codegen：字段与列名源自 DDL，`SemageFeeSectionConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageFeeSectionConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub fee_model_detail_id: i32,
    pub fee_section_config_id: i32,
    pub fee_range_min: Decimal,
    pub fee_range_max: Decimal,
    pub charge_type: i32,
    pub fee_rate: Decimal,
    pub fixed_fee: Decimal,
    pub remark_info: String,
}

/// `tb_semage_fee_section_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageFeeSectionConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    FeeModelDetailId,
    FeeSectionConfigId,
    FeeRangeMin,
    FeeRangeMax,
    ChargeType,
    FeeRate,
    FixedFee,
    RemarkInfo,
}

impl ColumnName for SemageFeeSectionConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::FeeModelDetailId, Self::FeeSectionConfigId, Self::FeeRangeMin, Self::FeeRangeMax, Self::ChargeType, Self::FeeRate, Self::FixedFee, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::FeeModelDetailId => "fee_model_detail_id",
            Self::FeeSectionConfigId => "fee_section_config_id",
            Self::FeeRangeMin => "fee_range_min",
            Self::FeeRangeMax => "fee_range_max",
            Self::ChargeType => "charge_type",
            Self::FeeRate => "fee_rate",
            Self::FixedFee => "fixed_fee",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageFeeSectionConfig {
    const ID: &'static str = "tb_semage_fee_section_config";
    type Column = SemageFeeSectionConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageFeeSectionConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageFeeSectionConfig {}
