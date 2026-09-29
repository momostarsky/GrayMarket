//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_static_risk_check_jour`（codegen：字段与列名源自 DDL，`SemageStaticRiskCheckJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageStaticRiskCheckJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub serial_no: String,
    pub co_no: i32,
    pub risk_item_config_no: i32,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub compli_status: i32,
    pub risk_item_type: String,
    pub risk_item_kind: i32,
    pub risk_calc_param_value: Decimal,
    pub risk_check_param_value: Decimal,
    pub risk_item_config_name: String,
    pub risk_item_code: String,
    pub risk_level: i32,
    pub compli_trig_id: i64,
    pub remark_info: String,
}

/// `tb_semage_static_risk_check_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageStaticRiskCheckJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    SerialNo,
    CoNo,
    RiskItemConfigNo,
    PdNo,
    PdUnitNo,
    AsacNo,
    ExchNo,
    SecuCode,
    CompliStatus,
    RiskItemType,
    RiskItemKind,
    RiskCalcParamValue,
    RiskCheckParamValue,
    RiskItemConfigName,
    RiskItemCode,
    RiskLevel,
    CompliTrigId,
    RemarkInfo,
}

impl ColumnName for SemageStaticRiskCheckJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::SerialNo, Self::CoNo, Self::RiskItemConfigNo, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::CompliStatus, Self::RiskItemType, Self::RiskItemKind, Self::RiskCalcParamValue, Self::RiskCheckParamValue, Self::RiskItemConfigName, Self::RiskItemCode, Self::RiskLevel, Self::CompliTrigId, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::SerialNo => "serial_no",
            Self::CoNo => "co_no",
            Self::RiskItemConfigNo => "risk_item_config_no",
            Self::PdNo => "pd_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::CompliStatus => "compli_status",
            Self::RiskItemType => "risk_item_type",
            Self::RiskItemKind => "risk_item_kind",
            Self::RiskCalcParamValue => "risk_calc_param_value",
            Self::RiskCheckParamValue => "risk_check_param_value",
            Self::RiskItemConfigName => "risk_item_config_name",
            Self::RiskItemCode => "risk_item_code",
            Self::RiskLevel => "risk_level",
            Self::CompliTrigId => "compli_trig_id",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageStaticRiskCheckJour {
    const ID: &'static str = "tb_semage_static_risk_check_jour";
    type Column = SemageStaticRiskCheckJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageStaticRiskCheckJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageStaticRiskCheckJour {}
