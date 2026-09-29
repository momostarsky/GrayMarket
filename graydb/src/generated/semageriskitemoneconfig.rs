//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_risk_item_one_config`（codegen：字段与列名源自 DDL，`SemageRiskItemOneConfigColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageRiskItemOneConfig {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub risk_item_config_batch_no: i32,
    pub risk_item_config_no: i32,
    pub risk_item_config_name: String,
    pub risk_item_config_content: String,
    pub risk_item_no: i32,
    pub risk_item_code: String,
    pub risk_level: i32,
    pub risk_item_oper_str: String,
    pub risk_item_order_dir_str: String,
    pub risk_item_co_str: String,
    pub risk_item_pd_str: String,
    pub risk_item_pdunit_str: String,
    pub risk_item_asac_str: String,
    pub money_type: i32,
    pub risk_item_value_str: String,
    pub comp_dir: i32,
    pub risk_item_cond_str: String,
    pub risk_item_exec_mode: i32,
    pub risk_item_value_str_two: String,
    pub comp_dir_two: i32,
    pub risk_item_exec_mode_two: i32,
    pub risk_item_value_str_three: String,
    pub comp_dir_three: i32,
    pub risk_item_exec_mode_three: i32,
    pub risk_item_value_str_four: String,
    pub comp_dir_four: i32,
    pub risk_item_exec_mode_four: i32,
    pub risk_item_start_time: i32,
    pub risk_item_end_time: i32,
    pub risk_item_start_date: i32,
    pub risk_item_end_date: i32,
    pub rule_flag: i32,
    pub remark_info: String,
    pub time_stamp: i64,
    pub modi_user_no: i32,
    pub risk_item_config_ctrl_str: String,
}

/// `tb_semage_risk_item_one_config` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageRiskItemOneConfigColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    RiskItemConfigBatchNo,
    RiskItemConfigNo,
    RiskItemConfigName,
    RiskItemConfigContent,
    RiskItemNo,
    RiskItemCode,
    RiskLevel,
    RiskItemOperStr,
    RiskItemOrderDirStr,
    RiskItemCoStr,
    RiskItemPdStr,
    RiskItemPdunitStr,
    RiskItemAsacStr,
    MoneyType,
    RiskItemValueStr,
    CompDir,
    RiskItemCondStr,
    RiskItemExecMode,
    RiskItemValueStrTwo,
    CompDirTwo,
    RiskItemExecModeTwo,
    RiskItemValueStrThree,
    CompDirThree,
    RiskItemExecModeThree,
    RiskItemValueStrFour,
    CompDirFour,
    RiskItemExecModeFour,
    RiskItemStartTime,
    RiskItemEndTime,
    RiskItemStartDate,
    RiskItemEndDate,
    RuleFlag,
    RemarkInfo,
    TimeStamp,
    ModiUserNo,
    RiskItemConfigCtrlStr,
}

impl ColumnName for SemageRiskItemOneConfigColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::RiskItemConfigBatchNo, Self::RiskItemConfigNo, Self::RiskItemConfigName, Self::RiskItemConfigContent, Self::RiskItemNo, Self::RiskItemCode, Self::RiskLevel, Self::RiskItemOperStr, Self::RiskItemOrderDirStr, Self::RiskItemCoStr, Self::RiskItemPdStr, Self::RiskItemPdunitStr, Self::RiskItemAsacStr, Self::MoneyType, Self::RiskItemValueStr, Self::CompDir, Self::RiskItemCondStr, Self::RiskItemExecMode, Self::RiskItemValueStrTwo, Self::CompDirTwo, Self::RiskItemExecModeTwo, Self::RiskItemValueStrThree, Self::CompDirThree, Self::RiskItemExecModeThree, Self::RiskItemValueStrFour, Self::CompDirFour, Self::RiskItemExecModeFour, Self::RiskItemStartTime, Self::RiskItemEndTime, Self::RiskItemStartDate, Self::RiskItemEndDate, Self::RuleFlag, Self::RemarkInfo, Self::TimeStamp, Self::ModiUserNo, Self::RiskItemConfigCtrlStr];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::RiskItemConfigBatchNo => "risk_item_config_batch_no",
            Self::RiskItemConfigNo => "risk_item_config_no",
            Self::RiskItemConfigName => "risk_item_config_name",
            Self::RiskItemConfigContent => "risk_item_config_content",
            Self::RiskItemNo => "risk_item_no",
            Self::RiskItemCode => "risk_item_code",
            Self::RiskLevel => "risk_level",
            Self::RiskItemOperStr => "risk_item_oper_str",
            Self::RiskItemOrderDirStr => "risk_item_order_dir_str",
            Self::RiskItemCoStr => "risk_item_co_str",
            Self::RiskItemPdStr => "risk_item_pd_str",
            Self::RiskItemPdunitStr => "risk_item_pdunit_str",
            Self::RiskItemAsacStr => "risk_item_asac_str",
            Self::MoneyType => "money_type",
            Self::RiskItemValueStr => "risk_item_value_str",
            Self::CompDir => "comp_dir",
            Self::RiskItemCondStr => "risk_item_cond_str",
            Self::RiskItemExecMode => "risk_item_exec_mode",
            Self::RiskItemValueStrTwo => "risk_item_value_str_two",
            Self::CompDirTwo => "comp_dir_two",
            Self::RiskItemExecModeTwo => "risk_item_exec_mode_two",
            Self::RiskItemValueStrThree => "risk_item_value_str_three",
            Self::CompDirThree => "comp_dir_three",
            Self::RiskItemExecModeThree => "risk_item_exec_mode_three",
            Self::RiskItemValueStrFour => "risk_item_value_str_four",
            Self::CompDirFour => "comp_dir_four",
            Self::RiskItemExecModeFour => "risk_item_exec_mode_four",
            Self::RiskItemStartTime => "risk_item_start_time",
            Self::RiskItemEndTime => "risk_item_end_time",
            Self::RiskItemStartDate => "risk_item_start_date",
            Self::RiskItemEndDate => "risk_item_end_date",
            Self::RuleFlag => "rule_flag",
            Self::RemarkInfo => "remark_info",
            Self::TimeStamp => "time_stamp",
            Self::ModiUserNo => "modi_user_no",
            Self::RiskItemConfigCtrlStr => "risk_item_config_ctrl_str",
        }
    }
}

impl DataTable for SemageRiskItemOneConfig {
    const ID: &'static str = "tb_semage_risk_item_one_config";
    type Column = SemageRiskItemOneConfigColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageRiskItemOneConfigColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageRiskItemOneConfig {}
