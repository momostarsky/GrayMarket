//! 由 `codegen` 从 `sql/jzdb_base_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_basemage_asac_info`（codegen：字段与列名源自 DDL，`BasemageAsacInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasemageAsacInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub asac_no: i32,
    pub pd_no: i32,
    pub asac_type: i32,
    pub asac_kind: i32,
    pub asac_name: String,
    pub asac_status: i32,
    pub invest_area: i32,
    pub trade_pwd: String,
    pub mage_pwd: String,
    pub phone: String,
    pub email: String,
    pub user_no: i32,
    pub busi_ctrl_str: String,
    pub enable_ctrl_str: String,
    pub out_acco_id: i32,
    pub pd_unit_no: i32,
    pub custo_id: i32,
    pub custo_acco_name: String,
    pub custo_acco: String,
    pub custo_acco_key: String,
    pub remark_info: String,
}

/// `tb_basemage_asac_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasemageAsacInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    AsacNo,
    PdNo,
    AsacType,
    AsacKind,
    AsacName,
    AsacStatus,
    InvestArea,
    TradePwd,
    MagePwd,
    Phone,
    Email,
    UserNo,
    BusiCtrlStr,
    EnableCtrlStr,
    OutAccoId,
    PdUnitNo,
    CustoId,
    CustoAccoName,
    CustoAcco,
    CustoAccoKey,
    RemarkInfo,
}

impl ColumnName for BasemageAsacInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::AsacNo, Self::PdNo, Self::AsacType, Self::AsacKind, Self::AsacName, Self::AsacStatus, Self::InvestArea, Self::TradePwd, Self::MagePwd, Self::Phone, Self::Email, Self::UserNo, Self::BusiCtrlStr, Self::EnableCtrlStr, Self::OutAccoId, Self::PdUnitNo, Self::CustoId, Self::CustoAccoName, Self::CustoAcco, Self::CustoAccoKey, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::AsacNo => "asac_no",
            Self::PdNo => "pd_no",
            Self::AsacType => "asac_type",
            Self::AsacKind => "asac_kind",
            Self::AsacName => "asac_name",
            Self::AsacStatus => "asac_status",
            Self::InvestArea => "invest_area",
            Self::TradePwd => "trade_pwd",
            Self::MagePwd => "mage_pwd",
            Self::Phone => "phone",
            Self::Email => "email",
            Self::UserNo => "user_no",
            Self::BusiCtrlStr => "busi_ctrl_str",
            Self::EnableCtrlStr => "enable_ctrl_str",
            Self::OutAccoId => "out_acco_id",
            Self::PdUnitNo => "pd_unit_no",
            Self::CustoId => "custo_id",
            Self::CustoAccoName => "custo_acco_name",
            Self::CustoAcco => "custo_acco",
            Self::CustoAccoKey => "custo_acco_key",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for BasemageAsacInfo {
    const ID: &'static str = "tb_basemage_asac_info";
    type Column = BasemageAsacInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            BasemageAsacInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for BasemageAsacInfo {}
