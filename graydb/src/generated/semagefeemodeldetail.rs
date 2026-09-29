//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_fee_model_detail`（codegen：字段与列名源自 DDL，`SemageFeeModelDetailColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageFeeModelDetail {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub fee_model_id: i32,
    pub fee_model_detail_id: i32,
    pub fee_kind: i32,
    pub exch_no: i32,
    pub secu_fee_type: i32,
    pub secu_type: i32,
    pub secu_code: String,
    pub broker_co_id: i32,
    pub money_type: i32,
    pub dma_mode: i32,
    pub order_dir: i32,
    pub order_kind: i32,
    pub charge_type: i32,
    pub math_round_type: i32,
    pub math_round_place: i32,
    pub fee_rate: Decimal,
    pub fixed_fee: Decimal,
    pub max_fee: Decimal,
    pub min_fee: Decimal,
    pub remark_info: String,
}

/// `tb_semage_fee_model_detail` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageFeeModelDetailColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    FeeModelId,
    FeeModelDetailId,
    FeeKind,
    ExchNo,
    SecuFeeType,
    SecuType,
    SecuCode,
    BrokerCoId,
    MoneyType,
    DmaMode,
    OrderDir,
    OrderKind,
    ChargeType,
    MathRoundType,
    MathRoundPlace,
    FeeRate,
    FixedFee,
    MaxFee,
    MinFee,
    RemarkInfo,
}

impl ColumnName for SemageFeeModelDetailColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::FeeModelId, Self::FeeModelDetailId, Self::FeeKind, Self::ExchNo, Self::SecuFeeType, Self::SecuType, Self::SecuCode, Self::BrokerCoId, Self::MoneyType, Self::DmaMode, Self::OrderDir, Self::OrderKind, Self::ChargeType, Self::MathRoundType, Self::MathRoundPlace, Self::FeeRate, Self::FixedFee, Self::MaxFee, Self::MinFee, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::FeeModelId => "fee_model_id",
            Self::FeeModelDetailId => "fee_model_detail_id",
            Self::FeeKind => "fee_kind",
            Self::ExchNo => "exch_no",
            Self::SecuFeeType => "secu_fee_type",
            Self::SecuType => "secu_type",
            Self::SecuCode => "secu_code",
            Self::BrokerCoId => "broker_co_id",
            Self::MoneyType => "money_type",
            Self::DmaMode => "dma_mode",
            Self::OrderDir => "order_dir",
            Self::OrderKind => "order_kind",
            Self::ChargeType => "charge_type",
            Self::MathRoundType => "math_round_type",
            Self::MathRoundPlace => "math_round_place",
            Self::FeeRate => "fee_rate",
            Self::FixedFee => "fixed_fee",
            Self::MaxFee => "max_fee",
            Self::MinFee => "min_fee",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageFeeModelDetail {
    const ID: &'static str = "tb_semage_fee_model_detail";
    type Column = SemageFeeModelDetailColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageFeeModelDetailColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageFeeModelDetail {}
