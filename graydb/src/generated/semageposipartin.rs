//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_posi_part_in`（codegen：字段与列名源自 DDL，`SemagePosiPartInColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemagePosiPartIn {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub co_no: i32,
    pub posi_part_batch_no: i32,
    pub posi_part_no: i32,
    pub pd_no: i32,
    pub pd_code: String,
    pub asac_no: i32,
    pub out_acco: String,
    pub exch_no: i32,
    pub secu_acco: String,
    pub secu_code: String,
    pub secu_name: String,
    pub posi_part_qty: Decimal,
    pub posi_part_cost: Decimal,
    pub posi_part_status: i32,
    pub remark_info: String,
    pub capital_acco: String,
    pub custo_acco: String,
    pub has_margin: i32,
}

/// `tb_semage_posi_part_in` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemagePosiPartInColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    CoNo,
    PosiPartBatchNo,
    PosiPartNo,
    PdNo,
    PdCode,
    AsacNo,
    OutAcco,
    ExchNo,
    SecuAcco,
    SecuCode,
    SecuName,
    PosiPartQty,
    PosiPartCost,
    PosiPartStatus,
    RemarkInfo,
    CapitalAcco,
    CustoAcco,
    HasMargin,
}

impl ColumnName for SemagePosiPartInColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::CoNo, Self::PosiPartBatchNo, Self::PosiPartNo, Self::PdNo, Self::PdCode, Self::AsacNo, Self::OutAcco, Self::ExchNo, Self::SecuAcco, Self::SecuCode, Self::SecuName, Self::PosiPartQty, Self::PosiPartCost, Self::PosiPartStatus, Self::RemarkInfo, Self::CapitalAcco, Self::CustoAcco, Self::HasMargin];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::CoNo => "co_no",
            Self::PosiPartBatchNo => "posi_part_batch_no",
            Self::PosiPartNo => "posi_part_no",
            Self::PdNo => "pd_no",
            Self::PdCode => "pd_code",
            Self::AsacNo => "asac_no",
            Self::OutAcco => "out_acco",
            Self::ExchNo => "exch_no",
            Self::SecuAcco => "secu_acco",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::PosiPartQty => "posi_part_qty",
            Self::PosiPartCost => "posi_part_cost",
            Self::PosiPartStatus => "posi_part_status",
            Self::RemarkInfo => "remark_info",
            Self::CapitalAcco => "capital_acco",
            Self::CustoAcco => "custo_acco",
            Self::HasMargin => "has_margin",
        }
    }
}

impl DataTable for SemagePosiPartIn {
    const ID: &'static str = "tb_semage_posi_part_in";
    type Column = SemagePosiPartInColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemagePosiPartInColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemagePosiPartIn {}
