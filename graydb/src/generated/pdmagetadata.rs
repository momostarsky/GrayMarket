//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdmage_ta_data`（codegen：字段与列名源自 DDL，`PdmageTaDataColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdmageTaData {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub file_name: String,
    pub ta_type: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub fund_code: String,
    pub busi_flag: i32,
    pub apply_date: i32,
    pub strike_share: Decimal,
    pub strike_amt: Decimal,
    pub money_type: i32,
    pub all_fee: Decimal,
    pub trade_fee: Decimal,
    pub trans_fee: Decimal,
    pub stamp_tax: Decimal,
    pub after_fee: Decimal,
    pub other_fee: Decimal,
    pub belong_fund_asset_fee: Decimal,
    pub settle_date: i32,
    pub deal_date: i32,
    pub unit_dividend: Decimal,
    pub dealer: String,
    pub valid_flag: i32,
    pub remark_info: String,
}

/// `tb_pdmage_ta_data` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdmageTaDataColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    FileName,
    TaType,
    CoNo,
    PdNo,
    FundCode,
    BusiFlag,
    ApplyDate,
    StrikeShare,
    StrikeAmt,
    MoneyType,
    AllFee,
    TradeFee,
    TransFee,
    StampTax,
    AfterFee,
    OtherFee,
    BelongFundAssetFee,
    SettleDate,
    DealDate,
    UnitDividend,
    Dealer,
    ValidFlag,
    RemarkInfo,
}

impl ColumnName for PdmageTaDataColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::FileName, Self::TaType, Self::CoNo, Self::PdNo, Self::FundCode, Self::BusiFlag, Self::ApplyDate, Self::StrikeShare, Self::StrikeAmt, Self::MoneyType, Self::AllFee, Self::TradeFee, Self::TransFee, Self::StampTax, Self::AfterFee, Self::OtherFee, Self::BelongFundAssetFee, Self::SettleDate, Self::DealDate, Self::UnitDividend, Self::Dealer, Self::ValidFlag, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::FileName => "file_name",
            Self::TaType => "ta_type",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::FundCode => "fund_code",
            Self::BusiFlag => "busi_flag",
            Self::ApplyDate => "apply_date",
            Self::StrikeShare => "strike_share",
            Self::StrikeAmt => "strike_amt",
            Self::MoneyType => "money_type",
            Self::AllFee => "all_fee",
            Self::TradeFee => "trade_fee",
            Self::TransFee => "trans_fee",
            Self::StampTax => "stamp_tax",
            Self::AfterFee => "after_fee",
            Self::OtherFee => "other_fee",
            Self::BelongFundAssetFee => "belong_fund_asset_fee",
            Self::SettleDate => "settle_date",
            Self::DealDate => "deal_date",
            Self::UnitDividend => "unit_dividend",
            Self::Dealer => "dealer",
            Self::ValidFlag => "valid_flag",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for PdmageTaData {
    const ID: &'static str = "tb_pdmage_ta_data";
    type Column = PdmageTaDataColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdmageTaDataColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdmageTaData {}
