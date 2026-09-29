//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_secu_code_model_fee`（codegen：字段与列名源自 DDL，`SemageSecuCodeModelFeeColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageSecuCodeModelFee {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub model_id: i64,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_fee_type: i32,
    pub order_dir: i32,
    pub amt_ratio: Decimal,
    pub amt_value: Decimal,
    pub par_value_ratio: Decimal,
    pub par_value_value: Decimal,
    pub max_fee: Decimal,
    pub min_fee: Decimal,
    pub float_ratio: Decimal,
    pub fee_choice: i32,
    pub remark_info: String,
}

/// `tb_semage_secu_code_model_fee` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageSecuCodeModelFeeColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    ModelId,
    ExchNo,
    SecuCode,
    SecuFeeType,
    OrderDir,
    AmtRatio,
    AmtValue,
    ParValueRatio,
    ParValueValue,
    MaxFee,
    MinFee,
    FloatRatio,
    FeeChoice,
    RemarkInfo,
}

impl ColumnName for SemageSecuCodeModelFeeColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::ModelId, Self::ExchNo, Self::SecuCode, Self::SecuFeeType, Self::OrderDir, Self::AmtRatio, Self::AmtValue, Self::ParValueRatio, Self::ParValueValue, Self::MaxFee, Self::MinFee, Self::FloatRatio, Self::FeeChoice, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::ModelId => "model_id",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuFeeType => "secu_fee_type",
            Self::OrderDir => "order_dir",
            Self::AmtRatio => "amt_ratio",
            Self::AmtValue => "amt_value",
            Self::ParValueRatio => "par_value_ratio",
            Self::ParValueValue => "par_value_value",
            Self::MaxFee => "max_fee",
            Self::MinFee => "min_fee",
            Self::FloatRatio => "float_ratio",
            Self::FeeChoice => "fee_choice",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageSecuCodeModelFee {
    const ID: &'static str = "tb_semage_secu_code_model_fee";
    type Column = SemageSecuCodeModelFeeColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageSecuCodeModelFeeColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageSecuCodeModelFee {}
