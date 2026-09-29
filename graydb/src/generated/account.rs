//! 由 `codegen` 从 `sql/schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::domain::AccountStatus;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `account_info`（codegen：字段与列名源自 DDL，`AccountColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub account_id: String,
    pub user_id: String,
    pub account_name: String,
    pub status: AccountStatus,
}

/// `account_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountColumn {
    AccountId,
    UserId,
    AccountName,
    Status,
}

impl ColumnName for AccountColumn {
    const ALL: &'static [Self] = &[Self::AccountId, Self::UserId, Self::AccountName, Self::Status];
    fn as_str(self) -> &'static str {
        match self {
            Self::AccountId => "account_id",
            Self::UserId => "user_id",
            Self::AccountName => "account_name",
            Self::Status => "status",
        }
    }
}

impl DataTable for Account {
    const ID: &'static str = "account_info";
    type Column = AccountColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            AccountColumn::AccountId => Some(ColVal::Text(&self.account_id)),
            AccountColumn::UserId => Some(ColVal::Text(&self.user_id)),
            _ => None,
        }
    }
}
