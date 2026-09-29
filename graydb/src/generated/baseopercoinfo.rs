//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_baseoper_co_info`（codegen：字段与列名源自 DDL，`BaseoperCoInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseoperCoInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub co_name: String,
    pub co_flname: String,
    pub co_type: i32,
    pub reg_code: String,
    pub register_address: String,
    pub found_date: i32,
    pub reg_date: i32,
    pub conta_name: String,
    pub phone: String,
    pub email: String,
    pub conta_addr: String,
    pub co_status: i32,
    pub create_pdunit_enable_flag: i32,
    pub pd_qty_max: i32,
    pub max_acco_count: i32,
    pub user_qty_max: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub instr_appr_oper: i32,
    pub comm_dist_oper: i32,
    pub comm_appo_exor: i32,
    pub rate_from_no: i32,
    pub float_ratio: Decimal,
    pub remark_info: String,
    pub ccass_id: String,
    pub en_short: String,
    pub broker_seat_id: String,
    pub src_bcan: String,
}

/// `tb_baseoper_co_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseoperCoInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    CoName,
    CoFlname,
    CoType,
    RegCode,
    RegisterAddress,
    FoundDate,
    RegDate,
    ContaName,
    Phone,
    Email,
    ContaAddr,
    CoStatus,
    CreatePdunitEnableFlag,
    PdQtyMax,
    MaxAccoCount,
    UserQtyMax,
    BusiCtrlStr,
    EnableCtrlStr,
    InstrApprOper,
    CommDistOper,
    CommAppoExor,
    RateFromNo,
    FloatRatio,
    RemarkInfo,
    CcassId,
    EnShort,
    BrokerSeatId,
    SrcBcan,
}

impl ColumnName for BaseoperCoInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::CoName, Self::CoFlname, Self::CoType, Self::RegCode, Self::RegisterAddress, Self::FoundDate, Self::RegDate, Self::ContaName, Self::Phone, Self::Email, Self::ContaAddr, Self::CoStatus, Self::CreatePdunitEnableFlag, Self::PdQtyMax, Self::MaxAccoCount, Self::UserQtyMax, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::InstrApprOper, Self::CommDistOper, Self::CommAppoExor, Self::RateFromNo, Self::FloatRatio, Self::RemarkInfo, Self::CcassId, Self::EnShort, Self::BrokerSeatId, Self::SrcBcan];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::CoName => "co_name",
            Self::CoFlname => "co_flname",
            Self::CoType => "co_type",
            Self::RegCode => "reg_code",
            Self::RegisterAddress => "register_address",
            Self::FoundDate => "found_date",
            Self::RegDate => "reg_date",
            Self::ContaName => "conta_name",
            Self::Phone => "phone",
            Self::Email => "email",
            Self::ContaAddr => "conta_addr",
            Self::CoStatus => "co_status",
            Self::CreatePdunitEnableFlag => "create_pdunit_enable_flag",
            Self::PdQtyMax => "pd_qty_max",
            Self::MaxAccoCount => "max_acco_count",
            Self::UserQtyMax => "user_qty_max",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::InstrApprOper => "instr_appr_oper",
            Self::CommDistOper => "comm_dist_oper",
            Self::CommAppoExor => "comm_appo_exor",
            Self::RateFromNo => "rate_from_no",
            Self::FloatRatio => "float_ratio",
            Self::RemarkInfo => "remark_info",
            Self::CcassId => "ccass_id",
            Self::EnShort => "en_short",
            Self::BrokerSeatId => "broker_seat_id",
            Self::SrcBcan => "src_bcan",
        }
    }
}

impl DataTable for BaseoperCoInfo {
    const ID: &'static str = "tb_baseoper_co_info";
    type Column = BaseoperCoInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BaseoperCoInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BaseoperCoInfo {}
