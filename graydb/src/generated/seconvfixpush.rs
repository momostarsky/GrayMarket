//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use rust_decimal::Decimal;
use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seconv_fixpush`（codegen：字段与列名源自 DDL，`SeconvFixpushColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeconvFixpush {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub co_no: i32,
    pub occur_date: i32,
    pub jour_no: i32,
    pub t2if_operator_no: String,
    pub t2if_op_entrust_way: String,
    pub t2if_client_id: String,
    pub t2if_fund_account: String,
    pub t2if_exchange_type: String,
    pub t2if_entrust_reference: String,
    pub t2if_order_no: String,
    pub t2if_entrust_no: i32,
    pub t2if_entrust_prop: String,
    pub t2if_business_id: String,
    pub t2if_serial_no: i32,
    pub t2if_real_type: String,
    pub t2if_deal_status: String,
    pub t2if_business_time: i32,
    pub t2if_business_amount: i32,
    pub t2if_business_balance: Decimal,
    pub t2if_position_str: String,
    pub t2if_shortsell_type: String,
    pub t2if_session_type: String,
    #[serde(rename = "FIX_SessionID")]
    pub fix_session_id: String,
    #[serde(rename = "FIX8_BeginString")]
    pub fix8_begin_string: String,
    #[serde(rename = "FIX35_MsgType")]
    pub fix35_msg_type: String,
    #[serde(rename = "FIX49_SenderCompID")]
    pub fix49_sender_comp_id: String,
    #[serde(rename = "FIX50_SenderSubID")]
    pub fix50_sender_sub_id: String,
    #[serde(rename = "FIX52_SendingTime")]
    pub fix52_sending_time: String,
    #[serde(rename = "FIX56_TargetCompID")]
    pub fix56_target_comp_id: String,
    #[serde(rename = "FIX57_TargetSubID")]
    pub fix57_target_sub_id: String,
    #[serde(rename = "FIX115_OnBehalfOfCompID")]
    pub fix115_on_behalf_of_comp_id: String,
    #[serde(rename = "FIX116_OnBehalfOfSubID")]
    pub fix116_on_behalf_of_sub_id: String,
    #[serde(rename = "FIX128_DeliverToCompID")]
    pub fix128_deliver_to_comp_id: String,
    #[serde(rename = "FIX129_DeliverToSubID")]
    pub fix129_deliver_to_sub_id: String,
    #[serde(rename = "FIX1_Account")]
    pub fix1_account: String,
    #[serde(rename = "FIX6_AvgPx")]
    pub fix6_avg_px: Decimal,
    #[serde(rename = "FIX11_ClOrdID")]
    pub fix11_cl_ord_id: String,
    #[serde(rename = "FIX14_CumQty")]
    pub fix14_cum_qty: Decimal,
    #[serde(rename = "FIX15_Currency")]
    pub fix15_currency: String,
    #[serde(rename = "FIX17_ExecID")]
    pub fix17_exec_id: String,
    #[serde(rename = "FIX18_ExecInst")]
    pub fix18_exec_inst: String,
    #[serde(rename = "FIX21_HandInst")]
    pub fix21_hand_inst: String,
    #[serde(rename = "FIX22_IDSource")]
    pub fix22_id_source: String,
    #[serde(rename = "FIX31_LastPx")]
    pub fix31_last_px: Decimal,
    #[serde(rename = "FIX32_LastQty")]
    pub fix32_last_qty: Decimal,
    #[serde(rename = "FIX37_OrderID")]
    pub fix37_order_id: String,
    #[serde(rename = "FIX38_OrderQty")]
    pub fix38_order_qty: Decimal,
    #[serde(rename = "FIX39_OrdStatus")]
    pub fix39_ord_status: String,
    #[serde(rename = "FIX40_OrderTyp")]
    pub fix40_order_typ: String,
    #[serde(rename = "FIX41_OrigClOrdID")]
    pub fix41_orig_cl_ord_id: String,
    #[serde(rename = "FIX44_Price")]
    pub fix44_price: Decimal,
    #[serde(rename = "FIX48_SecurityID")]
    pub fix48_security_id: String,
    #[serde(rename = "FIX54_Side")]
    pub fix54_side: String,
    #[serde(rename = "FIX55_Symbol")]
    pub fix55_symbol: String,
    #[serde(rename = "FIX58_Text")]
    pub fix58_text: String,
    #[serde(rename = "FIX59_Timeinforce")]
    pub fix59_timeinforce: String,
    #[serde(rename = "FIX60_TransactTime")]
    pub fix60_transact_time: String,
    #[serde(rename = "FIX65_SymbolSfx")]
    pub fix65_symbol_sfx: String,
    #[serde(rename = "FIX66_ListID")]
    pub fix66_list_id: String,
    #[serde(rename = "FIX77_OpenClose")]
    pub fix77_open_close: String,
    #[serde(rename = "FIX99_StopPx")]
    pub fix99_stop_px: Decimal,
    #[serde(rename = "FIX109_ClientID")]
    pub fix109_client_id: String,
    #[serde(rename = "FIX100_ExDestination")]
    pub fix100_ex_destination: String,
    #[serde(rename = "FIX102_CxlRejReason")]
    pub fix102_cxl_rej_reason: String,
    #[serde(rename = "FIX103_OrdRejReason")]
    pub fix103_ord_rej_reason: String,
    #[serde(rename = "FIX120_SettleCurrency")]
    pub fix120_settle_currency: String,
    #[serde(rename = "FIX126_ExpireTime")]
    pub fix126_expire_time: String,
    #[serde(rename = "FIX150_ExecType")]
    pub fix150_exec_type: String,
    #[serde(rename = "FIX151_LeavesQty")]
    pub fix151_leaves_qty: Decimal,
    #[serde(rename = "FIX167_SecurityType")]
    pub fix167_security_type: String,
    #[serde(rename = "FIX207_SecurityExchange")]
    pub fix207_security_exchange: String,
    #[serde(rename = "FIX432_ExpireDate")]
    pub fix432_expire_date: String,
    #[serde(rename = "FIX434_CxlRejResponseTo")]
    pub fix434_cxl_rej_response_to: String,
    #[serde(rename = "FIX448_PartyID")]
    pub fix448_party_id: String,
    #[serde(rename = "FIX447_PartyIDSource")]
    pub fix447_party_id_source: String,
    #[serde(rename = "FIX452_PartyRole")]
    pub fix452_party_role: i32,
    #[serde(rename = "FIX375_ContraBroker")]
    pub fix375_contra_broker: String,
    pub client_acc_code: String,
    pub client_order_id: String,
}

