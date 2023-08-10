// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountTxsRequest {
    /// Address of account
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub before: u64,
    #[prost(uint64, tag="3")]
    pub after: u64,
    #[prost(sint32, tag="4")]
    pub limit: i32,
    #[prost(uint64, tag="5")]
    pub skip: u64,
    #[prost(string, tag="6")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub module: ::prost::alloc::string::String,
    #[prost(sint64, tag="8")]
    pub from_number: i64,
    #[prost(sint64, tag="9")]
    pub to_number: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountTxsResponse {
    #[prost(message, optional, tag="1")]
    pub paging: ::core::option::Option<Paging>,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<TxDetailData>,
}
/// Paging defines the structure for required params for handling pagination
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paging {
    /// total number of txs saved in database
    #[prost(sint64, tag="1")]
    pub total: i64,
    /// can be either block height or index num
    #[prost(sint32, tag="2")]
    pub from: i32,
    /// can be either block height or index num
    #[prost(sint32, tag="3")]
    pub to: i32,
    /// count entries by subaccount, serving some places on helix
    #[prost(sint64, tag="4")]
    pub count_by_subaccount: i64,
}
/// TxDetailData wraps tx data includes details information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxDetailData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub code: u32,
    #[prost(bytes="vec", tag="6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub info: ::prost::alloc::string::String,
    #[prost(sint64, tag="9")]
    pub gas_wanted: i64,
    #[prost(sint64, tag="10")]
    pub gas_used: i64,
    #[prost(message, optional, tag="11")]
    pub gas_fee: ::core::option::Option<GasFee>,
    #[prost(string, tag="12")]
    pub codespace: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="13")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag="14")]
    pub tx_type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="15")]
    pub messages: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="16")]
    pub signatures: ::prost::alloc::vec::Vec<Signature>,
    #[prost(string, tag="17")]
    pub memo: ::prost::alloc::string::String,
    #[prost(uint64, tag="18")]
    pub tx_number: u64,
    /// Block timestamp in unix milli
    #[prost(uint64, tag="19")]
    pub block_unix_timestamp: u64,
    /// Transaction log indicating errors
    #[prost(string, tag="20")]
    pub error_log: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasFee {
    #[prost(message, repeated, tag="1")]
    pub amount: ::prost::alloc::vec::Vec<CosmosCoin>,
    #[prost(uint64, tag="2")]
    pub gas_limit: u64,
    #[prost(string, tag="3")]
    pub payer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub granter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosCoin {
    /// Coin denominator
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// Coin amount (big int)
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="2")]
    pub attributes: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Signature wraps tx signature
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(string, tag="1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub sequence: u64,
    #[prost(string, tag="4")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContractTxsRequest {
    /// Address of contract
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(sint32, tag="2")]
    pub limit: i32,
    #[prost(uint64, tag="3")]
    pub skip: u64,
    #[prost(sint64, tag="4")]
    pub from_number: i64,
    #[prost(sint64, tag="5")]
    pub to_number: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContractTxsResponse {
    #[prost(message, optional, tag="1")]
    pub paging: ::core::option::Option<Paging>,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<TxDetailData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    #[prost(uint64, tag="1")]
    pub before: u64,
    #[prost(uint64, tag="2")]
    pub after: u64,
    #[prost(sint32, tag="3")]
    pub limit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksResponse {
    #[prost(message, optional, tag="1")]
    pub paging: ::core::option::Option<Paging>,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<BlockInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(string, tag="2")]
    pub proposer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub moniker: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(sint64, tag="6")]
    pub num_pre_commits: i64,
    #[prost(sint64, tag="7")]
    pub num_txs: i64,
    #[prost(message, repeated, tag="8")]
    pub txs: ::prost::alloc::vec::Vec<TxDataRpc>,
    #[prost(string, tag="9")]
    pub timestamp: ::prost::alloc::string::String,
}
/// TxData wraps tx data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxDataRpc {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub codespace: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub messages: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub tx_number: u64,
    /// Transaction log indicating errors
    #[prost(string, tag="8")]
    pub error_log: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub code: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<BlockDetailInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockDetailInfo {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(string, tag="2")]
    pub proposer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub moniker: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(sint64, tag="6")]
    pub num_pre_commits: i64,
    #[prost(sint64, tag="7")]
    pub num_txs: i64,
    #[prost(sint64, tag="8")]
    pub total_txs: i64,
    #[prost(message, repeated, tag="9")]
    pub txs: ::prost::alloc::vec::Vec<TxData>,
    #[prost(string, tag="10")]
    pub timestamp: ::prost::alloc::string::String,
}
/// TxData wraps tx data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub codespace: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub messages: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub tx_number: u64,
    /// Transaction log indicating errors
    #[prost(string, tag="8")]
    pub error_log: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub code: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorsResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<Validator>,
}
/// Validator defines the structure for validator information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub moniker: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub operator_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub consensus_address: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub jailed: bool,
    #[prost(sint32, tag="6")]
    pub status: i32,
    #[prost(string, tag="7")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub delegator_shares: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub description: ::core::option::Option<ValidatorDescription>,
    #[prost(sint64, tag="10")]
    pub unbonding_height: i64,
    #[prost(string, tag="11")]
    pub unbonding_time: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub commission_rate: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub commission_max_rate: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub commission_max_change_rate: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub commission_update_time: ::prost::alloc::string::String,
    #[prost(uint64, tag="16")]
    pub proposed: u64,
    #[prost(uint64, tag="17")]
    pub signed: u64,
    #[prost(uint64, tag="18")]
    pub missed: u64,
    #[prost(string, tag="19")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="20")]
    pub uptimes: ::prost::alloc::vec::Vec<ValidatorUptime>,
    #[prost(message, repeated, tag="21")]
    pub slashing_events: ::prost::alloc::vec::Vec<SlashingEvent>,
    /// uptime percentage base on latest 10k block
    #[prost(double, tag="22")]
    pub uptime_percentage: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorDescription {
    #[prost(string, tag="1")]
    pub moniker: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub website: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub security_contact: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub details: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorUptime {
    #[prost(uint64, tag="1")]
    pub block_number: u64,
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlashingEvent {
    #[prost(uint64, tag="1")]
    pub block_number: u64,
    #[prost(string, tag="2")]
    pub block_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub power: u64,
    #[prost(string, tag="5")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub jailed: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub missed_blocks: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<Validator>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorUptimeRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorUptimeResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<ValidatorUptime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxsRequest {
    #[prost(uint64, tag="1")]
    pub before: u64,
    #[prost(uint64, tag="2")]
    pub after: u64,
    #[prost(sint32, tag="3")]
    pub limit: i32,
    #[prost(uint64, tag="4")]
    pub skip: u64,
    #[prost(string, tag="5")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub module: ::prost::alloc::string::String,
    #[prost(sint64, tag="7")]
    pub from_number: i64,
    #[prost(sint64, tag="8")]
    pub to_number: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxsResponse {
    #[prost(message, optional, tag="1")]
    pub paging: ::core::option::Option<Paging>,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<TxData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxByTxHashRequest {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxByTxHashResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<TxDetailData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeggyDepositTxsRequest {
    /// Sender address of deposit request
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Address of receiveer upon deposit
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(sint32, tag="3")]
    pub limit: i32,
    #[prost(uint64, tag="4")]
    pub skip: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeggyDepositTxsResponse {
    #[prost(message, repeated, tag="1")]
    pub field: ::prost::alloc::vec::Vec<PeggyDepositTx>,
}
/// PeggyDepositTx wraps tx data includes peggy deposit tx details information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeggyDepositTx {
    /// Sender address of deposit request
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Address of receiveer upon deposit
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    /// The event nonce of WithdrawalClaim event emitted by Ethereum chain upon
    /// deposit
    #[prost(uint64, tag="3")]
    pub event_nonce: u64,
    /// The block height of WithdrawalClaim event emitted by Ethereum chain upon
    /// deposit
    #[prost(uint64, tag="4")]
    pub event_height: u64,
    /// Amount of tokens being deposited
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
    /// Denom of tokens being deposited
    #[prost(string, tag="6")]
    pub denom: ::prost::alloc::string::String,
    /// orchestratorAddress who created batch request
    #[prost(string, tag="7")]
    pub orchestrator_address: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub state: ::prost::alloc::string::String,
    /// The claimType will be DepoistClaim for Deposits
    #[prost(sint32, tag="9")]
    pub claim_type: i32,
    #[prost(string, repeated, tag="10")]
    pub tx_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="11")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub updated_at: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeggyWithdrawalTxsRequest {
    /// Sender address of withdrawal request
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Address of receiveer upon withdrawal
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(sint32, tag="3")]
    pub limit: i32,
    #[prost(uint64, tag="4")]
    pub skip: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPeggyWithdrawalTxsResponse {
    #[prost(message, repeated, tag="1")]
    pub field: ::prost::alloc::vec::Vec<PeggyWithdrawalTx>,
}
/// PeggyWithdrawalTx wraps tx data includes peggy withdrawal tx details
/// information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeggyWithdrawalTx {
    /// Sender address of withdrawal request
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Address of receiveer upon withdrawal
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    /// Amount of tokens being withdrawan
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// Denom of tokens being withdrawan
    #[prost(string, tag="4")]
    pub denom: ::prost::alloc::string::String,
    /// The bridge fee paid by sender for withdrawal
    #[prost(string, tag="5")]
    pub bridge_fee: ::prost::alloc::string::String,
    /// A auto incremented unique ID representing the withdrawal request
    #[prost(uint64, tag="6")]
    pub outgoing_tx_id: u64,
    /// The timestamp after which Batch request will be discarded if not processed
    /// already
    #[prost(uint64, tag="7")]
    pub batch_timeout: u64,
    /// A auto incremented unique ID representing the Withdrawal Batches
    #[prost(uint64, tag="8")]
    pub batch_nonce: u64,
    /// orchestratorAddress who created batch request
    #[prost(string, tag="9")]
    pub orchestrator_address: ::prost::alloc::string::String,
    /// The event nonce of WithdrawalClaim event emitted by Ethereum chain upon
    /// batch withdrawal
    #[prost(uint64, tag="10")]
    pub event_nonce: u64,
    /// The block height of WithdrawalClaim event emitted by Ethereum chain upon
    /// batch withdrawal
    #[prost(uint64, tag="11")]
    pub event_height: u64,
    #[prost(string, tag="12")]
    pub state: ::prost::alloc::string::String,
    /// The claimType will be WithdrawalClaim for Batch Withdrawals
    #[prost(sint32, tag="13")]
    pub claim_type: i32,
    #[prost(string, repeated, tag="14")]
    pub tx_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="15")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub updated_at: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIbcTransferTxsRequest {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub src_channel: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub src_port: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub dest_channel: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub dest_port: ::prost::alloc::string::String,
    #[prost(sint32, tag="7")]
    pub limit: i32,
    #[prost(uint64, tag="8")]
    pub skip: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIbcTransferTxsResponse {
    #[prost(message, repeated, tag="1")]
    pub field: ::prost::alloc::vec::Vec<IbcTransferTx>,
}
/// IBCTransferTx wraps tx data includes ibc transfer tx details information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IbcTransferTx {
    /// the sender address
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    /// the port on which the packet will be sent
    #[prost(string, tag="3")]
    pub source_port: ::prost::alloc::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag="4")]
    pub source_channel: ::prost::alloc::string::String,
    /// identifies the port on the receiving chain
    #[prost(string, tag="5")]
    pub destination_port: ::prost::alloc::string::String,
    /// identifies the channel end on the receiving chain
    #[prost(string, tag="6")]
    pub destination_channel: ::prost::alloc::string::String,
    /// transfer amount
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    /// transafer denom
    #[prost(string, tag="8")]
    pub denom: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height. The timeout is disabled
    /// when set to 0
    #[prost(string, tag="9")]
    pub timeout_height: ::prost::alloc::string::String,
    /// Timeout timestamp (in nanoseconds) relative to the current block timestamp
    #[prost(uint64, tag="10")]
    pub timeout_timestamp: u64,
    /// number corresponds to the order of sends and receives, where a Packet with
    /// an earlier sequence number must be sent and received before a Packet with a
    /// later sequence number
    #[prost(uint64, tag="11")]
    pub packet_sequence: u64,
    #[prost(bytes="vec", tag="12")]
    pub data_hex: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="13")]
    pub state: ::prost::alloc::string::String,
    /// it's injective chain tx hash array
    #[prost(string, repeated, tag="14")]
    pub tx_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="15")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub updated_at: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWasmCodesRequest {
    #[prost(sint32, tag="1")]
    pub limit: i32,
    #[prost(sint64, tag="2")]
    pub from_number: i64,
    #[prost(sint64, tag="3")]
    pub to_number: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWasmCodesResponse {
    #[prost(message, optional, tag="1")]
    pub paging: ::core::option::Option<Paging>,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<WasmCode>,
}
/// Detail of cosmwasm stored code
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WasmCode {
    /// ID of stored wasmcode, sorted in descending order
    #[prost(uint64, tag="1")]
    pub code_id: u64,
    /// Tx hash of store code transaction
    #[prost(string, tag="2")]
    pub tx_hash: ::prost::alloc::string::String,
    /// Checksum of the cosmwasm code
    #[prost(message, optional, tag="3")]
    pub checksum: ::core::option::Option<Checksum>,
    /// Block time when the code is stored, in millisecond
    #[prost(uint64, tag="4")]
    pub created_at: u64,
    /// Contract type of the wasm code
    #[prost(string, tag="5")]
    pub contract_type: ::prost::alloc::string::String,
    /// version string of the wasm code
    #[prost(string, tag="6")]
    pub version: ::prost::alloc::string::String,
    /// describe instantiate permission
    #[prost(message, optional, tag="7")]
    pub permission: ::core::option::Option<ContractPermission>,
    /// code schema preview
    #[prost(string, tag="8")]
    pub code_schema: ::prost::alloc::string::String,
    /// code repo preview, may contain schema folder
    #[prost(string, tag="9")]
    pub code_view: ::prost::alloc::string::String,
    /// count number of contract instantiation from this code
    #[prost(uint64, tag="10")]
    pub instantiates: u64,
    /// creator of this code
    #[prost(string, tag="11")]
    pub creator: ::prost::alloc::string::String,
    /// monotonic order of the code stored
    #[prost(sint64, tag="12")]
    pub code_number: i64,
    /// id of the proposal that store this code
    #[prost(sint64, tag="13")]
    pub proposal_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checksum {
    /// Algorithm of hash function
    #[prost(string, tag="1")]
    pub algorithm: ::prost::alloc::string::String,
    /// Hash if apply algorithm to the cosmwasm bytecode
    #[prost(string, tag="2")]
    pub hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractPermission {
    /// Access type of instantiation
    #[prost(sint32, tag="1")]
    pub access_type: i32,
    /// Account address
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWasmCodeByIdRequest {
    /// Code ID of the code
    #[prost(sint64, tag="1")]
    pub code_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWasmCodeByIdResponse {
    /// ID of stored wasmcode, sorted in descending order
    #[prost(uint64, tag="1")]
    pub code_id: u64,
    /// Tx hash of store code transaction
    #[prost(string, tag="2")]
    pub tx_hash: ::prost::alloc::string::String,
    /// Checksum of the cosmwasm code
    #[prost(message, optional, tag="3")]
    pub checksum: ::core::option::Option<Checksum>,
    /// Block time when the code is stored, in millisecond
    #[prost(uint64, tag="4")]
    pub created_at: u64,
    /// Contract type of the wasm code
    #[prost(string, tag="5")]
    pub contract_type: ::prost::alloc::string::String,
    /// version string of the wasm code
    #[prost(string, tag="6")]
    pub version: ::prost::alloc::string::String,
    /// describe instantiate permission
    #[prost(message, optional, tag="7")]
    pub permission: ::core::option::Option<ContractPermission>,
    /// code schema preview
    #[prost(string, tag="8")]
    pub code_schema: ::prost::alloc::string::String,
    /// code repo preview, may contain schema folder
    #[prost(string, tag="9")]
    pub code_view: ::prost::alloc::string::String,
    /// count number of contract instantiation from this code
    #[prost(uint64, tag="10")]
    pub instantiates: u64,
    /// creator of this code
    #[prost(string, tag="11")]
    pub creator: ::prost::alloc::string::String,
    /// monotonic order of the code stored
    #[prost(sint64, tag="12")]
    pub code_number: i64,
    /// id of the proposal that store this code
    #[prost(sint64, tag="13")]
    pub proposal_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWasmContractsRequest {
    #[prost(sint32, tag="1")]
    pub limit: i32,
    #[prost(sint64, tag="2")]
    pub code_id: i64,
    #[prost(sint64, tag="3")]
    pub from_number: i64,
    #[prost(sint64, tag="4")]
    pub to_number: i64,
    #[prost(bool, tag="5")]
    pub assets_only: bool,
    #[prost(sint64, tag="6")]
    pub skip: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWasmContractsResponse {
    #[prost(message, optional, tag="1")]
    pub paging: ::core::option::Option<Paging>,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<WasmContract>,
}
/// Detail of cosmwasm instantiated contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WasmContract {
    /// General name of the contract
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    /// Address of the contract
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    /// hash of the instantiate transaction
    #[prost(string, tag="3")]
    pub tx_hash: ::prost::alloc::string::String,
    /// Address of the contract creator
    #[prost(string, tag="4")]
    pub creator: ::prost::alloc::string::String,
    /// Number of times call to execute contract
    #[prost(uint64, tag="5")]
    pub executes: u64,
    /// Block timestamp that contract was instantiated, in millisecond
    #[prost(uint64, tag="6")]
    pub instantiated_at: u64,
    /// init message when this contract was instantiated
    #[prost(string, tag="7")]
    pub init_message: ::prost::alloc::string::String,
    /// Block timestamp that contract was called, in millisecond
    #[prost(uint64, tag="8")]
    pub last_executed_at: u64,
    /// Contract funds
    #[prost(message, repeated, tag="9")]
    pub funds: ::prost::alloc::vec::Vec<ContractFund>,
    /// Code id of the contract
    #[prost(uint64, tag="10")]
    pub code_id: u64,
    /// Admin of the contract
    #[prost(string, tag="11")]
    pub admin: ::prost::alloc::string::String,
    /// Latest migrate message of the contract
    #[prost(string, tag="12")]
    pub current_migrate_message: ::prost::alloc::string::String,
    /// Monotonic contract number in database
    #[prost(sint64, tag="13")]
    pub contract_number: i64,
    /// Contract version string
    #[prost(string, tag="14")]
    pub version: ::prost::alloc::string::String,
    /// Contract type
    #[prost(string, tag="15")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="16")]
    pub cw20_metadata: ::core::option::Option<Cw20Metadata>,
    /// id of the proposal that instantiate this contract
    #[prost(sint64, tag="17")]
    pub proposal_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractFund {
    /// Denominator
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// Amount of denom
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
/// General cw20 metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cw20Metadata {
    #[prost(message, optional, tag="1")]
    pub token_info: ::core::option::Option<Cw20TokenInfo>,
    #[prost(message, optional, tag="2")]
    pub marketing_info: ::core::option::Option<Cw20MarketingInfo>,
}
/// Token name, symbol, decimal and so on
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cw20TokenInfo {
    /// General name of the token
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Symbol of then token
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    /// Decimal places of token
    #[prost(sint64, tag="3")]
    pub decimals: i64,
    /// Token's total supply
    #[prost(string, tag="4")]
    pub total_supply: ::prost::alloc::string::String,
}
/// Marketing info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cw20MarketingInfo {
    /// Project information
    #[prost(string, tag="1")]
    pub project: ::prost::alloc::string::String,
    /// Token's description
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// logo (url/embedded)
    #[prost(string, tag="3")]
    pub logo: ::prost::alloc::string::String,
    /// A random field for additional marketing info
    #[prost(bytes="vec", tag="4")]
    pub marketing: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWasmContractByAddressRequest {
    /// Contract address
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWasmContractByAddressResponse {
    /// General name of the contract
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    /// Address of the contract
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    /// hash of the instantiate transaction
    #[prost(string, tag="3")]
    pub tx_hash: ::prost::alloc::string::String,
    /// Address of the contract creator
    #[prost(string, tag="4")]
    pub creator: ::prost::alloc::string::String,
    /// Number of times call to execute contract
    #[prost(uint64, tag="5")]
    pub executes: u64,
    /// Block timestamp that contract was instantiated, in millisecond
    #[prost(uint64, tag="6")]
    pub instantiated_at: u64,
    /// init message when this contract was instantiated
    #[prost(string, tag="7")]
    pub init_message: ::prost::alloc::string::String,
    /// Block timestamp that contract was called, in millisecond
    #[prost(uint64, tag="8")]
    pub last_executed_at: u64,
    /// Contract funds
    #[prost(message, repeated, tag="9")]
    pub funds: ::prost::alloc::vec::Vec<ContractFund>,
    /// Code id of the contract
    #[prost(uint64, tag="10")]
    pub code_id: u64,
    /// Admin of the contract
    #[prost(string, tag="11")]
    pub admin: ::prost::alloc::string::String,
    /// Latest migrate message of the contract
    #[prost(string, tag="12")]
    pub current_migrate_message: ::prost::alloc::string::String,
    /// Monotonic contract number in database
    #[prost(sint64, tag="13")]
    pub contract_number: i64,
    /// Contract version string
    #[prost(string, tag="14")]
    pub version: ::prost::alloc::string::String,
    /// Contract type
    #[prost(string, tag="15")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="16")]
    pub cw20_metadata: ::core::option::Option<Cw20Metadata>,
    /// id of the proposal that instantiate this contract
    #[prost(sint64, tag="17")]
    pub proposal_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCw20BalanceRequest {
    /// address to list balance of
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(sint32, tag="2")]
    pub limit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCw20BalanceResponse {
    #[prost(message, repeated, tag="1")]
    pub field: ::prost::alloc::vec::Vec<WasmCw20Balance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WasmCw20Balance {
    /// Address of CW20 contract
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// Account address
    #[prost(string, tag="2")]
    pub account: ::prost::alloc::string::String,
    /// Account balance
    #[prost(string, tag="3")]
    pub balance: ::prost::alloc::string::String,
    /// update timestamp in milisecond
    #[prost(sint64, tag="4")]
    pub updated_at: i64,
    #[prost(message, optional, tag="5")]
    pub cw20_metadata: ::core::option::Option<Cw20Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelayersRequest {
    /// Specify multiple marketIDs to search.
    #[prost(string, repeated, tag="1")]
    pub market_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelayersResponse {
    #[prost(message, repeated, tag="1")]
    pub field: ::prost::alloc::vec::Vec<RelayerMarkets>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelayerMarkets {
    /// Market ID of the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Relayers list for specified market
    #[prost(message, repeated, tag="2")]
    pub relayers: ::prost::alloc::vec::Vec<Relayer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relayer {
    /// Relayer identifier
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Call to action. A link to the relayer
    #[prost(string, tag="2")]
    pub cta: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTxsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTxsResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub codespace: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub messages: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub tx_number: u64,
    /// Transaction log indicating errors
    #[prost(string, tag="8")]
    pub error_log: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub code: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamBlocksRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamBlocksResponse {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(string, tag="2")]
    pub proposer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub moniker: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(sint64, tag="6")]
    pub num_pre_commits: i64,
    #[prost(sint64, tag="7")]
    pub num_txs: i64,
    #[prost(message, repeated, tag="8")]
    pub txs: ::prost::alloc::vec::Vec<TxDataRpc>,
    #[prost(string, tag="9")]
    pub timestamp: ::prost::alloc::string::String,
}
include!("injective_explorer_rpc.tonic.rs");
// @@protoc_insertion_point(module)