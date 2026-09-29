//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_warning_jour`（codegen：字段与列名源自 DDL，`SemageRiskWarningJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskWarningJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub serial_no: String,
    pub comm_batch_no: i64,
    pub instr_no: String,
    pub orig_instr_no: String,
    pub order_batch_no: i64,
    pub external_no: String,
    pub orig_external_no: String,
    pub co_no: i32,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub exor_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub exch_sub_type: i32,
    pub secu_type: i32,
    pub exch_crncy_type: i32,
    pub secu_name: String,
    pub order_dir: i32,
    pub order_qty: Decimal,
    pub order_price: Decimal,
    pub last_price: Decimal,
    pub cost_price: Decimal,
    pub compli_status: i32,
    pub compli_trig_id: i64,
    pub risk_item_code: String,
    pub risk_item_oper_type: i32,
    pub risk_calc_param_value: Decimal,
    pub risk_check_param_value: Decimal,
    pub risk_item_exec_mode: i32,
    pub risk_item_config_no: i32,
    pub risk_item_kind: i32,
    pub risk_item_config_name: String,
    pub risk_item_type: String,
    pub busi_msg_content: String,
    pub appr_user_no: i32,
    pub appr_date: i32,
    pub appr_time: i32,
    pub appr_status: i32,
    pub appr_desc: String,
    pub remark_info: String,
    pub risk_source: i32,
    pub risk_level: i32,
}

/// `tb_semage_risk_warning_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskWarningJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    SerialNo,
    CommBatchNo,
    InstrNo,
    OrigInstrNo,
    OrderBatchNo,
    ExternalNo,
    OrigExternalNo,
    CoNo,
    PdNo,
    PdUnitNo,
    AsacNo,
    ExorNo,
    ExchNo,
    SecuCode,
    ExchSubType,
    SecuType,
    ExchCrncyType,
    SecuName,
    OrderDir,
    OrderQty,
    OrderPrice,
    LastPrice,
    CostPrice,
    CompliStatus,
    CompliTrigId,
    RiskItemCode,
    RiskItemOperType,
    RiskCalcParamValue,
    RiskCheckParamValue,
    RiskItemExecMode,
    RiskItemConfigNo,
    RiskItemKind,
    RiskItemConfigName,
    RiskItemType,
    BusiMsgContent,
    ApprUserNo,
    ApprDate,
    ApprTime,
    ApprStatus,
    ApprDesc,
    RemarkInfo,
    RiskSource,
    RiskLevel,
}

impl ColumnName for SemageRiskWarningJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::SerialNo, Self::CommBatchNo, Self::InstrNo, Self::OrigInstrNo, Self::OrderBatchNo, Self::ExternalNo, Self::OrigExternalNo, Self::CoNo, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::ExorNo, Self::ExchNo, Self::SecuCode, Self::ExchSubType, Self::SecuType, Self::ExchCrncyType, Self::SecuName, Self::OrderDir, Self::OrderQty, Self::OrderPrice, Self::LastPrice, Self::CostPrice, Self::CompliStatus, Self::CompliTrigId, Self::RiskItemCode, Self::RiskItemOperType, Self::RiskCalcParamValue, Self::RiskCheckParamValue, Self::RiskItemExecMode, Self::RiskItemConfigNo, Self::RiskItemKind, Self::RiskItemConfigName, Self::RiskItemType, Self::BusiMsgContent, Self::ApprUserNo, Self::ApprDate, Self::ApprTime, Self::ApprStatus, Self::ApprDesc, Self::RemarkInfo, Self::RiskSource, Self::RiskLevel];
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
            Self::CommBatchNo => "comm_batch_no",
            Self::InstrNo => "instr_no",
            Self::OrigInstrNo => "orig_instr_no",
            Self::OrderBatchNo => "order_batch_no",
            Self::ExternalNo => "external_no",
            Self::OrigExternalNo => "orig_external_no",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::ExorNo => "exor_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::ExchSubType => "exch_sub_type",
            Self::SecuType => "secu_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::SecuName => "secu_name",
            Self::OrderDir => "order_dir",
            Self::OrderQty => "order_qty",
            Self::OrderPrice => "order_price",
            Self::LastPrice => "last_price",
            Self::CostPrice => "cost_price",
            Self::CompliStatus => "compli_status",
            Self::CompliTrigId => "compli_trig_id",
            Self::RiskItemCode => "risk_item_code",
            Self::RiskItemOperType => "risk_item_oper_type",
            Self::RiskCalcParamValue => "risk_calc_param_value",
            Self::RiskCheckParamValue => "risk_check_param_value",
            Self::RiskItemExecMode => "risk_item_exec_mode",
            Self::RiskItemConfigNo => "risk_item_config_no",
            Self::RiskItemKind => "risk_item_kind",
            Self::RiskItemConfigName => "risk_item_config_name",
            Self::RiskItemType => "risk_item_type",
            Self::BusiMsgContent => "busi_msg_content",
            Self::ApprUserNo => "appr_user_no",
            Self::ApprDate => "appr_date",
            Self::ApprTime => "appr_time",
            Self::ApprStatus => "appr_status",
            Self::ApprDesc => "appr_desc",
            Self::RemarkInfo => "remark_info",
            Self::RiskSource => "risk_source",
            Self::RiskLevel => "risk_level",
        }
    }
}

impl DataTable for SemageRiskWarningJour {
    const ID: &'static str = "tb_semage_risk_warning_jour";
    type Column = SemageRiskWarningJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskWarningJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskWarningJour {}
