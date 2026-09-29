//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_fee_model`（codegen：字段与列名源自 DDL，`SemageFeeModelColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageFeeModel {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub fee_model_id: i32,
    pub fee_model_code: String,
    pub fee_model_name: String,
    pub remark_info: String,
}

/// `tb_semage_fee_model` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageFeeModelColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    FeeModelId,
    FeeModelCode,
    FeeModelName,
    RemarkInfo,
}

impl ColumnName for SemageFeeModelColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::FeeModelId, Self::FeeModelCode, Self::FeeModelName, Self::RemarkInfo];
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
            Self::FeeModelCode => "fee_model_code",
            Self::FeeModelName => "fee_model_name",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageFeeModel {
    const ID: &'static str = "tb_semage_fee_model";
    type Column = SemageFeeModelColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageFeeModelColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageFeeModel {}
