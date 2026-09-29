//! 由 `codegen` 从 `sql/jzdb_prod_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_pdmage_pd_unit_sum_unsettle`（codegen：字段与列名源自 DDL，`PdmagePdUnitSumUnsettleColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdmagePdUnitSumUnsettle {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub init_date: i32,
    pub instr_no: String,
    pub external_no: String,
    pub busi_flag: i32,
    pub order_oper_way: i32,
    pub co_no: i32,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub secu_name: String,
    pub settle_crncy_type: i32,
    pub exch_crncy_type: i32,
    pub secu_type: i32,
    pub invest_type: i32,
    pub asset_type: i32,
    pub order_dir: i32,
    pub strike_qty: Decimal,
    pub strike_price: Decimal,
    pub strike_amt: Decimal,
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
    pub pre_settle_amt: Decimal,
    pub pre_settle_qty: Decimal,
    pub capit_settle_date: i32,
    pub capit_settle_time: i32,
    pub capit_settle_status: i32,
    pub posi_settle_date: i32,
    pub posi_settle_time: i32,
    pub posi_settle_status: i32,
    pub buy_exch_rate: Decimal,
    pub sale_exch_rate: Decimal,
    pub valid_flag: i32,
    pub delay_flag: i32,
    pub busi_type: i32,
}

/// `tb_pdmage_pd_unit_sum_unsettle` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdmagePdUnitSumUnsettleColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    InitDate,
    InstrNo,
    ExternalNo,
    BusiFlag,
    OrderOperWay,
    CoNo,
    PdNo,
    PdUnitNo,
    AsacNo,
    ExchNo,
    SecuCode,
    SecuName,
    SettleCrncyType,
    ExchCrncyType,
    SecuType,
    InvestType,
    AssetType,
    OrderDir,
    StrikeQty,
    StrikePrice,
    StrikeAmt,
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
    PreSettleAmt,
    PreSettleQty,
    CapitSettleDate,
    CapitSettleTime,
    CapitSettleStatus,
    PosiSettleDate,
    PosiSettleTime,
    PosiSettleStatus,
    BuyExchRate,
    SaleExchRate,
    ValidFlag,
    DelayFlag,
    BusiType,
}

impl ColumnName for PdmagePdUnitSumUnsettleColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::InitDate, Self::InstrNo, Self::ExternalNo, Self::BusiFlag, Self::OrderOperWay, Self::CoNo, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::SecuName, Self::SettleCrncyType, Self::ExchCrncyType, Self::SecuType, Self::InvestType, Self::AssetType, Self::OrderDir, Self::StrikeQty, Self::StrikePrice, Self::StrikeAmt, Self::AllFee, Self::StampTax, Self::TransFee, Self::BrkageFee, Self::SECCharges, Self::OtherFee, Self::TradeCommis, Self::OtherCommis, Self::TradeFee, Self::FutuDeliFee, Self::PreSettleAmt, Self::PreSettleQty, Self::CapitSettleDate, Self::CapitSettleTime, Self::CapitSettleStatus, Self::PosiSettleDate, Self::PosiSettleTime, Self::PosiSettleStatus, Self::BuyExchRate, Self::SaleExchRate, Self::ValidFlag, Self::DelayFlag, Self::BusiType];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::InitDate => "init_date",
            Self::InstrNo => "instr_no",
            Self::ExternalNo => "external_no",
            Self::BusiFlag => "busi_flag",
            Self::OrderOperWay => "order_oper_way",
            Self::CoNo => "co_no",
            Self::PdNo => "pd_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::SecuName => "secu_name",
            Self::SettleCrncyType => "settle_crncy_type",
            Self::ExchCrncyType => "exch_crncy_type",
            Self::SecuType => "secu_type",
            Self::InvestType => "invest_type",
            Self::AssetType => "asset_type",
            Self::OrderDir => "order_dir",
            Self::StrikeQty => "strike_qty",
            Self::StrikePrice => "strike_price",
            Self::StrikeAmt => "strike_amt",
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
            Self::PreSettleAmt => "pre_settle_amt",
            Self::PreSettleQty => "pre_settle_qty",
            Self::CapitSettleDate => "capit_settle_date",
            Self::CapitSettleTime => "capit_settle_time",
            Self::CapitSettleStatus => "capit_settle_status",
            Self::PosiSettleDate => "posi_settle_date",
            Self::PosiSettleTime => "posi_settle_time",
            Self::PosiSettleStatus => "posi_settle_status",
            Self::BuyExchRate => "buy_exch_rate",
            Self::SaleExchRate => "sale_exch_rate",
            Self::ValidFlag => "valid_flag",
            Self::DelayFlag => "delay_flag",
            Self::BusiType => "busi_type",
        }
    }
}

impl DataTable for PdmagePdUnitSumUnsettle {
    const ID: &'static str = "tb_pdmage_pd_unit_sum_unsettle";
    type Column = PdmagePdUnitSumUnsettleColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            PdmagePdUnitSumUnsettleColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for PdmagePdUnitSumUnsettle {}
