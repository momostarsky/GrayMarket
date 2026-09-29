//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_posi_part_out`（codegen：字段与列名源自 DDL，`SemagePosiPartOutColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemagePosiPartOut {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub co_no: i32,
    pub posi_part_batch_no: i32,
    pub pd_no: i32,
    pub pd_code: String,
    pub pd_name: String,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub out_acco: String,
    pub exch_no: i32,
    pub secu_acco: String,
    pub secu_code: String,
    pub secu_name: String,
    pub posi_part_qty: Decimal,
    pub posi_part_cost: Decimal,
    pub posi_part_status: i32,
    pub confirm_flag: i32,
    pub initiator_no: i32,
    pub initiator_user_name: String,
    pub appr_date: i32,
    pub appr_time: i32,
    pub appr_user_no: i32,
    pub appr_user_name: String,
    pub appr_desc: String,
    pub confirm_remark: String,
    pub remark_info: String,
    pub posi_part_date: i32,
    pub capital_acco: String,
    pub custo_acco: String,
}

/// `tb_semage_posi_part_out` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemagePosiPartOutColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    CoNo,
    PosiPartBatchNo,
    PdNo,
    PdCode,
    PdName,
    PdUnitNo,
    AsacNo,
    OutAcco,
    ExchNo,
    SecuAcco,
    SecuCode,
    SecuName,
    PosiPartQty,
    PosiPartCost,
    PosiPartStatus,
    ConfirmFlag,
    InitiatorNo,
    InitiatorUserName,
    ApprDate,
    ApprTime,
    ApprUserNo,
    ApprUserName,
    ApprDesc,
    ConfirmRemark,
    RemarkInfo,
    PosiPartDate,
    CapitalAcco,
    CustoAcco,
}

impl ColumnName for SemagePosiPartOutColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::CoNo, Self::PosiPartBatchNo, Self::PdNo, Self::PdCode, Self::PdName, Self::PdUnitNo, Self::AsacNo, Self::OutAcco, Self::ExchNo, Self::SecuAcco, Self::SecuCode, Self::SecuName, Self::PosiPartQty, Self::PosiPartCost, Self::PosiPartStatus, Self::ConfirmFlag, Self::InitiatorNo, Self::InitiatorUserName, Self::ApprDate, Self::ApprTime, Self::ApprUserNo, Self::ApprUserName, Self::ApprDesc, Self::ConfirmRemark, Self::RemarkInfo, Self::PosiPartDate, Self::CapitalAcco, Self::CustoAcco];
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
            Self::PdNo => "pd_no",
            Self::PdCode => "pd_code",
            Self::PdName => "pd_name",
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::OutAcco => "out_acco",
            Self::ExchNo => "exch_no",
            Self::SecuAcco => "secu_acco",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::PosiPartQty => "posi_part_qty",
            Self::PosiPartCost => "posi_part_cost",
            Self::PosiPartStatus => "posi_part_status",
            Self::ConfirmFlag => "confirm_flag",
            Self::InitiatorNo => "initiator_no",
            Self::InitiatorUserName => "initiator_user_name",
            Self::ApprDate => "appr_date",
            Self::ApprTime => "appr_time",
            Self::ApprUserNo => "appr_user_no",
            Self::ApprUserName => "appr_user_name",
            Self::ApprDesc => "appr_desc",
            Self::ConfirmRemark => "confirm_remark",
            Self::RemarkInfo => "remark_info",
            Self::PosiPartDate => "posi_part_date",
            Self::CapitalAcco => "capital_acco",
            Self::CustoAcco => "custo_acco",
        }
    }
}

impl DataTable for SemagePosiPartOut {
    const ID: &'static str = "tb_semage_posi_part_out";
    type Column = SemagePosiPartOutColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemagePosiPartOutColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemagePosiPartOut {}
