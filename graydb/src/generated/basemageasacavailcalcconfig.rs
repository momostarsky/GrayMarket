//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_asac_availcalc_config`（codegen：字段与列名源自 DDL，`BasemageAsacAvailcalcConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageAsacAvailcalcConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub pd_unit_no: i32,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub capit_reback_days: i32,
    pub posi_reback_days: i32,
    pub capit_avail_type: i32,
    pub posi_avail_type: i32,
    pub remark_info: String,
}

/// `tb_basemage_asac_availcalc_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageAsacAvailcalcConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdNo,
    AsacNo,
    PdUnitNo,
    SettleCrncyType,
    ExchCrncyType,
    CapitRebackDays,
    PosiRebackDays,
    CapitAvailType,
    PosiAvailType,
    RemarkInfo,
}

impl ColumnName for BasemageAsacAvailcalcConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdNo, Self::AsacNo, Self::PdUnitNo, Self::SettleCrncyType, Self::ExchCrncyType, Self::CapitRebackDays, Self::PosiRebackDays, Self::CapitAvailType, Self::PosiAvailType, Self::RemarkInfo];
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
            Self::PdUnitNo => "pd_unit_no",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::CapitRebackDays => "capit_reback_days",
            Self::PosiRebackDays => "posi_reback_days",
            Self::CapitAvailType => "capit_avail_type",
            Self::PosiAvailType => "posi_avail_type",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageAsacAvailcalcConfig {
    const ID: &'static str = "tb_basemage_asac_availcalc_config";
    type Column = BasemageAsacAvailcalcConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageAsacAvailcalcConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageAsacAvailcalcConfig {}
