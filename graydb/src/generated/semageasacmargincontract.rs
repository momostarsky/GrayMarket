//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_semage_asac_margin_contract`（codegen：字段与列名源自 DDL，`SemageAsacMarginContractColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemageAsacMarginContract {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub serial_no: String,
    pub contra_no: String,
    pub init_date: i32,
    pub oper_mac: String,
    pub oper_ip: String,
    pub oper_way: i32,
    pub open_date: i32,
    pub open_time: i32,
    pub secu_source_type: i32,
    pub debt_return_date: i32,
    pub debt_stop_date: i32,
    pub extend_num: i32,
    pub pd_no: i32,
    pub co_no: i32,
    pub asac_no: i32,
    pub external_no: String,
    pub exch_no: i32,
    pub secu_code: String,
    pub debt_type: i32,
    pub debt_status: i32,
    pub debt_amt: Decimal,
    pub debt_qty: Decimal,
    pub debt_fee: Decimal,
    pub debt_interest: Decimal,
    pub back_balance: Decimal,
    pub back_amount: Decimal,
    pub return_interest_amt: Decimal,
    pub debt_year_radio: Decimal,
    pub remark_info: String,
}

/// `tb_semage_asac_margin_contract` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemageAsacMarginContractColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    SerialNo,
    ContraNo,
    InitDate,
    OperMac,
    OperIp,
    OperWay,
    OpenDate,
    OpenTime,
    SecuSourceType,
    DebtReturnDate,
    DebtStopDate,
    ExtendNum,
    PdNo,
    CoNo,
    AsacNo,
    ExternalNo,
    ExchNo,
    SecuCode,
    DebtType,
    DebtStatus,
    DebtAmt,
    DebtQty,
    DebtFee,
    DebtInterest,
    BackBalance,
    BackAmount,
    ReturnInterestAmt,
    DebtYearRadio,
    RemarkInfo,
}

impl ColumnName for SemageAsacMarginContractColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::SerialNo, Self::ContraNo, Self::InitDate, Self::OperMac, Self::OperIp, Self::OperWay, Self::OpenDate, Self::OpenTime, Self::SecuSourceType, Self::DebtReturnDate, Self::DebtStopDate, Self::ExtendNum, Self::PdNo, Self::CoNo, Self::AsacNo, Self::ExternalNo, Self::ExchNo, Self::SecuCode, Self::DebtType, Self::DebtStatus, Self::DebtAmt, Self::DebtQty, Self::DebtFee, Self::DebtInterest, Self::BackBalance, Self::BackAmount, Self::ReturnInterestAmt, Self::DebtYearRadio, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::SerialNo => "serial_no",
            Self::ContraNo => "contra_no",
            Self::InitDate => "init_date",
            Self::OperMac => "oper_mac",
            Self::OperIp => "oper_ip",
            Self::OperWay => "oper_way",
            Self::OpenDate => "open_date",
            Self::OpenTime => "open_time",
            Self::SecuSourceType => "secu_source_type",
            Self::DebtReturnDate => "debt_return_date",
            Self::DebtStopDate => "debt_stop_date",
            Self::ExtendNum => "extend_num",
            Self::PdNo => "pd_no",
            Self::CoNo => "co_no",
            Self::AsacNo => "asac_no",
            Self::ExternalNo => "external_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::DebtType => "debt_type",
            Self::DebtStatus => "debt_status",
            Self::DebtAmt => "debt_amt",
            Self::DebtQty => "debt_qty",
            Self::DebtFee => "debt_fee",
            Self::DebtInterest => "debt_interest",
            Self::BackBalance => "back_balance",
            Self::BackAmount => "back_amount",
            Self::ReturnInterestAmt => "return_interest_amt",
            Self::DebtYearRadio => "debt_year_radio",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SemageAsacMarginContract {
    const ID: &'static str = "tb_semage_asac_margin_contract";
    type Column = SemageAsacMarginContractColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SemageAsacMarginContractColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SemageAsacMarginContract {}
