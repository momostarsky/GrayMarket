//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_user_dbgrid_layout`（codegen：字段与列名源自 DDL，`BasemageUserDbgridLayoutColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageUserDbgridLayout {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub user_no: i32,
    pub parent_form: String,
    pub grid_name: String,
    pub grid_key: String,
    pub grid_field: String,
    pub field_width: Decimal,
    pub field_index: i32,
    pub client_type: i32,
    pub is_visible: i32,
    pub allow_sum: i32,
}

/// `tb_basemage_user_dbgrid_layout` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageUserDbgridLayoutColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    UserNo,
    ParentForm,
    GridName,
    GridKey,
    GridField,
    FieldWidth,
    FieldIndex,
    ClientType,
    IsVisible,
    AllowSum,
}

impl ColumnName for BasemageUserDbgridLayoutColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::UserNo, Self::ParentForm, Self::GridName, Self::GridKey, Self::GridField, Self::FieldWidth, Self::FieldIndex, Self::ClientType, Self::IsVisible, Self::AllowSum];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::UserNo => "user_no",
            Self::ParentForm => "parent_form",
            Self::GridName => "grid_name",
            Self::GridKey => "grid_key",
            Self::GridField => "grid_field",
            Self::FieldWidth => "field_width",
            Self::FieldIndex => "field_index",
            Self::ClientType => "client_type",
            Self::IsVisible => "is_visible",
            Self::AllowSum => "allow_sum",
        }
    }
}

impl DataTable for BasemageUserDbgridLayout {
    const ID: &'static str = "tb_basemage_user_dbgrid_layout";
    type Column = BasemageUserDbgridLayoutColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageUserDbgridLayoutColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageUserDbgridLayout {}
