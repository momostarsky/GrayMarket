//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_pd_info`（codegen：字段与列名源自 DDL，`BasemagePdInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemagePdInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub pd_no: i32,
    pub co_no: i32,
    pub pd_code: String,
    pub pd_name: String,
    pub pd_flname: String,
    pub pd_type: i32,
    pub invest_area: i32,
    pub pd_mode: i32,
    pub fund_reg_code: String,
    pub pd_manager: String,
    pub found_date: i32,
    pub coust_full_name: String,
    pub coust_acco: String,
    pub coust_acco_name: String,
    pub settle_crncy_type: i32,
    pub first_asset: Decimal,
    pub first_amt: Decimal,
    pub pd_share: Decimal,
    pub warn_posi_value: Decimal,
    pub close_posi_value: Decimal,
    pub target_posi_ratio: Decimal,
    pub beta_coeffi: Decimal,
    pub custom_pd_class: String,
    pub pd_status: i32,
    pub abolish_date: i32,
    pub instr_appr_oper: i32,
    pub comm_dist_oper: i32,
    pub comm_appo_exor: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub pd_unit_no: i32,
    pub float_ratio: Decimal,
    pub remark_info: String,
    pub eva_reconciliation: i32,
    pub set_of_book_no: i32,
    pub inside_pd_unit_no: i32,
    pub outside_pd_unit_no: i32,
    pub fund_code: String,
    pub ta_model_id: i64,
    pub ta_amt_settle_type: i32,
    pub ta_import_flag: i32,
    pub pd_init_date: i32,
}

/// `tb_basemage_pd_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemagePdInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    PdNo,
    CoNo,
    PdCode,
    PdName,
    PdFlname,
    PdType,
    InvestArea,
    PdMode,
    FundRegCode,
    PdManager,
    FoundDate,
    CoustFullName,
    CoustAcco,
    CoustAccoName,
    SettleCrncyType,
    FirstAsset,
    FirstAmt,
    PdShare,
    WarnPosiValue,
    ClosePosiValue,
    TargetPosiRatio,
    BetaCoeffi,
    CustomPdClass,
    PdStatus,
    AbolishDate,
    InstrApprOper,
    CommDistOper,
    CommAppoExor,
    BusiCtrlStr,
    EnableCtrlStr,
    PdUnitNo,
    FloatRatio,
    RemarkInfo,
    EvaReconciliation,
    SetOfBookNo,
    InsidePdUnitNo,
    OutsidePdUnitNo,
    FundCode,
    TaModelId,
    TaAmtSettleType,
    TaImportFlag,
    PdInitDate,
}

impl ColumnName for BasemagePdInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::PdNo, Self::CoNo, Self::PdCode, Self::PdName, Self::PdFlname, Self::PdType, Self::InvestArea, Self::PdMode, Self::FundRegCode, Self::PdManager, Self::FoundDate, Self::CoustFullName, Self::CoustAcco, Self::CoustAccoName, Self::SettleCrncyType, Self::FirstAsset, Self::FirstAmt, Self::PdShare, Self::WarnPosiValue, Self::ClosePosiValue, Self::TargetPosiRatio, Self::BetaCoeffi, Self::CustomPdClass, Self::PdStatus, Self::AbolishDate, Self::InstrApprOper, Self::CommDistOper, Self::CommAppoExor, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::PdUnitNo, Self::FloatRatio, Self::RemarkInfo, Self::EvaReconciliation, Self::SetOfBookNo, Self::InsidePdUnitNo, Self::OutsidePdUnitNo, Self::FundCode, Self::TaModelId, Self::TaAmtSettleType, Self::TaImportFlag, Self::PdInitDate];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::PdNo => "pd_no",
            Self::CoNo => "co_no",
            Self::PdCode => "pd_code",
            Self::PdName => "pd_name",
            Self::PdFlname => "pd_flname",
            Self::PdType => "pd_type",
            Self::InvestArea => "invest_area",
            Self::PdMode => "pd_mode",
            Self::FundRegCode => "fund_reg_code",
            Self::PdManager => "pd_manager",
            Self::FoundDate => "found_date",
            Self::CoustFullName => "coust_full_name",
            Self::CoustAcco => "coust_acco",
            Self::CoustAccoName => "coust_acco_name",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::FirstAsset => "first_asset",
            Self::FirstAmt => "first_amt",
            Self::PdShare => "pd_share",
            Self::WarnPosiValue => "warn_posi_value",
            Self::ClosePosiValue => "close_posi_value",
            Self::TargetPosiRatio => "target_posi_ratio",
            Self::BetaCoeffi => "beta_coeffi",
            Self::CustomPdClass => "custom_pd_class",
            Self::PdStatus => "pd_status",
            Self::AbolishDate => "abolish_date",
            Self::InstrApprOper => "instr_appr_oper",
            Self::CommDistOper => "comm_dist_oper",
            Self::CommAppoExor => "comm_appo_exor",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::PdUnitNo => "pd_unit_no",
            Self::FloatRatio => "float_ratio",
            Self::RemarkInfo => "remark_info",
            Self::EvaReconciliation => "eva_reconciliation",
            Self::SetOfBookNo => "set_of_book_no",
            Self::InsidePdUnitNo => "inside_pd_unit_no",
            Self::OutsidePdUnitNo => "outside_pd_unit_no",
            Self::FundCode => "fund_code",
            Self::TaModelId => "ta_model_id",
            Self::TaAmtSettleType => "ta_amt_settle_type",
            Self::TaImportFlag => "ta_import_flag",
            Self::PdInitDate => "pd_init_date",
        }
    }
}

impl DataTable for BasemagePdInfo {
    const ID: &'static str = "tb_basemage_pd_info";
    type Column = BasemagePdInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemagePdInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemagePdInfo {}
