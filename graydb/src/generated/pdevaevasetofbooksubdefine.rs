//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdeva_eva_set_of_book_sub_define`（codegen：字段与列名源自 DDL，`PdevaEvaSetOfBookSubDefineColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdevaEvaSetOfBookSubDefine {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub set_of_book_no: i32,
    pub financial_account_code: String,
    pub financial_account_name: String,
    pub asset_type: i32,
    pub freeze_flag: i32,
    pub direction: i32,
    pub subject_code: String,
    pub money_type: i32,
    pub pupil_flag: i32,
    pub exch_no: i32,
    pub remark_info: String,
}

/// `tb_pdeva_eva_set_of_book_sub_define` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdevaEvaSetOfBookSubDefineColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    SetOfBookNo,
    FinancialAccountCode,
    FinancialAccountName,
    AssetType,
    FreezeFlag,
    Direction,
    SubjectCode,
    MoneyType,
    PupilFlag,
    ExchNo,
    RemarkInfo,
}

impl ColumnName for PdevaEvaSetOfBookSubDefineColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::SetOfBookNo, Self::FinancialAccountCode, Self::FinancialAccountName, Self::AssetType, Self::FreezeFlag, Self::Direction, Self::SubjectCode, Self::MoneyType, Self::PupilFlag, Self::ExchNo, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::SetOfBookNo => "set_of_book_no",
            Self::FinancialAccountCode => "financial_account_code",
            Self::FinancialAccountName => "financial_account_name",
            Self::AssetType => "asset_type",
            Self::FreezeFlag => "freeze_flag",
            Self::Direction => "direction",
            Self::SubjectCode => "subject_code",
            Self::MoneyType => "money_type",
            Self::PupilFlag => "pupil_flag",
            Self::ExchNo => "exch_no",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for PdevaEvaSetOfBookSubDefine {
    const ID: &'static str = "tb_pdeva_eva_set_of_book_sub_define";
    type Column = PdevaEvaSetOfBookSubDefineColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdevaEvaSetOfBookSubDefineColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdevaEvaSetOfBookSubDefine {}
