//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdmage_pd_unit_frozen_unfrozen_capit`（codegen：字段与列名源自 DDL，`PdmagePdUnitFrozenUnfrozenCapitColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdmagePdUnitFrozenUnfrozenCapit {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub adjust_jour_no: i64,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub pd_unit_no: i32,
    pub settle_crncy_type: i32,
    pub busi_flag: i32,
    pub adjust_amt: Decimal,
    pub expired_date: i32,
    pub opor_no: i32,
    pub deal_status: i32,
    pub cancel_date: i32,
    pub cancel_time: i32,
    pub remark_info: String,
}

/// `tb_pdmage_pd_unit_frozen_unfrozen_capit` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdmagePdUnitFrozenUnfrozenCapitColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    AdjustJourNo,
    CoNo,
    PdNo,
    AsacNo,
    PdUnitNo,
    SettleCrncyType,
    BusiFlag,
    AdjustAmt,
    ExpiredDate,
    OporNo,
    DealStatus,
    CancelDate,
    CancelTime,
    RemarkInfo,
}

impl ColumnName for PdmagePdUnitFrozenUnfrozenCapitColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::AdjustJourNo, Self::CoNo, Self::PdNo, Self::AsacNo, Self::PdUnitNo, Self::SettleCrncyType, Self::BusiFlag, Self::AdjustAmt, Self::ExpiredDate, Self::OporNo, Self::DealStatus, Self::CancelDate, Self::CancelTime, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::AdjustJourNo => "adjust_jour_no",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::BusiFlag => "busi_flag",
            Self::AdjustAmt => "adjust_amt",
            Self::ExpiredDate => "expired_date",
            Self::OporNo => "opor_no",
            Self::DealStatus => "deal_status",
            Self::CancelDate => "cancel_date",
            Self::CancelTime => "cancel_time",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for PdmagePdUnitFrozenUnfrozenCapit {
    const ID: &'static str = "tb_pdmage_pd_unit_frozen_unfrozen_capit";
    type Column = PdmagePdUnitFrozenUnfrozenCapitColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdmagePdUnitFrozenUnfrozenCapitColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdmagePdUnitFrozenUnfrozenCapit {}
