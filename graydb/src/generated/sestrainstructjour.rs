//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_sestra_instructjour`（codegen：字段与列名源自 DDL，`SestraInstructjourColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SestraInstructjour {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub source_row_id: i64,
    pub last_update_times: i32,
    pub init_date: i32,
    pub serial_no: String,
    pub oper_ip: String,
    pub oper_mac: String,
    pub oper_info: String,
    pub instr_desc: String,
    pub user_no: i32,
    pub user_name: String,
    pub instr_type: i32,
    pub co_no: i32,
    pub comm_batch_no: i64,
    pub instr_no: String,
    pub terminal_batch_no: i64,
    pub orig_batch_no: i64,
    pub orig_instr_no: String,
    pub order_dir: i32,
    pub pd_no: i32,
    pub pd_unit_no: i32,
    pub asac_no: i32,
    pub exch_no: i32,
    pub secu_code: String,
    pub remark_info: String,
    pub remark_info2: String,
    pub reserved_field: String,
    pub reserved_field2: String,
}

/// `tb_sestra_instructjour` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SestraInstructjourColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    SourceRowId,
    LastUpdateTimes,
    InitDate,
    SerialNo,
    OperIp,
    OperMac,
    OperInfo,
    InstrDesc,
    UserNo,
    UserName,
    InstrType,
    CoNo,
    CommBatchNo,
    InstrNo,
    TerminalBatchNo,
    OrigBatchNo,
    OrigInstrNo,
    OrderDir,
    PdNo,
    PdUnitNo,
    AsacNo,
    ExchNo,
    SecuCode,
    RemarkInfo,
    RemarkInfo2,
    ReservedField,
    ReservedField2,
}

impl ColumnName for SestraInstructjourColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::SourceRowId, Self::LastUpdateTimes, Self::InitDate, Self::SerialNo, Self::OperIp, Self::OperMac, Self::OperInfo, Self::InstrDesc, Self::UserNo, Self::UserName, Self::InstrType, Self::CoNo, Self::CommBatchNo, Self::InstrNo, Self::TerminalBatchNo, Self::OrigBatchNo, Self::OrigInstrNo, Self::OrderDir, Self::PdNo, Self::PdUnitNo, Self::AsacNo, Self::ExchNo, Self::SecuCode, Self::RemarkInfo, Self::RemarkInfo2, Self::ReservedField, Self::ReservedField2];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::SourceRowId => "source_row_id",
            Self::LastUpdateTimes => "last_update_times",
            Self::InitDate => "init_date",
            Self::SerialNo => "serial_no",
            Self::OperIp => "oper_ip",
            Self::OperMac => "oper_mac",
            Self::OperInfo => "oper_info",
            Self::InstrDesc => "instr_desc",
            Self::UserNo => "user_no",
            Self::UserName => "user_name",
            Self::InstrType => "instr_type",
            Self::CoNo => "co_no",
            Self::CommBatchNo => "comm_batch_no",
            Self::InstrNo => "instr_no",
            Self::TerminalBatchNo => "terminal_batch_no",
            Self::OrigBatchNo => "orig_batch_no",
            Self::OrigInstrNo => "orig_instr_no",
            Self::OrderDir => "order_dir",
            Self::PdNo => "pd_no",
            Self::PdUnitNo => "pd_unit_no",
            Self::AsacNo => "asac_no",
            Self::ExchNo => "exch_no",
            Self::SecuCode => "secu_code",
            Self::RemarkInfo => "remark_info",
            Self::RemarkInfo2 => "remark_info2",
            Self::ReservedField => "reserved_field",
            Self::ReservedField2 => "reserved_field2",
        }
    }
}

impl DataTable for SestraInstructjour {
    const ID: &'static str = "tb_sestra_instructjour";
    type Column = SestraInstructjourColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SestraInstructjourColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SestraInstructjour {}