/// `tb_seconv_fixpush` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeconvFixpushColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CoNo,
    OccurDate,
    JourNo,
    T2ifOperatorNo,
    T2ifOpEntrustWay,
    T2ifClientId,
    T2ifFundAccount,
    T2ifExchangeType,
    T2ifEntrustReference,
    T2ifOrderNo,
    T2ifEntrustNo,
    T2ifEntrustProp,
    T2ifBusinessId,
    T2ifSerialNo,
    T2ifRealType,
    T2ifDealStatus,
    T2ifBusinessTime,
    T2ifBusinessAmount,
    T2ifBusinessBalance,
    T2ifPositionStr,
    T2ifShortsellType,
    T2ifSessionType,
    FIXSessionID,
    FIX8BeginString,
    FIX35MsgType,
    FIX49SenderCompID,
    FIX50SenderSubID,
    FIX52SendingTime,
    FIX56TargetCompID,
    FIX57TargetSubID,
    FIX115OnBehalfOfCompID,
    FIX116OnBehalfOfSubID,
    FIX128DeliverToCompID,
    FIX129DeliverToSubID,
    FIX1Account,
    FIX6AvgPx,
    FIX11ClOrdID,
    FIX14CumQty,
    FIX15Currency,
    FIX17ExecID,
    FIX18ExecInst,
    FIX21HandInst,
    FIX22IDSource,
    FIX31LastPx,
    FIX32LastQty,
    FIX37OrderID,
    FIX38OrderQty,
    FIX39OrdStatus,
    FIX40OrderTyp,
    FIX41OrigClOrdID,
    FIX44Price,
    FIX48SecurityID,
    FIX54Side,
    FIX55Symbol,
    FIX58Text,
    FIX59Timeinforce,
    FIX60TransactTime,
    FIX65SymbolSfx,
    FIX66ListID,
    FIX77OpenClose,
    FIX99StopPx,
    FIX109ClientID,
    FIX100ExDestination,
    FIX102CxlRejReason,
    FIX103OrdRejReason,
    FIX120SettleCurrency,
    FIX126ExpireTime,
    FIX150ExecType,
    FIX151LeavesQty,
    FIX167SecurityType,
    FIX207SecurityExchange,
    FIX432ExpireDate,
    FIX434CxlRejResponseTo,
    FIX448PartyID,
    FIX447PartyIDSource,
    FIX452PartyRole,
    FIX375ContraBroker,
    ClientAccCode,
    ClientOrderId,
}

