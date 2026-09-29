//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_asac_fee_model`（codegen：字段与列名源自 DDL，`SemageAsacFeeModelColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageAsacFeeModel {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub asac_no: i32,
    pub out_acco_id: i32,
    pub fee_model_type: i32,
    pub fee_model_kind: i32,
    pub model_id: i64,
    pub remark_info: String,
}

/// `tb_semage_asac_fee_model` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageAsacFeeModelColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    AsacNo,
    OutAccoId,
    FeeModelType,
    FeeModelKind,
    ModelId,
    RemarkInfo,
}

impl ColumnName for SemageAsacFeeModelColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::AsacNo, Self::OutAccoId, Self::FeeModelType, Self::FeeModelKind, Self::ModelId, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::AsacNo => "asac_no",
            Self::OutAccoId => "out_acco_id",
            Self::FeeModelType => "fee_model_type",
            Self::FeeModelKind => "fee_model_kind",
            Self::ModelId => "model_id",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageAsacFeeModel {
    const ID: &'static str = "tb_semage_asac_fee_model";
    type Column = SemageAsacFeeModelColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageAsacFeeModelColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageAsacFeeModel {}
