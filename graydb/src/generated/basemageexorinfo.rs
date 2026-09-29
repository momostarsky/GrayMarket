//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_exor_info`（codegen：字段与列名源自 DDL，`BasemageExorInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageExorInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exor_no: i32,
    pub trade_pwd: String,
    pub co_no: i32,
    pub exor_name: String,
    pub exor_status: i32,
    pub exor_group_code: i32,
    pub exor_class: i32,
    pub single_buy_limit_amt: Decimal,
    pub single_sale_limit_amt: Decimal,
    pub single_posi_limit_amt: Decimal,
    pub all_posi_limit_amt: Decimal,
    pub loss_limit_amt: Decimal,
    pub single_force_loss_amt: Decimal,
    pub single_force_loss_ratio: Decimal,
    pub risk_loss_point_ratio: Decimal,
    pub risk_limit_amt_ratio: Decimal,
    pub up_down_limit: Decimal,
    pub busi_ctrl_flag: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub remark_info: String,
}

/// `tb_basemage_exor_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageExorInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExorNo,
    TradePwd,
    CoNo,
    ExorName,
    ExorStatus,
    ExorGroupCode,
    ExorClass,
    SingleBuyLimitAmt,
    SingleSaleLimitAmt,
    SinglePosiLimitAmt,
    AllPosiLimitAmt,
    LossLimitAmt,
    SingleForceLossAmt,
    SingleForceLossRatio,
    RiskLossPointRatio,
    RiskLimitAmtRatio,
    UpDownLimit,
    BusiCtrlFlag,
    BusiCtrlStr,
    EnableCtrlStr,
    RemarkInfo,
}

impl ColumnName for BasemageExorInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExorNo, Self::TradePwd, Self::CoNo, Self::ExorName, Self::ExorStatus, Self::ExorGroupCode, Self::ExorClass, Self::SingleBuyLimitAmt, Self::SingleSaleLimitAmt, Self::SinglePosiLimitAmt, Self::AllPosiLimitAmt, Self::LossLimitAmt, Self::SingleForceLossAmt, Self::SingleForceLossRatio, Self::RiskLossPointRatio, Self::RiskLimitAmtRatio, Self::UpDownLimit, Self::BusiCtrlFlag, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExorNo => "exor_no",
            Self::TradePwd => "trade_pwd",
            Self::CoNo => "co_no",
            Self::ExorName => "exor_name",
            Self::ExorStatus => "exor_status",
            Self::ExorGroupCode => "exor_group_code",
            Self::ExorClass => "exor_class",
            Self::SingleBuyLimitAmt => "single_buy_limit_amt",
            Self::SingleSaleLimitAmt => "single_sale_limit_amt",
            Self::SinglePosiLimitAmt => "single_posi_limit_amt",
            Self::AllPosiLimitAmt => "all_posi_limit_amt",
            Self::LossLimitAmt => "loss_limit_amt",
            Self::SingleForceLossAmt => "single_force_loss_amt",
            Self::SingleForceLossRatio => "single_force_loss_ratio",
            Self::RiskLossPointRatio => "risk_loss_point_ratio",
            Self::RiskLimitAmtRatio => "risk_limit_amt_ratio",
            Self::UpDownLimit => "up_down_limit",
            Self::BusiCtrlFlag => "busi_ctrl_flag",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageExorInfo {
    const ID: &'static str = "tb_basemage_exor_info";
    type Column = BasemageExorInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageExorInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageExorInfo {}
