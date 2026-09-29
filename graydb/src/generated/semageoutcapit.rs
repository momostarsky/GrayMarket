//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_out_capit`（codegen：字段与列名源自 DDL，`SemageOutCapitColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageOutCapit {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub asac_no: i32,
    pub settle_crncy_type: i32,
    pub begin_amt: Decimal,
    pub curr_amt: Decimal,
    pub avail_amt: Decimal,
    pub fetch_amt: Decimal,
    pub frozen_amt: Decimal,
    pub unfrozen_amt: Decimal,
    pub stock_balance: Decimal,
    pub nav_asset: Decimal,
}

/// `tb_semage_out_capit` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageOutCapitColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    PdNo,
    AsacNo,
    SettleCrncyType,
    BeginAmt,
    CurrAmt,
    AvailAmt,
    FetchAmt,
    FrozenAmt,
    UnfrozenAmt,
    StockBalance,
    NavAsset,
}

impl ColumnName for SemageOutCapitColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::PdNo, Self::AsacNo, Self::SettleCrncyType, Self::BeginAmt, Self::CurrAmt, Self::AvailAmt, Self::FetchAmt, Self::FrozenAmt, Self::UnfrozenAmt, Self::StockBalance, Self::NavAsset];
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
            Self::SettleCrncyType => "settle_crncy_type",
            Self::BeginAmt => "begin_amt",
            Self::CurrAmt => "curr_amt",
            Self::AvailAmt => "avail_amt",
            Self::FetchAmt => "fetch_amt",
            Self::FrozenAmt => "frozen_amt",
            Self::UnfrozenAmt => "unfrozen_amt",
            Self::StockBalance => "stock_balance",
            Self::NavAsset => "nav_asset",
        }
    }
}

impl DataTable for SemageOutCapit {
    const ID: &'static str = "tb_semage_out_capit";
    type Column = SemageOutCapitColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageOutCapitColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageOutCapit {}
