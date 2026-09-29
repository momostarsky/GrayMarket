//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_co_custo_file`（codegen：字段与列名源自 DDL，`SemageCoCustoFileColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageCoCustoFile {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub custo_check_item: i32,
    pub custo_id: i32,
    pub date_type: i32,
    pub checkrisk_up: Decimal,
    pub file_name: String,
    pub file_addr: String,
}

/// `tb_semage_co_custo_file` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageCoCustoFileColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    CustoCheckItem,
    CustoId,
    DateType,
    CheckriskUp,
    FileName,
    FileAddr,
}

impl ColumnName for SemageCoCustoFileColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::CustoCheckItem, Self::CustoId, Self::DateType, Self::CheckriskUp, Self::FileName, Self::FileAddr];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::CustoCheckItem => "custo_check_item",
            Self::CustoId => "custo_id",
            Self::DateType => "date_type",
            Self::CheckriskUp => "checkrisk_up",
            Self::FileName => "file_name",
            Self::FileAddr => "file_addr",
        }
    }
}

impl DataTable for SemageCoCustoFile {
    const ID: &'static str = "tb_semage_co_custo_file";
    type Column = SemageCoCustoFileColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageCoCustoFileColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageCoCustoFile {}
