//! 由 `codegen` 从 `sql/schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::domain::{IdType, UserStatus};
use crate::tables::{ColVal, ColumnName, DataTable};

/// `user_info`（codegen：字段与列名源自 DDL，`UserColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub phone: String,
    pub id_type: IdType,
    pub id_number: String,
    pub status: UserStatus,
    pub opened_at: String,
}

/// `user_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserColumn {
    UserId,
    Username,
    Phone,
    IdType,
    IdNumber,
    Status,
    OpenedAt,
}

impl ColumnName for UserColumn {
    const ALL: &'static [Self] = &[Self::UserId, Self::Username, Self::Phone, Self::IdType, Self::IdNumber, Self::Status, Self::OpenedAt];
    fn as_str(self) -> &'static str {
        match self {
            Self::UserId => "user_id",
            Self::Username => "username",
            Self::Phone => "phone",
            Self::IdType => "id_type",
            Self::IdNumber => "id_number",
            Self::Status => "status",
            Self::OpenedAt => "opened_at",
        }
    }
}

impl DataTable for User {
    const ID: &'static str = "user_info";
    type Column = UserColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            UserColumn::UserId => Some(ColVal::Text(&self.user_id)),
            _ => None,
        }
    }
}
