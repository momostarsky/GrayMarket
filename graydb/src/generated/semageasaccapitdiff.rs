//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_asac_capit_diff`（codegen：字段与列名源自 DDL，`SemageAsacCapitDiffColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageAsacCapitDiff {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub date_type: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub custo_id: i32,
    pub settle_crncy_type: i32,
    pub sys_in_value: Decimal,
    pub sys_out_value: Decimal,
    pub check_value_diff: Decimal,
    pub check_status: i32,
    pub remark_info: String,
}

/// `tb_semage_asac_capit_diff` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageAsacCapitDiffColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    DateType,
    CoNo,
    PdNo,
    AsacNo,
    CustoId,
    SettleCrncyType,
    SysInValue,
    SysOutValue,
    CheckValueDiff,
    CheckStatus,
    RemarkInfo,
}

impl ColumnName for SemageAsacCapitDiffColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::DateType, Self::CoNo, Self::PdNo, Self::AsacNo, Self::CustoId, Self::SettleCrncyType, Self::SysInValue, Self::SysOutValue, Self::CheckValueDiff, Self::CheckStatus, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::DateType => "date_type",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::CustoId => "custo_id",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::SysInValue => "sys_in_value",
            Self::SysOutValue => "sys_out_value",
            Self::CheckValueDiff => "check_value_diff",
            Self::CheckStatus => "check_status",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageAsacCapitDiff {
    const ID: &'static str = "tb_semage_asac_capit_diff";
    type Column = SemageAsacCapitDiffColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageAsacCapitDiffColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageAsacCapitDiff {}