impl ColumnName for SeconvFixpushColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CoNo, Self::OccurDate, Self::JourNo, Self::T2ifOperatorNo, Self::T2ifOpEntrustWay, Self::T2ifClientId, Self::T2ifFundAccount, Self::T2ifExchangeType, Self::T2ifEntrustReference, Self::T2ifOrderNo, Self::T2ifEntrustNo, Self::T2ifEntrustProp, Self::T2ifBusinessId, Self::T2ifSerialNo, Self::T2ifRealType, Self::T2ifDealStatus, Self::T2ifBusinessTime, Self::T2ifBusinessAmount, Self::T2ifBusinessBalance, Self::T2ifPositionStr, Self::T2ifShortsellType, Self::T2ifSessionType, Self::FIXSessionID, Self::FIX8BeginString, Self::FIX35MsgType, Self::FIX49SenderCompID, Self::FIX50SenderSubID, Self::FIX52SendingTime, Self::FIX56TargetCompID, Self::FIX57TargetSubID, Self::FIX115OnBehalfOfCompID, Self::FIX116OnBehalfOfSubID, Self::FIX128DeliverToCompID, Self::FIX129DeliverToSubID, Self::FIX1Account, Self::FIX6AvgPx, Self::FIX11ClOrdID, Self::FIX14CumQty, Self::FIX15Currency, Self::FIX17ExecID, Self::FIX18ExecInst, Self::FIX21HandInst, Self::FIX22IDSource, Self::FIX31LastPx, Self::FIX32LastQty, Self::FIX37OrderID, Self::FIX38OrderQty, Self::FIX39OrdStatus, Self::FIX40OrderTyp, Self::FIX41OrigClOrdID, Self::FIX44Price, Self::FIX48SecurityID, Self::FIX54Side, Self::FIX55Symbol, Self::FIX58Text, Self::FIX59Timeinforce, Self::FIX60TransactTime, Self::FIX65SymbolSfx, Self::FIX66ListID, Self::FIX77OpenClose, Self::FIX99StopPx, Self::FIX109ClientID, Self::FIX100ExDestination, Self::FIX102CxlRejReason, Self::FIX103OrdRejReason, Self::FIX120SettleCurrency, Self::FIX126ExpireTime, Self::FIX150ExecType, Self::FIX151LeavesQty, Self::FIX167SecurityType, Self::FIX207SecurityExchange, Self::FIX432ExpireDate, Self::FIX434CxlRejResponseTo, Self::FIX448PartyID, Self::FIX447PartyIDSource, Self::FIX452PartyRole, Self::FIX375ContraBroker, Self::ClientAccCode, Self::ClientOrderId];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CoNo => "co_no",
            Self::OccurDate => "occur_date",
            Self::JourNo => "jour_no",
            Self::T2ifOperatorNo => "t2if_operator_no",
            Self::T2ifOpEntrustWay => "t2if_op_entrust_way",
            Self::T2ifClientId => "t2if_client_id",
            Self::T2ifFundAccount => "t2if_fund_account",
            Self::T2ifExchangeType => "t2if_exchange_type",
            Self::T2ifEntrustReference => "t2if_entrust_reference",
            Self::T2ifOrderNo => "t2if_order_no",
            Self::T2ifEntrustNo => "t2if_entrust_no",
            Self::T2ifEntrustProp => "t2if_entrust_prop",
            Self::T2ifBusinessId => "t2if_business_id",
            Self::T2ifSerialNo => "t2if_serial_no",
            Self::T2ifRealType => "t2if_real_type",
            Self::T2ifDealStatus => "t2if_deal_status",
            Self::T2ifBusinessTime => "t2if_business_time",
            Self::T2ifBusinessAmount => "t2if_business_amount",
            Self::T2ifBusinessBalance => "t2if_business_balance",
            Self::T2ifPositionStr => "t2if_position_str",
            Self::T2ifShortsellType => "t2if_shortsell_type",
            Self::T2ifSessionType => "t2if_session_type",
            Self::FIXSessionID => "FIX_SessionID",
            Self::FIX8BeginString => "FIX8_BeginString",
            Self::FIX35MsgType => "FIX35_MsgType",
            Self::FIX49SenderCompID => "FIX49_SenderCompID",
            Self::FIX50SenderSubID => "FIX50_SenderSubID",
            Self::FIX52SendingTime => "FIX52_SendingTime",
            Self::FIX56TargetCompID => "FIX56_TargetCompID",
            Self::FIX57TargetSubID => "FIX57_TargetSubID",
            Self::FIX115OnBehalfOfCompID => "FIX115_OnBehalfOfCompID",
            Self::FIX116OnBehalfOfSubID => "FIX116_OnBehalfOfSubID",
            Self::FIX128DeliverToCompID => "FIX128_DeliverToCompID",
            Self::FIX129DeliverToSubID => "FIX129_DeliverToSubID",
            Self::FIX1Account => "FIX1_Account",
            Self::FIX6AvgPx => "FIX6_AvgPx",
            Self::FIX11ClOrdID => "FIX11_ClOrdID",
            Self::FIX14CumQty => "FIX14_CumQty",
            Self::FIX15Currency => "FIX15_Currency",
            Self::FIX17ExecID => "FIX17_ExecID",
            Self::FIX18ExecInst => "FIX18_ExecInst",
            Self::FIX21HandInst => "FIX21_HandInst",
            Self::FIX22IDSource => "FIX22_IDSource",
            Self::FIX31LastPx => "FIX31_LastPx",
            Self::FIX32LastQty => "FIX32_LastQty",
            Self::FIX37OrderID => "FIX37_OrderID",
            Self::FIX38OrderQty => "FIX38_OrderQty",
            Self::FIX39OrdStatus => "FIX39_OrdStatus",
            Self::FIX40OrderTyp => "FIX40_OrderTyp",
            Self::FIX41OrigClOrdID => "FIX41_OrigClOrdID",
            Self::FIX44Price => "FIX44_Price",
            Self::FIX48SecurityID => "FIX48_SecurityID",
            Self::FIX54Side => "FIX54_Side",
            Self::FIX55Symbol => "FIX55_Symbol",
            Self::FIX58Text => "FIX58_Text",
            Self::FIX59Timeinforce => "FIX59_Timeinforce",
            Self::FIX60TransactTime => "FIX60_TransactTime",
            Self::FIX65SymbolSfx => "FIX65_SymbolSfx",
            Self::FIX66ListID => "FIX66_ListID",
            Self::FIX77OpenClose => "FIX77_OpenClose",
            Self::FIX99StopPx => "FIX99_StopPx",
            Self::FIX109ClientID => "FIX109_ClientID",
            Self::FIX100ExDestination => "FIX100_ExDestination",
            Self::FIX102CxlRejReason => "FIX102_CxlRejReason",
            Self::FIX103OrdRejReason => "FIX103_OrdRejReason",
            Self::FIX120SettleCurrency => "FIX120_SettleCurrency",
            Self::FIX126ExpireTime => "FIX126_ExpireTime",
            Self::FIX150ExecType => "FIX150_ExecType",
            Self::FIX151LeavesQty => "FIX151_LeavesQty",
            Self::FIX167SecurityType => "FIX167_SecurityType",
            Self::FIX207SecurityExchange => "FIX207_SecurityExchange",
            Self::FIX432ExpireDate => "FIX432_ExpireDate",
            Self::FIX434CxlRejResponseTo => "FIX434_CxlRejResponseTo",
            Self::FIX448PartyID => "FIX448_PartyID",
            Self::FIX447PartyIDSource => "FIX447_PartyIDSource",
            Self::FIX452PartyRole => "FIX452_PartyRole",
            Self::FIX375ContraBroker => "FIX375_ContraBroker",
            Self::ClientAccCode => "client_acc_code",
            Self::ClientOrderId => "client_order_id",
        }
    }
}

impl DataTable for SeconvFixpush {
    const ID: &'static str = "tb_seconv_fixpush";
    type Column = SeconvFixpushColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeconvFixpushColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeconvFixpush {}
