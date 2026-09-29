//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_busi_config_dict`（codegen：字段与列名源自 DDL，`BaseoperBusiConfigDictColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperBusiConfigDict {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub config_no: i64,
    pub config_name: String,
    pub config_default_value: String,
    pub config_memo: String,
}

/// `tb_baseoper_busi_config_dict` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperBusiConfigDictColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ConfigNo,
    ConfigName,
    ConfigDefaultValue,
    ConfigMemo,
}

impl ColumnName for BaseoperBusiConfigDictColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ConfigNo, Self::ConfigName, Self::ConfigDefaultValue, Self::ConfigMemo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ConfigNo => "config_no",
            Self::ConfigName => "config_name",
            Self::ConfigDefaultValue => "config_default_value",
            Self::ConfigMemo => "config_memo",
        }
    }
}

impl DataTable for BaseoperBusiConfigDict {
    const ID: &'static str = "tb_baseoper_busi_config_dict";
    type Column = BaseoperBusiConfigDictColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperBusiConfigDictColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperBusiConfigDict {}
