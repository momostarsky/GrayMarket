//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_out_object`（codegen：字段与列名源自 DDL，`SemageOutObjectColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageOutObject {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub pos_str: String,
    pub object_rights: i32,
    pub finance_bail_ratio: Decimal,
    pub shortsell_bail_ratio: Decimal,
    pub mortgage_ratio: Decimal,
}

/// `tb_semage_out_object` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageOutObjectColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdNo,
    AsacNo,
    ExchNo,
    SecuCode,
    SecuName,
    PosStr,
    ObjectRights,
    FinanceBailRatio,
    ShortsellBailRatio,
    MortgageRatio,
}

impl ColumnName for SemageOutObjectColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::PosStr, Self::ObjectRights, Self::FinanceBailRatio, Self::ShortsellBailRatio, Self::MortgageRatio];
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
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::PosStr => "pos_str",
            Self::ObjectRights => "object_rights",
            Self::FinanceBailRatio => "finance_bail_ratio",
            Self::ShortsellBailRatio => "shortsell_bail_ratio",
            Self::MortgageRatio => "mortgage_ratio",
        }
    }
}

impl DataTable for SemageOutObject {
    const ID: &'static str = "tb_semage_out_object";
    type Column = SemageOutObjectColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageOutObjectColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageOutObject {}
