--
-- PostgreSQL database dump
--

\restrict EGx0w3mcrLMcnbyJ1Fr5mcl1gPtOliAclXdDLdiOQcnuXXFxRu5WUcWq6JT5m2l

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: jzdb_secu; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA jzdb_secu;


ALTER SCHEMA jzdb_secu OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: tb_seconv_fixorder; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seconv_fixorder (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    occur_date integer NOT NULL,
    t2if_operator_no character varying(32) NOT NULL,
    t2if_op_entrust_way character varying(1) NOT NULL,
    t2if_client_id character varying(18) NOT NULL,
    t2if_fund_account character varying(18) NOT NULL,
    t2if_exchange_type character varying(4) NOT NULL,
    t2if_entrust_reference character varying(32) NOT NULL,
    t2if_order_no character varying(32) NOT NULL,
    t2if_entrust_no integer NOT NULL,
    t2if_entrust_prop character varying(3) NOT NULL,
    "FIX_SessionID" character varying(32) NOT NULL,
    "FIX8_BeginString" character varying(32) NOT NULL,
    "FIX35_MsgType" character varying(8) NOT NULL,
    "FIX49_SenderCompID" character varying(32) NOT NULL,
    "FIX50_SenderSubID" character varying(32) NOT NULL,
    "FIX52_SendingTime" character varying(32) NOT NULL,
    "FIX56_TargetCompID" character varying(32) NOT NULL,
    "FIX57_TargetSubID" character varying(32) NOT NULL,
    "FIX115_OnBehalfOfCompID" character varying(32) NOT NULL,
    "FIX116_OnBehalfOfSubID" character varying(32) NOT NULL,
    "FIX128_DeliverToCompID" character varying(32) NOT NULL,
    "FIX129_DeliverToSubID" character varying(32) NOT NULL,
    "FIX1_Account" character varying(32) NOT NULL,
    "FIX6_AvgPx" numeric(16,9) NOT NULL,
    "FIX11_ClOrdID" character varying(32) NOT NULL,
    "FIX14_CumQty" numeric(18,2) NOT NULL,
    "FIX15_Currency" character varying(8) NOT NULL,
    "FIX17_ExecID" character varying(32) NOT NULL,
    "FIX18_ExecInst" character varying(1) NOT NULL,
    "FIX21_HandInst" character varying(1) NOT NULL,
    "FIX22_IDSource" character varying(1) NOT NULL,
    "FIX31_LastPx" numeric(16,9) NOT NULL,
    "FIX32_LastQty" numeric(18,2) NOT NULL,
    "FIX38_OrderQty" numeric(18,2) NOT NULL,
    "FIX39_OrdStatus" character varying(1) NOT NULL,
    "FIX40_OrderTyp" character varying(1) NOT NULL,
    "FIX41_OrigClOrdID" character varying(32) NOT NULL,
    "FIX44_Price" numeric(16,9) NOT NULL,
    "FIX48_SecurityID" character varying(32) NOT NULL,
    "FIX54_Side" character varying(8) NOT NULL,
    "FIX55_Symbol" character varying(32) NOT NULL,
    "FIX58_Text" character varying(256) NOT NULL,
    "FIX59_Timeinforce" character varying(1) NOT NULL,
    "FIX60_TransactTime" character varying(32) NOT NULL,
    "FIX65_SymbolSfx" character varying(32) NOT NULL,
    "FIX66_ListID" character varying(32) NOT NULL,
    "FIX77_OpenClose" character varying(8) NOT NULL,
    "FIX99_StopPx" numeric(16,9) NOT NULL,
    "FIX109_ClientID" character varying(32) NOT NULL,
    "FIX100_ExDestination" character varying(32) NOT NULL,
    "FIX120_SettleCurrency" character varying(8) NOT NULL,
    "FIX126_ExpireTime" character varying(32) NOT NULL,
    "FIX102_CxlRejReason" character varying(8) NOT NULL,
    "FIX103_OrdRejReason" character varying(32) NOT NULL,
    "FIX150_ExecType" character varying(1) NOT NULL,
    "FIX151_LeavesQty" numeric(18,2) NOT NULL,
    "FIX167_SecurityType" character varying(8) NOT NULL,
    "FIX207_SecurityExchange" character varying(8) NOT NULL,
    "FIX432_ExpireDate" character varying(32) NOT NULL,
    "FIX434_CxlRejResponseTo" character varying(1) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    "FIX_RepeatClOrdIDFlag" character varying(1) NOT NULL,
    "FIX_RepeatClOrdID" character varying(32) NOT NULL,
    t2if_response_data character varying(4096) NOT NULL,
    t2if_error_no integer NOT NULL,
    t2if_error_info character varying(20) NOT NULL,
    "FIX6000_Strategy" character varying(32) NOT NULL,
    "FIX448_PartyID" character varying(32) NOT NULL,
    "FIX447_PartyIDSource" character varying(32) NOT NULL,
    "FIX452_PartyRole" integer NOT NULL,
    "FIX375_ContraBroker" character varying(32) NOT NULL,
    client_acc_code character varying(32) NOT NULL,
    client_order_id character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seconv_fixorder OWNER TO postgres;

--
-- Name: tb_seconv_fixorder_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seconv_fixorder ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seconv_fixorder_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seconv_fixpush; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seconv_fixpush (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    occur_date integer NOT NULL,
    jour_no integer NOT NULL,
    t2if_operator_no character varying(32) NOT NULL,
    t2if_op_entrust_way character varying(1) NOT NULL,
    t2if_client_id character varying(18) NOT NULL,
    t2if_fund_account character varying(18) NOT NULL,
    t2if_exchange_type character varying(4) NOT NULL,
    t2if_entrust_reference character varying(32) NOT NULL,
    t2if_order_no character varying(32) NOT NULL,
    t2if_entrust_no integer NOT NULL,
    t2if_entrust_prop character varying(3) NOT NULL,
    t2if_business_id character varying(32) NOT NULL,
    t2if_serial_no integer NOT NULL,
    t2if_real_type character varying(1) NOT NULL,
    t2if_deal_status character varying(1) NOT NULL,
    t2if_business_time integer NOT NULL,
    t2if_business_amount integer NOT NULL,
    t2if_business_balance numeric(16,2) NOT NULL,
    t2if_position_str character varying(100) NOT NULL,
    t2if_shortsell_type character varying(1) NOT NULL,
    t2if_session_type character varying(1) NOT NULL,
    "FIX_SessionID" character varying(32) NOT NULL,
    "FIX8_BeginString" character varying(32) NOT NULL,
    "FIX35_MsgType" character varying(8) NOT NULL,
    "FIX49_SenderCompID" character varying(32) NOT NULL,
    "FIX50_SenderSubID" character varying(32) NOT NULL,
    "FIX52_SendingTime" character varying(32) NOT NULL,
    "FIX56_TargetCompID" character varying(32) NOT NULL,
    "FIX57_TargetSubID" character varying(32) NOT NULL,
    "FIX115_OnBehalfOfCompID" character varying(32) NOT NULL,
    "FIX116_OnBehalfOfSubID" character varying(32) NOT NULL,
    "FIX128_DeliverToCompID" character varying(32) NOT NULL,
    "FIX129_DeliverToSubID" character varying(32) NOT NULL,
    "FIX1_Account" character varying(32) NOT NULL,
    "FIX6_AvgPx" numeric(16,9) NOT NULL,
    "FIX11_ClOrdID" character varying(32) NOT NULL,
    "FIX14_CumQty" numeric(18,2) NOT NULL,
    "FIX15_Currency" character varying(8) NOT NULL,
    "FIX17_ExecID" character varying(32) NOT NULL,
    "FIX18_ExecInst" character varying(1) NOT NULL,
    "FIX21_HandInst" character varying(1) NOT NULL,
    "FIX22_IDSource" character varying(1) NOT NULL,
    "FIX31_LastPx" numeric(16,9) NOT NULL,
    "FIX32_LastQty" numeric(18,2) NOT NULL,
    "FIX37_OrderID" character varying(32) NOT NULL,
    "FIX38_OrderQty" numeric(18,2) NOT NULL,
    "FIX39_OrdStatus" character varying(1) NOT NULL,
    "FIX40_OrderTyp" character varying(1) NOT NULL,
    "FIX41_OrigClOrdID" character varying(32) NOT NULL,
    "FIX44_Price" numeric(16,9) NOT NULL,
    "FIX48_SecurityID" character varying(32) NOT NULL,
    "FIX54_Side" character varying(8) NOT NULL,
    "FIX55_Symbol" character varying(32) NOT NULL,
    "FIX58_Text" character varying(256) NOT NULL,
    "FIX59_Timeinforce" character varying(1) NOT NULL,
    "FIX60_TransactTime" character varying(32) NOT NULL,
    "FIX65_SymbolSfx" character varying(32) NOT NULL,
    "FIX66_ListID" character varying(32) NOT NULL,
    "FIX77_OpenClose" character varying(8) NOT NULL,
    "FIX99_StopPx" numeric(16,9) NOT NULL,
    "FIX109_ClientID" character varying(32) NOT NULL,
    "FIX100_ExDestination" character varying(32) NOT NULL,
    "FIX102_CxlRejReason" character varying(8) NOT NULL,
    "FIX103_OrdRejReason" character varying(32) NOT NULL,
    "FIX120_SettleCurrency" character varying(8) NOT NULL,
    "FIX126_ExpireTime" character varying(32) NOT NULL,
    "FIX150_ExecType" character varying(1) NOT NULL,
    "FIX151_LeavesQty" numeric(18,2) NOT NULL,
    "FIX167_SecurityType" character varying(8) NOT NULL,
    "FIX207_SecurityExchange" character varying(8) NOT NULL,
    "FIX432_ExpireDate" character varying(32) NOT NULL,
    "FIX434_CxlRejResponseTo" character varying(1) NOT NULL,
    "FIX448_PartyID" character varying(32) NOT NULL,
    "FIX447_PartyIDSource" character varying(32) NOT NULL,
    "FIX452_PartyRole" integer NOT NULL,
    "FIX375_ContraBroker" character varying(32) NOT NULL,
    client_acc_code character varying(32) NOT NULL,
    client_order_id character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seconv_fixpush OWNER TO postgres;

--
-- Name: tb_seconv_fixpush_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seconv_fixpush ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seconv_fixpush_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_asset; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_asset (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    begin_evalu_nav_asset numeric(18,4) NOT NULL,
    curr_evalu_nav_asset numeric(18,4) NOT NULL,
    begin_nav_asset numeric(18,4) NOT NULL,
    curr_nav_asset numeric(18,4) NOT NULL,
    cash_asset numeric(18,4) NOT NULL,
    secu_asset numeric(16,4) NOT NULL,
    hk_thrgh_secu_asset numeric(18,4) NOT NULL,
    fund_asset numeric(16,4) NOT NULL,
    bond_asset numeric(16,4) NOT NULL,
    futu_asset numeric(18,4) NOT NULL,
    repo_asset numeric(18,4) NOT NULL,
    other_asset numeric(18,4) NOT NULL,
    out_nav_asset numeric(18,4) NOT NULL,
    secu_cash_asset numeric(18,4) NOT NULL,
    futu_cash_asset numeric(18,4) NOT NULL,
    sh_asecu_asset numeric(18,4) NOT NULL,
    sz_asecu_asset numeric(18,4) NOT NULL,
    bj_asecu_asset numeric(18,4) NOT NULL,
    sh_hk_secu_asset numeric(18,4) NOT NULL,
    sz_hk_secu_asset numeric(18,4) NOT NULL,
    money_fund_asset numeric(18,4) NOT NULL,
    not_money_fund_asset numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    loan_debt numeric(18,4) NOT NULL,
    futu_long_market_value numeric(18,4) NOT NULL,
    futu_short_market_value numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_asset OWNER TO postgres;

--
-- Name: tb_semage_asac_asset_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_asset ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_asset_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_capit; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_capit (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    payback_balance numeric(18,2) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    avail_adjust_amt numeric(18,4) NOT NULL,
    bank_balance numeric(18,4) NOT NULL,
    futu_bail numeric(18,2) NOT NULL,
    futu_bail_capt numeric(18,2) NOT NULL,
    pre_settle_amt numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_capit OWNER TO postgres;

--
-- Name: tb_semage_asac_capit_adjust_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_capit_adjust_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    adjust_jour_no bigint NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    before_curr_amt numeric(18,2) NOT NULL,
    before_frozen_amt numeric(18,2) NOT NULL,
    before_unfrozen_amt numeric(18,2) NOT NULL,
    before_pre_settle_amt numeric(18,4) NOT NULL,
    before_amt numeric(18,4) NOT NULL,
    busi_flag integer NOT NULL,
    adjust_amt numeric(18,4) NOT NULL,
    deal_status integer NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    pre_settle_amt numeric(18,4) NOT NULL,
    after_amt numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL,
    source_row_id bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_capit_adjust_jour OWNER TO postgres;

--
-- Name: tb_semage_asac_capit_adjust_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_capit_adjust_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_capit_adjust_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_capit_diff; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_capit_diff (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    date_type integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    custo_id integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    sys_in_value numeric(18,4) NOT NULL,
    sys_out_value numeric(18,4) NOT NULL,
    check_value_diff numeric(18,4) NOT NULL,
    check_status integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_capit_diff OWNER TO postgres;

--
-- Name: tb_semage_asac_capit_diff_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_capit_diff ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_capit_diff_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_capit_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_capit ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_capit_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_capit_trade; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_capit_trade (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    trade_frozen_amt numeric(18,4) NOT NULL,
    trade_unfrozen_amt numeric(18,4) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    fina_buy_instr_amt numeric(18,4) NOT NULL,
    fina_buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    loan_return_comm_amt numeric(16,4) NOT NULL,
    loan_return_order_amt numeric(16,4) NOT NULL,
    loan_return_strike_amt numeric(16,4) NOT NULL,
    fina_return_comm_amt numeric(18,4) NOT NULL,
    fina_return_order_amt numeric(18,4) NOT NULL,
    fina_return_strike_amt numeric(18,4) NOT NULL,
    return_strike_fee numeric(18,4) NOT NULL,
    debt_strike_fee numeric(16,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_capit_trade OWNER TO postgres;

--
-- Name: tb_semage_asac_capit_trade_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_capit_trade ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_capit_trade_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_capit_unsettle; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_capit_unsettle (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    order_date integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    settle_date integer NOT NULL,
    settle_time integer NOT NULL,
    busi_type integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_capit_unsettle OWNER TO postgres;

--
-- Name: tb_semage_asac_capit_unsettle_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_capit_unsettle ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_capit_unsettle_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_credit_asset; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_credit_asset (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    total_debit numeric(18,4) NOT NULL,
    funddebit numeric(18,2) NOT NULL,
    stock_debit numeric(18,2) NOT NULL,
    assure_ratio numeric(9,8) NOT NULL,
    avail_bail numeric(18,2) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    converted_margin numeric(16,4) NOT NULL,
    fina_converted_pandl numeric(18,4) NOT NULL,
    loan_converted_pandl numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_amt_remain_amt numeric(18,4) NOT NULL,
    loan_sell_amt_used_amt numeric(18,4) NOT NULL,
    fina_capt_margin numeric(18,4) NOT NULL,
    fina_order_capt_margin numeric(18,4) NOT NULL,
    loan_capt_margin numeric(18,4) NOT NULL,
    loan_order_capt_margin numeric(18,4) NOT NULL,
    debt_interest numeric(18,4) NOT NULL,
    debt_fee numeric(18,4) NOT NULL,
    fina_limit_max numeric(18,4) NOT NULL,
    finance_quota numeric(18,2) NOT NULL,
    loan_limit_max numeric(16,4) NOT NULL,
    shortsell_quota numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_credit_asset OWNER TO postgres;

--
-- Name: tb_semage_asac_credit_asset_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_credit_asset ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_credit_asset_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_fee_model; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_fee_model (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    asac_no integer NOT NULL,
    out_acco_id integer NOT NULL,
    fee_model_type integer NOT NULL,
    fee_model_kind integer NOT NULL,
    model_id bigint NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_fee_model OWNER TO postgres;

--
-- Name: tb_semage_asac_fee_model_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_fee_model ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_fee_model_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_margin_contract; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_margin_contract (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    serial_no character varying(64) NOT NULL,
    contra_no character varying(32) NOT NULL,
    init_date integer NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_way integer NOT NULL,
    open_date integer NOT NULL,
    open_time integer NOT NULL,
    secu_source_type integer NOT NULL,
    debt_return_date integer NOT NULL,
    debt_stop_date integer NOT NULL,
    extend_num integer NOT NULL,
    pd_no integer NOT NULL,
    co_no integer NOT NULL,
    asac_no integer NOT NULL,
    external_no character varying(32) NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    debt_type integer NOT NULL,
    debt_status integer NOT NULL,
    debt_amt numeric(18,4) NOT NULL,
    debt_qty numeric(18,2) NOT NULL,
    debt_fee numeric(18,4) NOT NULL,
    debt_interest numeric(18,4) NOT NULL,
    back_balance numeric(18,4) NOT NULL,
    back_amount numeric(18,2) NOT NULL,
    return_interest_amt numeric(18,4) NOT NULL,
    debt_year_radio numeric(9,8) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_margin_contract OWNER TO postgres;

--
-- Name: tb_semage_asac_margin_contract_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_margin_contract ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_margin_contract_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    forbid_order_dir character varying(64) NOT NULL,
    buy_mode integer NOT NULL,
    sell_mode integer NOT NULL,
    avail_qty numeric(18,2) NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    begin_max_loan_amount numeric(18,2) NOT NULL,
    curr_max_loan_amount numeric(18,2) NOT NULL,
    begin_shortsell_quota numeric(18,2) NOT NULL,
    curr_shortsell_quota numeric(18,2) NOT NULL,
    begin_used_loan_qty numeric(18,2) NOT NULL,
    curr_used_loan_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    unfrozen_qty numeric(18,2) NOT NULL,
    avail_adjust_qty numeric(18,2) NOT NULL,
    lock_secu_qty numeric(18,2) NOT NULL,
    alre_dist_qty numeric(18,2) NOT NULL,
    avail_list_qty numeric(18,2) NOT NULL,
    posi_qty_set numeric(18,2) NOT NULL,
    posi_copy_flag integer NOT NULL,
    t0_flag integer NOT NULL,
    last_price numeric(16,4) NOT NULL,
    pupil_flag integer NOT NULL,
    online_new_share_wait_qty numeric(18,2) NOT NULL,
    offline_new_share_wait_qty numeric(18,2) NOT NULL,
    dividend_qty numeric(18,2) NOT NULL,
    pla_qty numeric(16,4) NOT NULL,
    impawn_qty numeric(18,2) NOT NULL,
    realize_pandl numeric(18,2) NOT NULL,
    sum_realize_pandl numeric(16,4) NOT NULL,
    pre_settle_qty numeric(18,2) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    sell_qty numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_posi OWNER TO postgres;

--
-- Name: tb_semage_asac_posi_adjust_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_posi_adjust_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    adjust_jour_no bigint NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    invest_type integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    before_curr_qty numeric(18,2) NOT NULL,
    before_frozen_qty numeric(18,2) NOT NULL,
    before_unfrozen_qty numeric(18,2) NOT NULL,
    before_pre_settle_qty numeric(18,2) NOT NULL,
    before_qty numeric(18,2) NOT NULL,
    before_cost_amt numeric(18,2) NOT NULL,
    before_realize_pandl numeric(18,2) NOT NULL,
    before_sum_realize_pandl numeric(18,2) CONSTRAINT tb_semage_asac_posi_adjust_jo_before_sum_realize_pandl_not_null NOT NULL,
    busi_flag integer NOT NULL,
    adjust_qty numeric(18,2) NOT NULL,
    deal_status integer NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    unfrozen_qty numeric(18,2) NOT NULL,
    pre_settle_qty numeric(18,2) NOT NULL,
    after_qty numeric(18,2) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    realize_pandl numeric(18,2) NOT NULL,
    sum_realize_pandl numeric(16,4) NOT NULL,
    remark_info character varying(255) NOT NULL,
    source_row_id bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_posi_adjust_jour OWNER TO postgres;

--
-- Name: tb_semage_asac_posi_adjust_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_posi_adjust_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_posi_adjust_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_posi_diff; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_posi_diff (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    date_type integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    custo_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    sys_in_value numeric(18,4) NOT NULL,
    sys_out_value numeric(18,4) NOT NULL,
    check_value_diff numeric(18,4) NOT NULL,
    check_status integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_posi_diff OWNER TO postgres;

--
-- Name: tb_semage_asac_posi_diff_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_posi_diff ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_posi_diff_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_posi_unsettle; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_posi_unsettle (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    order_date integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    co_no integer NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    intrst_cost_amt numeric(18,4) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    sell_qty numeric(18,2) NOT NULL,
    settle_date integer NOT NULL,
    settle_time integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_posi_unsettle OWNER TO postgres;

--
-- Name: tb_semage_asac_posi_unsettle_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_posi_unsettle ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_posi_unsettle_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_settle_capit; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_settle_capit (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    payback_balance numeric(18,2) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    avail_adjust_amt numeric(18,4) NOT NULL,
    bank_balance numeric(18,4) NOT NULL,
    futu_bail numeric(18,2) NOT NULL,
    futu_bail_capt numeric(18,2) NOT NULL,
    pre_settle_amt numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_settle_capit OWNER TO postgres;

--
-- Name: tb_semage_asac_settle_capit_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_settle_capit ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_settle_capit_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_settle_capit_unsettle; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_settle_capit_unsettle (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    order_date integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    settle_date integer NOT NULL,
    settle_time integer NOT NULL,
    busi_type integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_settle_capit_unsettle OWNER TO postgres;

--
-- Name: tb_semage_asac_settle_capit_unsettle_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_settle_capit_unsettle ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_settle_capit_unsettle_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_settle_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_settle_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    forbid_order_dir character varying(64) NOT NULL,
    buy_mode integer NOT NULL,
    sell_mode integer NOT NULL,
    avail_qty numeric(18,2) NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    begin_max_loan_amount numeric(18,2) NOT NULL,
    curr_max_loan_amount numeric(18,2) NOT NULL,
    begin_shortsell_quota numeric(18,2) NOT NULL,
    curr_shortsell_quota numeric(18,2) NOT NULL,
    begin_used_loan_qty numeric(18,2) NOT NULL,
    curr_used_loan_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    unfrozen_qty numeric(18,2) NOT NULL,
    avail_adjust_qty numeric(18,2) NOT NULL,
    lock_secu_qty numeric(18,2) NOT NULL,
    alre_dist_qty numeric(18,2) NOT NULL,
    avail_list_qty numeric(18,2) NOT NULL,
    posi_qty_set numeric(18,2) NOT NULL,
    posi_copy_flag integer NOT NULL,
    t0_flag integer NOT NULL,
    last_price numeric(16,4) NOT NULL,
    pupil_flag integer NOT NULL,
    online_new_share_wait_qty numeric(18,2) NOT NULL,
    offline_new_share_wait_qty numeric(18,2) NOT NULL,
    dividend_qty numeric(18,2) NOT NULL,
    pla_qty numeric(16,4) NOT NULL,
    impawn_qty numeric(18,2) NOT NULL,
    realize_pandl numeric(18,2) NOT NULL,
    sum_realize_pandl numeric(16,4) NOT NULL,
    pre_settle_qty numeric(18,2) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    sell_qty numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_settle_posi OWNER TO postgres;

--
-- Name: tb_semage_asac_settle_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_settle_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_settle_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_settle_posi_unsettle; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_settle_posi_unsettle (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    order_date integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    co_no integer NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    intrst_cost_amt numeric(18,4) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    sell_qty numeric(18,2) NOT NULL,
    settle_date integer NOT NULL,
    settle_time integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_settle_posi_unsettle OWNER TO postgres;

--
-- Name: tb_semage_asac_settle_posi_unsettle_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_settle_posi_unsettle ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_settle_posi_unsettle_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_trade_capit_settle; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_trade_capit_settle (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_trade_capit_settle OWNER TO postgres;

--
-- Name: tb_semage_asac_trade_capit_settle_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_trade_capit_settle ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_trade_capit_settle_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_asac_trade_posi_settle; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_asac_trade_posi_settle (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    co_no integer NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    sell_qty numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_asac_trade_posi_settle OWNER TO postgres;

--
-- Name: tb_semage_asac_trade_posi_settle_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_asac_trade_posi_settle ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_asac_trade_posi_settle_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_co_custo_fieldvalue_map; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_co_custo_fieldvalue_map (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    custo_id integer NOT NULL,
    custo_field character varying(64) NOT NULL,
    table_field character varying(64) NOT NULL,
    custo_field_value character varying(64) NOT NULL,
    table_field_value character varying(64) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_co_custo_fieldvalue_map OWNER TO postgres;

--
-- Name: tb_semage_co_custo_fieldvalue_map_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_co_custo_fieldvalue_map ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_co_custo_fieldvalue_map_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_co_custo_file; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_co_custo_file (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    custo_check_item integer NOT NULL,
    custo_id integer NOT NULL,
    date_type integer NOT NULL,
    checkrisk_up numeric(18,4) NOT NULL,
    file_name character varying(255) NOT NULL,
    file_addr character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_co_custo_file OWNER TO postgres;

--
-- Name: tb_semage_co_custo_file_format; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_co_custo_file_format (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    custo_check_item integer NOT NULL,
    custo_id integer NOT NULL,
    custo_field character varying(64) NOT NULL,
    table_field character varying(64) NOT NULL,
    field_flag integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_co_custo_file_format OWNER TO postgres;

--
-- Name: tb_semage_co_custo_file_format_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_co_custo_file_format ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_co_custo_file_format_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_co_custo_file_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_co_custo_file ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_co_custo_file_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_ctmorder; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_ctmorder (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    out_acco_id integer NOT NULL,
    broker_co_id integer NOT NULL,
    channel_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    exor_no integer NOT NULL,
    dma integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    external_no character varying(32) NOT NULL,
    order_oper_way integer NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_id bigint NOT NULL,
    order_dir integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    report_date integer NOT NULL,
    report_no character varying(32) NOT NULL,
    strike_date integer NOT NULL,
    strike_time integer NOT NULL,
    strike_no character varying(64) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_price numeric(16,4) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_ctmorder OWNER TO postgres;

--
-- Name: tb_semage_ctmorder_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_ctmorder ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_ctmorder_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_ctmstrike_allocation; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_ctmstrike_allocation (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    out_acco_id integer NOT NULL,
    broker_co_id integer NOT NULL,
    channel_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    exor_no integer NOT NULL,
    dma integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    external_no character varying(32) NOT NULL,
    order_oper_way integer NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_id bigint NOT NULL,
    order_dir integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    report_date integer NOT NULL,
    report_no character varying(32) NOT NULL,
    strike_date integer NOT NULL,
    strike_time integer NOT NULL,
    strike_no character varying(64) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_price numeric(16,4) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    occur_amt numeric(18,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_ctmstrike_allocation OWNER TO postgres;

--
-- Name: tb_semage_ctmstrike_allocation_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_ctmstrike_allocation ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_ctmstrike_allocation_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_ctmstrike_block; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_ctmstrike_block (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    out_acco_id integer NOT NULL,
    broker_co_id integer NOT NULL,
    channel_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    exor_no integer NOT NULL,
    dma integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    external_no character varying(32) NOT NULL,
    order_oper_way integer NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_id bigint NOT NULL,
    order_dir integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    report_date integer NOT NULL,
    report_no character varying(32) NOT NULL,
    strike_date integer NOT NULL,
    strike_time integer NOT NULL,
    strike_no character varying(64) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_price numeric(16,4) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    occur_amt numeric(18,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_ctmstrike_block OWNER TO postgres;

--
-- Name: tb_semage_ctmstrike_block_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_ctmstrike_block ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_ctmstrike_block_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_exor_capit; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_exor_capit (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exor_group_code integer NOT NULL,
    exor_no integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_exor_capit OWNER TO postgres;

--
-- Name: tb_semage_exor_capit_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_exor_capit ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_exor_capit_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_exor_fee_model; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_exor_fee_model (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    exor_no integer NOT NULL,
    fee_model_type integer NOT NULL,
    fee_model_kind integer NOT NULL,
    model_id bigint NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_exor_fee_model OWNER TO postgres;

--
-- Name: tb_semage_exor_fee_model_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_exor_fee_model ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_exor_fee_model_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_exor_last_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_exor_last_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    busi_user_no integer NOT NULL,
    order_oper_way integer NOT NULL,
    channel_no integer NOT NULL,
    broker_branch_id integer NOT NULL,
    out_acco character varying(32) NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pdteam_no integer NOT NULL,
    exor_no integer NOT NULL,
    asac_no integer NOT NULL,
    invest_busi_kind integer NOT NULL,
    strategy_id bigint NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    order_dir integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    pupil_flag integer NOT NULL,
    realize_pandl numeric(18,2) NOT NULL,
    unsettle_pandl numeric(18,2) NOT NULL,
    income_ratio numeric(3,2) NOT NULL,
    open_time integer NOT NULL,
    close_time integer NOT NULL,
    trade_net_amt numeric(18,4) NOT NULL,
    trade_net_qty numeric(18,2) NOT NULL,
    buy_av_price numeric(16,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    sell_av_price numeric(16,4) NOT NULL,
    sell_qty numeric(18,2) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    order_id_str character varying(1024) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    pre_close_price numeric(16,4) NOT NULL,
    today_open_price numeric(16,4) NOT NULL,
    today_close_price numeric(16,4) NOT NULL,
    direction_flag integer NOT NULL,
    leave_net_qty numeric(18,2) NOT NULL,
    leave_buy_av_price numeric(16,4) NOT NULL,
    leave_buy_qty numeric(18,2) NOT NULL,
    leave_buy_amt numeric(18,4) NOT NULL,
    leave_sell_av_price numeric(16,4) NOT NULL,
    leave_sell_qty numeric(18,2) NOT NULL,
    leave_sell_amt numeric(18,4) NOT NULL,
    leave_unsettle_pandl numeric(18,4) NOT NULL,
    trade_type integer NOT NULL,
    open_date integer NOT NULL,
    posi_end_date integer NOT NULL,
    deal_flag integer NOT NULL,
    secu_source_type integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_exor_last_posi OWNER TO postgres;

--
-- Name: tb_semage_exor_last_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_exor_last_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_exor_last_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_exor_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_exor_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exor_group_code integer NOT NULL,
    exor_no integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    posi_qty_set numeric(18,2) NOT NULL,
    forbid_order_dir character varying(64) NOT NULL,
    buy_mode integer NOT NULL,
    sell_mode integer NOT NULL,
    trade_frozen_qty numeric(18,2) NOT NULL,
    trade_unfrozen_qty numeric(18,2) NOT NULL,
    net_trade_frozen_qty numeric(18,2) NOT NULL,
    trade_net_qty numeric(18,2) NOT NULL,
    buy_instr_qty numeric(18,2) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    buy_strike_qty numeric(18,2) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    fina_buy_strike_qty numeric(18,2) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_qty numeric(18,2) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_qty numeric(18,2) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_qty numeric(18,2) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    buy_strike_unfrozen_qty numeric(18,2) NOT NULL,
    loan_sell_instr_qty numeric(18,2) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_qty numeric(18,2) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_qty numeric(18,2) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    secuback_amount numeric(18,2) NOT NULL,
    used_loan_qty numeric(18,2) NOT NULL,
    loan_return_strike_amt numeric(16,4) NOT NULL,
    secu_source_type integer NOT NULL,
    t0_flag integer NOT NULL,
    last_price numeric(16,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_exor_posi OWNER TO postgres;

--
-- Name: tb_semage_exor_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_exor_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_exor_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_fee_model; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_fee_model (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    fee_model_id integer NOT NULL,
    fee_model_code character varying(32) NOT NULL,
    fee_model_name character varying(64) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_fee_model OWNER TO postgres;

--
-- Name: tb_semage_fee_model_binding; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_fee_model_binding (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    fee_model_id integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_fee_model_binding OWNER TO postgres;

--
-- Name: tb_semage_fee_model_binding_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_fee_model_binding ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_fee_model_binding_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_fee_model_detail; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_fee_model_detail (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    fee_model_id integer NOT NULL,
    fee_model_detail_id integer NOT NULL,
    fee_kind integer NOT NULL,
    exch_no integer NOT NULL,
    secu_fee_type integer NOT NULL,
    secu_type integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    broker_co_id integer NOT NULL,
    money_type integer NOT NULL,
    dma_mode integer NOT NULL,
    order_dir integer NOT NULL,
    order_kind integer NOT NULL,
    charge_type integer NOT NULL,
    math_round_type integer NOT NULL,
    math_round_place integer NOT NULL,
    fee_rate numeric(18,12) NOT NULL,
    fixed_fee numeric(18,4) NOT NULL,
    max_fee numeric(18,4) NOT NULL,
    min_fee numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_fee_model_detail OWNER TO postgres;

--
-- Name: tb_semage_fee_model_detail_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_fee_model_detail ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_fee_model_detail_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_fee_model_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_fee_model ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_fee_model_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_fee_section_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_fee_section_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    fee_model_detail_id integer NOT NULL,
    fee_section_config_id integer NOT NULL,
    fee_range_min numeric(18,4) NOT NULL,
    fee_range_max numeric(18,4) NOT NULL,
    charge_type integer NOT NULL,
    fee_rate numeric(18,12) NOT NULL,
    fixed_fee numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_fee_section_config OWNER TO postgres;

--
-- Name: tb_semage_fee_section_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_fee_section_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_fee_section_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_loan_secu_pool; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_loan_secu_pool (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    channel_no integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    shortsell_quota numeric(18,2) NOT NULL,
    max_loan_amount numeric(18,2) NOT NULL,
    start_date integer NOT NULL,
    expire_date integer NOT NULL,
    secu_source_type integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_loan_secu_pool OWNER TO postgres;

--
-- Name: tb_semage_loan_secu_pool_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_loan_secu_pool ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_loan_secu_pool_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_out_capit; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_out_capit (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    stock_balance numeric(18,2) NOT NULL,
    nav_asset numeric(16,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_out_capit OWNER TO postgres;

--
-- Name: tb_semage_out_capit_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_out_capit ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_out_capit_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_out_capit_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_out_capit_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    asac_no integer NOT NULL,
    broker_co_id integer NOT NULL,
    out_acco character varying(32) NOT NULL,
    settle_crncy_type integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    stock_balance numeric(18,2) NOT NULL,
    nav_asset numeric(16,4) NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    if_error_no integer NOT NULL,
    if_error_info character varying(20) NOT NULL,
    busi_error_info character varying(255) NOT NULL,
    busi_msg_content character varying(4096) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_out_capit_rsp OWNER TO postgres;

--
-- Name: tb_semage_out_capit_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_out_capit_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_out_capit_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_out_credit_asset; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_out_credit_asset (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    total_debit numeric(18,4) NOT NULL,
    funddebit numeric(18,2) NOT NULL,
    stock_debit numeric(18,2) NOT NULL,
    assure_ratio numeric(9,8) NOT NULL,
    avail_bail numeric(18,2) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    converted_margin numeric(16,4) NOT NULL,
    fina_converted_pandl numeric(18,4) NOT NULL,
    loan_converted_pandl numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_amt_remain_amt numeric(18,4) NOT NULL,
    loan_sell_amt_used_amt numeric(18,4) NOT NULL,
    fina_capt_margin numeric(18,4) NOT NULL,
    fina_order_capt_margin numeric(18,4) NOT NULL,
    loan_capt_margin numeric(18,4) NOT NULL,
    loan_order_capt_margin numeric(18,4) NOT NULL,
    debt_interest numeric(18,4) NOT NULL,
    debt_fee numeric(18,4) NOT NULL,
    fina_limit_max numeric(18,4) NOT NULL,
    finance_quota numeric(18,2) NOT NULL,
    loan_limit_max numeric(16,4) NOT NULL,
    shortsell_quota numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_out_credit_asset OWNER TO postgres;

--
-- Name: tb_semage_out_credit_asset_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_out_credit_asset ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_out_credit_asset_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_out_object; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_out_object (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    pos_str character varying(64) NOT NULL,
    object_rights integer NOT NULL,
    finance_bail_ratio numeric(9,8) NOT NULL,
    shortsell_bail_ratio numeric(9,8) NOT NULL,
    mortgage_ratio numeric(9,8) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_out_object OWNER TO postgres;

--
-- Name: tb_semage_out_object_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_out_object ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_out_object_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_out_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_out_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    avail_qty numeric(18,2) NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    begin_max_loan_amount numeric(18,2) NOT NULL,
    curr_max_loan_amount numeric(18,2) NOT NULL,
    begin_shortsell_quota numeric(18,2) NOT NULL,
    curr_shortsell_quota numeric(18,2) NOT NULL,
    begin_used_loan_qty numeric(18,2) NOT NULL,
    curr_used_loan_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    intrst_cost_amt numeric(18,4) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    unfrozen_qty numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_out_posi OWNER TO postgres;

--
-- Name: tb_semage_out_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_out_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_out_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_out_posi_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_out_posi_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    asac_no integer NOT NULL,
    broker_co_id integer NOT NULL,
    out_acco character varying(32) NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    avail_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    intrst_cost_amt numeric(18,4) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    unfrozen_qty numeric(18,2) NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    if_error_no integer NOT NULL,
    if_error_info character varying(20) NOT NULL,
    busi_error_info character varying(255) NOT NULL,
    busi_msg_content character varying(4096) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_out_posi_rsp OWNER TO postgres;

--
-- Name: tb_semage_out_posi_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_out_posi_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_out_posi_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_posi_capital_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_posi_capital_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    posi_capit_jour_no bigint NOT NULL,
    opor_no integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    invest_type integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    occur_time integer NOT NULL,
    busi_flag integer NOT NULL,
    busi_type integer NOT NULL,
    order_dir integer NOT NULL,
    settle_speed integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    occur_qty numeric(18,2) NOT NULL,
    occur_last_qty numeric(18,2) NOT NULL,
    occur_amt numeric(18,4) NOT NULL,
    after_occur_amt numeric(18,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    trade_fee numeric(18,2) NOT NULL,
    futu_deli_fee numeric(18,2) NOT NULL,
    occur_price numeric(16,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    busin_optype integer NOT NULL,
    busin_jour_no character varying(64) NOT NULL,
    strike_date integer NOT NULL,
    strike_no character varying(64) NOT NULL,
    posi_capit_jour_status integer NOT NULL,
    reviewed_opor_no integer NOT NULL,
    subject_code character varying(32) NOT NULL,
    subject_occur_amt numeric(18,4) NOT NULL,
    subject_after_amt numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_posi_capital_jour OWNER TO postgres;

--
-- Name: tb_semage_posi_capital_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_posi_capital_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_posi_capital_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_posi_part_file; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_posi_part_file (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    co_no integer NOT NULL,
    posi_part_batch_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_code character varying(32) NOT NULL,
    file_addr character varying(255) NOT NULL,
    file_name character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_posi_part_file OWNER TO postgres;

--
-- Name: tb_semage_posi_part_file_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_posi_part_file ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_posi_part_file_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_posi_part_in; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_posi_part_in (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    co_no integer NOT NULL,
    posi_part_batch_no integer NOT NULL,
    posi_part_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_code character varying(32) NOT NULL,
    asac_no integer NOT NULL,
    out_acco character varying(32) NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    posi_part_qty numeric(18,2) NOT NULL,
    posi_part_cost numeric(18,4) NOT NULL,
    posi_part_status integer NOT NULL,
    remark_info character varying(255) NOT NULL,
    capital_acco character varying(64) NOT NULL,
    custo_acco character varying(64) NOT NULL,
    has_margin integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_posi_part_in OWNER TO postgres;

--
-- Name: tb_semage_posi_part_in_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_posi_part_in ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_posi_part_in_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_posi_part_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_posi_part_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    co_no integer NOT NULL,
    jour_no integer NOT NULL,
    posi_part_batch_no integer NOT NULL,
    posi_part_no integer NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_posi_part_jour OWNER TO postgres;

--
-- Name: tb_semage_posi_part_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_posi_part_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_posi_part_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_posi_part_out; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_posi_part_out (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    co_no integer NOT NULL,
    posi_part_batch_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_code character varying(32) NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    out_acco character varying(32) NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    posi_part_qty numeric(18,2) NOT NULL,
    posi_part_cost numeric(18,4) NOT NULL,
    posi_part_status integer NOT NULL,
    confirm_flag integer NOT NULL,
    initiator_no integer NOT NULL,
    initiator_user_name character varying(255) NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_user_name character varying(255) NOT NULL,
    appr_desc character varying(255) NOT NULL,
    confirm_remark character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    posi_part_date integer NOT NULL,
    capital_acco character varying(64) NOT NULL,
    custo_acco character varying(64) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_posi_part_out OWNER TO postgres;

--
-- Name: tb_semage_posi_part_out_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_posi_part_out ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_posi_part_out_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_busictrl_item_disableconfig; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_busictrl_item_disableconfig (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer CONSTRAINT tb_semage_risk_busictrl_item_disableconfi_update_times_not_null NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_no integer CONSTRAINT tb_semage_risk_busictrl_item_disab_risk_item_config_no_not_null NOT NULL,
    risk_item_config_name character varying(64) CONSTRAINT tb_semage_risk_busictrl_item_dis_risk_item_config_name_not_null NOT NULL,
    risk_item_config_content character varying(1024) CONSTRAINT tb_semage_risk_busictrl_item__risk_item_config_content_not_null NOT NULL,
    risk_item_no integer CONSTRAINT tb_semage_risk_busictrl_item_disableconfi_risk_item_no_not_null NOT NULL,
    risk_item_code character varying(16) CONSTRAINT tb_semage_risk_busictrl_item_disablecon_risk_item_code_not_null NOT NULL,
    risk_level integer NOT NULL,
    risk_item_cond_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_disabl_risk_item_cond_str_not_null NOT NULL,
    risk_item_oper_str character varying(64) CONSTRAINT tb_semage_risk_busictrl_item_disabl_risk_item_oper_str_not_null NOT NULL,
    risk_item_order_dir_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_d_risk_item_order_dir_str_not_null NOT NULL,
    risk_item_co_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_disablec_risk_item_co_str_not_null NOT NULL,
    risk_item_pd_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_disablec_risk_item_pd_str_not_null NOT NULL,
    risk_item_pdunit_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_disa_risk_item_pdunit_str_not_null NOT NULL,
    risk_item_asac_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_disabl_risk_item_asac_str_not_null NOT NULL,
    risk_item_start_time integer CONSTRAINT tb_semage_risk_busictrl_item_disa_risk_item_start_time_not_null NOT NULL,
    risk_item_end_time integer CONSTRAINT tb_semage_risk_busictrl_item_disabl_risk_item_end_time_not_null NOT NULL,
    risk_item_start_date integer CONSTRAINT tb_semage_risk_busictrl_item_disa_risk_item_start_date_not_null NOT NULL,
    risk_item_end_date integer CONSTRAINT tb_semage_risk_busictrl_item_disabl_risk_item_end_date_not_null NOT NULL,
    rule_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL,
    modi_user_no integer CONSTRAINT tb_semage_risk_busictrl_item_disableconfi_modi_user_no_not_null NOT NULL,
    risk_item_config_ctrl_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_risk_item_config_ctrl_str_not_null NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_busictrl_item_disableconfig OWNER TO postgres;

--
-- Name: tb_semage_risk_busictrl_item_disableconfig_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_busictrl_item_disableconfig ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_busictrl_item_disableconfig_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_busictrl_item_enableconfig; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_busictrl_item_enableconfig (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_no integer CONSTRAINT tb_semage_risk_busictrl_item_enabl_risk_item_config_no_not_null NOT NULL,
    risk_item_config_name character varying(64) CONSTRAINT tb_semage_risk_busictrl_item_ena_risk_item_config_name_not_null NOT NULL,
    risk_item_config_content character varying(1024) CONSTRAINT tb_semage_risk_busictrl_item_risk_item_config_content_not_null1 NOT NULL,
    risk_item_no integer NOT NULL,
    risk_item_code character varying(16) CONSTRAINT tb_semage_risk_busictrl_item_enableconf_risk_item_code_not_null NOT NULL,
    risk_level integer NOT NULL,
    risk_item_cond_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_enable_risk_item_cond_str_not_null NOT NULL,
    risk_item_oper_str character varying(64) CONSTRAINT tb_semage_risk_busictrl_item_enable_risk_item_oper_str_not_null NOT NULL,
    risk_item_order_dir_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_e_risk_item_order_dir_str_not_null NOT NULL,
    risk_item_co_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_enableco_risk_item_co_str_not_null NOT NULL,
    risk_item_pd_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_enableco_risk_item_pd_str_not_null NOT NULL,
    risk_item_pdunit_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_enab_risk_item_pdunit_str_not_null NOT NULL,
    risk_item_asac_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_enable_risk_item_asac_str_not_null NOT NULL,
    risk_item_start_time integer CONSTRAINT tb_semage_risk_busictrl_item_enab_risk_item_start_time_not_null NOT NULL,
    risk_item_end_time integer CONSTRAINT tb_semage_risk_busictrl_item_enable_risk_item_end_time_not_null NOT NULL,
    risk_item_start_date integer CONSTRAINT tb_semage_risk_busictrl_item_enab_risk_item_start_date_not_null NOT NULL,
    risk_item_end_date integer CONSTRAINT tb_semage_risk_busictrl_item_enable_risk_item_end_date_not_null NOT NULL,
    rule_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL,
    modi_user_no integer NOT NULL,
    risk_item_config_ctrl_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_ite_risk_item_config_ctrl_str_not_null1 NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_busictrl_item_enableconfig OWNER TO postgres;

--
-- Name: tb_semage_risk_busictrl_item_enableconfig_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_busictrl_item_enableconfig ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_busictrl_item_enableconfig_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_busictrl_item_secupoolconfig; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_busictrl_item_secupoolconfig (
    row_id bigint NOT NULL,
    create_date integer CONSTRAINT tb_semage_risk_busictrl_item_secupoolconfi_create_date_not_null NOT NULL,
    create_time integer CONSTRAINT tb_semage_risk_busictrl_item_secupoolconfi_create_time_not_null NOT NULL,
    update_date integer CONSTRAINT tb_semage_risk_busictrl_item_secupoolconfi_update_date_not_null NOT NULL,
    update_time integer CONSTRAINT tb_semage_risk_busictrl_item_secupoolconfi_update_time_not_null NOT NULL,
    update_times integer CONSTRAINT tb_semage_risk_busictrl_item_secupoolconf_update_times_not_null NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_no integer CONSTRAINT tb_semage_risk_busictrl_item_secup_risk_item_config_no_not_null NOT NULL,
    risk_item_config_name character varying(64) CONSTRAINT tb_semage_risk_busictrl_item_sec_risk_item_config_name_not_null NOT NULL,
    risk_item_config_content character varying(1024) CONSTRAINT tb_semage_risk_busictrl_item_risk_item_config_content_not_null2 NOT NULL,
    risk_item_no integer CONSTRAINT tb_semage_risk_busictrl_item_secupoolconf_risk_item_no_not_null NOT NULL,
    risk_item_code character varying(16) CONSTRAINT tb_semage_risk_busictrl_item_secupoolco_risk_item_code_not_null NOT NULL,
    risk_level integer NOT NULL,
    risk_item_cond_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_secupo_risk_item_cond_str_not_null NOT NULL,
    risk_item_oper_str character varying(64) CONSTRAINT tb_semage_risk_busictrl_item_secupo_risk_item_oper_str_not_null NOT NULL,
    risk_item_order_dir_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_s_risk_item_order_dir_str_not_null NOT NULL,
    risk_item_co_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_secupool_risk_item_co_str_not_null NOT NULL,
    risk_item_pd_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_secupool_risk_item_pd_str_not_null NOT NULL,
    risk_item_pdunit_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_secu_risk_item_pdunit_str_not_null NOT NULL,
    risk_item_asac_str character varying(4096) CONSTRAINT tb_semage_risk_busictrl_item_secupo_risk_item_asac_str_not_null NOT NULL,
    risk_item_start_time integer CONSTRAINT tb_semage_risk_busictrl_item_secu_risk_item_start_time_not_null NOT NULL,
    risk_item_end_time integer CONSTRAINT tb_semage_risk_busictrl_item_secupo_risk_item_end_time_not_null NOT NULL,
    risk_item_start_date integer CONSTRAINT tb_semage_risk_busictrl_item_secu_risk_item_start_date_not_null NOT NULL,
    risk_item_end_date integer CONSTRAINT tb_semage_risk_busictrl_item_secupo_risk_item_end_date_not_null NOT NULL,
    rule_flag integer NOT NULL,
    remark_info character varying(255) CONSTRAINT tb_semage_risk_busictrl_item_secupoolconfi_remark_info_not_null NOT NULL,
    time_stamp bigint NOT NULL,
    modi_user_no integer CONSTRAINT tb_semage_risk_busictrl_item_secupoolconf_modi_user_no_not_null NOT NULL,
    risk_item_config_ctrl_str character varying(255) CONSTRAINT tb_semage_risk_busictrl_ite_risk_item_config_ctrl_str_not_null2 NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_busictrl_item_secupoolconfig OWNER TO postgres;

--
-- Name: tb_semage_risk_busictrl_item_secupoolconfig_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_busictrl_item_secupoolconfig ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_busictrl_item_secupoolconfig_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_code_item_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_code_item_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_code_item_no integer NOT NULL,
    risk_code_item_content character varying(1024) NOT NULL,
    risk_code_item_config_type integer CONSTRAINT tb_semage_risk_code_item_co_risk_code_item_config_type_not_null NOT NULL,
    risk_code_item_config_str character varying(4096) CONSTRAINT tb_semage_risk_code_item_con_risk_code_item_config_str_not_null NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_code_item_config OWNER TO postgres;

--
-- Name: tb_semage_risk_code_item_config_custcode; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_code_item_config_custcode (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_code_item_no integer CONSTRAINT tb_semage_risk_code_item_config_cust_risk_code_item_no_not_null NOT NULL,
    risk_code_item_config_type integer CONSTRAINT tb_semage_risk_code_item_c_risk_code_item_config_type_not_null1 NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_code_item_config_custcode OWNER TO postgres;

--
-- Name: tb_semage_risk_code_item_config_custcode_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_code_item_config_custcode ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_code_item_config_custcode_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_code_item_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_code_item_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_code_item_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_cond_item_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_cond_item_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_cond_item_config_no integer CONSTRAINT tb_semage_risk_cond_item_conf_risk_cond_item_config_no_not_null NOT NULL,
    risk_cond_item_config_content character varying(1024) CONSTRAINT tb_semage_risk_cond_item_co_risk_cond_item_config_cont_not_null NOT NULL,
    risk_cond_item_no integer NOT NULL,
    risk_cond_item_code character varying(16) NOT NULL,
    risk_cond_item_value_str character varying(1024) CONSTRAINT tb_semage_risk_cond_item_conf_risk_cond_item_value_str_not_null NOT NULL,
    risk_cond_item_start_time integer CONSTRAINT tb_semage_risk_cond_item_con_risk_cond_item_start_time_not_null NOT NULL,
    risk_cond_item_end_time integer CONSTRAINT tb_semage_risk_cond_item_confi_risk_cond_item_end_time_not_null NOT NULL,
    risk_cond_item_start_date integer CONSTRAINT tb_semage_risk_cond_item_con_risk_cond_item_start_date_not_null NOT NULL,
    risk_cond_item_end_date integer CONSTRAINT tb_semage_risk_cond_item_confi_risk_cond_item_end_date_not_null NOT NULL,
    comp_dir integer NOT NULL,
    rule_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL,
    risk_item_config_ctrl_str character varying(255) CONSTRAINT tb_semage_risk_cond_item_con_risk_item_config_ctrl_str_not_null NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_cond_item_config OWNER TO postgres;

--
-- Name: tb_semage_risk_cond_item_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_cond_item_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_cond_item_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_exposure; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_exposure (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    exposure_no integer NOT NULL,
    exposure_name character varying(255) NOT NULL,
    exposure_type integer NOT NULL,
    exposure_level integer NOT NULL,
    modi_user_no integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_exposure OWNER TO postgres;

--
-- Name: tb_semage_risk_exposure_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_exposure_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_batch_no integer CONSTRAINT tb_semage_risk_exposure_conf_risk_item_config_batch_no_not_null NOT NULL,
    risk_item_config_no integer NOT NULL,
    risk_item_no integer NOT NULL,
    risk_exposure_str character varying(4096) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_exposure_config OWNER TO postgres;

--
-- Name: tb_semage_risk_exposure_config_review; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_exposure_config_review (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_review_jour_no bigint CONSTRAINT tb_semage_risk_exposure_config_rev_risk_review_jour_no_not_null NOT NULL,
    risk_item_config_batch_no integer CONSTRAINT tb_semage_risk_exposure_con_risk_item_config_batch_no_not_null1 NOT NULL,
    risk_item_config_no integer CONSTRAINT tb_semage_risk_exposure_config_rev_risk_item_config_no_not_null NOT NULL,
    risk_item_no integer NOT NULL,
    risk_exposure_str character varying(4096) CONSTRAINT tb_semage_risk_exposure_config_revie_risk_exposure_str_not_null NOT NULL,
    review_status integer NOT NULL,
    valid_flag integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_exposure_config_review OWNER TO postgres;

--
-- Name: tb_semage_risk_exposure_config_review_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_exposure_config_review ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_exposure_config_review_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_exposure_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_exposure_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_exposure_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_exposure_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_exposure ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_exposure_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_exposure_value; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_exposure_value (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    exposure_no integer NOT NULL,
    exposure_level_value integer NOT NULL,
    exposure_value numeric(18,4) NOT NULL,
    modi_user_no integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_exposure_value OWNER TO postgres;

--
-- Name: tb_semage_risk_exposure_value_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_exposure_value ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_exposure_value_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_item_code_dim_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_item_code_dim_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_batch_no integer CONSTRAINT tb_semage_risk_item_code_dim_risk_item_config_batch_no_not_null NOT NULL,
    risk_item_config_no integer CONSTRAINT tb_semage_risk_item_code_dim_confi_risk_item_config_no_not_null NOT NULL,
    risk_item_no integer NOT NULL,
    risk_code_item_config_type integer CONSTRAINT tb_semage_risk_item_code_di_risk_code_item_config_type_not_null NOT NULL,
    risk_item_codeitemno_str character varying(4096) CONSTRAINT tb_semage_risk_item_code_dim__risk_item_codeitemno_str_not_null NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_item_code_dim_config OWNER TO postgres;

--
-- Name: tb_semage_risk_item_code_dim_config_review; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_item_code_dim_config_review (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer CONSTRAINT tb_semage_risk_item_code_dim_config_revie_update_times_not_null NOT NULL,
    co_no integer NOT NULL,
    risk_review_jour_no bigint CONSTRAINT tb_semage_risk_item_code_dim_confi_risk_review_jour_no_not_null NOT NULL,
    risk_item_config_batch_no integer CONSTRAINT tb_semage_risk_item_code_di_risk_item_config_batch_no_not_null1 NOT NULL,
    risk_item_config_no integer CONSTRAINT tb_semage_risk_item_code_dim_conf_risk_item_config_no_not_null1 NOT NULL,
    risk_item_no integer CONSTRAINT tb_semage_risk_item_code_dim_config_revie_risk_item_no_not_null NOT NULL,
    risk_code_item_config_type integer CONSTRAINT tb_semage_risk_item_code_d_risk_code_item_config_type_not_null1 NOT NULL,
    risk_item_codeitemno_str character varying(4096) CONSTRAINT tb_semage_risk_item_code_dim_risk_item_codeitemno_str_not_null1 NOT NULL,
    review_status integer CONSTRAINT tb_semage_risk_item_code_dim_config_revi_review_status_not_null NOT NULL,
    valid_flag integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_item_code_dim_config_review OWNER TO postgres;

--
-- Name: tb_semage_risk_item_code_dim_config_review_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_item_code_dim_config_review ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_item_code_dim_config_review_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_item_code_dim_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_item_code_dim_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_item_code_dim_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_item_config_review; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_item_config_review (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    risk_review_jour_no bigint NOT NULL,
    risk_item_kind integer NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_batch_no integer CONSTRAINT tb_semage_risk_item_config_r_risk_item_config_batch_no_not_null NOT NULL,
    risk_item_config_no integer NOT NULL,
    risk_item_config_name character varying(64) CONSTRAINT tb_semage_risk_item_config_revie_risk_item_config_name_not_null NOT NULL,
    risk_item_config_content character varying(1024) CONSTRAINT tb_semage_risk_item_config_re_risk_item_config_content_not_null NOT NULL,
    risk_item_no integer NOT NULL,
    risk_item_code character varying(16) NOT NULL,
    risk_level integer NOT NULL,
    risk_item_oper_str character varying(64) NOT NULL,
    risk_item_order_dir_str character varying(255) CONSTRAINT tb_semage_risk_item_config_rev_risk_item_order_dir_str_not_null NOT NULL,
    risk_item_co_str character varying(255) NOT NULL,
    risk_item_pd_str character varying(4096) NOT NULL,
    risk_item_pdunit_str character varying(4096) NOT NULL,
    risk_item_asac_str character varying(4096) NOT NULL,
    money_type integer NOT NULL,
    risk_item_value_str character varying(64) NOT NULL,
    comp_dir integer NOT NULL,
    risk_item_cond_str character varying(255) NOT NULL,
    risk_item_exec_mode integer NOT NULL,
    risk_item_value_str_two character varying(64) CONSTRAINT tb_semage_risk_item_config_rev_risk_item_value_str_two_not_null NOT NULL,
    comp_dir_two integer NOT NULL,
    risk_item_exec_mode_two integer CONSTRAINT tb_semage_risk_item_config_rev_risk_item_exec_mode_two_not_null NOT NULL,
    risk_item_value_str_three character varying(64) CONSTRAINT tb_semage_risk_item_config_r_risk_item_value_str_three_not_null NOT NULL,
    comp_dir_three integer NOT NULL,
    risk_item_exec_mode_three integer CONSTRAINT tb_semage_risk_item_config_r_risk_item_exec_mode_three_not_null NOT NULL,
    risk_item_value_str_four character varying(64) CONSTRAINT tb_semage_risk_item_config_re_risk_item_value_str_four_not_null NOT NULL,
    comp_dir_four integer NOT NULL,
    risk_item_exec_mode_four integer CONSTRAINT tb_semage_risk_item_config_re_risk_item_exec_mode_four_not_null NOT NULL,
    risk_item_start_time integer NOT NULL,
    risk_item_end_time integer NOT NULL,
    risk_item_start_date integer NOT NULL,
    risk_item_end_date integer NOT NULL,
    rule_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL,
    modi_user_no integer NOT NULL,
    data_oper_type integer NOT NULL,
    review_status integer NOT NULL,
    valid_flag integer NOT NULL,
    reviewed_opor_no integer NOT NULL,
    reviewed_date integer NOT NULL,
    reviewed_time integer NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    risk_item_config_ctrl_str character varying(255) CONSTRAINT tb_semage_risk_item_config_r_risk_item_config_ctrl_str_not_null NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_item_config_review OWNER TO postgres;

--
-- Name: tb_semage_risk_item_config_review_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_item_config_review ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_item_config_review_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_item_one_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_item_one_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_batch_no integer CONSTRAINT tb_semage_risk_item_one_conf_risk_item_config_batch_no_not_null NOT NULL,
    risk_item_config_no integer NOT NULL,
    risk_item_config_name character varying(64) NOT NULL,
    risk_item_config_content character varying(1024) CONSTRAINT tb_semage_risk_item_one_confi_risk_item_config_content_not_null NOT NULL,
    risk_item_no integer NOT NULL,
    risk_item_code character varying(16) NOT NULL,
    risk_level integer NOT NULL,
    risk_item_oper_str character varying(64) NOT NULL,
    risk_item_order_dir_str character varying(255) NOT NULL,
    risk_item_co_str character varying(255) NOT NULL,
    risk_item_pd_str character varying(4096) NOT NULL,
    risk_item_pdunit_str character varying(4096) NOT NULL,
    risk_item_asac_str character varying(4096) NOT NULL,
    money_type integer NOT NULL,
    risk_item_value_str character varying(64) NOT NULL,
    comp_dir integer NOT NULL,
    risk_item_cond_str character varying(255) NOT NULL,
    risk_item_exec_mode integer NOT NULL,
    risk_item_value_str_two character varying(64) NOT NULL,
    comp_dir_two integer NOT NULL,
    risk_item_exec_mode_two integer NOT NULL,
    risk_item_value_str_three character varying(64) CONSTRAINT tb_semage_risk_item_one_conf_risk_item_value_str_three_not_null NOT NULL,
    comp_dir_three integer NOT NULL,
    risk_item_exec_mode_three integer CONSTRAINT tb_semage_risk_item_one_conf_risk_item_exec_mode_three_not_null NOT NULL,
    risk_item_value_str_four character varying(64) CONSTRAINT tb_semage_risk_item_one_confi_risk_item_value_str_four_not_null NOT NULL,
    comp_dir_four integer NOT NULL,
    risk_item_exec_mode_four integer CONSTRAINT tb_semage_risk_item_one_confi_risk_item_exec_mode_four_not_null NOT NULL,
    risk_item_start_time integer NOT NULL,
    risk_item_end_time integer NOT NULL,
    risk_item_start_date integer NOT NULL,
    risk_item_end_date integer NOT NULL,
    rule_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL,
    modi_user_no integer NOT NULL,
    risk_item_config_ctrl_str character varying(255) CONSTRAINT tb_semage_risk_item_one_conf_risk_item_config_ctrl_str_not_null NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_item_one_config OWNER TO postgres;

--
-- Name: tb_semage_risk_item_one_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_item_one_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_item_one_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_item_riskgroup_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_item_riskgroup_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_no integer CONSTRAINT tb_semage_risk_item_riskgroup_conf_risk_item_config_no_not_null NOT NULL,
    risk_item_kind integer NOT NULL,
    workgroup_id integer NOT NULL,
    risk_appr_up numeric(18,4) NOT NULL,
    risk_appr_down numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_item_riskgroup_config OWNER TO postgres;

--
-- Name: tb_semage_risk_item_riskgroup_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_item_riskgroup_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_item_riskgroup_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_item_union_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_item_union_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_batch_no integer CONSTRAINT tb_semage_risk_item_union_co_risk_item_config_batch_no_not_null NOT NULL,
    risk_item_config_no integer NOT NULL,
    risk_item_config_name character varying(64) NOT NULL,
    risk_item_config_content character varying(1024) CONSTRAINT tb_semage_risk_item_union_con_risk_item_config_content_not_null NOT NULL,
    risk_item_no integer NOT NULL,
    risk_item_code character varying(16) NOT NULL,
    risk_level integer NOT NULL,
    risk_item_oper_str character varying(64) NOT NULL,
    risk_item_order_dir_str character varying(255) CONSTRAINT tb_semage_risk_item_union_conf_risk_item_order_dir_str_not_null NOT NULL,
    risk_item_co_str character varying(255) NOT NULL,
    risk_item_pd_str character varying(4096) NOT NULL,
    risk_item_pdunit_str character varying(4096) NOT NULL,
    risk_item_asac_str character varying(4096) NOT NULL,
    money_type integer NOT NULL,
    risk_item_value_str character varying(64) NOT NULL,
    comp_dir integer NOT NULL,
    risk_item_cond_str character varying(255) NOT NULL,
    risk_item_exec_mode integer NOT NULL,
    risk_item_value_str_two character varying(64) CONSTRAINT tb_semage_risk_item_union_conf_risk_item_value_str_two_not_null NOT NULL,
    comp_dir_two integer NOT NULL,
    risk_item_exec_mode_two integer CONSTRAINT tb_semage_risk_item_union_conf_risk_item_exec_mode_two_not_null NOT NULL,
    risk_item_value_str_three character varying(64) CONSTRAINT tb_semage_risk_item_union_co_risk_item_value_str_three_not_null NOT NULL,
    comp_dir_three integer NOT NULL,
    risk_item_exec_mode_three integer CONSTRAINT tb_semage_risk_item_union_co_risk_item_exec_mode_three_not_null NOT NULL,
    risk_item_value_str_four character varying(64) CONSTRAINT tb_semage_risk_item_union_con_risk_item_value_str_four_not_null NOT NULL,
    comp_dir_four integer NOT NULL,
    risk_item_exec_mode_four integer CONSTRAINT tb_semage_risk_item_union_con_risk_item_exec_mode_four_not_null NOT NULL,
    risk_item_start_time integer NOT NULL,
    risk_item_end_time integer NOT NULL,
    risk_item_start_date integer NOT NULL,
    risk_item_end_date integer NOT NULL,
    rule_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL,
    modi_user_no integer NOT NULL,
    risk_item_config_ctrl_str character varying(255) CONSTRAINT tb_semage_risk_item_union_co_risk_item_config_ctrl_str_not_null NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_item_union_config OWNER TO postgres;

--
-- Name: tb_semage_risk_item_union_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_item_union_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_item_union_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_warning_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_warning_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    serial_no character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    order_batch_no bigint NOT NULL,
    external_no character varying(32) NOT NULL,
    orig_external_no character varying(32) NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    exor_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    secu_name character varying(64) NOT NULL,
    order_dir integer NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    order_price numeric(16,4) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    risk_item_code character varying(16) NOT NULL,
    risk_item_oper_type integer NOT NULL,
    risk_calc_param_value numeric(18,4) NOT NULL,
    risk_check_param_value numeric(18,4) NOT NULL,
    risk_item_exec_mode integer NOT NULL,
    risk_item_config_no integer NOT NULL,
    risk_item_kind integer NOT NULL,
    risk_item_config_name character varying(64) NOT NULL,
    risk_item_type character varying(6) NOT NULL,
    busi_msg_content character varying(4096) NOT NULL,
    appr_user_no integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_desc character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    risk_source integer NOT NULL,
    risk_level integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_warning_jour OWNER TO postgres;

--
-- Name: tb_semage_risk_warning_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_warning_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_warning_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_risk_warning_jour_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_risk_warning_jour_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    serial_no character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    order_batch_no bigint NOT NULL,
    external_no character varying(32) NOT NULL,
    orig_external_no character varying(32) NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    exor_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    secu_name character varying(64) NOT NULL,
    order_dir integer NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    order_price numeric(16,4) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    risk_item_code character varying(16) NOT NULL,
    risk_item_oper_type integer NOT NULL,
    risk_calc_param_value numeric(18,4) NOT NULL,
    risk_check_param_value numeric(18,4) NOT NULL,
    risk_item_exec_mode integer NOT NULL,
    risk_item_config_no integer NOT NULL,
    risk_item_kind integer NOT NULL,
    risk_item_config_name character varying(64) NOT NULL,
    risk_item_type character varying(6) NOT NULL,
    busi_msg_content character varying(4096) NOT NULL,
    appr_user_no integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_desc character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    risk_source integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_risk_warning_jour_rsp OWNER TO postgres;

--
-- Name: tb_semage_risk_warning_jour_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_risk_warning_jour_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_risk_warning_jour_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_secu_code_model_fee; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_secu_code_model_fee (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    model_id bigint NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_fee_type integer NOT NULL,
    order_dir integer NOT NULL,
    amt_ratio numeric(9,8) NOT NULL,
    amt_value numeric(18,2) NOT NULL,
    par_value_ratio numeric(3,2) NOT NULL,
    par_value_value numeric(18,2) NOT NULL,
    max_fee numeric(18,4) NOT NULL,
    min_fee numeric(18,4) NOT NULL,
    float_ratio numeric(3,2) NOT NULL,
    fee_choice integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_secu_code_model_fee OWNER TO postgres;

--
-- Name: tb_semage_secu_code_model_fee_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_secu_code_model_fee ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_secu_code_model_fee_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_secu_code_pool; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_secu_code_pool (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    secu_code_pool_dim_no integer NOT NULL,
    secu_code_pool_no integer NOT NULL,
    secu_code_pool_name character varying(64) NOT NULL,
    secu_code_pool_type integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_secu_code_pool OWNER TO postgres;

--
-- Name: tb_semage_secu_code_pool_detail; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_secu_code_pool_detail (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    secu_code_pool_dim_no integer NOT NULL,
    secu_code_pool_no integer NOT NULL,
    secu_code_pool_type integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_secu_code_pool_detail OWNER TO postgres;

--
-- Name: tb_semage_secu_code_pool_detail_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_secu_code_pool_detail ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_secu_code_pool_detail_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_secu_code_pool_level; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_secu_code_pool_level (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    secu_code_pool_dim_level integer CONSTRAINT tb_semage_secu_code_pool_leve_secu_code_pool_dim_level_not_null NOT NULL,
    secu_code_pool_dim_no integer NOT NULL,
    secu_code_pool_dim_name character varying(64) NOT NULL,
    secu_code_pool_dim_type integer NOT NULL,
    pd_no_str character varying(4096) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_secu_code_pool_level OWNER TO postgres;

--
-- Name: tb_semage_secu_code_pool_level_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_secu_code_pool_level ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_secu_code_pool_level_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_secu_code_pool_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_secu_code_pool ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_secu_code_pool_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_secu_fee_model; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_secu_fee_model (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    model_id bigint NOT NULL,
    model_name character varying(64) NOT NULL,
    fee_model_type integer NOT NULL,
    fee_model_kind integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_secu_fee_model OWNER TO postgres;

--
-- Name: tb_semage_secu_fee_model_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_secu_fee_model ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_secu_fee_model_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_secu_type_model_fee; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_secu_type_model_fee (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    model_id bigint NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    secu_fee_type integer NOT NULL,
    order_dir integer NOT NULL,
    amt_ratio numeric(9,8) NOT NULL,
    amt_value numeric(18,2) NOT NULL,
    par_value_ratio numeric(3,2) NOT NULL,
    par_value_value numeric(18,2) NOT NULL,
    max_fee numeric(18,4) NOT NULL,
    min_fee numeric(18,4) NOT NULL,
    float_ratio numeric(3,2) NOT NULL,
    fee_choice integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_secu_type_model_fee OWNER TO postgres;

--
-- Name: tb_semage_secu_type_model_fee_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_secu_type_model_fee ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_secu_type_model_fee_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_semage_static_risk_check_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_semage_static_risk_check_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    serial_no character varying(64) NOT NULL,
    co_no integer NOT NULL,
    risk_item_config_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    compli_status integer NOT NULL,
    risk_item_type character varying(6) NOT NULL,
    risk_item_kind integer NOT NULL,
    risk_calc_param_value numeric(18,4) NOT NULL,
    risk_check_param_value numeric(18,4) CONSTRAINT tb_semage_static_risk_check_jou_risk_check_param_value_not_null NOT NULL,
    risk_item_config_name character varying(64) NOT NULL,
    risk_item_code character varying(16) NOT NULL,
    risk_level integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_semage_static_risk_check_jour OWNER TO postgres;

--
-- Name: tb_semage_static_risk_check_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_semage_static_risk_check_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_semage_static_risk_check_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_asset_main_type; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_asset_main_type (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    asset_main_type integer NOT NULL,
    asset_type_str character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_asset_main_type OWNER TO postgres;

--
-- Name: tb_seoper_asset_main_type_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_asset_main_type ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_asset_main_type_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_bond_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_bond_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    trade_code character varying(32) NOT NULL,
    target_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    issue_date integer NOT NULL,
    end_date integer NOT NULL,
    value_date integer NOT NULL,
    next_value_date integer NOT NULL,
    begin_trade_date integer NOT NULL,
    bond_limit numeric(18,2) NOT NULL,
    issue_price numeric(16,4) NOT NULL,
    par_value numeric(16,4) NOT NULL,
    intrst_ratio numeric(3,2) NOT NULL,
    intrst_days integer NOT NULL,
    pay_inteval integer NOT NULL,
    bond_accr_intrst numeric(3,2) NOT NULL,
    bond_rate_type integer NOT NULL,
    inteval_days integer NOT NULL,
    net_price_flag integer NOT NULL,
    last_trade_date integer NOT NULL,
    rights_type integer NOT NULL,
    trans_begin_date integer NOT NULL,
    trans_end_date integer NOT NULL,
    exec_begin_date integer NOT NULL,
    exec_end_date integer NOT NULL,
    impawn_ratio numeric(3,2) NOT NULL,
    pay_intrst_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_bond_info OWNER TO postgres;

--
-- Name: tb_seoper_bond_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_bond_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_bond_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_busi_rec_no; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_busi_rec_no (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    record_no_type integer NOT NULL,
    curr_no bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_busi_rec_no OWNER TO postgres;

--
-- Name: tb_seoper_busi_rec_no_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_busi_rec_no ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_busi_rec_no_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_co_dep_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_co_dep_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    exch_no integer NOT NULL,
    init_date integer NOT NULL,
    datatohis_date integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_co_dep_info OWNER TO postgres;

--
-- Name: tb_seoper_co_dep_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_co_dep_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_co_dep_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_co_exch_rate; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_co_exch_rate (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    for_crncy_type integer NOT NULL,
    crncy_type integer NOT NULL,
    buy_exch_rate numeric(18,12) NOT NULL,
    sale_exch_rate numeric(18,12) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_co_exch_rate OWNER TO postgres;

--
-- Name: tb_seoper_co_exch_rate_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_co_exch_rate ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_co_exch_rate_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_comp_action; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_comp_action (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    settle_crncy_type integer NOT NULL,
    busi_flag integer NOT NULL,
    dividend_calc_unit integer NOT NULL,
    dividend_amt numeric(18,4) NOT NULL,
    dividend_qty numeric(18,2) NOT NULL,
    tranaddshare_qty numeric(18,2) NOT NULL,
    pla_qty numeric(16,4) NOT NULL,
    pla_price numeric(16,4) NOT NULL,
    notic_date integer NOT NULL,
    reg_date integer NOT NULL,
    entry_date integer NOT NULL,
    begin_trade_date integer NOT NULL,
    exdividend_date integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_comp_action OWNER TO postgres;

--
-- Name: tb_seoper_comp_action_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_comp_action ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_comp_action_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_countries_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_countries_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    country_name character varying(64) NOT NULL,
    country_name_en character varying(64) NOT NULL,
    country_fullname_en character varying(64) NOT NULL,
    country_code_two character varying(64) NOT NULL,
    country_code_three character varying(64) NOT NULL,
    country_code_no integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_countries_info OWNER TO postgres;

--
-- Name: tb_seoper_countries_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_countries_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_countries_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_crncy_exchcode_config; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_crncy_exchcode_config (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    out_sys_no integer NOT NULL,
    crncy_exch_code character varying(255) NOT NULL,
    crncy_type integer NOT NULL,
    for_crncy_type integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_crncy_exchcode_config OWNER TO postgres;

--
-- Name: tb_seoper_crncy_exchcode_config_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_crncy_exchcode_config ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_crncy_exchcode_config_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_ex_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_ex_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    exch_type integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    exch_status integer NOT NULL,
    distric integer NOT NULL,
    time_lag integer NOT NULL,
    no_exch_date_str character varying(2048) NOT NULL,
    no_settle_date_str character varying(2048) NOT NULL,
    posi_settle_days integer NOT NULL,
    capit_settle_days integer NOT NULL,
    settle_days integer NOT NULL,
    mou_flag integer NOT NULL,
    ex_init_date integer NOT NULL,
    summer_time integer NOT NULL,
    winter_time integer NOT NULL,
    summer_time_begindate integer NOT NULL,
    winter_time_begindate integer NOT NULL,
    price_type_str character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_ex_info OWNER TO postgres;

--
-- Name: tb_seoper_ex_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_ex_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_ex_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_ex_time; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_ex_time (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    trd_time_frame integer NOT NULL,
    begin_time integer NOT NULL,
    end_time integer NOT NULL,
    allow_withdrw_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_ex_time OWNER TO postgres;

--
-- Name: tb_seoper_ex_time_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_ex_time ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_ex_time_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_fund_code_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_fund_code_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    pinyin_short character varying(16) NOT NULL,
    exch_sub_type integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    secu_type integer NOT NULL,
    secu_sub_type integer NOT NULL,
    asset_type integer NOT NULL,
    fund_disclname character varying(255) NOT NULL,
    fund_class integer NOT NULL,
    fund_floattype integer NOT NULL,
    fund_manager character varying(64) NOT NULL,
    fund_investadvisor character varying(255) NOT NULL,
    fund_investstyle integer NOT NULL,
    fund_type integer NOT NULL,
    fund_kind integer NOT NULL,
    fund_nature integer NOT NULL,
    fund_lowestsumsubll numeric(18,4) NOT NULL,
    fund_lowestsumpurll numeric(18,4) NOT NULL,
    fund_lowestsumredemption numeric(18,2) NOT NULL,
    fund_establishmentdate integer NOT NULL,
    unit_nav numeric(18,12) NOT NULL,
    sum_unit_nav numeric(18,12) NOT NULL,
    fund_share numeric(18,4) NOT NULL,
    fund_size numeric(18,4) NOT NULL,
    fund_nvdailygrowthrate character varying(16) NOT NULL,
    fund_rrinselectedweek character varying(16) NOT NULL,
    fund_rrinsingleweek character varying(16) NOT NULL,
    fund_rrinselectedmonth character varying(16) NOT NULL,
    fund_rrinsinglemonth character varying(16) NOT NULL,
    fund_rrinthreemonth character varying(16) NOT NULL,
    fund_rrinsixmonth character varying(16) NOT NULL,
    fund_rrsincethisyear character varying(16) NOT NULL,
    fund_rrinsingleyear character varying(16) NOT NULL,
    fund_rrintwoyear character varying(16) NOT NULL,
    fund_annualizedrrintwoyear character varying(16) NOT NULL,
    fund_rrinthreeyear character varying(16) NOT NULL,
    fund_annualizedrrinthreeyear character varying(16) NOT NULL,
    fund_rrinfiveyear character varying(16) NOT NULL,
    fund_annualizedrrinfiveyear character varying(16) NOT NULL,
    fund_rrintenyear character varying(16) NOT NULL,
    fund_annualizedrrintenyear character varying(16) NOT NULL,
    fund_rrsincestart character varying(16) NOT NULL,
    fund_annualizedrrsincestart character varying(16) NOT NULL,
    remark_info character varying(255) NOT NULL,
    fund_status integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_fund_code_info OWNER TO postgres;

--
-- Name: tb_seoper_fund_code_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_fund_code_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_fund_code_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_hk_exch_rate; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_hk_exch_rate (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    exch_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    buy_ref_rate numeric(9,8) NOT NULL,
    sell_ref_rate numeric(9,8) NOT NULL,
    settle_buy_rate numeric(9,8) NOT NULL,
    settle_sell_rate numeric(9,8) NOT NULL,
    pboc_rate numeric(9,8) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_hk_exch_rate OWNER TO postgres;

--
-- Name: tb_seoper_hk_exch_rate_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_hk_exch_rate ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_hk_exch_rate_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_hk_limit_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_hk_limit_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    begin_limit numeric(18,4) NOT NULL,
    remain_limit numeric(18,4) NOT NULL,
    limit_status integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_hk_limit_info OWNER TO postgres;

--
-- Name: tb_seoper_hk_limit_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_hk_limit_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_hk_limit_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_hk_settle_date; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_hk_settle_date (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    settle_date integer NOT NULL,
    set_type integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_hk_settle_date OWNER TO postgres;

--
-- Name: tb_seoper_hk_settle_date_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_hk_settle_date ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_hk_settle_date_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_issuer_secu_code; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_issuer_secu_code (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    contrs_exch_no integer NOT NULL,
    contrs_secu_code character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_issuer_secu_code OWNER TO postgres;

--
-- Name: tb_seoper_issuer_secu_code_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_issuer_secu_code ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_issuer_secu_code_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_margin_offset_secu; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_margin_offset_secu (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    channel_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    mortgage_ratio numeric(9,8) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_margin_offset_secu OWNER TO postgres;

--
-- Name: tb_seoper_margin_offset_secu_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_margin_offset_secu ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_margin_offset_secu_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_margin_ratio_allocation; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_margin_ratio_allocation (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    channel_no integer NOT NULL,
    co_no integer NOT NULL,
    secu_type integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    finance_bail_ratio numeric(9,8) NOT NULL,
    shortsell_bail_ratio numeric(9,8) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_margin_ratio_allocation OWNER TO postgres;

--
-- Name: tb_seoper_margin_ratio_allocation_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_margin_ratio_allocation ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_margin_ratio_allocation_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_margin_underly; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_margin_underly (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    fina_status integer NOT NULL,
    loan_status integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_margin_underly OWNER TO postgres;

--
-- Name: tb_seoper_margin_underly_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_margin_underly ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_margin_underly_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_new_secu_code_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_new_secu_code_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_name character varying(64) NOT NULL,
    trade_code character varying(32) NOT NULL,
    target_code character varying(32) NOT NULL,
    apply_date integer NOT NULL,
    apply_limit numeric(18,2) NOT NULL,
    begin_trade_date integer NOT NULL,
    issue_price numeric(16,4) NOT NULL,
    apply_pay_date integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_new_secu_code_info OWNER TO postgres;

--
-- Name: tb_seoper_new_secu_code_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_new_secu_code_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_new_secu_code_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_otcsecu_code_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_otcsecu_code_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    pinyin_short character varying(16) NOT NULL,
    exch_sub_type integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    secu_type integer NOT NULL,
    secu_sub_type integer NOT NULL,
    asset_type integer NOT NULL,
    par_value numeric(16,4) NOT NULL,
    type_unit integer NOT NULL,
    report_unit integer NOT NULL,
    min_unit integer NOT NULL,
    max_qty numeric(18,2) NOT NULL,
    min_qty numeric(18,2) NOT NULL,
    param_detail_flag integer NOT NULL,
    price_up numeric(16,4) NOT NULL,
    price_down numeric(16,4) NOT NULL,
    step_price numeric(16,4) NOT NULL,
    fair_price numeric(16,4) NOT NULL,
    stop_status integer NOT NULL,
    hk_secu_flag integer NOT NULL,
    total_secu_issue numeric(18,2) NOT NULL,
    circl_secu_capit numeric(18,2) NOT NULL,
    time_stamp bigint NOT NULL,
    t0_flag integer NOT NULL,
    issuer integer NOT NULL,
    issue_date integer NOT NULL,
    online_begin_trade_date integer NOT NULL,
    offline_begin_trade_date integer NOT NULL,
    ric character varying(64) NOT NULL,
    bloomberg character varying(64) NOT NULL,
    cusip character varying(64) NOT NULL,
    sedol character varying(64) NOT NULL,
    isin character varying(64) NOT NULL,
    issue_country integer NOT NULL,
    risk_country integer NOT NULL,
    up_limit_ratio integer NOT NULL,
    down_limit_ratio integer NOT NULL,
    settle_days integer NOT NULL,
    last_price numeric(16,4) NOT NULL,
    pre_close_price numeric(16,4) NOT NULL,
    today_open_price numeric(16,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_otcsecu_code_info OWNER TO postgres;

--
-- Name: tb_seoper_otcsecu_code_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_otcsecu_code_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_otcsecu_code_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_risk_code_item_sysconfig; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_risk_code_item_sysconfig (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_code_item_no integer NOT NULL,
    risk_code_item_content character varying(1024) CONSTRAINT tb_seoper_risk_code_item_syscon_risk_code_item_content_not_null NOT NULL,
    risk_code_item_config_type integer CONSTRAINT tb_seoper_risk_code_item_sy_risk_code_item_config_type_not_null NOT NULL,
    risk_code_item_config_str character varying(4096) CONSTRAINT tb_seoper_risk_code_item_sys_risk_code_item_config_str_not_null NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_risk_code_item_sysconfig OWNER TO postgres;

--
-- Name: tb_seoper_risk_code_item_sysconfig_code; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_risk_code_item_sysconfig_code (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    risk_code_item_no integer CONSTRAINT tb_seoper_risk_code_item_sysconfig_c_risk_code_item_no_not_null NOT NULL,
    risk_code_item_config_type integer CONSTRAINT tb_seoper_risk_code_item_s_risk_code_item_config_type_not_null1 NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    remark_info character varying(255) NOT NULL,
    time_stamp bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_risk_code_item_sysconfig_code OWNER TO postgres;

--
-- Name: tb_seoper_risk_code_item_sysconfig_code_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_risk_code_item_sysconfig_code ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_risk_code_item_sysconfig_code_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_risk_code_item_sysconfig_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_risk_code_item_sysconfig ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_risk_code_item_sysconfig_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_code_busi_arg; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_code_busi_arg (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    order_dir integer NOT NULL,
    cash_frozen_type integer NOT NULL,
    order_split_flag integer NOT NULL,
    min_unit integer NOT NULL,
    max_qty numeric(18,2) NOT NULL,
    min_qty numeric(18,2) NOT NULL,
    time_stamp bigint NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_code_busi_arg OWNER TO postgres;

--
-- Name: tb_seoper_secu_code_busi_arg_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_code_busi_arg ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_code_busi_arg_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_code_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_code_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    pinyin_short character varying(16) NOT NULL,
    exch_sub_type integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    secu_type integer NOT NULL,
    secu_sub_type integer NOT NULL,
    asset_type integer NOT NULL,
    par_value numeric(16,4) NOT NULL,
    type_unit integer NOT NULL,
    report_unit integer NOT NULL,
    min_unit integer NOT NULL,
    max_qty numeric(18,2) NOT NULL,
    min_qty numeric(18,2) NOT NULL,
    param_detail_flag integer NOT NULL,
    price_up numeric(16,4) NOT NULL,
    price_down numeric(16,4) NOT NULL,
    step_price numeric(16,4) NOT NULL,
    fair_price numeric(16,4) NOT NULL,
    stop_status integer NOT NULL,
    hk_secu_flag integer NOT NULL,
    total_secu_issue numeric(18,2) NOT NULL,
    circl_secu_capit numeric(18,2) NOT NULL,
    time_stamp bigint NOT NULL,
    t0_flag integer NOT NULL,
    issuer integer NOT NULL,
    issue_date integer NOT NULL,
    online_begin_trade_date integer NOT NULL,
    offline_begin_trade_date integer NOT NULL,
    ric character varying(64) NOT NULL,
    bloomberg character varying(64) NOT NULL,
    cusip character varying(64) NOT NULL,
    sedol character varying(64) NOT NULL,
    isin character varying(64) NOT NULL,
    issue_country integer NOT NULL,
    risk_country integer NOT NULL,
    up_limit_ratio integer NOT NULL,
    down_limit_ratio integer NOT NULL,
    settle_days integer NOT NULL,
    last_price numeric(16,4) NOT NULL,
    pre_close_price numeric(16,4) NOT NULL,
    today_open_price numeric(16,4) NOT NULL,
    remark_info character varying(255) NOT NULL,
    mengine_no integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_code_info OWNER TO postgres;

--
-- Name: tb_seoper_secu_code_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_code_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_code_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_code_map; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_code_map (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    trade_code character varying(32) NOT NULL,
    target_code character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_code_map OWNER TO postgres;

--
-- Name: tb_seoper_secu_code_map_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_code_map ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_code_map_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_inner_code_map; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_inner_code_map (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_innder_code character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_inner_code_map OWNER TO postgres;

--
-- Name: tb_seoper_secu_inner_code_map_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_inner_code_map ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_inner_code_map_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_quot; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_quot (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    up_limit_price numeric(16,4) NOT NULL,
    down_limit_price numeric(16,4) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    pre_close_price numeric(16,4) NOT NULL,
    today_open_price numeric(16,4) NOT NULL,
    today_close_price numeric(16,4) NOT NULL,
    today_max_price numeric(16,4) NOT NULL,
    today_min_price numeric(16,4) NOT NULL,
    buy_price_1 numeric(16,4) NOT NULL,
    buy_qty_1 numeric(18,2) NOT NULL,
    buy_price_2 numeric(16,4) NOT NULL,
    buy_qty_2 numeric(18,2) NOT NULL,
    buy_price_3 numeric(16,4) NOT NULL,
    buy_qty_3 numeric(18,2) NOT NULL,
    buy_price_4 numeric(16,4) NOT NULL,
    buy_qty_4 numeric(18,2) NOT NULL,
    buy_price_5 numeric(16,4) NOT NULL,
    buy_qty_5 numeric(18,2) NOT NULL,
    sell_price_1 numeric(16,4) NOT NULL,
    sell_qty_1 numeric(18,2) NOT NULL,
    sell_price_2 numeric(16,4) NOT NULL,
    sell_qty_2 numeric(18,2) NOT NULL,
    sell_price_3 numeric(16,4) NOT NULL,
    sell_qty_3 numeric(18,2) NOT NULL,
    sell_price_4 numeric(16,4) NOT NULL,
    sell_qty_4 numeric(18,2) NOT NULL,
    sell_price_5 numeric(16,4) NOT NULL,
    sell_qty_5 numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    time_stamp bigint NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_quot OWNER TO postgres;

--
-- Name: tb_seoper_secu_quot_daily_msg; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_quot_daily_msg (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    msg_content character varying(1024) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_quot_daily_msg OWNER TO postgres;

--
-- Name: tb_seoper_secu_quot_daily_msg_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_quot_daily_msg ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_quot_daily_msg_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_quot_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_quot ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_quot_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_repo_param; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_repo_param (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    target_code character varying(32) NOT NULL,
    secu_type integer NOT NULL,
    repo_days integer NOT NULL,
    repo_first_settle_date integer NOT NULL,
    repo_back_date integer NOT NULL,
    cash_capt_days integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_repo_param OWNER TO postgres;

--
-- Name: tb_seoper_secu_repo_param_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_repo_param ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_repo_param_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_strike_quot; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_strike_quot (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    order_dir integer NOT NULL,
    strike_price numeric(16,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    time_stamp bigint NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_strike_quot OWNER TO postgres;

--
-- Name: tb_seoper_secu_strike_quot_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_strike_quot ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_strike_quot_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_tmplat; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_tmplat (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code_feature character varying(16) NOT NULL,
    secu_name_feature character varying(16) NOT NULL,
    model_name character varying(64) NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_tmplat OWNER TO postgres;

--
-- Name: tb_seoper_secu_tmplat_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_tmplat ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_tmplat_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_type; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_type (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    par_value numeric(16,4) NOT NULL,
    type_unit integer NOT NULL,
    report_unit integer NOT NULL,
    min_unit integer NOT NULL,
    max_qty numeric(18,2) NOT NULL,
    min_qty numeric(18,2) NOT NULL,
    step_price numeric(16,4) NOT NULL,
    param_detail_flag integer NOT NULL,
    time_stamp bigint NOT NULL,
    t0_flag integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_type OWNER TO postgres;

--
-- Name: tb_seoper_secu_type_alert; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_type_alert (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    secu_type_out integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_type_alert OWNER TO postgres;

--
-- Name: tb_seoper_secu_type_alert_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_type_alert ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_type_alert_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_type_busi_arg; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_type_busi_arg (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    order_dir integer NOT NULL,
    cash_frozen_type integer NOT NULL,
    order_split_flag integer NOT NULL,
    min_unit integer NOT NULL,
    max_qty numeric(18,2) NOT NULL,
    min_qty numeric(18,2) NOT NULL,
    time_stamp bigint NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_type_busi_arg OWNER TO postgres;

--
-- Name: tb_seoper_secu_type_busi_arg_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_type_busi_arg ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_type_busi_arg_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_type_ctm; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_type_ctm (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    secu_type_out integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_type_ctm OWNER TO postgres;

--
-- Name: tb_seoper_secu_type_ctm_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_type_ctm ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_type_ctm_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_type_out; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_type_out (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    secu_type_out integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_type_out OWNER TO postgres;

--
-- Name: tb_seoper_secu_type_out_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_type_out ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_type_out_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_type_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_type ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_type_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_type_stepprice_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_type_stepprice_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    secu_type integer NOT NULL,
    price_up numeric(16,4) NOT NULL,
    price_down numeric(16,4) NOT NULL,
    step_price numeric(16,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_type_stepprice_info OWNER TO postgres;

--
-- Name: tb_seoper_secu_type_stepprice_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_type_stepprice_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_type_stepprice_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_secu_type_time; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_secu_type_time (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    begin_time integer NOT NULL,
    end_time integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_secu_type_time OWNER TO postgres;

--
-- Name: tb_seoper_secu_type_time_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_secu_type_time ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_secu_type_time_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_swap_code_info; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_swap_code_info (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    exch_no integer NOT NULL,
    futu_code character varying(32) NOT NULL,
    futu_name character varying(64) NOT NULL,
    pinyin_short character varying(16) NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    futu_type integer NOT NULL,
    asset_type integer NOT NULL,
    type_unit integer NOT NULL,
    report_unit integer NOT NULL,
    min_unit integer NOT NULL,
    max_qty numeric(18,2) NOT NULL,
    min_qty numeric(18,2) NOT NULL,
    param_detail_flag integer NOT NULL,
    price_up numeric(16,4) NOT NULL,
    price_down numeric(16,4) NOT NULL,
    step_price numeric(16,4) NOT NULL,
    fair_price numeric(16,4) NOT NULL,
    remark_info character varying(255) NOT NULL,
    t0_flag integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_swap_code_info OWNER TO postgres;

--
-- Name: tb_seoper_swap_code_info_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_swap_code_info ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_swap_code_info_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_sys_secu_code_fee; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_sys_secu_code_fee (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_fee_type integer NOT NULL,
    order_dir integer NOT NULL,
    amt_ratio numeric(9,8) NOT NULL,
    amt_value numeric(18,2) NOT NULL,
    par_value_ratio numeric(3,2) NOT NULL,
    par_value_value numeric(18,2) NOT NULL,
    max_fee numeric(18,4) NOT NULL,
    min_fee numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_sys_secu_code_fee OWNER TO postgres;

--
-- Name: tb_seoper_sys_secu_code_fee_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_sys_secu_code_fee ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_sys_secu_code_fee_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seoper_sys_secu_type_fee; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seoper_sys_secu_type_fee (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    exch_no integer NOT NULL,
    exch_sub_type integer NOT NULL,
    secu_type integer NOT NULL,
    secu_fee_type integer NOT NULL,
    order_dir integer NOT NULL,
    amt_ratio numeric(9,8) NOT NULL,
    amt_value numeric(18,2) NOT NULL,
    par_value_ratio numeric(3,2) NOT NULL,
    par_value_value numeric(18,2) NOT NULL,
    max_fee numeric(18,4) NOT NULL,
    min_fee numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seoper_sys_secu_type_fee OWNER TO postgres;

--
-- Name: tb_seoper_sys_secu_type_fee_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seoper_sys_secu_type_fee ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seoper_sys_secu_type_fee_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seotcsecu_asac_capit; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seotcsecu_asac_capit (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    payback_balance numeric(18,2) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    avail_adjust_amt numeric(18,4) NOT NULL,
    bank_balance numeric(18,4) NOT NULL,
    futu_bail numeric(18,2) NOT NULL,
    futu_bail_capt numeric(18,2) NOT NULL,
    pre_settle_amt numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seotcsecu_asac_capit OWNER TO postgres;

--
-- Name: tb_seotcsecu_asac_capit_adjust_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    adjust_jour_no bigint NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    before_curr_amt numeric(18,2) NOT NULL,
    before_frozen_amt numeric(18,2) NOT NULL,
    before_unfrozen_amt numeric(18,2) CONSTRAINT tb_seotcsecu_asac_capit_adjust_jou_before_unfrozen_amt_not_null NOT NULL,
    before_pre_settle_amt numeric(18,4) CONSTRAINT tb_seotcsecu_asac_capit_adjust_j_before_pre_settle_amt_not_null NOT NULL,
    before_amt numeric(18,4) NOT NULL,
    busi_flag integer NOT NULL,
    adjust_amt numeric(18,4) NOT NULL,
    deal_status integer NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    pre_settle_amt numeric(18,4) NOT NULL,
    after_amt numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL,
    source_row_id bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour OWNER TO postgres;

--
-- Name: tb_seotcsecu_asac_capit_adjust_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seotcsecu_asac_capit_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seotcsecu_asac_capit ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seotcsecu_asac_capit_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seotcsecu_asac_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seotcsecu_asac_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    unit_nav numeric(18,12) NOT NULL,
    posi_market_value numeric(18,2) NOT NULL,
    open_date integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seotcsecu_asac_posi OWNER TO postgres;

--
-- Name: tb_seotcsecu_asac_posi_adjust_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    adjust_jour_no bigint NOT NULL,
    jour_flag integer NOT NULL,
    adjust_qty numeric(18,2) NOT NULL,
    adjust_amt numeric(18,4) NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    unit_nav numeric(18,12) NOT NULL,
    posi_market_value numeric(18,2) NOT NULL,
    open_date integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour OWNER TO postgres;

--
-- Name: tb_seotcsecu_asac_posi_adjust_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seotcsecu_asac_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seotcsecu_asac_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seotcsecu_asac_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_asac_capit; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_asac_capit (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    payback_balance numeric(18,2) NOT NULL,
    instr_avail_amt numeric(18,4) NOT NULL,
    hk_avail_amt numeric(18,4) NOT NULL,
    hk_instr_avail_amt numeric(18,4) NOT NULL,
    avail_bail numeric(18,2) NOT NULL,
    instr_avail_margin numeric(18,4) NOT NULL,
    bank_balance numeric(18,4) NOT NULL,
    futu_bail numeric(18,2) NOT NULL,
    futu_bail_capt numeric(18,2) NOT NULL,
    "T1_avail_amt" numeric(18,4) NOT NULL,
    "T2_avail_amt" numeric(18,4) NOT NULL,
    "T1_instr_avail_amt" numeric(18,4) NOT NULL,
    "T2_instr_avail_amt" numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_asac_capit OWNER TO postgres;

--
-- Name: tb_sestra_asac_capit_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_asac_capit ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_asac_capit_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_asac_capit_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_asac_capit_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    payback_balance numeric(18,2) NOT NULL,
    instr_avail_amt numeric(18,4) NOT NULL,
    hk_avail_amt numeric(18,4) NOT NULL,
    hk_instr_avail_amt numeric(18,4) NOT NULL,
    avail_bail numeric(18,2) NOT NULL,
    instr_avail_margin numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_asac_capit_rsp OWNER TO postgres;

--
-- Name: tb_sestra_asac_capit_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_asac_capit_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_asac_capit_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_asac_capit_trade; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_asac_capit_trade (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    trade_frozen_amt numeric(18,4) NOT NULL,
    trade_unfrozen_amt numeric(18,4) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    fina_buy_instr_amt numeric(18,4) NOT NULL,
    fina_buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    loan_return_comm_amt numeric(16,4) NOT NULL,
    loan_return_order_amt numeric(16,4) NOT NULL,
    loan_return_strike_amt numeric(16,4) NOT NULL,
    fina_return_comm_amt numeric(18,4) NOT NULL,
    fina_return_order_amt numeric(18,4) NOT NULL,
    fina_return_strike_amt numeric(18,4) NOT NULL,
    return_strike_fee numeric(18,4) NOT NULL,
    debt_strike_fee numeric(16,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_asac_capit_trade OWNER TO postgres;

--
-- Name: tb_sestra_asac_capit_trade_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_asac_capit_trade ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_asac_capit_trade_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_asac_capit_trade_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_asac_capit_trade_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    trade_frozen_amt numeric(18,4) NOT NULL,
    trade_unfrozen_amt numeric(18,4) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    fina_buy_instr_amt numeric(18,4) NOT NULL,
    fina_buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    loan_return_comm_amt numeric(16,4) NOT NULL,
    loan_return_order_amt numeric(16,4) NOT NULL,
    loan_return_strike_amt numeric(16,4) NOT NULL,
    fina_return_comm_amt numeric(18,4) NOT NULL,
    fina_return_order_amt numeric(18,4) NOT NULL,
    fina_return_strike_amt numeric(18,4) NOT NULL,
    return_strike_fee numeric(18,4) NOT NULL,
    debt_strike_fee numeric(16,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_asac_capit_trade_rsp OWNER TO postgres;

--
-- Name: tb_sestra_asac_capit_trade_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_asac_capit_trade_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_asac_capit_trade_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_asac_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_asac_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    co_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_name character varying(64) NOT NULL,
    forbid_order_dir character varying(64) NOT NULL,
    buy_mode integer NOT NULL,
    sell_mode integer NOT NULL,
    avail_qty numeric(18,2) NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    begin_max_loan_amount numeric(18,2) NOT NULL,
    curr_max_loan_amount numeric(18,2) NOT NULL,
    begin_shortsell_quota numeric(18,2) NOT NULL,
    curr_shortsell_quota numeric(18,2) NOT NULL,
    begin_used_loan_qty numeric(18,2) NOT NULL,
    curr_used_loan_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    pupil_flag integer NOT NULL,
    online_new_share_wait_qty numeric(18,2) NOT NULL,
    offline_new_share_wait_qty numeric(18,2) NOT NULL,
    dividend_qty numeric(18,2) NOT NULL,
    pla_qty numeric(16,4) NOT NULL,
    impawn_qty numeric(18,2) NOT NULL,
    realize_pandl numeric(18,2) NOT NULL,
    sum_realize_pandl numeric(16,4) NOT NULL,
    "T1_avail_qty" numeric(18,2) NOT NULL,
    instr_avail_qty numeric(18,2) NOT NULL,
    "T1_instr_avail_qty" numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_asac_posi OWNER TO postgres;

--
-- Name: tb_sestra_asac_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_asac_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_asac_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_asac_posi_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_asac_posi_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    co_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_name character varying(64) NOT NULL,
    forbid_order_dir character varying(64) NOT NULL,
    buy_mode integer NOT NULL,
    sell_mode integer NOT NULL,
    avail_qty numeric(18,2) NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    begin_max_loan_amount numeric(18,2) NOT NULL,
    curr_max_loan_amount numeric(18,2) NOT NULL,
    begin_shortsell_quota numeric(18,2) NOT NULL,
    curr_shortsell_quota numeric(18,2) NOT NULL,
    begin_used_loan_qty numeric(18,2) NOT NULL,
    curr_used_loan_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_asac_posi_rsp OWNER TO postgres;

--
-- Name: tb_sestra_asac_posi_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_asac_posi_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_asac_posi_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_asac_posi_trade; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_asac_posi_trade (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    last_update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    main_flag integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    co_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    trade_frozen_qty numeric(18,2) NOT NULL,
    trade_unfrozen_qty numeric(18,2) NOT NULL,
    net_trade_frozen_qty numeric(18,2) NOT NULL,
    trade_net_qty numeric(18,2) NOT NULL,
    buy_instr_qty numeric(18,2) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_qty numeric(18,2) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    buy_strike_qty numeric(18,2) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_qty numeric(18,2) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_qty numeric(18,2) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_qty numeric(18,2) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    buy_strike_unfrozen_qty numeric(18,2) NOT NULL,
    loan_sell_instr_qty numeric(18,2) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_qty numeric(18,2) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_qty numeric(18,2) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    secuback_amount numeric(18,2) NOT NULL,
    used_loan_qty numeric(18,2) NOT NULL,
    loan_return_strike_amt numeric(16,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_asac_posi_trade OWNER TO postgres;

--
-- Name: tb_sestra_asac_posi_trade_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_asac_posi_trade ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_asac_posi_trade_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_asac_posi_trade_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_asac_posi_trade_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    last_update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    main_flag integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    co_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    trade_frozen_qty numeric(18,2) NOT NULL,
    trade_unfrozen_qty numeric(18,2) NOT NULL,
    net_trade_frozen_qty numeric(18,2) NOT NULL,
    trade_net_qty numeric(18,2) NOT NULL,
    buy_instr_qty numeric(18,2) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_qty numeric(18,2) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    buy_strike_qty numeric(18,2) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_qty numeric(18,2) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_qty numeric(18,2) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_qty numeric(18,2) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    buy_strike_unfrozen_qty numeric(18,2) NOT NULL,
    loan_sell_instr_qty numeric(18,2) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_qty numeric(18,2) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_qty numeric(18,2) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    secuback_amount numeric(18,2) NOT NULL,
    used_loan_qty numeric(18,2) NOT NULL,
    loan_return_strike_amt numeric(16,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_asac_posi_trade_rsp OWNER TO postgres;

--
-- Name: tb_sestra_asac_posi_trade_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_asac_posi_trade_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_asac_posi_trade_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_bondrepo; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_bondrepo (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    exch_rate numeric(18,12) NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    target_code character varying(32) NOT NULL,
    instr_no character varying(32) NOT NULL,
    order_dir integer NOT NULL,
    repo_qty numeric(18,2) NOT NULL,
    repo_amt numeric(18,4) NOT NULL,
    repo_rate numeric(18,12) NOT NULL,
    repo_trade_date integer NOT NULL,
    external_no character varying(32) NOT NULL,
    out_order_id character varying(32) NOT NULL,
    strike_no character varying(64) NOT NULL,
    repo_days integer NOT NULL,
    cash_capt_days integer NOT NULL,
    repo_back_date integer NOT NULL,
    repo_back_amt numeric(18,4) NOT NULL,
    repo_back_intrst numeric(16,4) NOT NULL,
    repo_back_trade_date integer NOT NULL,
    repo_status character varying(2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_bondrepo OWNER TO postgres;

--
-- Name: tb_sestra_bondrepo_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_bondrepo ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_bondrepo_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_command; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_command (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    busi_user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    exch_unit integer NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    terminal_second_external_no character varying(32) NOT NULL,
    stop_price numeric(16,4) NOT NULL,
    order_dir integer NOT NULL,
    exor_no integer NOT NULL,
    exor_name character varying(32) NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_status integer NOT NULL,
    strike_status integer NOT NULL,
    settle_date integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    executor_no integer NOT NULL,
    initiator_user_name character varying(255) NOT NULL,
    executor_user_name character varying(255) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_price_type integer NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    instr_cancel_qty numeric(18,2) NOT NULL,
    cancel_time integer NOT NULL,
    cancel_times integer NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    order_frozen_amt numeric(18,4) NOT NULL,
    order_frozen_qty numeric(18,2) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    net_price_flag integer NOT NULL,
    begin_date integer NOT NULL,
    begin_time integer NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    market_begin_date integer NOT NULL,
    market_begin_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_user_name character varying(255) NOT NULL,
    appr_desc character varying(255) NOT NULL,
    comm_dist_oper integer NOT NULL,
    disp_status integer NOT NULL,
    disp_remark character varying(255) NOT NULL,
    baset_id integer NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    exter_comm_flag integer NOT NULL,
    complete_flag integer NOT NULL,
    valid_flag integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    comb_code character varying(32) NOT NULL,
    asac_type integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    instr_deal_status integer NOT NULL,
    order_in_way integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_command OWNER TO postgres;

--
-- Name: tb_sestra_command_copy1; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_command_copy1 (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    busi_user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    exch_unit integer NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    terminal_second_external_no character varying(32) NOT NULL,
    stop_price numeric(16,4) NOT NULL,
    order_dir integer NOT NULL,
    exor_no integer NOT NULL,
    exor_name character varying(32) NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_status integer NOT NULL,
    strike_status integer NOT NULL,
    settle_date integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    executor_no integer NOT NULL,
    initiator_user_name character varying(255) NOT NULL,
    executor_user_name character varying(255) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_price_type integer NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    instr_cancel_qty numeric(18,2) NOT NULL,
    cancel_time integer NOT NULL,
    cancel_times integer NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    order_frozen_amt numeric(18,4) NOT NULL,
    order_frozen_qty numeric(18,2) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    net_price_flag integer NOT NULL,
    begin_date integer NOT NULL,
    begin_time integer NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    market_begin_date integer NOT NULL,
    market_begin_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_user_name character varying(255) NOT NULL,
    appr_desc character varying(255) NOT NULL,
    comm_dist_oper integer NOT NULL,
    disp_status integer NOT NULL,
    disp_remark character varying(255) NOT NULL,
    baset_id integer NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    exter_comm_flag integer NOT NULL,
    complete_flag integer NOT NULL,
    valid_flag integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    comb_code character varying(32) NOT NULL,
    asac_type integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    instr_deal_status integer NOT NULL,
    order_in_way integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_command_copy1 OWNER TO postgres;

--
-- Name: tb_sestra_command_copy1_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_command_copy1 ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_command_copy1_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_command_copy2; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_command_copy2 (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    busi_user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    exch_unit integer NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    terminal_second_external_no character varying(32) NOT NULL,
    stop_price numeric(16,4) NOT NULL,
    order_dir integer NOT NULL,
    exor_no integer NOT NULL,
    exor_name character varying(32) NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_status integer NOT NULL,
    strike_status integer NOT NULL,
    settle_date integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    executor_no integer NOT NULL,
    initiator_user_name character varying(255) NOT NULL,
    executor_user_name character varying(255) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_price_type integer NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    instr_cancel_qty numeric(18,2) NOT NULL,
    cancel_time integer NOT NULL,
    cancel_times integer NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    order_frozen_amt numeric(18,4) NOT NULL,
    order_frozen_qty numeric(18,2) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    net_price_flag integer NOT NULL,
    begin_date integer NOT NULL,
    begin_time integer NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    market_begin_date integer NOT NULL,
    market_begin_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_user_name character varying(255) NOT NULL,
    appr_desc character varying(255) NOT NULL,
    comm_dist_oper integer NOT NULL,
    disp_status integer NOT NULL,
    disp_remark character varying(255) NOT NULL,
    baset_id integer NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    exter_comm_flag integer NOT NULL,
    complete_flag integer NOT NULL,
    valid_flag integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    comb_code character varying(32) NOT NULL,
    asac_type integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    instr_deal_status integer NOT NULL,
    order_in_way integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_command_copy2 OWNER TO postgres;

--
-- Name: tb_sestra_command_copy2_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_command_copy2 ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_command_copy2_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_command_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_command ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_command_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_command_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_command_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    busi_user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    exch_unit integer NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    terminal_second_external_no character varying(32) NOT NULL,
    stop_price numeric(16,4) NOT NULL,
    order_dir integer NOT NULL,
    exor_no integer NOT NULL,
    exor_name character varying(32) NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_status integer NOT NULL,
    strike_status integer NOT NULL,
    settle_date integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    executor_no integer NOT NULL,
    initiator_user_name character varying(255) NOT NULL,
    executor_user_name character varying(255) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_price_type integer NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    instr_cancel_qty numeric(18,2) NOT NULL,
    cancel_time integer NOT NULL,
    cancel_times integer NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    order_frozen_amt numeric(18,4) NOT NULL,
    order_frozen_qty numeric(18,2) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    net_price_flag integer NOT NULL,
    begin_date integer NOT NULL,
    begin_time integer NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    market_begin_date integer NOT NULL,
    market_begin_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_user_name character varying(255) NOT NULL,
    appr_desc character varying(255) NOT NULL,
    comm_dist_oper integer NOT NULL,
    disp_status integer NOT NULL,
    disp_remark character varying(255) NOT NULL,
    baset_id integer NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    exter_comm_flag integer NOT NULL,
    complete_flag integer NOT NULL,
    valid_flag integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    comb_code character varying(32) NOT NULL,
    asac_type integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    instr_deal_status integer NOT NULL,
    order_in_way integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_command_rsp OWNER TO postgres;

--
-- Name: tb_sestra_command_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_command_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_command_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_expordermodify_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_expordermodify_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    serial_no character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    busi_flag integer NOT NULL,
    busi_menu_no integer NOT NULL,
    busi_oper_ip character varying(32) NOT NULL,
    busi_oper_mac character varying(32) NOT NULL,
    busi_oper_menu_no integer NOT NULL,
    busi_oper_way integer NOT NULL,
    busi_oper_context character varying(2048) NOT NULL,
    busi_oper_info character varying(1024) NOT NULL,
    busi_user_no integer NOT NULL,
    busi_func_code character varying(16) NOT NULL,
    busi_msg_content character varying(4096) NOT NULL,
    busi_msg_status integer NOT NULL,
    out_order_id character varying(32) NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    occur_last_qty numeric(18,2) NOT NULL,
    after_occur_amt numeric(18,4) NOT NULL,
    occur_amt numeric(18,4) NOT NULL,
    occur_date integer NOT NULL,
    occur_time integer NOT NULL,
    occur_qty numeric(18,2) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    comm_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    channel_no integer NOT NULL,
    exor_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    out_acco_id integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    external_no character varying(32) NOT NULL,
    orig_external_no character varying(32) NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_dir integer NOT NULL,
    price_type integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    order_status integer NOT NULL,
    withdrw_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    order_rsp_status integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_expordermodify_jour OWNER TO postgres;

--
-- Name: tb_sestra_expordermodify_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_expordermodify_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_expordermodify_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_expordermodify_jour_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_expordermodify_jour_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    serial_no character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    busi_flag integer NOT NULL,
    busi_menu_no integer NOT NULL,
    busi_oper_ip character varying(32) NOT NULL,
    busi_oper_mac character varying(32) NOT NULL,
    busi_oper_menu_no integer NOT NULL,
    busi_oper_way integer NOT NULL,
    busi_oper_context character varying(2048) NOT NULL,
    busi_oper_info character varying(1024) NOT NULL,
    busi_user_no integer NOT NULL,
    busi_func_code character varying(16) NOT NULL,
    busi_msg_content character varying(4096) NOT NULL,
    busi_msg_status integer NOT NULL,
    out_order_id character varying(32) NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    occur_last_qty numeric(18,2) NOT NULL,
    after_occur_amt numeric(18,4) NOT NULL,
    occur_amt numeric(18,4) NOT NULL,
    occur_date integer NOT NULL,
    occur_time integer NOT NULL,
    occur_qty numeric(18,2) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    comm_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    channel_no integer NOT NULL,
    exor_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    out_acco_id integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    external_no character varying(32) NOT NULL,
    orig_external_no character varying(32) NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_dir integer NOT NULL,
    price_type integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    order_status integer NOT NULL,
    withdrw_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    order_rsp_status integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_expordermodify_jour_rsp OWNER TO postgres;

--
-- Name: tb_sestra_expordermodify_jour_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_expordermodify_jour_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_expordermodify_jour_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_instructapprove; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_instructapprove (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    appr_no character varying(32) NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    order_dir integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_desc character varying(255) NOT NULL,
    exter_comm_flag integer NOT NULL,
    asac_type integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_instructapprove OWNER TO postgres;

--
-- Name: tb_sestra_instructapprove_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_instructapprove ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_instructapprove_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_instructapprove_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_instructapprove_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    appr_no character varying(32) NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    order_dir integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_desc character varying(255) NOT NULL,
    exter_comm_flag integer NOT NULL,
    asac_type integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_instructapprove_rsp OWNER TO postgres;

--
-- Name: tb_sestra_instructapprove_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_instructapprove_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_instructapprove_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_instructjour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_instructjour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    serial_no character varying(64) NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    instr_desc character varying(255) NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    instr_type integer NOT NULL,
    co_no integer NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    order_dir integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_instructjour OWNER TO postgres;

--
-- Name: tb_sestra_instructjour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_instructjour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_instructjour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_instructjour_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_instructjour_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    serial_no character varying(64) NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    instr_desc character varying(255) NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    instr_type integer NOT NULL,
    co_no integer NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    order_dir integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_instructjour_rsp OWNER TO postgres;

--
-- Name: tb_sestra_instructjour_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_instructjour_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_instructjour_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_multidaycommand; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_multidaycommand (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    busi_user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    exch_unit integer NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    terminal_second_external_no character varying(32) NOT NULL,
    stop_price numeric(16,4) NOT NULL,
    order_dir integer NOT NULL,
    exor_no integer NOT NULL,
    exor_name character varying(32) NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_status integer NOT NULL,
    strike_status integer NOT NULL,
    settle_date integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    executor_no integer NOT NULL,
    initiator_user_name character varying(255) NOT NULL,
    executor_user_name character varying(255) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_price_type integer NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    instr_cancel_qty numeric(18,2) NOT NULL,
    cancel_time integer NOT NULL,
    cancel_times integer NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    order_frozen_amt numeric(18,4) NOT NULL,
    order_frozen_qty numeric(18,2) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    net_price_flag integer NOT NULL,
    begin_date integer NOT NULL,
    begin_time integer NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    market_begin_date integer NOT NULL,
    market_begin_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_user_name character varying(255) NOT NULL,
    appr_desc character varying(255) NOT NULL,
    comm_dist_oper integer NOT NULL,
    disp_status integer NOT NULL,
    disp_remark character varying(255) NOT NULL,
    baset_id integer NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    exter_comm_flag integer NOT NULL,
    complete_flag integer NOT NULL,
    valid_flag integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    comb_code character varying(32) NOT NULL,
    asac_type integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    instr_deal_status integer NOT NULL,
    order_in_way integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_multidaycommand OWNER TO postgres;

--
-- Name: tb_sestra_multidaycommand_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_multidaycommand ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_multidaycommand_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_order; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_order (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    out_order_id character varying(32) NOT NULL,
    report_date integer NOT NULL,
    report_no character varying(32) NOT NULL,
    init_date integer NOT NULL,
    settle_date integer NOT NULL,
    order_oper_way integer NOT NULL,
    order_in_way integer NOT NULL,
    trade_type integer NOT NULL,
    valid_type integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    dma integer NOT NULL,
    secu_source_type integer NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    terminal_second_external_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    terminal_msg_id character varying(64) NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    channel_no integer NOT NULL,
    exor_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_no integer NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    external_no character varying(32) NOT NULL,
    orig_external_no character varying(32) NOT NULL,
    report_time integer NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_dir integer NOT NULL,
    price_type integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    order_status integer NOT NULL,
    order_frozen_amt numeric(18,4) NOT NULL,
    order_frozen_qty numeric(18,2) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    withdrw_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_fee numeric(18,4) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    net_trade_frozen_qty numeric(18,2) NOT NULL,
    buy_strike_unfrozen_qty numeric(18,2) NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    remark_info character varying(255) NOT NULL,
    rsp_info character varying(255) NOT NULL,
    withdrw_flag integer NOT NULL,
    invest_type integer NOT NULL,
    order_deal_type integer NOT NULL,
    client_acc_code character varying(32) NOT NULL,
    client_order_id character varying(32) NOT NULL,
    "FIX448_PartyID" character varying(32) NOT NULL,
    "FIX447_PartyIDSource" character varying(32) NOT NULL,
    "FIX452_PartyRole" integer NOT NULL,
    "FIX375_ContraBroker" character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_order OWNER TO postgres;

--
-- Name: tb_sestra_order_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_order ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_order_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_order_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_order_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    out_order_id character varying(32) NOT NULL,
    report_date integer NOT NULL,
    report_no character varying(32) NOT NULL,
    init_date integer NOT NULL,
    settle_date integer NOT NULL,
    order_oper_way integer NOT NULL,
    order_in_way integer NOT NULL,
    trade_type integer NOT NULL,
    valid_type integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    dma integer NOT NULL,
    secu_source_type integer NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    terminal_second_external_no character varying(32) NOT NULL,
    terminal_batch_no bigint NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    terminal_msg_id character varying(64) NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    channel_no integer NOT NULL,
    exor_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_no integer NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    external_no character varying(32) NOT NULL,
    orig_external_no character varying(32) NOT NULL,
    report_time integer NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_dir integer NOT NULL,
    price_type integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    order_status integer NOT NULL,
    order_frozen_amt numeric(18,4) NOT NULL,
    order_frozen_qty numeric(18,2) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    withdrw_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_fee numeric(18,4) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    net_trade_frozen_qty numeric(18,2) NOT NULL,
    buy_strike_unfrozen_qty numeric(18,2) NOT NULL,
    compli_status integer NOT NULL,
    compli_trig_id bigint NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    remark_info character varying(255) NOT NULL,
    rsp_info character varying(255) NOT NULL,
    withdrw_flag integer NOT NULL,
    client_acc_code character varying(32) NOT NULL,
    client_order_id character varying(32) NOT NULL,
    "FIX448_PartyID" character varying(32) NOT NULL,
    "FIX447_PartyIDSource" character varying(32) NOT NULL,
    "FIX452_PartyRole" integer NOT NULL,
    "FIX375_ContraBroker" character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_order_rsp OWNER TO postgres;

--
-- Name: tb_sestra_order_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_order_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_order_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_ordersum; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_ordersum (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    exch_no integer NOT NULL,
    comm_batch_no bigint NOT NULL,
    terminal_batch_no bigint NOT NULL,
    init_date integer NOT NULL,
    order_oper_way integer NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    channel_no integer NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    exor_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    settle_date integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_dir integer NOT NULL,
    price_type integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    order_status integer NOT NULL,
    withdrw_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    client_acc_code character varying(32) NOT NULL,
    client_order_id character varying(32) NOT NULL,
    "FIX448_PartyID" character varying(32) NOT NULL,
    "FIX447_PartyIDSource" character varying(32) NOT NULL,
    "FIX452_PartyRole" integer NOT NULL,
    "FIX375_ContraBroker" character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_ordersum OWNER TO postgres;

--
-- Name: tb_sestra_ordersum_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_ordersum ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_ordersum_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_ordersum_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_ordersum_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    exch_no integer NOT NULL,
    comm_batch_no bigint NOT NULL,
    terminal_batch_no bigint NOT NULL,
    init_date integer NOT NULL,
    order_oper_way integer NOT NULL,
    terminal_external_no character varying(32) NOT NULL,
    busi_msg_id character varying(64) NOT NULL,
    channel_no integer NOT NULL,
    user_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    exor_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    settle_date integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_dir integer NOT NULL,
    price_type integer NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    order_status integer NOT NULL,
    withdrw_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    client_acc_code character varying(32) NOT NULL,
    client_order_id character varying(32) NOT NULL,
    "FIX448_PartyID" character varying(32) NOT NULL,
    "FIX447_PartyIDSource" character varying(32) NOT NULL,
    "FIX452_PartyRole" integer NOT NULL,
    "FIX375_ContraBroker" character varying(32) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_ordersum_rsp OWNER TO postgres;

--
-- Name: tb_sestra_ordersum_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_ordersum_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_ordersum_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_pd_unit_capit; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_pd_unit_capit (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    payback_balance numeric(18,2) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    avail_adjust_amt numeric(18,4) NOT NULL,
    instr_avail_amt numeric(18,4) NOT NULL,
    hk_avail_amt numeric(18,4) NOT NULL,
    hk_instr_avail_amt numeric(18,4) NOT NULL,
    avail_bail numeric(18,2) NOT NULL,
    instr_avail_margin numeric(18,4) NOT NULL,
    bank_balance numeric(18,4) NOT NULL,
    futu_bail numeric(18,2) NOT NULL,
    futu_bail_capt numeric(18,2) NOT NULL,
    "T1_avail_amt" numeric(18,4) NOT NULL,
    "T2_avail_amt" numeric(18,4) NOT NULL,
    "T1_instr_avail_amt" numeric(18,4) NOT NULL,
    "T2_instr_avail_amt" numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_pd_unit_capit OWNER TO postgres;

--
-- Name: tb_sestra_pd_unit_capit_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_pd_unit_capit ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_pd_unit_capit_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_pd_unit_capit_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_pd_unit_capit_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    payback_balance numeric(18,2) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    avail_adjust_amt numeric(18,4) NOT NULL,
    instr_avail_amt numeric(18,4) NOT NULL,
    hk_avail_amt numeric(18,4) NOT NULL,
    hk_instr_avail_amt numeric(18,4) NOT NULL,
    avail_bail numeric(18,2) NOT NULL,
    instr_avail_margin numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_pd_unit_capit_rsp OWNER TO postgres;

--
-- Name: tb_sestra_pd_unit_capit_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_pd_unit_capit_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_pd_unit_capit_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_pd_unit_capit_trade; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_pd_unit_capit_trade (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    main_flag integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    trade_frozen_amt numeric(18,4) NOT NULL,
    trade_unfrozen_amt numeric(18,4) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    fina_buy_instr_amt numeric(18,4) NOT NULL,
    fina_buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    loan_return_comm_amt numeric(16,4) NOT NULL,
    loan_return_order_amt numeric(16,4) NOT NULL,
    loan_return_strike_amt numeric(16,4) NOT NULL,
    fina_return_comm_amt numeric(18,4) NOT NULL,
    fina_return_order_amt numeric(18,4) NOT NULL,
    fina_return_strike_amt numeric(18,4) NOT NULL,
    return_strike_fee numeric(18,4) NOT NULL,
    debt_strike_fee numeric(16,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_pd_unit_capit_trade OWNER TO postgres;

--
-- Name: tb_sestra_pd_unit_capit_trade_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_pd_unit_capit_trade ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_pd_unit_capit_trade_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_pd_unit_capit_trade_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_pd_unit_capit_trade_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    main_flag integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    co_no integer NOT NULL,
    trade_frozen_amt numeric(18,4) NOT NULL,
    trade_unfrozen_amt numeric(18,4) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    fina_buy_instr_amt numeric(18,4) NOT NULL,
    fina_buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    loan_return_comm_amt numeric(16,4) NOT NULL,
    loan_return_order_amt numeric(16,4) CONSTRAINT tb_sestra_pd_unit_capit_trade_rs_loan_return_order_amt_not_null NOT NULL,
    loan_return_strike_amt numeric(16,4) CONSTRAINT tb_sestra_pd_unit_capit_trade_r_loan_return_strike_amt_not_null NOT NULL,
    fina_return_comm_amt numeric(18,4) NOT NULL,
    fina_return_order_amt numeric(18,4) CONSTRAINT tb_sestra_pd_unit_capit_trade_rs_fina_return_order_amt_not_null NOT NULL,
    fina_return_strike_amt numeric(18,4) CONSTRAINT tb_sestra_pd_unit_capit_trade_r_fina_return_strike_amt_not_null NOT NULL,
    return_strike_fee numeric(18,4) NOT NULL,
    debt_strike_fee numeric(16,4) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_pd_unit_capit_trade_rsp OWNER TO postgres;

--
-- Name: tb_sestra_pd_unit_capit_trade_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_pd_unit_capit_trade_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_pd_unit_capit_trade_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_pd_unit_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_pd_unit_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    last_update_times integer NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    co_no integer NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    forbid_order_dir character varying(64) NOT NULL,
    buy_mode integer NOT NULL,
    sell_mode integer NOT NULL,
    avail_qty numeric(18,2) NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    begin_max_loan_amount numeric(18,2) NOT NULL,
    curr_max_loan_amount numeric(18,2) NOT NULL,
    begin_shortsell_quota numeric(18,2) NOT NULL,
    curr_shortsell_quota numeric(18,2) NOT NULL,
    begin_used_loan_qty numeric(18,2) NOT NULL,
    curr_used_loan_qty numeric(18,2) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    unfrozen_qty numeric(18,2) NOT NULL,
    avail_adjust_qty numeric(18,2) NOT NULL,
    lock_secu_qty numeric(18,2) NOT NULL,
    pupil_flag integer NOT NULL,
    online_new_share_wait_qty numeric(18,2) NOT NULL,
    offline_new_share_wait_qty numeric(18,2) NOT NULL,
    dividend_qty numeric(18,2) NOT NULL,
    pla_qty numeric(16,4) NOT NULL,
    impawn_qty numeric(18,2) NOT NULL,
    realize_pandl numeric(18,2) NOT NULL,
    sum_realize_pandl numeric(16,4) NOT NULL,
    "T1_avail_qty" numeric(18,2) NOT NULL,
    instr_avail_qty numeric(18,2) NOT NULL,
    "T1_instr_avail_qty" numeric(18,2) NOT NULL,
    intrst_cost_amt numeric(18,4) NOT NULL,
    buy_fee numeric(18,4) NOT NULL,
    sell_fee numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_pd_unit_posi OWNER TO postgres;

--
-- Name: tb_sestra_pd_unit_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_pd_unit_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_pd_unit_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_pd_unit_posi_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_pd_unit_posi_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    last_update_times integer NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    co_no integer NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    forbid_order_dir character varying(64) NOT NULL,
    buy_mode integer NOT NULL,
    sell_mode integer NOT NULL,
    avail_qty numeric(18,2) NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    cost_amt numeric(18,4) NOT NULL,
    begin_max_loan_amount numeric(18,2) NOT NULL,
    curr_max_loan_amount numeric(18,2) NOT NULL,
    begin_shortsell_quota numeric(18,2) NOT NULL,
    curr_shortsell_quota numeric(18,2) NOT NULL,
    begin_used_loan_qty numeric(18,2) NOT NULL,
    curr_used_loan_qty numeric(18,2) NOT NULL,
    frozen_qty numeric(18,2) NOT NULL,
    unfrozen_qty numeric(18,2) NOT NULL,
    avail_adjust_qty numeric(18,2) NOT NULL,
    lock_secu_qty numeric(18,2) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_pd_unit_posi_rsp OWNER TO postgres;

--
-- Name: tb_sestra_pd_unit_posi_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_pd_unit_posi_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_pd_unit_posi_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_pd_unit_posi_trade; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_pd_unit_posi_trade (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    last_update_times integer NOT NULL,
    pd_unit_no integer NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    main_flag integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    co_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    trade_frozen_qty numeric(18,2) NOT NULL,
    trade_unfrozen_qty numeric(18,2) NOT NULL,
    net_trade_frozen_qty numeric(18,2) NOT NULL,
    trade_net_qty numeric(18,2) NOT NULL,
    buy_instr_qty numeric(18,2) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_qty numeric(18,2) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    buy_strike_qty numeric(18,2) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_qty numeric(18,2) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_qty numeric(18,2) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_qty numeric(18,2) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    buy_strike_unfrozen_qty numeric(18,2) NOT NULL,
    loan_sell_instr_qty numeric(18,2) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_qty numeric(18,2) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_qty numeric(18,2) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    secuback_amount numeric(18,2) NOT NULL,
    used_loan_qty numeric(18,2) NOT NULL,
    loan_return_strike_amt numeric(16,4) NOT NULL,
    buy_fee numeric(18,4) NOT NULL,
    sell_fee numeric(18,4) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_pd_unit_posi_trade OWNER TO postgres;

--
-- Name: tb_sestra_pd_unit_posi_trade_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_pd_unit_posi_trade ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_pd_unit_posi_trade_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_pd_unit_posi_trade_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_pd_unit_posi_trade_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    last_update_times integer NOT NULL,
    pd_unit_no integer NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    main_flag integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    co_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    trade_frozen_qty numeric(18,2) NOT NULL,
    trade_unfrozen_qty numeric(18,2) NOT NULL,
    net_trade_frozen_qty numeric(18,2) NOT NULL,
    trade_net_qty numeric(18,2) NOT NULL,
    buy_instr_qty numeric(18,2) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    buy_qty numeric(18,2) NOT NULL,
    buy_amt numeric(18,4) NOT NULL,
    fina_buy_strike_qty numeric(18,2) NOT NULL,
    fina_buy_strike_amt numeric(18,4) NOT NULL,
    buy_strike_qty numeric(18,2) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_instr_qty numeric(18,2) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    sell_qty numeric(18,2) NOT NULL,
    sell_amt numeric(18,4) NOT NULL,
    sell_strike_qty numeric(18,2) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    buy_strike_unfrozen_qty numeric(18,2) CONSTRAINT tb_sestra_pd_unit_posi_trade_r_buy_strike_unfrozen_qty_not_null NOT NULL,
    loan_sell_instr_qty numeric(18,2) NOT NULL,
    loan_sell_instr_amt numeric(18,4) NOT NULL,
    loan_sell_qty numeric(18,2) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    loan_sell_strike_qty numeric(18,2) NOT NULL,
    loan_sell_strike_amt numeric(18,4) NOT NULL,
    secuback_amount numeric(18,2) NOT NULL,
    used_loan_qty numeric(18,2) NOT NULL,
    loan_return_strike_amt numeric(16,4) CONSTRAINT tb_sestra_pd_unit_posi_trade_rs_loan_return_strike_amt_not_null NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_pd_unit_posi_trade_rsp OWNER TO postgres;

--
-- Name: tb_sestra_pd_unit_posi_trade_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_pd_unit_posi_trade_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_pd_unit_posi_trade_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_strike; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_strike (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    exch_no integer NOT NULL,
    out_order_id character varying(32) NOT NULL,
    strike_date integer NOT NULL,
    strike_no character varying(64) NOT NULL,
    order_dir integer NOT NULL,
    init_date integer NOT NULL,
    settle_date integer NOT NULL,
    busi_user_no integer NOT NULL,
    user_no integer NOT NULL,
    channel_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    dma integer NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    exor_no integer NOT NULL,
    exor_name character varying(32) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    pd_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_no integer NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    external_no character varying(32) NOT NULL,
    orig_external_no character varying(32) NOT NULL,
    strike_time integer NOT NULL,
    report_date integer NOT NULL,
    report_no character varying(32) NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_id bigint NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_price numeric(16,4) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    strike_frozen_amt numeric(18,4) NOT NULL,
    strike_unfrozen_amt numeric(18,4) NOT NULL,
    strike_frozen_qty numeric(18,2) NOT NULL,
    strike_unfrozen_qty numeric(18,2) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    broker_co_id integer NOT NULL,
    invest_type integer NOT NULL,
    client_acc_code character varying(32) NOT NULL,
    client_order_id character varying(32) NOT NULL,
    broker_seat_id character varying(4) NOT NULL,
    counterparty_broker_code character varying(32) NOT NULL,
    counterparty_broker_seat_id character varying(4) NOT NULL,
    counterparty_client_acc_code character varying(32) NOT NULL,
    counterparty_client_order_id character varying(32) NOT NULL,
    src_bcan character varying(32) NOT NULL,
    dst_bcan character varying(32) NOT NULL,
    strike_jour_id character varying(16) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_strike OWNER TO postgres;

--
-- Name: tb_sestra_strike_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_strike ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_strike_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_strike_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_strike_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    exch_no integer NOT NULL,
    out_order_id character varying(32) NOT NULL,
    strike_date integer NOT NULL,
    strike_no character varying(64) NOT NULL,
    order_dir integer NOT NULL,
    init_date integer NOT NULL,
    settle_date integer NOT NULL,
    busi_user_no integer NOT NULL,
    user_no integer NOT NULL,
    channel_no integer NOT NULL,
    user_name character varying(64) NOT NULL,
    order_oper_way integer NOT NULL,
    dma integer NOT NULL,
    comm_batch_no bigint NOT NULL,
    instr_no character varying(32) NOT NULL,
    orig_instr_no character varying(32) NOT NULL,
    exor_no integer NOT NULL,
    exor_name character varying(32) NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    pd_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_no integer NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_no integer NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    external_no character varying(32) NOT NULL,
    orig_external_no character varying(32) NOT NULL,
    strike_time integer NOT NULL,
    report_date integer NOT NULL,
    report_no character varying(32) NOT NULL,
    order_date integer NOT NULL,
    order_time integer NOT NULL,
    order_batch_no bigint NOT NULL,
    order_id bigint NOT NULL,
    order_price numeric(16,4) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_price numeric(16,4) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    strike_frozen_amt numeric(18,4) NOT NULL,
    strike_unfrozen_amt numeric(18,4) NOT NULL,
    strike_frozen_qty numeric(18,2) NOT NULL,
    strike_unfrozen_qty numeric(18,2) NOT NULL,
    all_fee numeric(18,2) NOT NULL,
    stamp_tax numeric(18,2) NOT NULL,
    trans_fee numeric(18,2) NOT NULL,
    brkage_fee numeric(18,2) NOT NULL,
    "SEC_charges" numeric(18,2) NOT NULL,
    other_fee numeric(18,2) NOT NULL,
    trade_commis numeric(18,2) NOT NULL,
    other_commis numeric(18,2) NOT NULL,
    client_acc_code character varying(32) NOT NULL,
    client_order_id character varying(32) NOT NULL,
    broker_seat_id character varying(4) NOT NULL,
    counterparty_broker_code character varying(32) NOT NULL,
    counterparty_broker_seat_id character varying(4) NOT NULL,
    counterparty_client_acc_code character varying(32) NOT NULL,
    counterparty_client_order_id character varying(32) NOT NULL,
    src_bcan character varying(32) NOT NULL,
    dst_bcan character varying(32) NOT NULL,
    strike_jour_id character varying(16) NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_strike_rsp OWNER TO postgres;

--
-- Name: tb_sestra_strike_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_strike_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_strike_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_sumcommand; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_sumcommand (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    busi_user_no integer NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    exch_unit integer NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    terminal_batch_no bigint NOT NULL,
    stop_price numeric(16,4) NOT NULL,
    order_dir integer NOT NULL,
    exor_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_status integer NOT NULL,
    strike_status integer NOT NULL,
    settle_date integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    executor_no integer NOT NULL,
    initiator_user_name character varying(255) NOT NULL,
    executor_user_name character varying(255) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_price_type integer NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    instr_cancel_qty numeric(18,2) NOT NULL,
    instr_await_cancel_qty numeric(18,2) NOT NULL,
    instr_appr_qty numeric(18,2) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    net_price_flag integer NOT NULL,
    begin_date integer NOT NULL,
    begin_time integer NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    market_begin_date integer NOT NULL,
    market_begin_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    max_complete_date integer NOT NULL,
    max_complete_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_user_name character varying(255) NOT NULL,
    appr_desc character varying(255) NOT NULL,
    comm_dist_oper integer NOT NULL,
    disp_status integer NOT NULL,
    disp_remark character varying(255) NOT NULL,
    disp_date integer NOT NULL,
    disp_time integer NOT NULL,
    buy_instr_qty numeric(18,2) NOT NULL,
    sell_instr_qty numeric(18,2) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    buy_order_qty numeric(18,2) NOT NULL,
    sell_order_qty numeric(18,2) NOT NULL,
    buy_strike_qty numeric(18,2) NOT NULL,
    sell_strike_qty numeric(18,2) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    sum_buy_amt numeric(18,4) NOT NULL,
    sum_buy_qty numeric(18,2) NOT NULL,
    sum_sell_amt numeric(18,4) NOT NULL,
    sum_sell_qty numeric(18,2) NOT NULL,
    baset_id integer NOT NULL,
    exter_comm_flag integer NOT NULL,
    valid_flag integer NOT NULL,
    complete_flag integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    comb_code character varying(32) NOT NULL,
    compli_status integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    order_in_way integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_sumcommand OWNER TO postgres;

--
-- Name: tb_sestra_sumcommand_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_sumcommand ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_sumcommand_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_sestra_sumcommand_rsp; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_sestra_sumcommand_rsp (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    source_row_id bigint NOT NULL,
    last_update_times integer NOT NULL,
    init_date integer NOT NULL,
    oper_ip character varying(32) NOT NULL,
    oper_mac character varying(32) NOT NULL,
    oper_info character varying(1024) NOT NULL,
    user_no integer NOT NULL,
    busi_user_no integer NOT NULL,
    order_oper_way integer NOT NULL,
    instr_type integer NOT NULL,
    exch_unit integer NOT NULL,
    co_no integer NOT NULL,
    co_name character varying(64) NOT NULL,
    comm_batch_no bigint NOT NULL,
    orig_batch_no bigint NOT NULL,
    terminal_batch_no bigint NOT NULL,
    stop_price numeric(16,4) NOT NULL,
    order_dir integer NOT NULL,
    exor_no integer NOT NULL,
    pd_no integer NOT NULL,
    pd_unit_no integer NOT NULL,
    asac_no integer NOT NULL,
    pd_name character varying(64) NOT NULL,
    pd_unit_name character varying(64) NOT NULL,
    asac_name character varying(64) NOT NULL,
    out_acco_id integer NOT NULL,
    exch_no integer NOT NULL,
    secu_acco character varying(16) NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    secu_type integer NOT NULL,
    asset_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    instr_status integer NOT NULL,
    strike_status integer NOT NULL,
    settle_date integer NOT NULL,
    instr_date integer NOT NULL,
    instr_time integer NOT NULL,
    initiator_no integer NOT NULL,
    executor_no integer NOT NULL,
    initiator_user_name character varying(255) NOT NULL,
    executor_user_name character varying(255) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    limit_price numeric(16,4) NOT NULL,
    actual_limit_price numeric(16,4) NOT NULL,
    instr_price_type integer NOT NULL,
    instr_qty numeric(18,2) NOT NULL,
    instr_amt numeric(18,4) NOT NULL,
    instr_cancel_qty numeric(18,2) NOT NULL,
    instr_await_cancel_qty numeric(18,2) NOT NULL,
    instr_appr_qty numeric(18,2) NOT NULL,
    order_qty numeric(18,2) NOT NULL,
    waste_qty numeric(18,2) NOT NULL,
    strike_qty numeric(18,2) NOT NULL,
    strike_amt numeric(18,4) NOT NULL,
    instr_frozen_amt numeric(18,4) NOT NULL,
    instr_frozen_qty numeric(18,2) NOT NULL,
    net_price_flag integer NOT NULL,
    begin_date integer NOT NULL,
    begin_time integer NOT NULL,
    expire_date integer NOT NULL,
    expire_time integer NOT NULL,
    market_begin_date integer NOT NULL,
    market_begin_time integer NOT NULL,
    market_expire_date integer NOT NULL,
    market_expire_time integer NOT NULL,
    complete_date integer NOT NULL,
    complete_time integer NOT NULL,
    max_complete_date integer NOT NULL,
    max_complete_time integer NOT NULL,
    appr_date integer NOT NULL,
    appr_time integer NOT NULL,
    appr_status integer NOT NULL,
    appr_user_no integer NOT NULL,
    appr_user_name character varying(255) NOT NULL,
    appr_desc character varying(255) NOT NULL,
    comm_dist_oper integer NOT NULL,
    disp_status integer NOT NULL,
    disp_remark character varying(255) NOT NULL,
    disp_date integer NOT NULL,
    disp_time integer NOT NULL,
    buy_instr_qty numeric(18,2) NOT NULL,
    sell_instr_qty numeric(18,2) NOT NULL,
    buy_instr_amt numeric(18,4) NOT NULL,
    sell_instr_amt numeric(18,4) NOT NULL,
    buy_order_qty numeric(18,2) NOT NULL,
    sell_order_qty numeric(18,2) NOT NULL,
    buy_strike_qty numeric(18,2) NOT NULL,
    sell_strike_qty numeric(18,2) NOT NULL,
    buy_strike_amt numeric(18,4) NOT NULL,
    sell_strike_amt numeric(18,4) NOT NULL,
    total_strike_amt numeric(18,4) NOT NULL,
    total_strike_qty numeric(18,2) NOT NULL,
    sum_buy_amt numeric(18,4) NOT NULL,
    sum_buy_qty numeric(18,2) NOT NULL,
    sum_sell_amt numeric(18,4) NOT NULL,
    sum_sell_qty numeric(18,2) NOT NULL,
    baset_id integer NOT NULL,
    exter_comm_flag integer NOT NULL,
    valid_flag integer NOT NULL,
    complete_flag integer NOT NULL,
    comb_trade_flag integer NOT NULL,
    comb_code character varying(32) NOT NULL,
    compli_status integer NOT NULL,
    read_flag integer NOT NULL,
    instr_desc character varying(255) NOT NULL,
    param_info character varying(255) NOT NULL,
    remark_info character varying(255) NOT NULL,
    remark_info2 character varying(255) NOT NULL,
    reserved_field character varying(255) NOT NULL,
    reserved_field2 character varying(255) NOT NULL,
    order_in_way integer NOT NULL
);


ALTER TABLE jzdb_secu.tb_sestra_sumcommand_rsp OWNER TO postgres;

--
-- Name: tb_sestra_sumcommand_rsp_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_sestra_sumcommand_rsp ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_sestra_sumcommand_rsp_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seswap_asac_capit; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seswap_asac_capit (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    begin_amt numeric(18,4) NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    avail_amt numeric(18,4) NOT NULL,
    fetch_amt numeric(18,4) NOT NULL,
    loan_sell_amt numeric(18,4) NOT NULL,
    fina_debt numeric(18,4) NOT NULL,
    payback_balance numeric(18,2) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    avail_adjust_amt numeric(18,4) NOT NULL,
    bank_balance numeric(18,4) NOT NULL,
    futu_bail numeric(18,2) NOT NULL,
    futu_bail_capt numeric(18,2) NOT NULL,
    pre_settle_amt numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seswap_asac_capit OWNER TO postgres;

--
-- Name: tb_seswap_asac_capit_adjust_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seswap_asac_capit_adjust_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    adjust_jour_no bigint NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    settle_crncy_type integer NOT NULL,
    before_curr_amt numeric(18,2) NOT NULL,
    before_frozen_amt numeric(18,2) NOT NULL,
    before_unfrozen_amt numeric(18,2) NOT NULL,
    before_pre_settle_amt numeric(18,4) NOT NULL,
    before_amt numeric(18,4) NOT NULL,
    busi_flag integer NOT NULL,
    adjust_amt numeric(18,4) NOT NULL,
    deal_status integer NOT NULL,
    curr_amt numeric(18,4) NOT NULL,
    frozen_amt numeric(18,4) NOT NULL,
    unfrozen_amt numeric(18,4) NOT NULL,
    pre_settle_amt numeric(18,4) NOT NULL,
    after_amt numeric(18,4) NOT NULL,
    remark_info character varying(255) NOT NULL,
    source_row_id bigint NOT NULL
);


ALTER TABLE jzdb_secu.tb_seswap_asac_capit_adjust_jour OWNER TO postgres;

--
-- Name: tb_seswap_asac_capit_adjust_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seswap_asac_capit_adjust_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seswap_asac_capit_adjust_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seswap_asac_capit_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seswap_asac_capit ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seswap_asac_capit_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seswap_asac_posi; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seswap_asac_posi (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    swap_code_no character varying(64) NOT NULL,
    swap_code_name character varying(255) NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    lngsht_type integer NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    notional_amt numeric(18,4) NOT NULL,
    exchcrncytype_costprice numeric(16,4) NOT NULL,
    exchcrncytype_notional_amt numeric(18,4) NOT NULL,
    last_price numeric(16,4) NOT NULL,
    posi_market_value numeric(18,2) NOT NULL,
    exchcrncytype_marketvalue numeric(18,4) NOT NULL,
    open_rate numeric(18,12) NOT NULL,
    begin_date integer NOT NULL,
    end_date integer NOT NULL,
    fee_rate numeric(18,12) NOT NULL,
    futu_margin_ratio numeric(9,8) NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seswap_asac_posi OWNER TO postgres;

--
-- Name: tb_seswap_asac_posi_adjust_jour; Type: TABLE; Schema: jzdb_secu; Owner: postgres
--

CREATE TABLE jzdb_secu.tb_seswap_asac_posi_adjust_jour (
    row_id bigint NOT NULL,
    create_date integer NOT NULL,
    create_time integer NOT NULL,
    update_date integer NOT NULL,
    update_time integer NOT NULL,
    update_times integer NOT NULL,
    init_date integer NOT NULL,
    adjust_jour_no bigint NOT NULL,
    jour_flag integer NOT NULL,
    adjust_qty numeric(18,2) NOT NULL,
    adjust_amt numeric(18,4) NOT NULL,
    co_no integer NOT NULL,
    pd_no integer NOT NULL,
    asac_no integer NOT NULL,
    exch_no integer NOT NULL,
    secu_code character varying(32) NOT NULL,
    secu_name character varying(64) NOT NULL,
    swap_code_no character varying(64) NOT NULL,
    swap_code_name character varying(255) NOT NULL,
    secu_type integer NOT NULL,
    invest_type integer NOT NULL,
    asset_type integer NOT NULL,
    lngsht_type integer NOT NULL,
    begin_qty numeric(18,2) NOT NULL,
    curr_qty numeric(18,2) NOT NULL,
    cost_price numeric(16,4) NOT NULL,
    notional_amt numeric(18,4) NOT NULL,
    exchcrncytype_costprice numeric(16,4) CONSTRAINT tb_seswap_asac_posi_adjust_jou_exchcrncytype_costprice_not_null NOT NULL,
    exchcrncytype_notional_amt numeric(18,4) CONSTRAINT tb_seswap_asac_posi_adjust__exchcrncytype_notional_amt_not_null NOT NULL,
    last_price numeric(16,4) NOT NULL,
    posi_market_value numeric(18,2) NOT NULL,
    exchcrncytype_marketvalue numeric(18,4) CONSTRAINT tb_seswap_asac_posi_adjust_j_exchcrncytype_marketvalue_not_null NOT NULL,
    open_rate numeric(18,12) NOT NULL,
    begin_date integer NOT NULL,
    end_date integer NOT NULL,
    fee_rate numeric(18,12) NOT NULL,
    futu_margin_ratio numeric(9,8) NOT NULL,
    settle_crncy_type integer NOT NULL,
    exch_crncy_type integer NOT NULL,
    remark_info character varying(255) NOT NULL
);


ALTER TABLE jzdb_secu.tb_seswap_asac_posi_adjust_jour OWNER TO postgres;

--
-- Name: tb_seswap_asac_posi_adjust_jour_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seswap_asac_posi_adjust_jour ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seswap_asac_posi_adjust_jour_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seswap_asac_posi_row_id_seq; Type: SEQUENCE; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE jzdb_secu.tb_seswap_asac_posi ALTER COLUMN row_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME jzdb_secu.tb_seswap_asac_posi_row_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tb_seconv_fixorder tb_seconv_fixorder_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seconv_fixorder
    ADD CONSTRAINT tb_seconv_fixorder_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seconv_fixpush tb_seconv_fixpush_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seconv_fixpush
    ADD CONSTRAINT tb_seconv_fixpush_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_asset tb_semage_asac_asset_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_asset
    ADD CONSTRAINT tb_semage_asac_asset_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_capit_adjust_jour tb_semage_asac_capit_adjust_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_capit_adjust_jour
    ADD CONSTRAINT tb_semage_asac_capit_adjust_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_capit_diff tb_semage_asac_capit_diff_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_capit_diff
    ADD CONSTRAINT tb_semage_asac_capit_diff_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_capit tb_semage_asac_capit_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_capit
    ADD CONSTRAINT tb_semage_asac_capit_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_capit_trade tb_semage_asac_capit_trade_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_capit_trade
    ADD CONSTRAINT tb_semage_asac_capit_trade_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_capit_unsettle tb_semage_asac_capit_unsettle_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_capit_unsettle
    ADD CONSTRAINT tb_semage_asac_capit_unsettle_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_credit_asset tb_semage_asac_credit_asset_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_credit_asset
    ADD CONSTRAINT tb_semage_asac_credit_asset_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_fee_model tb_semage_asac_fee_model_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_fee_model
    ADD CONSTRAINT tb_semage_asac_fee_model_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_margin_contract tb_semage_asac_margin_contract_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_margin_contract
    ADD CONSTRAINT tb_semage_asac_margin_contract_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_posi_adjust_jour tb_semage_asac_posi_adjust_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_posi_adjust_jour
    ADD CONSTRAINT tb_semage_asac_posi_adjust_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_posi_diff tb_semage_asac_posi_diff_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_posi_diff
    ADD CONSTRAINT tb_semage_asac_posi_diff_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_posi tb_semage_asac_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_posi
    ADD CONSTRAINT tb_semage_asac_posi_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_posi_unsettle tb_semage_asac_posi_unsettle_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_posi_unsettle
    ADD CONSTRAINT tb_semage_asac_posi_unsettle_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_settle_capit tb_semage_asac_settle_capit_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_settle_capit
    ADD CONSTRAINT tb_semage_asac_settle_capit_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_settle_capit_unsettle tb_semage_asac_settle_capit_unsettle_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_settle_capit_unsettle
    ADD CONSTRAINT tb_semage_asac_settle_capit_unsettle_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_settle_posi tb_semage_asac_settle_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_settle_posi
    ADD CONSTRAINT tb_semage_asac_settle_posi_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_settle_posi_unsettle tb_semage_asac_settle_posi_unsettle_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_settle_posi_unsettle
    ADD CONSTRAINT tb_semage_asac_settle_posi_unsettle_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_trade_capit_settle tb_semage_asac_trade_capit_settle_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_trade_capit_settle
    ADD CONSTRAINT tb_semage_asac_trade_capit_settle_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_asac_trade_posi_settle tb_semage_asac_trade_posi_settle_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_asac_trade_posi_settle
    ADD CONSTRAINT tb_semage_asac_trade_posi_settle_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_co_custo_fieldvalue_map tb_semage_co_custo_fieldvalue_map_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_co_custo_fieldvalue_map
    ADD CONSTRAINT tb_semage_co_custo_fieldvalue_map_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_co_custo_file_format tb_semage_co_custo_file_format_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_co_custo_file_format
    ADD CONSTRAINT tb_semage_co_custo_file_format_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_co_custo_file tb_semage_co_custo_file_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_co_custo_file
    ADD CONSTRAINT tb_semage_co_custo_file_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_ctmorder tb_semage_ctmorder_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_ctmorder
    ADD CONSTRAINT tb_semage_ctmorder_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_ctmstrike_allocation tb_semage_ctmstrike_allocation_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_ctmstrike_allocation
    ADD CONSTRAINT tb_semage_ctmstrike_allocation_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_ctmstrike_block tb_semage_ctmstrike_block_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_ctmstrike_block
    ADD CONSTRAINT tb_semage_ctmstrike_block_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_exor_capit tb_semage_exor_capit_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_exor_capit
    ADD CONSTRAINT tb_semage_exor_capit_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_exor_fee_model tb_semage_exor_fee_model_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_exor_fee_model
    ADD CONSTRAINT tb_semage_exor_fee_model_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_exor_last_posi tb_semage_exor_last_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_exor_last_posi
    ADD CONSTRAINT tb_semage_exor_last_posi_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_exor_posi tb_semage_exor_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_exor_posi
    ADD CONSTRAINT tb_semage_exor_posi_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_fee_model_binding tb_semage_fee_model_binding_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_fee_model_binding
    ADD CONSTRAINT tb_semage_fee_model_binding_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_fee_model_detail tb_semage_fee_model_detail_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_fee_model_detail
    ADD CONSTRAINT tb_semage_fee_model_detail_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_fee_model tb_semage_fee_model_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_fee_model
    ADD CONSTRAINT tb_semage_fee_model_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_fee_section_config tb_semage_fee_section_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_fee_section_config
    ADD CONSTRAINT tb_semage_fee_section_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_loan_secu_pool tb_semage_loan_secu_pool_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_loan_secu_pool
    ADD CONSTRAINT tb_semage_loan_secu_pool_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_out_capit tb_semage_out_capit_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_out_capit
    ADD CONSTRAINT tb_semage_out_capit_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_out_capit_rsp tb_semage_out_capit_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_out_capit_rsp
    ADD CONSTRAINT tb_semage_out_capit_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_out_credit_asset tb_semage_out_credit_asset_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_out_credit_asset
    ADD CONSTRAINT tb_semage_out_credit_asset_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_out_object tb_semage_out_object_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_out_object
    ADD CONSTRAINT tb_semage_out_object_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_out_posi tb_semage_out_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_out_posi
    ADD CONSTRAINT tb_semage_out_posi_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_out_posi_rsp tb_semage_out_posi_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_out_posi_rsp
    ADD CONSTRAINT tb_semage_out_posi_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_posi_capital_jour tb_semage_posi_capital_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_posi_capital_jour
    ADD CONSTRAINT tb_semage_posi_capital_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_posi_part_file tb_semage_posi_part_file_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_posi_part_file
    ADD CONSTRAINT tb_semage_posi_part_file_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_posi_part_in tb_semage_posi_part_in_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_posi_part_in
    ADD CONSTRAINT tb_semage_posi_part_in_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_posi_part_jour tb_semage_posi_part_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_posi_part_jour
    ADD CONSTRAINT tb_semage_posi_part_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_posi_part_out tb_semage_posi_part_out_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_posi_part_out
    ADD CONSTRAINT tb_semage_posi_part_out_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_busictrl_item_disableconfig tb_semage_risk_busictrl_item_disableconfig_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_busictrl_item_disableconfig
    ADD CONSTRAINT tb_semage_risk_busictrl_item_disableconfig_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_busictrl_item_enableconfig tb_semage_risk_busictrl_item_enableconfig_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_busictrl_item_enableconfig
    ADD CONSTRAINT tb_semage_risk_busictrl_item_enableconfig_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_busictrl_item_secupoolconfig tb_semage_risk_busictrl_item_secupoolconfig_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_busictrl_item_secupoolconfig
    ADD CONSTRAINT tb_semage_risk_busictrl_item_secupoolconfig_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_code_item_config_custcode tb_semage_risk_code_item_config_custcode_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_code_item_config_custcode
    ADD CONSTRAINT tb_semage_risk_code_item_config_custcode_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_code_item_config tb_semage_risk_code_item_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_code_item_config
    ADD CONSTRAINT tb_semage_risk_code_item_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_cond_item_config tb_semage_risk_cond_item_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_cond_item_config
    ADD CONSTRAINT tb_semage_risk_cond_item_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_exposure_config tb_semage_risk_exposure_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_exposure_config
    ADD CONSTRAINT tb_semage_risk_exposure_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_exposure_config_review tb_semage_risk_exposure_config_review_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_exposure_config_review
    ADD CONSTRAINT tb_semage_risk_exposure_config_review_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_exposure tb_semage_risk_exposure_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_exposure
    ADD CONSTRAINT tb_semage_risk_exposure_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_exposure_value tb_semage_risk_exposure_value_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_exposure_value
    ADD CONSTRAINT tb_semage_risk_exposure_value_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_item_code_dim_config tb_semage_risk_item_code_dim_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_item_code_dim_config
    ADD CONSTRAINT tb_semage_risk_item_code_dim_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_item_code_dim_config_review tb_semage_risk_item_code_dim_config_review_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_item_code_dim_config_review
    ADD CONSTRAINT tb_semage_risk_item_code_dim_config_review_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_item_config_review tb_semage_risk_item_config_review_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_item_config_review
    ADD CONSTRAINT tb_semage_risk_item_config_review_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_item_one_config tb_semage_risk_item_one_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_item_one_config
    ADD CONSTRAINT tb_semage_risk_item_one_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_item_riskgroup_config tb_semage_risk_item_riskgroup_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_item_riskgroup_config
    ADD CONSTRAINT tb_semage_risk_item_riskgroup_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_item_union_config tb_semage_risk_item_union_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_item_union_config
    ADD CONSTRAINT tb_semage_risk_item_union_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_warning_jour tb_semage_risk_warning_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_warning_jour
    ADD CONSTRAINT tb_semage_risk_warning_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_risk_warning_jour_rsp tb_semage_risk_warning_jour_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_risk_warning_jour_rsp
    ADD CONSTRAINT tb_semage_risk_warning_jour_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_secu_code_model_fee tb_semage_secu_code_model_fee_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_secu_code_model_fee
    ADD CONSTRAINT tb_semage_secu_code_model_fee_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_secu_code_pool_detail tb_semage_secu_code_pool_detail_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_secu_code_pool_detail
    ADD CONSTRAINT tb_semage_secu_code_pool_detail_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_secu_code_pool_level tb_semage_secu_code_pool_level_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_secu_code_pool_level
    ADD CONSTRAINT tb_semage_secu_code_pool_level_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_secu_code_pool tb_semage_secu_code_pool_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_secu_code_pool
    ADD CONSTRAINT tb_semage_secu_code_pool_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_secu_fee_model tb_semage_secu_fee_model_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_secu_fee_model
    ADD CONSTRAINT tb_semage_secu_fee_model_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_secu_type_model_fee tb_semage_secu_type_model_fee_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_secu_type_model_fee
    ADD CONSTRAINT tb_semage_secu_type_model_fee_pkey PRIMARY KEY (row_id);


--
-- Name: tb_semage_static_risk_check_jour tb_semage_static_risk_check_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_semage_static_risk_check_jour
    ADD CONSTRAINT tb_semage_static_risk_check_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_asset_main_type tb_seoper_asset_main_type_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_asset_main_type
    ADD CONSTRAINT tb_seoper_asset_main_type_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_bond_info tb_seoper_bond_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_bond_info
    ADD CONSTRAINT tb_seoper_bond_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_busi_rec_no tb_seoper_busi_rec_no_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_busi_rec_no
    ADD CONSTRAINT tb_seoper_busi_rec_no_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_co_dep_info tb_seoper_co_dep_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_co_dep_info
    ADD CONSTRAINT tb_seoper_co_dep_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_co_exch_rate tb_seoper_co_exch_rate_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_co_exch_rate
    ADD CONSTRAINT tb_seoper_co_exch_rate_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_comp_action tb_seoper_comp_action_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_comp_action
    ADD CONSTRAINT tb_seoper_comp_action_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_countries_info tb_seoper_countries_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_countries_info
    ADD CONSTRAINT tb_seoper_countries_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_crncy_exchcode_config tb_seoper_crncy_exchcode_config_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_crncy_exchcode_config
    ADD CONSTRAINT tb_seoper_crncy_exchcode_config_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_ex_info tb_seoper_ex_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_ex_info
    ADD CONSTRAINT tb_seoper_ex_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_ex_time tb_seoper_ex_time_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_ex_time
    ADD CONSTRAINT tb_seoper_ex_time_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_fund_code_info tb_seoper_fund_code_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_fund_code_info
    ADD CONSTRAINT tb_seoper_fund_code_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_hk_exch_rate tb_seoper_hk_exch_rate_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_hk_exch_rate
    ADD CONSTRAINT tb_seoper_hk_exch_rate_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_hk_limit_info tb_seoper_hk_limit_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_hk_limit_info
    ADD CONSTRAINT tb_seoper_hk_limit_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_hk_settle_date tb_seoper_hk_settle_date_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_hk_settle_date
    ADD CONSTRAINT tb_seoper_hk_settle_date_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_issuer_secu_code tb_seoper_issuer_secu_code_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_issuer_secu_code
    ADD CONSTRAINT tb_seoper_issuer_secu_code_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_margin_offset_secu tb_seoper_margin_offset_secu_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_margin_offset_secu
    ADD CONSTRAINT tb_seoper_margin_offset_secu_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_margin_ratio_allocation tb_seoper_margin_ratio_allocation_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_margin_ratio_allocation
    ADD CONSTRAINT tb_seoper_margin_ratio_allocation_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_margin_underly tb_seoper_margin_underly_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_margin_underly
    ADD CONSTRAINT tb_seoper_margin_underly_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_new_secu_code_info tb_seoper_new_secu_code_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_new_secu_code_info
    ADD CONSTRAINT tb_seoper_new_secu_code_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_otcsecu_code_info tb_seoper_otcsecu_code_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_otcsecu_code_info
    ADD CONSTRAINT tb_seoper_otcsecu_code_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_risk_code_item_sysconfig_code tb_seoper_risk_code_item_sysconfig_code_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_risk_code_item_sysconfig_code
    ADD CONSTRAINT tb_seoper_risk_code_item_sysconfig_code_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_risk_code_item_sysconfig tb_seoper_risk_code_item_sysconfig_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_risk_code_item_sysconfig
    ADD CONSTRAINT tb_seoper_risk_code_item_sysconfig_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_code_busi_arg tb_seoper_secu_code_busi_arg_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_code_busi_arg
    ADD CONSTRAINT tb_seoper_secu_code_busi_arg_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_code_info tb_seoper_secu_code_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_code_info
    ADD CONSTRAINT tb_seoper_secu_code_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_code_map tb_seoper_secu_code_map_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_code_map
    ADD CONSTRAINT tb_seoper_secu_code_map_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_inner_code_map tb_seoper_secu_inner_code_map_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_inner_code_map
    ADD CONSTRAINT tb_seoper_secu_inner_code_map_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_quot_daily_msg tb_seoper_secu_quot_daily_msg_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_quot_daily_msg
    ADD CONSTRAINT tb_seoper_secu_quot_daily_msg_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_quot tb_seoper_secu_quot_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_quot
    ADD CONSTRAINT tb_seoper_secu_quot_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_repo_param tb_seoper_secu_repo_param_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_repo_param
    ADD CONSTRAINT tb_seoper_secu_repo_param_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_strike_quot tb_seoper_secu_strike_quot_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_strike_quot
    ADD CONSTRAINT tb_seoper_secu_strike_quot_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_tmplat tb_seoper_secu_tmplat_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_tmplat
    ADD CONSTRAINT tb_seoper_secu_tmplat_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_type_alert tb_seoper_secu_type_alert_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_type_alert
    ADD CONSTRAINT tb_seoper_secu_type_alert_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_type_busi_arg tb_seoper_secu_type_busi_arg_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_type_busi_arg
    ADD CONSTRAINT tb_seoper_secu_type_busi_arg_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_type_ctm tb_seoper_secu_type_ctm_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_type_ctm
    ADD CONSTRAINT tb_seoper_secu_type_ctm_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_type_out tb_seoper_secu_type_out_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_type_out
    ADD CONSTRAINT tb_seoper_secu_type_out_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_type tb_seoper_secu_type_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_type
    ADD CONSTRAINT tb_seoper_secu_type_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_type_stepprice_info tb_seoper_secu_type_stepprice_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_type_stepprice_info
    ADD CONSTRAINT tb_seoper_secu_type_stepprice_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_secu_type_time tb_seoper_secu_type_time_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_secu_type_time
    ADD CONSTRAINT tb_seoper_secu_type_time_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_swap_code_info tb_seoper_swap_code_info_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_swap_code_info
    ADD CONSTRAINT tb_seoper_swap_code_info_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_sys_secu_code_fee tb_seoper_sys_secu_code_fee_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_sys_secu_code_fee
    ADD CONSTRAINT tb_seoper_sys_secu_code_fee_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seoper_sys_secu_type_fee tb_seoper_sys_secu_type_fee_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seoper_sys_secu_type_fee
    ADD CONSTRAINT tb_seoper_sys_secu_type_fee_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seotcsecu_asac_capit_adjust_jour tb_seotcsecu_asac_capit_adjust_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour
    ADD CONSTRAINT tb_seotcsecu_asac_capit_adjust_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seotcsecu_asac_capit tb_seotcsecu_asac_capit_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seotcsecu_asac_capit
    ADD CONSTRAINT tb_seotcsecu_asac_capit_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seotcsecu_asac_posi_adjust_jour tb_seotcsecu_asac_posi_adjust_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour
    ADD CONSTRAINT tb_seotcsecu_asac_posi_adjust_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seotcsecu_asac_posi tb_seotcsecu_asac_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seotcsecu_asac_posi
    ADD CONSTRAINT tb_seotcsecu_asac_posi_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_asac_capit tb_sestra_asac_capit_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_asac_capit
    ADD CONSTRAINT tb_sestra_asac_capit_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_asac_capit_rsp tb_sestra_asac_capit_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_asac_capit_rsp
    ADD CONSTRAINT tb_sestra_asac_capit_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_asac_capit_trade tb_sestra_asac_capit_trade_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_asac_capit_trade
    ADD CONSTRAINT tb_sestra_asac_capit_trade_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_asac_capit_trade_rsp tb_sestra_asac_capit_trade_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_asac_capit_trade_rsp
    ADD CONSTRAINT tb_sestra_asac_capit_trade_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_asac_posi tb_sestra_asac_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_asac_posi
    ADD CONSTRAINT tb_sestra_asac_posi_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_asac_posi_rsp tb_sestra_asac_posi_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_asac_posi_rsp
    ADD CONSTRAINT tb_sestra_asac_posi_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_asac_posi_trade tb_sestra_asac_posi_trade_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_asac_posi_trade
    ADD CONSTRAINT tb_sestra_asac_posi_trade_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_asac_posi_trade_rsp tb_sestra_asac_posi_trade_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_asac_posi_trade_rsp
    ADD CONSTRAINT tb_sestra_asac_posi_trade_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_bondrepo tb_sestra_bondrepo_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_bondrepo
    ADD CONSTRAINT tb_sestra_bondrepo_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_command_copy1 tb_sestra_command_copy1_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_command_copy1
    ADD CONSTRAINT tb_sestra_command_copy1_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_command_copy2 tb_sestra_command_copy2_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_command_copy2
    ADD CONSTRAINT tb_sestra_command_copy2_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_command tb_sestra_command_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_command
    ADD CONSTRAINT tb_sestra_command_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_command_rsp tb_sestra_command_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_command_rsp
    ADD CONSTRAINT tb_sestra_command_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_expordermodify_jour tb_sestra_expordermodify_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_expordermodify_jour
    ADD CONSTRAINT tb_sestra_expordermodify_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_expordermodify_jour_rsp tb_sestra_expordermodify_jour_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_expordermodify_jour_rsp
    ADD CONSTRAINT tb_sestra_expordermodify_jour_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_instructapprove tb_sestra_instructapprove_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_instructapprove
    ADD CONSTRAINT tb_sestra_instructapprove_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_instructapprove_rsp tb_sestra_instructapprove_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_instructapprove_rsp
    ADD CONSTRAINT tb_sestra_instructapprove_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_instructjour tb_sestra_instructjour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_instructjour
    ADD CONSTRAINT tb_sestra_instructjour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_instructjour_rsp tb_sestra_instructjour_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_instructjour_rsp
    ADD CONSTRAINT tb_sestra_instructjour_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_multidaycommand tb_sestra_multidaycommand_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_multidaycommand
    ADD CONSTRAINT tb_sestra_multidaycommand_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_order tb_sestra_order_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_order
    ADD CONSTRAINT tb_sestra_order_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_order_rsp tb_sestra_order_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_order_rsp
    ADD CONSTRAINT tb_sestra_order_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_ordersum tb_sestra_ordersum_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_ordersum
    ADD CONSTRAINT tb_sestra_ordersum_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_ordersum_rsp tb_sestra_ordersum_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_ordersum_rsp
    ADD CONSTRAINT tb_sestra_ordersum_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_pd_unit_capit tb_sestra_pd_unit_capit_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_pd_unit_capit
    ADD CONSTRAINT tb_sestra_pd_unit_capit_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_pd_unit_capit_rsp tb_sestra_pd_unit_capit_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_pd_unit_capit_rsp
    ADD CONSTRAINT tb_sestra_pd_unit_capit_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_pd_unit_capit_trade tb_sestra_pd_unit_capit_trade_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_pd_unit_capit_trade
    ADD CONSTRAINT tb_sestra_pd_unit_capit_trade_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_pd_unit_capit_trade_rsp tb_sestra_pd_unit_capit_trade_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_pd_unit_capit_trade_rsp
    ADD CONSTRAINT tb_sestra_pd_unit_capit_trade_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_pd_unit_posi tb_sestra_pd_unit_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_pd_unit_posi
    ADD CONSTRAINT tb_sestra_pd_unit_posi_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_pd_unit_posi_rsp tb_sestra_pd_unit_posi_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_pd_unit_posi_rsp
    ADD CONSTRAINT tb_sestra_pd_unit_posi_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_pd_unit_posi_trade tb_sestra_pd_unit_posi_trade_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_pd_unit_posi_trade
    ADD CONSTRAINT tb_sestra_pd_unit_posi_trade_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_pd_unit_posi_trade_rsp tb_sestra_pd_unit_posi_trade_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_pd_unit_posi_trade_rsp
    ADD CONSTRAINT tb_sestra_pd_unit_posi_trade_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_strike tb_sestra_strike_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_strike
    ADD CONSTRAINT tb_sestra_strike_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_strike_rsp tb_sestra_strike_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_strike_rsp
    ADD CONSTRAINT tb_sestra_strike_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_sumcommand tb_sestra_sumcommand_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_sumcommand
    ADD CONSTRAINT tb_sestra_sumcommand_pkey PRIMARY KEY (row_id);


--
-- Name: tb_sestra_sumcommand_rsp tb_sestra_sumcommand_rsp_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_sestra_sumcommand_rsp
    ADD CONSTRAINT tb_sestra_sumcommand_rsp_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seswap_asac_capit_adjust_jour tb_seswap_asac_capit_adjust_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seswap_asac_capit_adjust_jour
    ADD CONSTRAINT tb_seswap_asac_capit_adjust_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seswap_asac_capit tb_seswap_asac_capit_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seswap_asac_capit
    ADD CONSTRAINT tb_seswap_asac_capit_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seswap_asac_posi_adjust_jour tb_seswap_asac_posi_adjust_jour_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seswap_asac_posi_adjust_jour
    ADD CONSTRAINT tb_seswap_asac_posi_adjust_jour_pkey PRIMARY KEY (row_id);


--
-- Name: tb_seswap_asac_posi tb_seswap_asac_posi_pkey; Type: CONSTRAINT; Schema: jzdb_secu; Owner: postgres
--

ALTER TABLE ONLY jzdb_secu.tb_seswap_asac_posi
    ADD CONSTRAINT tb_seswap_asac_posi_pkey PRIMARY KEY (row_id);


--
-- Name: idx_tb_seconv_fixorder_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seconv_fixorder_1 ON jzdb_secu.tb_seconv_fixorder USING btree (occur_date, co_no, t2if_fund_account, t2if_entrust_reference);


--
-- Name: idx_tb_seconv_fixorder_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seconv_fixorder_2 ON jzdb_secu.tb_seconv_fixorder USING btree (occur_date, co_no, "FIX1_Account", "FIX11_ClOrdID");


--
-- Name: idx_tb_seconv_fixorder_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seconv_fixorder_3 ON jzdb_secu.tb_seconv_fixorder USING btree (occur_date, co_no, client_acc_code, client_order_id);


--
-- Name: idx_tb_seconv_fixpush_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seconv_fixpush_1 ON jzdb_secu.tb_seconv_fixpush USING btree (occur_date, co_no, jour_no);


--
-- Name: idx_tb_seconv_fixpush_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seconv_fixpush_2 ON jzdb_secu.tb_seconv_fixpush USING btree (occur_date, co_no, "FIX1_Account", "FIX11_ClOrdID");


--
-- Name: idx_tb_seconv_fixpush_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seconv_fixpush_3 ON jzdb_secu.tb_seconv_fixpush USING btree (occur_date, co_no, t2if_fund_account, t2if_entrust_reference);


--
-- Name: idx_tb_seconv_fixpush_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seconv_fixpush_4 ON jzdb_secu.tb_seconv_fixpush USING btree (occur_date, co_no, client_acc_code, client_order_id);


--
-- Name: idx_tb_semage_asac_asset_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_asset_1 ON jzdb_secu.tb_semage_asac_asset USING btree (co_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_semage_asac_asset_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_asset_2 ON jzdb_secu.tb_semage_asac_asset USING btree (co_no);


--
-- Name: idx_tb_semage_asac_asset_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_asset_3 ON jzdb_secu.tb_semage_asac_asset USING btree (co_no, pd_no);


--
-- Name: idx_tb_semage_asac_capit_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_capit_1 ON jzdb_secu.tb_semage_asac_capit USING btree (pd_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_semage_asac_capit_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_2 ON jzdb_secu.tb_semage_asac_capit USING btree (asac_no);


--
-- Name: idx_tb_semage_asac_capit_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_3 ON jzdb_secu.tb_semage_asac_capit USING btree (co_no);


--
-- Name: idx_tb_semage_asac_capit_adjust_jour_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_adjust_jour_5 ON jzdb_secu.tb_semage_asac_capit_adjust_jour USING btree (init_date);


--
-- Name: idx_tb_semage_asac_capit_diff_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_capit_diff_1 ON jzdb_secu.tb_semage_asac_capit_diff USING btree (init_date, co_no, pd_no, asac_no, settle_crncy_type, custo_id);


--
-- Name: idx_tb_semage_asac_capit_trade_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_capit_trade_1 ON jzdb_secu.tb_semage_asac_capit_trade USING btree (pd_no, asac_no, exch_crncy_type);


--
-- Name: idx_tb_semage_asac_capit_trade_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_trade_2 ON jzdb_secu.tb_semage_asac_capit_trade USING btree (pd_no);


--
-- Name: idx_tb_semage_asac_capit_trade_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_trade_3 ON jzdb_secu.tb_semage_asac_capit_trade USING btree (asac_no);


--
-- Name: idx_tb_semage_asac_capit_trade_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_trade_4 ON jzdb_secu.tb_semage_asac_capit_trade USING btree (co_no);


--
-- Name: idx_tb_semage_asac_capit_unsettle_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_capit_unsettle_1 ON jzdb_secu.tb_semage_asac_capit_unsettle USING btree (pd_no, asac_no, settle_crncy_type, exch_crncy_type, order_date, settle_date, busi_type);


--
-- Name: idx_tb_semage_asac_capit_unsettle_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_unsettle_2 ON jzdb_secu.tb_semage_asac_capit_unsettle USING btree (asac_no);


--
-- Name: idx_tb_semage_asac_capit_unsettle_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_unsettle_3 ON jzdb_secu.tb_semage_asac_capit_unsettle USING btree (co_no);


--
-- Name: idx_tb_semage_asac_capit_unsettle_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_unsettle_4 ON jzdb_secu.tb_semage_asac_capit_unsettle USING btree (settle_date);


--
-- Name: idx_tb_semage_asac_capit_unsettle_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_capit_unsettle_5 ON jzdb_secu.tb_semage_asac_capit_unsettle USING btree (busi_type);


--
-- Name: idx_tb_semage_asac_credit_asset_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_credit_asset_1 ON jzdb_secu.tb_semage_asac_credit_asset USING btree (co_no, pd_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_semage_asac_credit_asset_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_credit_asset_3 ON jzdb_secu.tb_semage_asac_credit_asset USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_asac_fee_model_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_fee_model_1 ON jzdb_secu.tb_semage_asac_fee_model USING btree (co_no, asac_no, out_acco_id, fee_model_type, fee_model_kind);


--
-- Name: idx_tb_semage_asac_fee_model_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_fee_model_2 ON jzdb_secu.tb_semage_asac_fee_model USING btree (co_no, asac_no, out_acco_id, model_id);


--
-- Name: idx_tb_semage_asac_fee_model_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_fee_model_3 ON jzdb_secu.tb_semage_asac_fee_model USING btree (model_id);


--
-- Name: idx_tb_semage_asac_margin_contract_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_margin_contract_1 ON jzdb_secu.tb_semage_asac_margin_contract USING btree (serial_no, co_no, pd_no, asac_no);


--
-- Name: idx_tb_semage_asac_margin_contract_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_margin_contract_3 ON jzdb_secu.tb_semage_asac_margin_contract USING btree (pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_margin_contract_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_margin_contract_4 ON jzdb_secu.tb_semage_asac_margin_contract USING btree (asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_margin_contract_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_margin_contract_5 ON jzdb_secu.tb_semage_asac_margin_contract USING btree (co_no);


--
-- Name: idx_tb_semage_asac_margin_contract_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_margin_contract_6 ON jzdb_secu.tb_semage_asac_margin_contract USING btree (init_date);


--
-- Name: idx_tb_semage_asac_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_posi_1 ON jzdb_secu.tb_semage_asac_posi USING btree (co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_posi_2 ON jzdb_secu.tb_semage_asac_posi USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_asac_posi_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_posi_3 ON jzdb_secu.tb_semage_asac_posi USING btree (co_no, pd_no);


--
-- Name: idx_tb_semage_asac_posi_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_posi_4 ON jzdb_secu.tb_semage_asac_posi USING btree (exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_posi_adjust_jour_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_posi_adjust_jour_5 ON jzdb_secu.tb_semage_asac_posi_adjust_jour USING btree (init_date);


--
-- Name: idx_tb_semage_asac_posi_diff_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_posi_diff_1 ON jzdb_secu.tb_semage_asac_posi_diff USING btree (init_date, co_no, pd_no, asac_no, exch_no, secu_code, custo_id);


--
-- Name: idx_tb_semage_asac_posi_unsettle_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_posi_unsettle_1 ON jzdb_secu.tb_semage_asac_posi_unsettle USING btree (co_no, pd_no, asac_no, exch_no, secu_code, order_date, settle_date);


--
-- Name: idx_tb_semage_asac_posi_unsettle_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_posi_unsettle_2 ON jzdb_secu.tb_semage_asac_posi_unsettle USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_asac_posi_unsettle_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_posi_unsettle_3 ON jzdb_secu.tb_semage_asac_posi_unsettle USING btree (co_no, pd_no);


--
-- Name: idx_tb_semage_asac_posi_unsettle_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_posi_unsettle_4 ON jzdb_secu.tb_semage_asac_posi_unsettle USING btree (exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_posi_unsettle_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_posi_unsettle_5 ON jzdb_secu.tb_semage_asac_posi_unsettle USING btree (settle_date);


--
-- Name: idx_tb_semage_asac_settle_capit_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_settle_capit_1 ON jzdb_secu.tb_semage_asac_settle_capit USING btree (pd_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_semage_asac_settle_capit_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_capit_2 ON jzdb_secu.tb_semage_asac_settle_capit USING btree (asac_no);


--
-- Name: idx_tb_semage_asac_settle_capit_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_capit_3 ON jzdb_secu.tb_semage_asac_settle_capit USING btree (co_no);


--
-- Name: idx_tb_semage_asac_settle_capit_unsettle_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_settle_capit_unsettle_1 ON jzdb_secu.tb_semage_asac_settle_capit_unsettle USING btree (pd_no, asac_no, settle_crncy_type, exch_crncy_type, order_date, settle_date, busi_type);


--
-- Name: idx_tb_semage_asac_settle_capit_unsettle_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_capit_unsettle_2 ON jzdb_secu.tb_semage_asac_settle_capit_unsettle USING btree (asac_no);


--
-- Name: idx_tb_semage_asac_settle_capit_unsettle_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_capit_unsettle_3 ON jzdb_secu.tb_semage_asac_settle_capit_unsettle USING btree (co_no);


--
-- Name: idx_tb_semage_asac_settle_capit_unsettle_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_capit_unsettle_4 ON jzdb_secu.tb_semage_asac_settle_capit_unsettle USING btree (settle_date);


--
-- Name: idx_tb_semage_asac_settle_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_settle_posi_1 ON jzdb_secu.tb_semage_asac_settle_posi USING btree (co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_settle_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_posi_2 ON jzdb_secu.tb_semage_asac_settle_posi USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_asac_settle_posi_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_posi_3 ON jzdb_secu.tb_semage_asac_settle_posi USING btree (co_no, pd_no);


--
-- Name: idx_tb_semage_asac_settle_posi_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_posi_4 ON jzdb_secu.tb_semage_asac_settle_posi USING btree (exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_settle_posi_unsettle_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_settle_posi_unsettle_1 ON jzdb_secu.tb_semage_asac_settle_posi_unsettle USING btree (co_no, pd_no, asac_no, exch_no, secu_code, order_date, settle_date);


--
-- Name: idx_tb_semage_asac_settle_posi_unsettle_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_posi_unsettle_2 ON jzdb_secu.tb_semage_asac_settle_posi_unsettle USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_asac_settle_posi_unsettle_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_posi_unsettle_3 ON jzdb_secu.tb_semage_asac_settle_posi_unsettle USING btree (co_no, pd_no);


--
-- Name: idx_tb_semage_asac_settle_posi_unsettle_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_posi_unsettle_4 ON jzdb_secu.tb_semage_asac_settle_posi_unsettle USING btree (exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_settle_posi_unsettle_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_settle_posi_unsettle_5 ON jzdb_secu.tb_semage_asac_settle_posi_unsettle USING btree (settle_date);


--
-- Name: idx_tb_semage_asac_trade_capit_settle_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_trade_capit_settle_1 ON jzdb_secu.tb_semage_asac_trade_capit_settle USING btree (pd_no, asac_no, settle_crncy_type, exch_crncy_type);


--
-- Name: idx_tb_semage_asac_trade_capit_settle_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_trade_capit_settle_2 ON jzdb_secu.tb_semage_asac_trade_capit_settle USING btree (asac_no);


--
-- Name: idx_tb_semage_asac_trade_capit_settle_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_trade_capit_settle_3 ON jzdb_secu.tb_semage_asac_trade_capit_settle USING btree (co_no);


--
-- Name: idx_tb_semage_asac_trade_posi_settle_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_asac_trade_posi_settle_1 ON jzdb_secu.tb_semage_asac_trade_posi_settle USING btree (co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_asac_trade_posi_settle_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_trade_posi_settle_2 ON jzdb_secu.tb_semage_asac_trade_posi_settle USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_asac_trade_posi_settle_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_trade_posi_settle_3 ON jzdb_secu.tb_semage_asac_trade_posi_settle USING btree (co_no, pd_no);


--
-- Name: idx_tb_semage_asac_trade_posi_settle_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_asac_trade_posi_settle_4 ON jzdb_secu.tb_semage_asac_trade_posi_settle USING btree (exch_no, secu_code);


--
-- Name: idx_tb_semage_co_custo_fieldvalue_map_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_co_custo_fieldvalue_map_1 ON jzdb_secu.tb_semage_co_custo_fieldvalue_map USING btree (co_no, custo_id, custo_field, table_field);


--
-- Name: idx_tb_semage_co_custo_file_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_co_custo_file_1 ON jzdb_secu.tb_semage_co_custo_file USING btree (co_no, custo_check_item, custo_id);


--
-- Name: idx_tb_semage_co_custo_file_format_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_co_custo_file_format_1 ON jzdb_secu.tb_semage_co_custo_file_format USING btree (co_no, custo_check_item, custo_id, custo_field);


--
-- Name: idx_tb_semage_ctmorder_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_ctmorder_1 ON jzdb_secu.tb_semage_ctmorder USING btree (init_date, external_no);


--
-- Name: idx_tb_semage_ctmorder_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmorder_2 ON jzdb_secu.tb_semage_ctmorder USING btree (init_date, co_no, pd_no, asac_no, pd_unit_no, out_acco_id, channel_no, external_no, exch_no, secu_code, order_dir, dma);


--
-- Name: idx_tb_semage_ctmorder_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmorder_3 ON jzdb_secu.tb_semage_ctmorder USING btree (init_date, co_no, pd_no, out_acco_id, channel_no, exch_no, secu_code, order_dir, dma);


--
-- Name: idx_tb_semage_ctmorder_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmorder_4 ON jzdb_secu.tb_semage_ctmorder USING btree (init_date, co_no, pd_no, out_acco_id, channel_no, exch_no, secu_code, order_dir);


--
-- Name: idx_tb_semage_ctmorder_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmorder_5 ON jzdb_secu.tb_semage_ctmorder USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_semage_ctmorder_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmorder_6 ON jzdb_secu.tb_semage_ctmorder USING btree (init_date, co_no);


--
-- Name: idx_tb_semage_ctmstrike_allocation_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_ctmstrike_allocation_1 ON jzdb_secu.tb_semage_ctmstrike_allocation USING btree (init_date, co_no, pd_no, asac_no, pd_unit_no, out_acco_id, channel_no, external_no, exch_no, secu_code, order_dir, dma);


--
-- Name: idx_tb_semage_ctmstrike_allocation_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmstrike_allocation_2 ON jzdb_secu.tb_semage_ctmstrike_allocation USING btree (init_date, co_no, pd_no, asac_no, out_acco_id, channel_no, exch_no, secu_code, order_dir, dma);


--
-- Name: idx_tb_semage_ctmstrike_allocation_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmstrike_allocation_3 ON jzdb_secu.tb_semage_ctmstrike_allocation USING btree (init_date, co_no, pd_no, asac_no, out_acco_id, channel_no, exch_no, secu_code, order_dir);


--
-- Name: idx_tb_semage_ctmstrike_allocation_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmstrike_allocation_4 ON jzdb_secu.tb_semage_ctmstrike_allocation USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_semage_ctmstrike_allocation_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmstrike_allocation_5 ON jzdb_secu.tb_semage_ctmstrike_allocation USING btree (init_date, co_no);


--
-- Name: idx_tb_semage_ctmstrike_block_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_ctmstrike_block_1 ON jzdb_secu.tb_semage_ctmstrike_block USING btree (init_date, co_no, pd_no, asac_no, pd_unit_no, out_acco_id, channel_no, external_no, exch_no, secu_code, order_dir, dma);


--
-- Name: idx_tb_semage_ctmstrike_block_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmstrike_block_2 ON jzdb_secu.tb_semage_ctmstrike_block USING btree (init_date, co_no, pd_no, out_acco_id, channel_no, exch_no, secu_code, order_dir, dma);


--
-- Name: idx_tb_semage_ctmstrike_block_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmstrike_block_3 ON jzdb_secu.tb_semage_ctmstrike_block USING btree (init_date, co_no, pd_no, out_acco_id, channel_no, exch_no, secu_code, order_dir);


--
-- Name: idx_tb_semage_ctmstrike_block_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmstrike_block_4 ON jzdb_secu.tb_semage_ctmstrike_block USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_semage_ctmstrike_block_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_ctmstrike_block_5 ON jzdb_secu.tb_semage_ctmstrike_block USING btree (init_date, co_no);


--
-- Name: idx_tb_semage_exor_capit_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_exor_capit_1 ON jzdb_secu.tb_semage_exor_capit USING btree (pd_no, pd_unit_no, asac_no, exor_no, settle_crncy_type);


--
-- Name: idx_tb_semage_exor_capit_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_capit_2 ON jzdb_secu.tb_semage_exor_capit USING btree (pd_no, settle_crncy_type);


--
-- Name: idx_tb_semage_exor_capit_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_capit_3 ON jzdb_secu.tb_semage_exor_capit USING btree (co_no);


--
-- Name: idx_tb_semage_exor_fee_model_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_exor_fee_model_1 ON jzdb_secu.tb_semage_exor_fee_model USING btree (co_no, exor_no, fee_model_type, fee_model_kind);


--
-- Name: idx_tb_semage_exor_fee_model_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_fee_model_2 ON jzdb_secu.tb_semage_exor_fee_model USING btree (co_no, exor_no, model_id);


--
-- Name: idx_tb_semage_exor_fee_model_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_fee_model_3 ON jzdb_secu.tb_semage_exor_fee_model USING btree (model_id);


--
-- Name: idx_tb_semage_exor_last_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_exor_last_posi_1 ON jzdb_secu.tb_semage_exor_last_posi USING btree (co_no, exor_no, asac_no, secu_source_type, exch_no, secu_code);


--
-- Name: idx_tb_semage_exor_last_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_last_posi_2 ON jzdb_secu.tb_semage_exor_last_posi USING btree (asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_exor_last_posi_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_last_posi_3 ON jzdb_secu.tb_semage_exor_last_posi USING btree (exor_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_exor_last_posi_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_last_posi_4 ON jzdb_secu.tb_semage_exor_last_posi USING btree (co_no);


--
-- Name: idx_tb_semage_exor_last_posi_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_last_posi_5 ON jzdb_secu.tb_semage_exor_last_posi USING btree (trade_type);


--
-- Name: idx_tb_semage_exor_last_posi_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_last_posi_6 ON jzdb_secu.tb_semage_exor_last_posi USING btree (deal_flag);


--
-- Name: idx_tb_semage_exor_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_exor_posi_1 ON jzdb_secu.tb_semage_exor_posi USING btree (pd_no, pd_unit_no, asac_no, exor_no, exch_no, secu_code, secu_source_type);


--
-- Name: idx_tb_semage_exor_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_posi_2 ON jzdb_secu.tb_semage_exor_posi USING btree (pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_exor_posi_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_posi_3 ON jzdb_secu.tb_semage_exor_posi USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_exor_posi_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_exor_posi_4 ON jzdb_secu.tb_semage_exor_posi USING btree (co_no);


--
-- Name: idx_tb_semage_fee_model_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_fee_model_1 ON jzdb_secu.tb_semage_fee_model USING btree (co_no, fee_model_id);


--
-- Name: idx_tb_semage_fee_model_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_fee_model_2 ON jzdb_secu.tb_semage_fee_model USING btree (co_no, fee_model_code);


--
-- Name: idx_tb_semage_fee_model_binding_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_fee_model_binding_1 ON jzdb_secu.tb_semage_fee_model_binding USING btree (co_no, pd_no);


--
-- Name: idx_tb_semage_fee_model_binding_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_fee_model_binding_2 ON jzdb_secu.tb_semage_fee_model_binding USING btree (co_no, fee_model_id);


--
-- Name: idx_tb_semage_fee_model_detail_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_fee_model_detail_1 ON jzdb_secu.tb_semage_fee_model_detail USING btree (co_no, fee_model_detail_id);


--
-- Name: idx_tb_semage_fee_model_detail_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_fee_model_detail_2 ON jzdb_secu.tb_semage_fee_model_detail USING btree (co_no, fee_model_id);


--
-- Name: idx_tb_semage_fee_section_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_fee_section_config_1 ON jzdb_secu.tb_semage_fee_section_config USING btree (co_no, fee_section_config_id);


--
-- Name: idx_tb_semage_fee_section_config_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_fee_section_config_2 ON jzdb_secu.tb_semage_fee_section_config USING btree (co_no, fee_model_detail_id);


--
-- Name: idx_tb_semage_loan_secu_pool_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_loan_secu_pool_1 ON jzdb_secu.tb_semage_loan_secu_pool USING btree (channel_no, co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_out_capit_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_out_capit_1 ON jzdb_secu.tb_semage_out_capit USING btree (co_no, pd_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_semage_out_capit_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_capit_2 ON jzdb_secu.tb_semage_out_capit USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_out_capit_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_capit_rsp_1 ON jzdb_secu.tb_semage_out_capit_rsp USING btree (init_date);


--
-- Name: idx_tb_semage_out_credit_asset_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_credit_asset_1 ON jzdb_secu.tb_semage_out_credit_asset USING btree (co_no, pd_no, asac_no);


--
-- Name: idx_tb_semage_out_credit_asset_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_credit_asset_2 ON jzdb_secu.tb_semage_out_credit_asset USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_out_credit_asset_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_credit_asset_3 ON jzdb_secu.tb_semage_out_credit_asset USING btree (co_no, pd_no);


--
-- Name: idx_tb_semage_out_object_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_out_object_1 ON jzdb_secu.tb_semage_out_object USING btree (co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_out_object_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_object_2 ON jzdb_secu.tb_semage_out_object USING btree (co_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_out_object_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_object_3 ON jzdb_secu.tb_semage_out_object USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_out_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_out_posi_1 ON jzdb_secu.tb_semage_out_posi USING btree (co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_out_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_posi_2 ON jzdb_secu.tb_semage_out_posi USING btree (co_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_out_posi_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_posi_3 ON jzdb_secu.tb_semage_out_posi USING btree (co_no, asac_no);


--
-- Name: idx_tb_semage_out_posi_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_out_posi_rsp_1 ON jzdb_secu.tb_semage_out_posi_rsp USING btree (init_date);


--
-- Name: idx_tb_semage_posi_capital_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_posi_capital_jour_1 ON jzdb_secu.tb_semage_posi_capital_jour USING btree (init_date, posi_capit_jour_no);


--
-- Name: idx_tb_semage_posi_capital_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_capital_jour_2 ON jzdb_secu.tb_semage_posi_capital_jour USING btree (co_no, pd_no, pd_unit_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_posi_part_file_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_posi_part_file_1 ON jzdb_secu.tb_semage_posi_part_file USING btree (init_date, co_no, posi_part_batch_no, file_name);


--
-- Name: idx_tb_semage_posi_part_file_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_file_2 ON jzdb_secu.tb_semage_posi_part_file USING btree (init_date, co_no, posi_part_batch_no);


--
-- Name: idx_tb_semage_posi_part_file_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_file_3 ON jzdb_secu.tb_semage_posi_part_file USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_semage_posi_part_file_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_file_4 ON jzdb_secu.tb_semage_posi_part_file USING btree (init_date, co_no, pd_code);


--
-- Name: idx_tb_semage_posi_part_in_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_posi_part_in_1 ON jzdb_secu.tb_semage_posi_part_in USING btree (init_date, co_no, posi_part_batch_no, posi_part_no);


--
-- Name: idx_tb_semage_posi_part_in_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_in_2 ON jzdb_secu.tb_semage_posi_part_in USING btree (init_date, co_no, posi_part_batch_no);


--
-- Name: idx_tb_semage_posi_part_in_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_in_3 ON jzdb_secu.tb_semage_posi_part_in USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_semage_posi_part_in_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_in_4 ON jzdb_secu.tb_semage_posi_part_in USING btree (init_date, co_no, pd_code);


--
-- Name: idx_tb_semage_posi_part_in_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_in_5 ON jzdb_secu.tb_semage_posi_part_in USING btree (init_date, co_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_posi_part_in_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_in_6 ON jzdb_secu.tb_semage_posi_part_in USING btree (init_date, co_no, out_acco, asac_no);


--
-- Name: idx_tb_semage_posi_part_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_posi_part_jour_1 ON jzdb_secu.tb_semage_posi_part_jour USING btree (init_date, jour_no);


--
-- Name: idx_tb_semage_posi_part_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_jour_2 ON jzdb_secu.tb_semage_posi_part_jour USING btree (init_date, co_no, posi_part_batch_no);


--
-- Name: idx_tb_semage_posi_part_jour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_jour_3 ON jzdb_secu.tb_semage_posi_part_jour USING btree (init_date, co_no, user_no);


--
-- Name: idx_tb_semage_posi_part_out_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_posi_part_out_1 ON jzdb_secu.tb_semage_posi_part_out USING btree (init_date, co_no, posi_part_batch_no);


--
-- Name: idx_tb_semage_posi_part_out_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_out_2 ON jzdb_secu.tb_semage_posi_part_out USING btree (init_date, co_no, pd_code);


--
-- Name: idx_tb_semage_posi_part_out_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_out_3 ON jzdb_secu.tb_semage_posi_part_out USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_semage_posi_part_out_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_out_4 ON jzdb_secu.tb_semage_posi_part_out USING btree (init_date, co_no, initiator_no);


--
-- Name: idx_tb_semage_posi_part_out_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_out_5 ON jzdb_secu.tb_semage_posi_part_out USING btree (init_date, co_no, appr_user_no);


--
-- Name: idx_tb_semage_posi_part_out_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_out_6 ON jzdb_secu.tb_semage_posi_part_out USING btree (init_date, co_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_posi_part_out_7; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_out_7 ON jzdb_secu.tb_semage_posi_part_out USING btree (init_date, co_no, posi_part_status);


--
-- Name: idx_tb_semage_posi_part_out_8; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_posi_part_out_8 ON jzdb_secu.tb_semage_posi_part_out USING btree (init_date, co_no, out_acco, asac_no);


--
-- Name: idx_tb_semage_risk_busictrl_item_disableconfig_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_busictrl_item_disableconfig_1 ON jzdb_secu.tb_semage_risk_busictrl_item_disableconfig USING btree (co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_busictrl_item_disableconfig_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_busictrl_item_disableconfig_2 ON jzdb_secu.tb_semage_risk_busictrl_item_disableconfig USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_busictrl_item_disableconfig_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_busictrl_item_disableconfig_3 ON jzdb_secu.tb_semage_risk_busictrl_item_disableconfig USING btree (co_no, risk_item_code);


--
-- Name: idx_tb_semage_risk_busictrl_item_enableconfig_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_busictrl_item_enableconfig_1 ON jzdb_secu.tb_semage_risk_busictrl_item_enableconfig USING btree (co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_busictrl_item_enableconfig_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_busictrl_item_enableconfig_2 ON jzdb_secu.tb_semage_risk_busictrl_item_enableconfig USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_busictrl_item_enableconfig_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_busictrl_item_enableconfig_3 ON jzdb_secu.tb_semage_risk_busictrl_item_enableconfig USING btree (co_no, risk_item_code);


--
-- Name: idx_tb_semage_risk_busictrl_item_secupoolconfig_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_busictrl_item_secupoolconfig_1 ON jzdb_secu.tb_semage_risk_busictrl_item_secupoolconfig USING btree (co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_busictrl_item_secupoolconfig_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_busictrl_item_secupoolconfig_2 ON jzdb_secu.tb_semage_risk_busictrl_item_secupoolconfig USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_busictrl_item_secupoolconfig_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_busictrl_item_secupoolconfig_3 ON jzdb_secu.tb_semage_risk_busictrl_item_secupoolconfig USING btree (co_no, risk_item_code);


--
-- Name: idx_tb_semage_risk_code_item_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_code_item_config_1 ON jzdb_secu.tb_semage_risk_code_item_config USING btree (co_no, risk_code_item_no);


--
-- Name: idx_tb_semage_risk_code_item_config_custcode_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_code_item_config_custcode_1 ON jzdb_secu.tb_semage_risk_code_item_config_custcode USING btree (co_no, risk_code_item_no, risk_code_item_config_type, exch_no, secu_code);


--
-- Name: idx_tb_semage_risk_cond_item_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_cond_item_config_1 ON jzdb_secu.tb_semage_risk_cond_item_config USING btree (co_no, risk_cond_item_config_no);


--
-- Name: idx_tb_semage_risk_cond_item_config_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_cond_item_config_2 ON jzdb_secu.tb_semage_risk_cond_item_config USING btree (co_no, risk_cond_item_no);


--
-- Name: idx_tb_semage_risk_cond_item_config_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_cond_item_config_3 ON jzdb_secu.tb_semage_risk_cond_item_config USING btree (co_no, risk_cond_item_code);


--
-- Name: idx_tb_semage_risk_exposure_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_exposure_1 ON jzdb_secu.tb_semage_risk_exposure USING btree (co_no, exposure_no);


--
-- Name: idx_tb_semage_risk_exposure_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_exposure_2 ON jzdb_secu.tb_semage_risk_exposure USING btree (co_no, exposure_level);


--
-- Name: idx_tb_semage_risk_exposure_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_exposure_3 ON jzdb_secu.tb_semage_risk_exposure USING btree (co_no, exposure_type);


--
-- Name: idx_tb_semage_risk_exposure_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_exposure_config_1 ON jzdb_secu.tb_semage_risk_exposure_config USING btree (co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_exposure_config_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_exposure_config_2 ON jzdb_secu.tb_semage_risk_exposure_config USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_exposure_config_review_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_exposure_config_review_1 ON jzdb_secu.tb_semage_risk_exposure_config_review USING btree (co_no, risk_review_jour_no);


--
-- Name: idx_tb_semage_risk_exposure_config_review_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_exposure_config_review_2 ON jzdb_secu.tb_semage_risk_exposure_config_review USING btree (co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_exposure_config_review_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_exposure_config_review_3 ON jzdb_secu.tb_semage_risk_exposure_config_review USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_exposure_config_review_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_exposure_config_review_4 ON jzdb_secu.tb_semage_risk_exposure_config_review USING btree (review_status);


--
-- Name: idx_tb_semage_risk_exposure_config_review_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_exposure_config_review_5 ON jzdb_secu.tb_semage_risk_exposure_config_review USING btree (valid_flag);


--
-- Name: idx_tb_semage_risk_exposure_value_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_exposure_value_1 ON jzdb_secu.tb_semage_risk_exposure_value USING btree (co_no, exposure_no, exposure_level_value);


--
-- Name: idx_tb_semage_risk_exposure_value_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_exposure_value_2 ON jzdb_secu.tb_semage_risk_exposure_value USING btree (co_no, exposure_level_value);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_item_code_dim_config_1 ON jzdb_secu.tb_semage_risk_item_code_dim_config USING btree (co_no, risk_item_config_no, risk_code_item_config_type);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_code_dim_config_2 ON jzdb_secu.tb_semage_risk_item_code_dim_config USING btree (co_no, risk_item_config_batch_no);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_code_dim_config_3 ON jzdb_secu.tb_semage_risk_item_code_dim_config USING btree (co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_code_dim_config_4 ON jzdb_secu.tb_semage_risk_item_code_dim_config USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_code_dim_config_5 ON jzdb_secu.tb_semage_risk_item_code_dim_config USING btree (co_no, risk_code_item_config_type);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_review_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_item_code_dim_config_review_1 ON jzdb_secu.tb_semage_risk_item_code_dim_config_review USING btree (co_no, risk_review_jour_no, risk_code_item_config_type);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_review_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_code_dim_config_review_2 ON jzdb_secu.tb_semage_risk_item_code_dim_config_review USING btree (co_no, risk_review_jour_no);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_review_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_code_dim_config_review_3 ON jzdb_secu.tb_semage_risk_item_code_dim_config_review USING btree (review_status);


--
-- Name: idx_tb_semage_risk_item_code_dim_config_review_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_code_dim_config_review_4 ON jzdb_secu.tb_semage_risk_item_code_dim_config_review USING btree (valid_flag);


--
-- Name: idx_tb_semage_risk_item_config_review_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_item_config_review_1 ON jzdb_secu.tb_semage_risk_item_config_review USING btree (co_no, risk_review_jour_no);


--
-- Name: idx_tb_semage_risk_item_config_review_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_config_review_2 ON jzdb_secu.tb_semage_risk_item_config_review USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_item_config_review_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_config_review_3 ON jzdb_secu.tb_semage_risk_item_config_review USING btree (co_no, risk_item_code);


--
-- Name: idx_tb_semage_risk_item_config_review_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_config_review_4 ON jzdb_secu.tb_semage_risk_item_config_review USING btree (co_no, reviewed_date, review_status);


--
-- Name: idx_tb_semage_risk_item_config_review_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_config_review_5 ON jzdb_secu.tb_semage_risk_item_config_review USING btree (co_no, reviewed_date, review_status, risk_item_no);


--
-- Name: idx_tb_semage_risk_item_config_review_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_config_review_6 ON jzdb_secu.tb_semage_risk_item_config_review USING btree (review_status);


--
-- Name: idx_tb_semage_risk_item_config_review_7; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_config_review_7 ON jzdb_secu.tb_semage_risk_item_config_review USING btree (valid_flag);


--
-- Name: idx_tb_semage_risk_item_one_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_item_one_config_1 ON jzdb_secu.tb_semage_risk_item_one_config USING btree (co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_item_one_config_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_one_config_2 ON jzdb_secu.tb_semage_risk_item_one_config USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_item_one_config_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_one_config_3 ON jzdb_secu.tb_semage_risk_item_one_config USING btree (co_no, risk_item_code);


--
-- Name: idx_tb_semage_risk_item_one_config_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_one_config_4 ON jzdb_secu.tb_semage_risk_item_one_config USING btree (co_no, risk_item_config_batch_no);


--
-- Name: idx_tb_semage_risk_item_riskgroup_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_item_riskgroup_config_1 ON jzdb_secu.tb_semage_risk_item_riskgroup_config USING btree (co_no, risk_item_config_no, risk_item_kind, workgroup_id);


--
-- Name: idx_tb_semage_risk_item_riskgroup_config_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_riskgroup_config_2 ON jzdb_secu.tb_semage_risk_item_riskgroup_config USING btree (co_no, workgroup_id);


--
-- Name: idx_tb_semage_risk_item_riskgroup_config_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_riskgroup_config_3 ON jzdb_secu.tb_semage_risk_item_riskgroup_config USING btree (co_no);


--
-- Name: idx_tb_semage_risk_item_union_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_item_union_config_1 ON jzdb_secu.tb_semage_risk_item_union_config USING btree (co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_item_union_config_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_union_config_2 ON jzdb_secu.tb_semage_risk_item_union_config USING btree (co_no, risk_item_no);


--
-- Name: idx_tb_semage_risk_item_union_config_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_union_config_3 ON jzdb_secu.tb_semage_risk_item_union_config USING btree (co_no, risk_item_code);


--
-- Name: idx_tb_semage_risk_item_union_config_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_item_union_config_4 ON jzdb_secu.tb_semage_risk_item_union_config USING btree (co_no, risk_item_config_batch_no);


--
-- Name: idx_tb_semage_risk_warning_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_risk_warning_jour_1 ON jzdb_secu.tb_semage_risk_warning_jour USING btree (init_date, serial_no);


--
-- Name: idx_tb_semage_risk_warning_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_warning_jour_2 ON jzdb_secu.tb_semage_risk_warning_jour USING btree (init_date, co_no);


--
-- Name: idx_tb_semage_risk_warning_jour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_warning_jour_3 ON jzdb_secu.tb_semage_risk_warning_jour USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, exor_no);


--
-- Name: idx_tb_semage_risk_warning_jour_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_warning_jour_4 ON jzdb_secu.tb_semage_risk_warning_jour USING btree (init_date, co_no, risk_item_exec_mode, risk_item_config_no);


--
-- Name: idx_tb_semage_risk_warning_jour_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_warning_jour_5 ON jzdb_secu.tb_semage_risk_warning_jour USING btree (init_date, co_no, appr_user_no, appr_status);


--
-- Name: idx_tb_semage_risk_warning_jour_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_warning_jour_6 ON jzdb_secu.tb_semage_risk_warning_jour USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_semage_risk_warning_jour_7; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_warning_jour_7 ON jzdb_secu.tb_semage_risk_warning_jour USING btree (init_date, co_no, risk_source);


--
-- Name: idx_tb_semage_risk_warning_jour_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_risk_warning_jour_rsp_1 ON jzdb_secu.tb_semage_risk_warning_jour_rsp USING btree (init_date, serial_no);


--
-- Name: idx_tb_semage_secu_code_model_fee_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_secu_code_model_fee_1 ON jzdb_secu.tb_semage_secu_code_model_fee USING btree (model_id, exch_no, secu_code, secu_fee_type, order_dir);


--
-- Name: idx_tb_semage_secu_code_model_fee_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_model_fee_2 ON jzdb_secu.tb_semage_secu_code_model_fee USING btree (co_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_secu_code_pool_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_secu_code_pool_1 ON jzdb_secu.tb_semage_secu_code_pool USING btree (co_no, secu_code_pool_no);


--
-- Name: idx_tb_semage_secu_code_pool_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_2 ON jzdb_secu.tb_semage_secu_code_pool USING btree (co_no, secu_code_pool_type);


--
-- Name: idx_tb_semage_secu_code_pool_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_3 ON jzdb_secu.tb_semage_secu_code_pool USING btree (co_no, secu_code_pool_dim_no);


--
-- Name: idx_tb_semage_secu_code_pool_detail_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_secu_code_pool_detail_1 ON jzdb_secu.tb_semage_secu_code_pool_detail USING btree (co_no, secu_code_pool_no, exch_no, secu_code);


--
-- Name: idx_tb_semage_secu_code_pool_detail_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_detail_2 ON jzdb_secu.tb_semage_secu_code_pool_detail USING btree (co_no, secu_code_pool_no);


--
-- Name: idx_tb_semage_secu_code_pool_detail_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_detail_3 ON jzdb_secu.tb_semage_secu_code_pool_detail USING btree (exch_no, secu_code);


--
-- Name: idx_tb_semage_secu_code_pool_detail_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_detail_4 ON jzdb_secu.tb_semage_secu_code_pool_detail USING btree (secu_code_pool_type);


--
-- Name: idx_tb_semage_secu_code_pool_detail_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_detail_5 ON jzdb_secu.tb_semage_secu_code_pool_detail USING btree (secu_code_pool_dim_no);


--
-- Name: idx_tb_semage_secu_code_pool_level_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_secu_code_pool_level_1 ON jzdb_secu.tb_semage_secu_code_pool_level USING btree (co_no, secu_code_pool_dim_no);


--
-- Name: idx_tb_semage_secu_code_pool_level_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_level_2 ON jzdb_secu.tb_semage_secu_code_pool_level USING btree (co_no, secu_code_pool_dim_level);


--
-- Name: idx_tb_semage_secu_code_pool_level_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_level_3 ON jzdb_secu.tb_semage_secu_code_pool_level USING btree (co_no, secu_code_pool_dim_type);


--
-- Name: idx_tb_semage_secu_code_pool_level_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_code_pool_level_4 ON jzdb_secu.tb_semage_secu_code_pool_level USING btree (co_no);


--
-- Name: idx_tb_semage_secu_fee_model_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_secu_fee_model_1 ON jzdb_secu.tb_semage_secu_fee_model USING btree (model_id);


--
-- Name: idx_tb_semage_secu_fee_model_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_fee_model_2 ON jzdb_secu.tb_semage_secu_fee_model USING btree (co_no, fee_model_type, fee_model_kind);


--
-- Name: idx_tb_semage_secu_fee_model_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_fee_model_3 ON jzdb_secu.tb_semage_secu_fee_model USING btree (co_no, model_id);


--
-- Name: idx_tb_semage_secu_type_model_fee_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_secu_type_model_fee_1 ON jzdb_secu.tb_semage_secu_type_model_fee USING btree (model_id, exch_no, exch_sub_type, secu_type, secu_fee_type, order_dir);


--
-- Name: idx_tb_semage_secu_type_model_fee_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_secu_type_model_fee_2 ON jzdb_secu.tb_semage_secu_type_model_fee USING btree (co_no, exch_no, exch_sub_type, secu_type);


--
-- Name: idx_tb_semage_static_risk_check_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_semage_static_risk_check_jour_1 ON jzdb_secu.tb_semage_static_risk_check_jour USING btree (init_date, serial_no);


--
-- Name: idx_tb_semage_static_risk_check_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_static_risk_check_jour_2 ON jzdb_secu.tb_semage_static_risk_check_jour USING btree (init_date, co_no);


--
-- Name: idx_tb_semage_static_risk_check_jour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_static_risk_check_jour_3 ON jzdb_secu.tb_semage_static_risk_check_jour USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no);


--
-- Name: idx_tb_semage_static_risk_check_jour_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_static_risk_check_jour_4 ON jzdb_secu.tb_semage_static_risk_check_jour USING btree (init_date, co_no, risk_item_config_no);


--
-- Name: idx_tb_semage_static_risk_check_jour_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_semage_static_risk_check_jour_5 ON jzdb_secu.tb_semage_static_risk_check_jour USING btree (init_date, co_no, compli_status);


--
-- Name: idx_tb_seoper_asset_main_type_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_asset_main_type_1 ON jzdb_secu.tb_seoper_asset_main_type USING btree (asset_main_type);


--
-- Name: idx_tb_seoper_bond_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_bond_info_1 ON jzdb_secu.tb_seoper_bond_info USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_bond_info_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_bond_info_2 ON jzdb_secu.tb_seoper_bond_info USING btree (exch_no, trade_code);


--
-- Name: idx_tb_seoper_bond_info_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_bond_info_3 ON jzdb_secu.tb_seoper_bond_info USING btree (time_stamp);


--
-- Name: idx_tb_seoper_busi_rec_no_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_busi_rec_no_1 ON jzdb_secu.tb_seoper_busi_rec_no USING btree (co_no, record_no_type);


--
-- Name: idx_tb_seoper_co_dep_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_co_dep_info_1 ON jzdb_secu.tb_seoper_co_dep_info USING btree (co_no, exch_no);


--
-- Name: idx_tb_seoper_co_exch_rate_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_co_exch_rate_1 ON jzdb_secu.tb_seoper_co_exch_rate USING btree (co_no, for_crncy_type, crncy_type);


--
-- Name: idx_tb_seoper_comp_action_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_comp_action_1 ON jzdb_secu.tb_seoper_comp_action USING btree (init_date, exch_no, secu_code);


--
-- Name: idx_tb_seoper_comp_action_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_comp_action_2 ON jzdb_secu.tb_seoper_comp_action USING btree (entry_date, exdividend_date, begin_trade_date);


--
-- Name: idx_tb_seoper_countries_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_countries_info_1 ON jzdb_secu.tb_seoper_countries_info USING btree (country_name);


--
-- Name: idx_tb_seoper_countries_info_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_countries_info_2 ON jzdb_secu.tb_seoper_countries_info USING btree (country_code_two);


--
-- Name: idx_tb_seoper_countries_info_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_countries_info_3 ON jzdb_secu.tb_seoper_countries_info USING btree (country_code_three);


--
-- Name: idx_tb_seoper_countries_info_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_countries_info_4 ON jzdb_secu.tb_seoper_countries_info USING btree (country_code_no);


--
-- Name: idx_tb_seoper_crncy_exchcode_config_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_crncy_exchcode_config_1 ON jzdb_secu.tb_seoper_crncy_exchcode_config USING btree (out_sys_no, crncy_exch_code);


--
-- Name: idx_tb_seoper_ex_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_ex_info_1 ON jzdb_secu.tb_seoper_ex_info USING btree (exch_no, exch_sub_type);


--
-- Name: idx_tb_seoper_ex_info_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_ex_info_2 ON jzdb_secu.tb_seoper_ex_info USING btree (ex_init_date);


--
-- Name: idx_tb_seoper_ex_time_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_ex_time_1 ON jzdb_secu.tb_seoper_ex_time USING btree (exch_no, exch_sub_type, trd_time_frame);


--
-- Name: idx_tb_seoper_fund_code_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_fund_code_info_1 ON jzdb_secu.tb_seoper_fund_code_info USING btree (exch_no, secu_code, co_no);


--
-- Name: idx_tb_seoper_fund_code_info_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_fund_code_info_2 ON jzdb_secu.tb_seoper_fund_code_info USING btree (co_no, exch_no);


--
-- Name: idx_tb_seoper_fund_code_info_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_fund_code_info_3 ON jzdb_secu.tb_seoper_fund_code_info USING btree (secu_name);


--
-- Name: idx_tb_seoper_fund_code_info_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_fund_code_info_4 ON jzdb_secu.tb_seoper_fund_code_info USING btree (pinyin_short);


--
-- Name: idx_tb_seoper_hk_exch_rate_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_hk_exch_rate_1 ON jzdb_secu.tb_seoper_hk_exch_rate USING btree (init_date, exch_no);


--
-- Name: idx_tb_seoper_hk_limit_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_hk_limit_info_1 ON jzdb_secu.tb_seoper_hk_limit_info USING btree (exch_no);


--
-- Name: idx_tb_seoper_hk_settle_date_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_hk_settle_date_1 ON jzdb_secu.tb_seoper_hk_settle_date USING btree (settle_date, set_type);


--
-- Name: idx_tb_seoper_issuer_secu_code_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_issuer_secu_code_1 ON jzdb_secu.tb_seoper_issuer_secu_code USING btree (exch_no, secu_code, contrs_exch_no);


--
-- Name: idx_tb_seoper_issuer_secu_code_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_issuer_secu_code_2 ON jzdb_secu.tb_seoper_issuer_secu_code USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_margin_offset_secu_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_margin_offset_secu_1 ON jzdb_secu.tb_seoper_margin_offset_secu USING btree (channel_no, exch_no, secu_code);


--
-- Name: idx_tb_seoper_margin_ratio_allocation_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_margin_ratio_allocation_1 ON jzdb_secu.tb_seoper_margin_ratio_allocation USING btree (channel_no, co_no, secu_type, exch_no, secu_code);


--
-- Name: idx_tb_seoper_margin_underly_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_margin_underly_1 ON jzdb_secu.tb_seoper_margin_underly USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_new_secu_code_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_new_secu_code_info_1 ON jzdb_secu.tb_seoper_new_secu_code_info USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_new_secu_code_info_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_new_secu_code_info_2 ON jzdb_secu.tb_seoper_new_secu_code_info USING btree (apply_date);


--
-- Name: idx_tb_seoper_otcsecu_code_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_otcsecu_code_info_1 ON jzdb_secu.tb_seoper_otcsecu_code_info USING btree (exch_no, secu_code, co_no);


--
-- Name: idx_tb_seoper_otcsecu_code_info_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_otcsecu_code_info_2 ON jzdb_secu.tb_seoper_otcsecu_code_info USING btree (co_no, exch_no);


--
-- Name: idx_tb_seoper_risk_code_item_sysconfig_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_risk_code_item_sysconfig_1 ON jzdb_secu.tb_seoper_risk_code_item_sysconfig USING btree (risk_code_item_no);


--
-- Name: idx_tb_seoper_risk_code_item_sysconfig_code_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_risk_code_item_sysconfig_code_1 ON jzdb_secu.tb_seoper_risk_code_item_sysconfig_code USING btree (risk_code_item_no, risk_code_item_config_type, exch_no, secu_code);


--
-- Name: idx_tb_seoper_secu_code_busi_arg_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_code_busi_arg_1 ON jzdb_secu.tb_seoper_secu_code_busi_arg USING btree (exch_no, secu_code, order_dir);


--
-- Name: idx_tb_seoper_secu_code_busi_arg_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_code_busi_arg_2 ON jzdb_secu.tb_seoper_secu_code_busi_arg USING btree (time_stamp);


--
-- Name: idx_tb_seoper_secu_code_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_code_info_1 ON jzdb_secu.tb_seoper_secu_code_info USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_secu_code_info_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_code_info_2 ON jzdb_secu.tb_seoper_secu_code_info USING btree (bloomberg);


--
-- Name: idx_tb_seoper_secu_code_info_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_code_info_3 ON jzdb_secu.tb_seoper_secu_code_info USING btree (time_stamp);


--
-- Name: idx_tb_seoper_secu_code_map_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_code_map_1 ON jzdb_secu.tb_seoper_secu_code_map USING btree (exch_no, trade_code);


--
-- Name: idx_tb_seoper_secu_inner_code_map_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_inner_code_map_1 ON jzdb_secu.tb_seoper_secu_inner_code_map USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_secu_inner_code_map_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_inner_code_map_2 ON jzdb_secu.tb_seoper_secu_inner_code_map USING btree (secu_innder_code);


--
-- Name: idx_tb_seoper_secu_quot_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_quot_1 ON jzdb_secu.tb_seoper_secu_quot USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_secu_quot_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_quot_2 ON jzdb_secu.tb_seoper_secu_quot USING btree (time_stamp);


--
-- Name: idx_tb_seoper_secu_quot_daily_msg_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_quot_daily_msg_1 ON jzdb_secu.tb_seoper_secu_quot_daily_msg USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_secu_repo_param_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_repo_param_1 ON jzdb_secu.tb_seoper_secu_repo_param USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_secu_strike_quot_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_strike_quot_1 ON jzdb_secu.tb_seoper_secu_strike_quot USING btree (exch_no, secu_code);


--
-- Name: idx_tb_seoper_secu_strike_quot_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_strike_quot_2 ON jzdb_secu.tb_seoper_secu_strike_quot USING btree (time_stamp);


--
-- Name: idx_tb_seoper_secu_tmplat_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_tmplat_1 ON jzdb_secu.tb_seoper_secu_tmplat USING btree (exch_no, secu_code_feature, secu_name_feature);


--
-- Name: idx_tb_seoper_secu_type_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_type_1 ON jzdb_secu.tb_seoper_secu_type USING btree (exch_no, exch_sub_type, secu_type);


--
-- Name: idx_tb_seoper_secu_type_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_2 ON jzdb_secu.tb_seoper_secu_type USING btree (asset_type);


--
-- Name: idx_tb_seoper_secu_type_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_3 ON jzdb_secu.tb_seoper_secu_type USING btree (time_stamp);


--
-- Name: idx_tb_seoper_secu_type_alert_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_type_alert_1 ON jzdb_secu.tb_seoper_secu_type_alert USING btree (exch_no, exch_sub_type, secu_type);


--
-- Name: idx_tb_seoper_secu_type_alert_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_alert_2 ON jzdb_secu.tb_seoper_secu_type_alert USING btree (asset_type);


--
-- Name: idx_tb_seoper_secu_type_alert_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_alert_3 ON jzdb_secu.tb_seoper_secu_type_alert USING btree (secu_type_out);


--
-- Name: idx_tb_seoper_secu_type_busi_arg_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_type_busi_arg_1 ON jzdb_secu.tb_seoper_secu_type_busi_arg USING btree (exch_no, exch_sub_type, secu_type, order_dir);


--
-- Name: idx_tb_seoper_secu_type_busi_arg_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_busi_arg_2 ON jzdb_secu.tb_seoper_secu_type_busi_arg USING btree (time_stamp);


--
-- Name: idx_tb_seoper_secu_type_ctm_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_type_ctm_1 ON jzdb_secu.tb_seoper_secu_type_ctm USING btree (exch_no, exch_sub_type, secu_type);


--
-- Name: idx_tb_seoper_secu_type_ctm_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_ctm_2 ON jzdb_secu.tb_seoper_secu_type_ctm USING btree (asset_type);


--
-- Name: idx_tb_seoper_secu_type_ctm_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_ctm_3 ON jzdb_secu.tb_seoper_secu_type_ctm USING btree (secu_type_out);


--
-- Name: idx_tb_seoper_secu_type_out_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_type_out_1 ON jzdb_secu.tb_seoper_secu_type_out USING btree (exch_no, exch_sub_type, secu_type);


--
-- Name: idx_tb_seoper_secu_type_out_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_out_2 ON jzdb_secu.tb_seoper_secu_type_out USING btree (asset_type);


--
-- Name: idx_tb_seoper_secu_type_out_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_secu_type_out_3 ON jzdb_secu.tb_seoper_secu_type_out USING btree (secu_type_out);


--
-- Name: idx_tb_seoper_secu_type_stepprice_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_type_stepprice_info_1 ON jzdb_secu.tb_seoper_secu_type_stepprice_info USING btree (exch_no, exch_sub_type, secu_type, price_up, price_down);


--
-- Name: idx_tb_seoper_secu_type_time_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_secu_type_time_1 ON jzdb_secu.tb_seoper_secu_type_time USING btree (exch_no, exch_sub_type, secu_type);


--
-- Name: idx_tb_seoper_swap_code_info_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_swap_code_info_1 ON jzdb_secu.tb_seoper_swap_code_info USING btree (exch_no, futu_code, co_no);


--
-- Name: idx_tb_seoper_swap_code_info_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_swap_code_info_2 ON jzdb_secu.tb_seoper_swap_code_info USING btree (co_no, exch_no);


--
-- Name: idx_tb_seoper_swap_code_info_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seoper_swap_code_info_3 ON jzdb_secu.tb_seoper_swap_code_info USING btree (co_no, exch_no, secu_code);


--
-- Name: idx_tb_seoper_sys_secu_code_fee_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_sys_secu_code_fee_1 ON jzdb_secu.tb_seoper_sys_secu_code_fee USING btree (exch_no, secu_code, secu_fee_type, order_dir);


--
-- Name: idx_tb_seoper_sys_secu_type_fee_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seoper_sys_secu_type_fee_1 ON jzdb_secu.tb_seoper_sys_secu_type_fee USING btree (exch_no, exch_sub_type, secu_type, secu_fee_type, order_dir);


--
-- Name: idx_tb_seotcsecu_asac_capit_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seotcsecu_asac_capit_1 ON jzdb_secu.tb_seotcsecu_asac_capit USING btree (pd_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_seotcsecu_asac_capit_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_capit_2 ON jzdb_secu.tb_seotcsecu_asac_capit USING btree (asac_no);


--
-- Name: idx_tb_seotcsecu_asac_capit_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_capit_3 ON jzdb_secu.tb_seotcsecu_asac_capit USING btree (co_no);


--
-- Name: idx_tb_seotcsecu_asac_capit_adjust_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_capit_adjust_jour_1 ON jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour USING btree (init_date, co_no, pd_no, asac_no, settle_crncy_type, adjust_jour_no);


--
-- Name: idx_tb_seotcsecu_asac_capit_adjust_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_capit_adjust_jour_2 ON jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour USING btree (co_no, pd_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_seotcsecu_asac_capit_adjust_jour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_capit_adjust_jour_3 ON jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour USING btree (co_no, asac_no);


--
-- Name: idx_tb_seotcsecu_asac_capit_adjust_jour_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_capit_adjust_jour_4 ON jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour USING btree (co_no);


--
-- Name: idx_tb_seotcsecu_asac_capit_adjust_jour_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_capit_adjust_jour_5 ON jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour USING btree (init_date);


--
-- Name: idx_tb_seotcsecu_asac_capit_adjust_jour_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_capit_adjust_jour_6 ON jzdb_secu.tb_seotcsecu_asac_capit_adjust_jour USING btree (source_row_id);


--
-- Name: idx_tb_seotcsecu_asac_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seotcsecu_asac_posi_1 ON jzdb_secu.tb_seotcsecu_asac_posi USING btree (co_no, asac_no, exch_no, secu_code, pd_no);


--
-- Name: idx_tb_seotcsecu_asac_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_posi_2 ON jzdb_secu.tb_seotcsecu_asac_posi USING btree (co_no, pd_no);


--
-- Name: idx_tb_seotcsecu_asac_posi_adjust_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_posi_adjust_jour_1 ON jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour USING btree (init_date, adjust_jour_no, jour_flag, co_no);


--
-- Name: idx_tb_seotcsecu_asac_posi_adjust_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_posi_adjust_jour_2 ON jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour USING btree (co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_seotcsecu_asac_posi_adjust_jour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_posi_adjust_jour_3 ON jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour USING btree (co_no, asac_no);


--
-- Name: idx_tb_seotcsecu_asac_posi_adjust_jour_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seotcsecu_asac_posi_adjust_jour_4 ON jzdb_secu.tb_seotcsecu_asac_posi_adjust_jour USING btree (exch_no, secu_code);


--
-- Name: idx_tb_sestra_asac_capit_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_asac_capit_1 ON jzdb_secu.tb_sestra_asac_capit USING btree (init_date, co_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_sestra_asac_capit_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_2 ON jzdb_secu.tb_sestra_asac_capit USING btree (init_date, co_no, asac_no);


--
-- Name: idx_tb_sestra_asac_capit_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_3 ON jzdb_secu.tb_sestra_asac_capit USING btree (init_date, co_no);


--
-- Name: idx_tb_sestra_asac_capit_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_rsp_1 ON jzdb_secu.tb_sestra_asac_capit_rsp USING btree (init_date, co_no, asac_no, settle_crncy_type, last_update_times);


--
-- Name: idx_tb_sestra_asac_capit_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_rsp_2 ON jzdb_secu.tb_sestra_asac_capit_rsp USING btree (init_date, co_no, asac_no);


--
-- Name: idx_tb_sestra_asac_capit_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_rsp_3 ON jzdb_secu.tb_sestra_asac_capit_rsp USING btree (init_date, co_no);


--
-- Name: idx_tb_sestra_asac_capit_trade_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_asac_capit_trade_1 ON jzdb_secu.tb_sestra_asac_capit_trade USING btree (init_date, co_no, pd_no, asac_no, settle_crncy_type, exch_crncy_type);


--
-- Name: idx_tb_sestra_asac_capit_trade_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_trade_2 ON jzdb_secu.tb_sestra_asac_capit_trade USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_asac_capit_trade_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_trade_3 ON jzdb_secu.tb_sestra_asac_capit_trade USING btree (init_date, co_no, asac_no);


--
-- Name: idx_tb_sestra_asac_capit_trade_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_trade_rsp_1 ON jzdb_secu.tb_sestra_asac_capit_trade_rsp USING btree (init_date, co_no, pd_no, asac_no, settle_crncy_type, exch_crncy_type, last_update_times);


--
-- Name: idx_tb_sestra_asac_capit_trade_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_trade_rsp_2 ON jzdb_secu.tb_sestra_asac_capit_trade_rsp USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_asac_capit_trade_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_capit_trade_rsp_3 ON jzdb_secu.tb_sestra_asac_capit_trade_rsp USING btree (init_date, co_no, asac_no);


--
-- Name: idx_tb_sestra_asac_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_asac_posi_1 ON jzdb_secu.tb_sestra_asac_posi USING btree (init_date, co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_sestra_asac_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_2 ON jzdb_secu.tb_sestra_asac_posi USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_asac_posi_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_3 ON jzdb_secu.tb_sestra_asac_posi USING btree (init_date, co_no, asac_no);


--
-- Name: idx_tb_sestra_asac_posi_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_rsp_1 ON jzdb_secu.tb_sestra_asac_posi_rsp USING btree (init_date, co_no, pd_no, asac_no, exch_no, secu_code, last_update_times);


--
-- Name: idx_tb_sestra_asac_posi_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_rsp_2 ON jzdb_secu.tb_sestra_asac_posi_rsp USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_asac_posi_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_rsp_3 ON jzdb_secu.tb_sestra_asac_posi_rsp USING btree (init_date, co_no, asac_no);


--
-- Name: idx_tb_sestra_asac_posi_trade_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_asac_posi_trade_1 ON jzdb_secu.tb_sestra_asac_posi_trade USING btree (init_date, co_no, pd_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_sestra_asac_posi_trade_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_trade_2 ON jzdb_secu.tb_sestra_asac_posi_trade USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_asac_posi_trade_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_trade_3 ON jzdb_secu.tb_sestra_asac_posi_trade USING btree (init_date, co_no, asac_no);


--
-- Name: idx_tb_sestra_asac_posi_trade_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_trade_rsp_1 ON jzdb_secu.tb_sestra_asac_posi_trade_rsp USING btree (init_date, co_no, pd_no, asac_no, exch_no, secu_code, last_update_times);


--
-- Name: idx_tb_sestra_asac_posi_trade_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_trade_rsp_2 ON jzdb_secu.tb_sestra_asac_posi_trade_rsp USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_asac_posi_trade_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_asac_posi_trade_rsp_3 ON jzdb_secu.tb_sestra_asac_posi_trade_rsp USING btree (init_date, co_no, asac_no);


--
-- Name: idx_tb_sestra_bondrepo_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_bondrepo_1 ON jzdb_secu.tb_sestra_bondrepo USING btree (init_date, co_no, external_no);


--
-- Name: idx_tb_sestra_bondrepo_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_bondrepo_2 ON jzdb_secu.tb_sestra_bondrepo USING btree (init_date, co_no, asac_no, out_order_id);


--
-- Name: idx_tb_sestra_bondrepo_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_bondrepo_3 ON jzdb_secu.tb_sestra_bondrepo USING btree (repo_back_date);


--
-- Name: idx_tb_sestra_bondrepo_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_bondrepo_4 ON jzdb_secu.tb_sestra_bondrepo USING btree (co_no, pd_no, asac_no, settle_crncy_type, repo_back_date);


--
-- Name: idx_tb_sestra_bondrepo_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_bondrepo_5 ON jzdb_secu.tb_sestra_bondrepo USING btree (co_no, pd_no, pd_unit_no, asac_no, settle_crncy_type, repo_back_date);


--
-- Name: idx_tb_sestra_command_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_command_1 ON jzdb_secu.tb_sestra_command USING btree (init_date, instr_no);


--
-- Name: idx_tb_sestra_command_10; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_10 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, instr_no, valid_flag);


--
-- Name: idx_tb_sestra_command_11; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_11 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, orig_instr_no);


--
-- Name: idx_tb_sestra_command_12; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_12 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, orig_batch_no);


--
-- Name: idx_tb_sestra_command_13; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_13 ON jzdb_secu.tb_sestra_command_copy1 USING btree (expire_date, complete_flag);


--
-- Name: idx_tb_sestra_command_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_2 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, terminal_external_no, pd_no, pd_unit_no, asac_no);


--
-- Name: idx_tb_sestra_command_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_3 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_sestra_command_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_4 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no);


--
-- Name: idx_tb_sestra_command_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_5 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, exor_no);


--
-- Name: idx_tb_sestra_command_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_6 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no);


--
-- Name: idx_tb_sestra_command_7; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_7 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, exor_no, instr_type);


--
-- Name: idx_tb_sestra_command_8; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_8 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, exter_comm_flag);


--
-- Name: idx_tb_sestra_command_9; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_9 ON jzdb_secu.tb_sestra_command_copy1 USING btree (init_date, co_no, exor_no, order_oper_way);


--
-- Name: idx_tb_sestra_command_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_command_rsp_1 ON jzdb_secu.tb_sestra_command_rsp USING btree (init_date, instr_no);


--
-- Name: idx_tb_sestra_expordermodify_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_expordermodify_jour_1 ON jzdb_secu.tb_sestra_expordermodify_jour USING btree (init_date, serial_no);


--
-- Name: idx_tb_sestra_expordermodify_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_expordermodify_jour_2 ON jzdb_secu.tb_sestra_expordermodify_jour USING btree (init_date, exch_no, secu_code);


--
-- Name: idx_tb_sestra_expordermodify_jour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_expordermodify_jour_3 ON jzdb_secu.tb_sestra_expordermodify_jour USING btree (init_date, order_dir);


--
-- Name: idx_tb_sestra_expordermodify_jour_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_expordermodify_jour_4 ON jzdb_secu.tb_sestra_expordermodify_jour USING btree (init_date, pd_no, pd_unit_no, asac_no);


--
-- Name: idx_tb_sestra_expordermodify_jour_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_expordermodify_jour_rsp_1 ON jzdb_secu.tb_sestra_expordermodify_jour_rsp USING btree (init_date, serial_no);


--
-- Name: idx_tb_sestra_instructapprove_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_instructapprove_1 ON jzdb_secu.tb_sestra_instructapprove USING btree (init_date, appr_no);


--
-- Name: idx_tb_sestra_instructapprove_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_2 ON jzdb_secu.tb_sestra_instructapprove USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_sestra_instructapprove_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_3 ON jzdb_secu.tb_sestra_instructapprove USING btree (init_date, co_no);


--
-- Name: idx_tb_sestra_instructapprove_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_4 ON jzdb_secu.tb_sestra_instructapprove USING btree (init_date, co_no, user_no);


--
-- Name: idx_tb_sestra_instructapprove_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_5 ON jzdb_secu.tb_sestra_instructapprove USING btree (init_date, co_no, orig_batch_no);


--
-- Name: idx_tb_sestra_instructapprove_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_rsp_1 ON jzdb_secu.tb_sestra_instructapprove_rsp USING btree (init_date, appr_no);


--
-- Name: idx_tb_sestra_instructapprove_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_rsp_2 ON jzdb_secu.tb_sestra_instructapprove_rsp USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_sestra_instructapprove_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_rsp_3 ON jzdb_secu.tb_sestra_instructapprove_rsp USING btree (init_date, co_no);


--
-- Name: idx_tb_sestra_instructapprove_rsp_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_rsp_4 ON jzdb_secu.tb_sestra_instructapprove_rsp USING btree (init_date, co_no, user_no);


--
-- Name: idx_tb_sestra_instructapprove_rsp_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructapprove_rsp_5 ON jzdb_secu.tb_sestra_instructapprove_rsp USING btree (init_date, co_no, orig_batch_no);


--
-- Name: idx_tb_sestra_instructjour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_instructjour_1 ON jzdb_secu.tb_sestra_instructjour USING btree (init_date, serial_no);


--
-- Name: idx_tb_sestra_instructjour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_2 ON jzdb_secu.tb_sestra_instructjour USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_sestra_instructjour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_3 ON jzdb_secu.tb_sestra_instructjour USING btree (init_date, co_no);


--
-- Name: idx_tb_sestra_instructjour_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_4 ON jzdb_secu.tb_sestra_instructjour USING btree (init_date, co_no, user_no);


--
-- Name: idx_tb_sestra_instructjour_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_5 ON jzdb_secu.tb_sestra_instructjour USING btree (init_date, co_no, orig_batch_no);


--
-- Name: idx_tb_sestra_instructjour_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_rsp_1 ON jzdb_secu.tb_sestra_instructjour_rsp USING btree (init_date, serial_no);


--
-- Name: idx_tb_sestra_instructjour_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_rsp_2 ON jzdb_secu.tb_sestra_instructjour_rsp USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_sestra_instructjour_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_rsp_3 ON jzdb_secu.tb_sestra_instructjour_rsp USING btree (init_date, co_no);


--
-- Name: idx_tb_sestra_instructjour_rsp_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_rsp_4 ON jzdb_secu.tb_sestra_instructjour_rsp USING btree (init_date, co_no, user_no);


--
-- Name: idx_tb_sestra_instructjour_rsp_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_instructjour_rsp_5 ON jzdb_secu.tb_sestra_instructjour_rsp USING btree (init_date, co_no, orig_batch_no);


--
-- Name: idx_tb_sestra_multidaycommand_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_multidaycommand_1 ON jzdb_secu.tb_sestra_multidaycommand USING btree (init_date, instr_no);


--
-- Name: idx_tb_sestra_multidaycommand_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_multidaycommand_2 ON jzdb_secu.tb_sestra_multidaycommand USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_sestra_multidaycommand_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_multidaycommand_3 ON jzdb_secu.tb_sestra_multidaycommand USING btree (instr_date);


--
-- Name: idx_tb_sestra_order_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_order_1 ON jzdb_secu.tb_sestra_order USING btree (init_date, busi_msg_id);


--
-- Name: idx_tb_sestra_order_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_order_2 ON jzdb_secu.tb_sestra_order USING btree (init_date, external_no);


--
-- Name: idx_tb_sestra_order_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_order_rsp_1 ON jzdb_secu.tb_sestra_order_rsp USING btree (init_date, busi_msg_id);


--
-- Name: idx_tb_sestra_ordersum_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_ordersum_1 ON jzdb_secu.tb_sestra_ordersum USING btree (init_date, co_no, order_batch_no);


--
-- Name: idx_tb_sestra_ordersum_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_ordersum_rsp_1 ON jzdb_secu.tb_sestra_ordersum_rsp USING btree (init_date, co_no, order_batch_no);


--
-- Name: idx_tb_sestra_pd_unit_capit_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_pd_unit_capit_1 ON jzdb_secu.tb_sestra_pd_unit_capit USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_sestra_pd_unit_capit_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_2 ON jzdb_secu.tb_sestra_pd_unit_capit USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_pd_unit_capit_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_3 ON jzdb_secu.tb_sestra_pd_unit_capit USING btree (init_date, co_no, pd_unit_no);


--
-- Name: idx_tb_sestra_pd_unit_capit_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_rsp_1 ON jzdb_secu.tb_sestra_pd_unit_capit_rsp USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, settle_crncy_type, last_update_times);


--
-- Name: idx_tb_sestra_pd_unit_capit_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_rsp_2 ON jzdb_secu.tb_sestra_pd_unit_capit_rsp USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_pd_unit_capit_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_rsp_3 ON jzdb_secu.tb_sestra_pd_unit_capit_rsp USING btree (init_date, co_no, pd_unit_no);


--
-- Name: idx_tb_sestra_pd_unit_capit_trade_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_pd_unit_capit_trade_1 ON jzdb_secu.tb_sestra_pd_unit_capit_trade USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, settle_crncy_type, exch_crncy_type);


--
-- Name: idx_tb_sestra_pd_unit_capit_trade_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_trade_2 ON jzdb_secu.tb_sestra_pd_unit_capit_trade USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_pd_unit_capit_trade_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_trade_3 ON jzdb_secu.tb_sestra_pd_unit_capit_trade USING btree (init_date, co_no, pd_unit_no);


--
-- Name: idx_tb_sestra_pd_unit_capit_trade_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_trade_rsp_1 ON jzdb_secu.tb_sestra_pd_unit_capit_trade_rsp USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, settle_crncy_type, exch_crncy_type, last_update_times);


--
-- Name: idx_tb_sestra_pd_unit_capit_trade_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_trade_rsp_2 ON jzdb_secu.tb_sestra_pd_unit_capit_trade_rsp USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_pd_unit_capit_trade_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_capit_trade_rsp_3 ON jzdb_secu.tb_sestra_pd_unit_capit_trade_rsp USING btree (init_date, co_no, pd_unit_no);


--
-- Name: idx_tb_sestra_pd_unit_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_pd_unit_posi_1 ON jzdb_secu.tb_sestra_pd_unit_posi USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_sestra_pd_unit_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_2 ON jzdb_secu.tb_sestra_pd_unit_posi USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_pd_unit_posi_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_3 ON jzdb_secu.tb_sestra_pd_unit_posi USING btree (init_date, co_no, pd_unit_no);


--
-- Name: idx_tb_sestra_pd_unit_posi_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_rsp_1 ON jzdb_secu.tb_sestra_pd_unit_posi_rsp USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, exch_no, secu_code, last_update_times);


--
-- Name: idx_tb_sestra_pd_unit_posi_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_rsp_2 ON jzdb_secu.tb_sestra_pd_unit_posi_rsp USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_pd_unit_posi_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_rsp_3 ON jzdb_secu.tb_sestra_pd_unit_posi_rsp USING btree (init_date, co_no, pd_unit_no);


--
-- Name: idx_tb_sestra_pd_unit_posi_trade_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_pd_unit_posi_trade_1 ON jzdb_secu.tb_sestra_pd_unit_posi_trade USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, exch_no, secu_code);


--
-- Name: idx_tb_sestra_pd_unit_posi_trade_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_trade_2 ON jzdb_secu.tb_sestra_pd_unit_posi_trade USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_pd_unit_posi_trade_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_trade_3 ON jzdb_secu.tb_sestra_pd_unit_posi_trade USING btree (init_date, co_no, pd_unit_no);


--
-- Name: idx_tb_sestra_pd_unit_posi_trade_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_trade_rsp_1 ON jzdb_secu.tb_sestra_pd_unit_posi_trade_rsp USING btree (init_date, co_no, pd_no, pd_unit_no, asac_no, exch_no, secu_code, last_update_times);


--
-- Name: idx_tb_sestra_pd_unit_posi_trade_rsp_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_trade_rsp_2 ON jzdb_secu.tb_sestra_pd_unit_posi_trade_rsp USING btree (init_date, co_no, pd_no);


--
-- Name: idx_tb_sestra_pd_unit_posi_trade_rsp_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_pd_unit_posi_trade_rsp_3 ON jzdb_secu.tb_sestra_pd_unit_posi_trade_rsp USING btree (init_date, co_no, pd_unit_no);


--
-- Name: idx_tb_sestra_strike_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_strike_1 ON jzdb_secu.tb_sestra_strike USING btree (init_date, asac_no, exch_no, strike_no);


--
-- Name: idx_tb_sestra_strike_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_strike_rsp_1 ON jzdb_secu.tb_sestra_strike_rsp USING btree (init_date, asac_no, exch_no, strike_no);


--
-- Name: idx_tb_sestra_sumcommand_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_sestra_sumcommand_1 ON jzdb_secu.tb_sestra_sumcommand USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_sestra_sumcommand_rsp_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_sestra_sumcommand_rsp_1 ON jzdb_secu.tb_sestra_sumcommand_rsp USING btree (init_date, co_no, comm_batch_no);


--
-- Name: idx_tb_seswap_asac_capit_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seswap_asac_capit_1 ON jzdb_secu.tb_seswap_asac_capit USING btree (pd_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_seswap_asac_capit_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_capit_2 ON jzdb_secu.tb_seswap_asac_capit USING btree (asac_no);


--
-- Name: idx_tb_seswap_asac_capit_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_capit_3 ON jzdb_secu.tb_seswap_asac_capit USING btree (co_no);


--
-- Name: idx_tb_seswap_asac_capit_adjust_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_capit_adjust_jour_1 ON jzdb_secu.tb_seswap_asac_capit_adjust_jour USING btree (init_date, co_no, pd_no, asac_no, settle_crncy_type, adjust_jour_no);


--
-- Name: idx_tb_seswap_asac_capit_adjust_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_capit_adjust_jour_2 ON jzdb_secu.tb_seswap_asac_capit_adjust_jour USING btree (co_no, pd_no, asac_no, settle_crncy_type);


--
-- Name: idx_tb_seswap_asac_capit_adjust_jour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_capit_adjust_jour_3 ON jzdb_secu.tb_seswap_asac_capit_adjust_jour USING btree (co_no, asac_no);


--
-- Name: idx_tb_seswap_asac_capit_adjust_jour_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_capit_adjust_jour_4 ON jzdb_secu.tb_seswap_asac_capit_adjust_jour USING btree (co_no);


--
-- Name: idx_tb_seswap_asac_capit_adjust_jour_5; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_capit_adjust_jour_5 ON jzdb_secu.tb_seswap_asac_capit_adjust_jour USING btree (init_date);


--
-- Name: idx_tb_seswap_asac_capit_adjust_jour_6; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_capit_adjust_jour_6 ON jzdb_secu.tb_seswap_asac_capit_adjust_jour USING btree (source_row_id);


--
-- Name: idx_tb_seswap_asac_posi_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE UNIQUE INDEX idx_tb_seswap_asac_posi_1 ON jzdb_secu.tb_seswap_asac_posi USING btree (co_no, pd_no, asac_no, exch_no, secu_code, lngsht_type, swap_code_no);


--
-- Name: idx_tb_seswap_asac_posi_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_posi_2 ON jzdb_secu.tb_seswap_asac_posi USING btree (co_no, pd_no, asac_no);


--
-- Name: idx_tb_seswap_asac_posi_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_posi_3 ON jzdb_secu.tb_seswap_asac_posi USING btree (co_no, asac_no);


--
-- Name: idx_tb_seswap_asac_posi_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_posi_4 ON jzdb_secu.tb_seswap_asac_posi USING btree (co_no, swap_code_no);


--
-- Name: idx_tb_seswap_asac_posi_adjust_jour_1; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_posi_adjust_jour_1 ON jzdb_secu.tb_seswap_asac_posi_adjust_jour USING btree (init_date, adjust_jour_no, jour_flag, co_no);


--
-- Name: idx_tb_seswap_asac_posi_adjust_jour_2; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_posi_adjust_jour_2 ON jzdb_secu.tb_seswap_asac_posi_adjust_jour USING btree (co_no, pd_no, asac_no, exch_no, secu_code, lngsht_type, swap_code_no);


--
-- Name: idx_tb_seswap_asac_posi_adjust_jour_3; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_posi_adjust_jour_3 ON jzdb_secu.tb_seswap_asac_posi_adjust_jour USING btree (co_no, asac_no);


--
-- Name: idx_tb_seswap_asac_posi_adjust_jour_4; Type: INDEX; Schema: jzdb_secu; Owner: postgres
--

CREATE INDEX idx_tb_seswap_asac_posi_adjust_jour_4 ON jzdb_secu.tb_seswap_asac_posi_adjust_jour USING btree (exch_no, secu_code);


--
-- PostgreSQL database dump complete
--

\unrestrict EGx0w3mcrLMcnbyJ1Fr5mcl1gPtOliAclXdDLdiOQcnuXXFxRu5WUcWq6JT5m2l

