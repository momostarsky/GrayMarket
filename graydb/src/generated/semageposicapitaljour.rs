//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_posi_capital_jour`（codegen：字段与列名源自 DDL，`SemagePosiCapitalJourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemagePosiCapitalJour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub posi_capit_jour_no: i64,
    pub opor_no: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub secu_acco: String,
    pub invest_type: i32,
    pub settle_crncy_type: i32,
    pub occur_time: i32,
    pub busi_flag: i32,
    pub busi_type: i32,
    pub order_dir: i32,
    pub settle_speed: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub occur_qty: Decimal,
    pub occur_last_qty: Decimal,
    pub occur_amt: Decimal,
    pub after_occur_amt: Decimal,
    pub all_fee: Decimal,
    pub stamp_tax: Decimal,
    pub trans_fee: Decimal,
    pub brkage_fee: Decimal,
    #[serde(rename = "SEC_charges")]
    pub sec_charges: Decimal,
    pub other_fee: Decimal,
    pub trade_commis: Decimal,
    pub other_commis: Decimal,
    pub trade_fee: Decimal,
    pub futu_deli_fee: Decimal,
    pub occur_price: Decimal,
    pub strike_qty: Decimal,
    pub strike_amt: Decimal,
    pub busin_optype: i32,
    pub busin_jour_no: String,
    pub strike_date: i32,
    pub strike_no: String,
    pub posi_capit_jour_status: i32,
    pub reviewed_opor_no: i32,
    pub subject_code: String,
    pub subject_occur_amt: Decimal,
    pub subject_after_amt: Decimal,
    pub remark_info: String,
}

/// `tb_semage_posi_capital_jour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemagePosiCapitalJourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    PosiCapitJourNo,
    OporNo,
    CoNo,
    PdNo,
    PdUnitNo,
    AsacNo,
    SecuAcco,
    InvestType,
    SettleCrncyType,
    OccurTime,
    BusiFlag,
    BusiType,
    OrderDir,
    SettleSpeed,
    ExchNo,
    SecuCode,
    OccurQty,
    OccurLastQty,
    OccurAmt,
    AfterOccurAmt,
    AllFee,
    StampTax,
    TransFee,
    BrkageFee,
    SECCharges,
    OtherFee,
    TradeCommis,
    OtherCommis,
    TradeFee,
    FutuDeliFee,
    OccurPrice,
    StrikeQty,
    StrikeAmt,
    BusinOptype,
    BusinJourNo,
    StrikeDate,
    StrikeNo,
    PosiCapitJourStatus,
    ReviewedOporNo,
    SubjectCode,
    SubjectOccurAmt,
    SubjectAfterAmt,
    RemarkInfo,
}

impl ColumnName for SemagePosiCapitalJourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::PosiCapitJourNo, Self::OporNo, Self::CoNo, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::SecuAcco, Self::InvestType, Self::SettleCrncyType, Self::OccurTime, Self::BusiFlag, Self::BusiType, Self::OrderDir, Self::SettleSpeed, Self::ExchNo, Self::SecuCode, Self::OccurQty, Self::OccurLastQty, Self::OccurAmt, Self::AfterOccurAmt, Self::AllFee, Self::StampTax, Self::TransFee, Self::BrkageFee, Self::SECCharges, Self::OtherFee, Self::TradeCommis, Self::OtherCommis, Self::TradeFee, Self::FutuDeliFee, Self::OccurPrice, Self::StrikeQty, Self::StrikeAmt, Self::BusinOptype, Self::BusinJourNo, Self::StrikeDate, Self::StrikeNo, Self::PosiCapitJourStatus, Self::ReviewedOporNo, Self::SubjectCode, Self::SubjectOccurAmt, Self::SubjectAfterAmt, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::PosiCapitJourNo => "posi_capit_jour_no",
            Self::OporNo => "opor_no",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::SecuAcco => "secu_acco",
            Self::InvestType => "invest_type",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::OccurTime => "occur_time",
            Self::BusiFlag => "busi_flag",
            Self::BusiType => "busi_type",
            Self::OrderDir => "order_dir",
            Self::SettleSpeed => "settle_speed",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::OccurQty => "occur_qty",
            Self::OccurLastQty => "occur_last_qty",
            Self::OccurAmt => "occur_amt",
            Self::AfterOccurAmt => "after_occur_amt",
            Self::AllFee => "all_fee",
            Self::StampTax => "stamp_tax",
            Self::TransFee => "trans_fee",
            Self::BrkageFee => "brkage_fee",
            Self::SECCharges => "SEC_charges",
            Self::OtherFee => "other_fee",
            Self::TradeCommis => "trade_commis",
            Self::OtherCommis => "other_commis",
            Self::TradeFee => "trade_fee",
            Self::FutuDeliFee => "futu_deli_fee",
            Self::OccurPrice => "occur_price",
            Self::StrikeQty => "strike_qty",
            Self::StrikeAmt => "strike_amt",
            Self::BusinOptype => "busin_optype",
            Self::BusinJourNo => "busin_jour_no",
            Self::StrikeDate => "strike_date",
            Self::StrikeNo => "strike_no",
            Self::PosiCapitJourStatus => "posi_capit_jour_status",
            Self::ReviewedOporNo => "reviewed_opor_no",
            Self::SubjectCode => "subject_code",
            Self::SubjectOccurAmt => "subject_occur_amt",
            Self::SubjectAfterAmt => "subject_after_amt",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemagePosiCapitalJour {
    const ID: &'static str = "tb_semage_posi_capital_jour";
    type Column = SemagePosiCapitalJourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemagePosiCapitalJourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemagePosiCapitalJour {}
