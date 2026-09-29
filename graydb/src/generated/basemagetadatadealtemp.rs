//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_ta_data_deal_temp`（codegen：字段与列名源自 DDL，`BasemageTaDataDealTempColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageTaDataDealTemp {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub model_id: i64,
    pub model_name: String,
    pub apply_settle_node: i32,
    pub redeem_settle_node: i32,
    pub ta_frozen_type: i32,
    pub ta_frozen_node: i32,
    pub apply_enable_mode: i32,
}

/// `tb_basemage_ta_data_deal_temp` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageTaDataDealTempColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    ModelId,
    ModelName,
    ApplySettleNode,
    RedeemSettleNode,
    TaFrozenType,
    TaFrozenNode,
    ApplyEnableMode,
}

impl ColumnName for BasemageTaDataDealTempColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::ModelId, Self::ModelName, Self::ApplySettleNode, Self::RedeemSettleNode, Self::TaFrozenType, Self::TaFrozenNode, Self::ApplyEnableMode];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::ModelId => "model_id",
            Self::ModelName => "model_name",
            Self::ApplySettleNode => "apply_settle_node",
            Self::RedeemSettleNode => "redeem_settle_node",
            Self::TaFrozenType => "ta_frozen_type",
            Self::TaFrozenNode => "ta_frozen_node",
            Self::ApplyEnableMode => "apply_enable_mode",
        }
    }
}

impl DataTable for BasemageTaDataDealTemp {
    const ID: &'static str = "tb_basemage_ta_data_deal_temp";
    type Column = BasemageTaDataDealTempColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageTaDataDealTempColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageTaDataDealTemp {}
