//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_ctmcheck_risk`（codegen：字段与列名源自 DDL，`BasemageCtmcheckRiskColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageCtmcheckRisk {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub secu_type: i32,
    pub check_item: i32,
    pub exch_crncy_type: i32,
    pub check_rule: i32,
    pub checkrisk_up: Decimal,
    pub checkrisk_down: Decimal,
}

/// `tb_basemage_ctmcheck_risk` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageCtmcheckRiskColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    SecuType,
    CheckItem,
    ExchCrncyType,
    CheckRule,
    CheckriskUp,
    CheckriskDown,
}

impl ColumnName for BasemageCtmcheckRiskColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::SecuType, Self::CheckItem, Self::ExchCrncyType, Self::CheckRule, Self::CheckriskUp, Self::CheckriskDown];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::SecuType => "secu_type",
            Self::CheckItem => "check_item",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::CheckRule => "check_rule",
            Self::CheckriskUp => "checkrisk_up",
            Self::CheckriskDown => "checkrisk_down",
        }
    }
}

impl DataTable for BasemageCtmcheckRisk {
    const ID: &'static str = "tb_basemage_ctmcheck_risk";
    type Column = BasemageCtmcheckRiskColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageCtmcheckRiskColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageCtmcheckRisk {}
