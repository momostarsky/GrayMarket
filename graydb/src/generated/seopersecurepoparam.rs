//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_secu_repo_param`（codegen：字段与列名源自 DDL，`SeoperSecuRepoParamColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperSecuRepoParam {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub target_code: String,
    pub secu_type: i32,
    pub repo_days: i32,
    pub repo_first_settle_date: i32,
    pub repo_back_date: i32,
    pub cash_capt_days: i32,
}

/// `tb_seoper_secu_repo_param` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperSecuRepoParamColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    ExchNo,
    SecuCode,
    SecuName,
    TargetCode,
    SecuType,
    RepoDays,
    RepoFirstSettleDate,
    RepoBackDate,
    CashCaptDays,
}

impl ColumnName for SeoperSecuRepoParamColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::TargetCode, Self::SecuType, Self::RepoDays, Self::RepoFirstSettleDate, Self::RepoBackDate, Self::CashCaptDays];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::TargetCode => "target_code",
            Self::SecuType => "secu_type",
            Self::RepoDays => "repo_days",
            Self::RepoFirstSettleDate => "repo_first_settle_date",
            Self::RepoBackDate => "repo_back_date",
            Self::CashCaptDays => "cash_capt_days",
        }
    }
}

impl DataTable for SeoperSecuRepoParam {
    const ID: &'static str = "tb_seoper_secu_repo_param";
    type Column = SeoperSecuRepoParamColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperSecuRepoParamColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperSecuRepoParam {}
