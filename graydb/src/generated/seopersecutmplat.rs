//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_secu_tmplat`（codegen：字段与列名源自 DDL，`SeoperSecuTmplatColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSecuTmplat {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub secu_code_feature: String,
    pub secu_name_feature: String,
    pub model_name: String,
    pub exch_sub_type: i32,
    pub secu_type: i32,
    pub remark_info: String,
}

/// `tb_seoper_secu_tmplat` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSecuTmplatColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    SecuCodeFeature,
    SecuNameFeature,
    ModelName,
    ExchSubType,
    SecuType,
    RemarkInfo,
}

impl ColumnName for SeoperSecuTmplatColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::SecuCodeFeature, Self::SecuNameFeature, Self::ModelName, Self::ExchSubType, Self::SecuType, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::SecuCodeFeature => "secu_code_feature",
            Self::SecuNameFeature => "secu_name_feature",
            Self::ModelName => "model_name",
            Self::ExchSubType => "exch_sub_type",
            Self::SecuType => "secu_type",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperSecuTmplat {
    const ID: &'static str = "tb_seoper_secu_tmplat";
    type Column = SeoperSecuTmplatColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSecuTmplatColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSecuTmplat {}
