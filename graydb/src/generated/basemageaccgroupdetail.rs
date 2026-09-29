//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_acc_group_detail`（codegen：字段与列名源自 DDL，`BasemageAccGroupDetailColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageAccGroupDetail {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub group_no: i32,
    pub group_id: i32,
    pub exor_no: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub pd_unit_no: i32,
    pub weight_ratio: Decimal,
    pub remark_info: String,
    pub discount_ratio: Decimal,
}

/// `tb_basemage_acc_group_detail` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageAccGroupDetailColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    GroupNo,
    GroupId,
    ExorNo,
    CoNo,
    PdNo,
    AsacNo,
    PdUnitNo,
    WeightRatio,
    RemarkInfo,
    DiscountRatio,
}

impl ColumnName for BasemageAccGroupDetailColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::GroupNo, Self::GroupId, Self::ExorNo, Self::CoNo, Self::PdNo, Self::AsacNo, Self::PdUnitNo, Self::WeightRatio, Self::RemarkInfo, Self::DiscountRatio];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::GroupNo => "group_no",
            Self::GroupId => "group_id",
            Self::ExorNo => "exor_no",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::WeightRatio => "weight_ratio",
            Self::RemarkInfo => "remark_info",
            Self::DiscountRatio => "discount_ratio",
        }
    }
}

impl DataTable for BasemageAccGroupDetail {
    const ID: &'static str = "tb_basemage_acc_group_detail";
    type Column = BasemageAccGroupDetailColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageAccGroupDetailColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageAccGroupDetail {}
