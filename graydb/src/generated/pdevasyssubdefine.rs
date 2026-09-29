//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdeva_sys_sub_define`（codegen：字段与列名源自 DDL，`PdevaSysSubDefineColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdevaSysSubDefine {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub subject_code: String,
    pub subject_name: String,
    pub asset_type: i32,
    pub diff_crncy: i32,
    pub sub_source: i32,
    pub invest_area: i32,
    pub busi_flag: i32,
    pub remark_info: String,
}

/// `tb_pdeva_sys_sub_define` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdevaSysSubDefineColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    SubjectCode,
    SubjectName,
    AssetType,
    DiffCrncy,
    SubSource,
    InvestArea,
    BusiFlag,
    RemarkInfo,
}

impl ColumnName for PdevaSysSubDefineColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::SubjectCode, Self::SubjectName, Self::AssetType, Self::DiffCrncy, Self::SubSource, Self::InvestArea, Self::BusiFlag, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::SubjectCode => "subject_code",
            Self::SubjectName => "subject_name",
            Self::AssetType => "asset_type",
            Self::DiffCrncy => "diff_crncy",
            Self::SubSource => "sub_source",
            Self::InvestArea => "invest_area",
            Self::BusiFlag => "busi_flag",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for PdevaSysSubDefine {
    const ID: &'static str = "tb_pdeva_sys_sub_define";
    type Column = PdevaSysSubDefineColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdevaSysSubDefineColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdevaSysSubDefine {}
