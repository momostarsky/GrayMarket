//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_menu_formfunc`（codegen：字段与列名源自 DDL，`BaseoperMenuFormfuncColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperMenuFormfunc {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub busi_menu_no: i32,
    pub form_func_code: String,
    pub func_name: String,
    pub form_func_rights_bit: i32,
    pub menu_name_flag: String,
}

/// `tb_baseoper_menu_formfunc` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperMenuFormfuncColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    BusiMenuNo,
    FormFuncCode,
    FuncName,
    FormFuncRightsBit,
    MenuNameFlag,
}

impl ColumnName for BaseoperMenuFormfuncColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::BusiMenuNo, Self::FormFuncCode, Self::FuncName, Self::FormFuncRightsBit, Self::MenuNameFlag];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::BusiMenuNo => "busi_menu_no",
            Self::FormFuncCode => "form_func_code",
            Self::FuncName => "func_name",
            Self::FormFuncRightsBit => "form_func_rights_bit",
            Self::MenuNameFlag => "menu_name_flag",
        }
    }
}

impl DataTable for BaseoperMenuFormfunc {
    const ID: &'static str = "tb_baseoper_menu_formfunc";
    type Column = BaseoperMenuFormfuncColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperMenuFormfuncColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperMenuFormfunc {}
