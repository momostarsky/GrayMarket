//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_user_pwd_jour`（codegen：字段与列名源自 DDL，`BasemageUserPwdJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageUserPwdJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub user_no: i32,
    pub user_pwd: String,
    pub effective_date: i32,
    pub effective_time: i32,
    pub valid_flag: i32,
}

/// `tb_basemage_user_pwd_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageUserPwdJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    UserNo,
    UserPwd,
    EffectiveDate,
    EffectiveTime,
    ValidFlag,
}

impl ColumnName for BasemageUserPwdJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::UserNo, Self::UserPwd, Self::EffectiveDate, Self::EffectiveTime, Self::ValidFlag];
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
            Self::UserPwd => "user_pwd",
            Self::EffectiveDate => "effective_date",
            Self::EffectiveTime => "effective_time",
            Self::ValidFlag => "valid_flag",
        }
    }
}

impl DataTable for BasemageUserPwdJour {
    const ID: &'static str = "tb_basemage_user_pwd_jour";
    type Column = BasemageUserPwdJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageUserPwdJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageUserPwdJour {}
