//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdbank_bkac_capit_jour`（codegen：字段与列名源自 DDL，`PdbankBkacCapitJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdbankBkacCapitJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub bk_capit_jour_no: i64,
    pub co_no: i32,
    pub pd_no: i32,
    pub bank_acco_no: i32,
    pub crncy_type: i32,
    pub before_curr_amt: Decimal,
    pub before_intrst_rate: i32,
    pub before_intrst_base_amt: i64,
    pub before_pre_entry_intrst_amt: Decimal,
    pub busi_flag: i32,
    pub occur_amt: Decimal,
    pub deal_status: i32,
    pub curr_amt: Decimal,
    pub intrst_rate: Decimal,
    pub intrst_base_amt: Decimal,
    pub pre_entry_intrst_amt: Decimal,
    pub remark_info: String,
}

/// `tb_pdbank_bkac_capit_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdbankBkacCapitJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    BkCapitJourNo,
    CoNo,
    PdNo,
    BankAccoNo,
    CrncyType,
    BeforeCurrAmt,
    BeforeIntrstRate,
    BeforeIntrstBaseAmt,
    BeforePreEntryIntrstAmt,
    BusiFlag,
    OccurAmt,
    DealStatus,
    CurrAmt,
    IntrstRate,
    IntrstBaseAmt,
    PreEntryIntrstAmt,
    RemarkInfo,
}

impl ColumnName for PdbankBkacCapitJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::BkCapitJourNo, Self::CoNo, Self::PdNo, Self::BankAccoNo, Self::CrncyType, Self::BeforeCurrAmt, Self::BeforeIntrstRate, Self::BeforeIntrstBaseAmt, Self::BeforePreEntryIntrstAmt, Self::BusiFlag, Self::OccurAmt, Self::DealStatus, Self::CurrAmt, Self::IntrstRate, Self::IntrstBaseAmt, Self::PreEntryIntrstAmt, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::BkCapitJourNo => "bk_capit_jour_no",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::BankAccoNo => "bank_acco_no",
            Self::CrncyType => "crncy_type",
            Self::BeforeCurrAmt => "before_curr_amt",
            Self::BeforeIntrstRate => "before_intrst_rate",
            Self::BeforeIntrstBaseAmt => "before_intrst_base_amt",
            Self::BeforePreEntryIntrstAmt => "before_pre_entry_intrst_amt",
            Self::BusiFlag => "busi_flag",
            Self::OccurAmt => "occur_amt",
            Self::DealStatus => "deal_status",
            Self::CurrAmt => "curr_amt",
            Self::IntrstRate => "intrst_rate",
            Self::IntrstBaseAmt => "intrst_base_amt",
            Self::PreEntryIntrstAmt => "pre_entry_intrst_amt",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for PdbankBkacCapitJour {
    const ID: &'static str = "tb_pdbank_bkac_capit_jour";
    type Column = PdbankBkacCapitJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdbankBkacCapitJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdbankBkacCapitJour {}
