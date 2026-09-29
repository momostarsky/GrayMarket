//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_pd_unit_info`（codegen：字段与列名源自 DDL，`BasemagePdUnitInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemagePdUnitInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub pd_unit_no: i32,
    pub pd_unit_name: String,
    pub pd_unit_status: i32,
    pub main_flag: i32,
    pub pd_no: i32,
    pub co_no: i32,
    pub pd_code: String,
    pub found_date: i32,
    pub pd_unit_share: Decimal,
    pub fund_ratio: Decimal,
    pub warn_posi_value: Decimal,
    pub close_posi_value: Decimal,
    pub target_posi_ratio: Decimal,
    pub beta_coeffi: Decimal,
    pub abolish_date: i32,
    pub instr_appr_oper: i32,
    pub comm_dist_oper: i32,
    pub comm_appo_exor: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub float_ratio: Decimal,
    pub remark_info: String,
    pub allow_secu_type: String,
    pub allow_oper_exch: String,
    pub ta_weight: Decimal,
}

/// `tb_basemage_pd_unit_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemagePdUnitInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    PdUnitNo,
    PdUnitName,
    PdUnitStatus,
    MainFlag,
    PdNo,
    CoNo,
    PdCode,
    FoundDate,
    PdUnitShare,
    FundRatio,
    WarnPosiValue,
    ClosePosiValue,
    TargetPosiRatio,
    BetaCoeffi,
    AbolishDate,
    InstrApprOper,
    CommDistOper,
    CommAppoExor,
    BusiCtrlStr,
    EnableCtrlStr,
    FloatRatio,
    RemarkInfo,
    AllowSecuType,
    AllowOperExch,
    TaWeight,
}

impl ColumnName for BasemagePdUnitInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::PdUnitNo, Self::PdUnitName, Self::PdUnitStatus, Self::MainFlag, Self::PdNo, Self::CoNo, Self::PdCode, Self::FoundDate, Self::PdUnitShare, Self::FundRatio, Self::WarnPosiValue, Self::ClosePosiValue, Self::TargetPosiRatio, Self::BetaCoeffi, Self::AbolishDate, Self::InstrApprOper, Self::CommDistOper, Self::CommAppoExor, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::FloatRatio, Self::RemarkInfo, Self::AllowSecuType, Self::AllowOperExch, Self::TaWeight];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::PdUnitNo => "pd_unit_no",
            Self::PdUnitName => "pd_unit_name",
            Self::PdUnitStatus => "pd_unit_status",
            Self::MainFlag => "main_flag",
            Self::PdNo => "pd_no",
            Self::CoNo => "co_no",
            Self::PdCode => "pd_code",
            Self::FoundDate => "found_date",
            Self::PdUnitShare => "pd_unit_share",
            Self::FundRatio => "fund_ratio",
            Self::WarnPosiValue => "warn_posi_value",
            Self::ClosePosiValue => "close_posi_value",
            Self::TargetPosiRatio => "target_posi_ratio",
            Self::BetaCoeffi => "beta_coeffi",
            Self::AbolishDate => "abolish_date",
            Self::InstrApprOper => "instr_appr_oper",
            Self::CommDistOper => "comm_dist_oper",
            Self::CommAppoExor => "comm_appo_exor",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::FloatRatio => "float_ratio",
            Self::RemarkInfo => "remark_info",
            Self::AllowSecuType => "allow_secu_type",
            Self::AllowOperExch => "allow_oper_exch",
            Self::TaWeight => "ta_weight",
        }
    }
}

impl DataTable for BasemagePdUnitInfo {
    const ID: &'static str = "tb_basemage_pd_unit_info";
    type Column = BasemagePdUnitInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemagePdUnitInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemagePdUnitInfo {}
