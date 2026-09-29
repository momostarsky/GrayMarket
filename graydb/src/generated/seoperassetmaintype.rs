//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_asset_main_type`（codegen：字段与列名源自 DDL，`SeoperAssetMainTypeColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperAssetMainType {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub asset_main_type: i32,
    pub asset_type_str: String,
    pub remark_info: String,
}

/// `tb_seoper_asset_main_type` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperAssetMainTypeColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    AssetMainType,
    AssetTypeStr,
    RemarkInfo,
}

impl ColumnName for SeoperAssetMainTypeColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::AssetMainType, Self::AssetTypeStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::AssetMainType => "asset_main_type",
            Self::AssetTypeStr => "asset_type_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperAssetMainType {
    const ID: &'static str = "tb_seoper_asset_main_type";
    type Column = SeoperAssetMainTypeColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperAssetMainTypeColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperAssetMainType {}
