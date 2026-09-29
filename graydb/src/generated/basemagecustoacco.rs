//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_custo_acco`（codegen：字段与列名源自 DDL，`BasemageCustoAccoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageCustoAcco {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub custo_id: i32,
    pub custo_acco_no: i32,
    pub custo_acco_name: String,
    pub custo_acco: String,
    pub custo_acco_key: String,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub remark_info: String,
}

/// `tb_basemage_custo_acco` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageCustoAccoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdNo,
    AsacNo,
    CustoId,
    CustoAccoNo,
    CustoAccoName,
    CustoAcco,
    CustoAccoKey,
    BusiCtrlStr,
    EnableCtrlStr,
    RemarkInfo,
}

impl ColumnName for BasemageCustoAccoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdNo, Self::AsacNo, Self::CustoId, Self::CustoAccoNo, Self::CustoAccoName, Self::CustoAcco, Self::CustoAccoKey, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::AsacNo => "asac_no",
            Self::CustoId => "custo_id",
            Self::CustoAccoNo => "custo_acco_no",
            Self::CustoAccoName => "custo_acco_name",
            Self::CustoAcco => "custo_acco",
            Self::CustoAccoKey => "custo_acco_key",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageCustoAcco {
    const ID: &'static str = "tb_basemage_custo_acco";
    type Column = BasemageCustoAccoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageCustoAccoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageCustoAcco {}
