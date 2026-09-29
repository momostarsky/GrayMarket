//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_workgroup`（codegen：字段与列名源自 DDL，`BasemageWorkgroupColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageWorkgroup {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub workgroup_id: i32,
    pub workgroup_type: i32,
    pub workgroup_name: String,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub remark_info: String,
}

/// `tb_basemage_workgroup` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageWorkgroupColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    WorkgroupId,
    WorkgroupType,
    WorkgroupName,
    PdNo,
    PdUnitNo,
    RemarkInfo,
}

impl ColumnName for BasemageWorkgroupColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::WorkgroupId, Self::WorkgroupType, Self::WorkgroupName, Self::PdNo, Self::PdUnitNo, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::WorkgroupId => "workgroup_id",
            Self::WorkgroupType => "workgroup_type",
            Self::WorkgroupName => "workgroup_name",
            Self::PdNo => "pd_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageWorkgroup {
    const ID: &'static str = "tb_basemage_workgroup";
    type Column = BasemageWorkgroupColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageWorkgroupColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageWorkgroup {}
