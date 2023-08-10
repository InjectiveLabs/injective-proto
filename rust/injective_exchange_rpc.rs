// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxRequest {
    /// Transaction hash in hex without 0x prefix (Cosmos-like).
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxResponse {
    /// Hex-encoded Tendermint transaction hash
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    /// The block height
    #[prost(sint64, tag="2")]
    pub height: i64,
    /// Tx index in the block
    #[prost(uint32, tag="3")]
    pub index: u32,
    /// Namespace for the resp code
    #[prost(string, tag="4")]
    pub codespace: ::prost::alloc::string::String,
    /// Response code
    #[prost(uint32, tag="5")]
    pub code: u32,
    /// Result bytes, if any
    #[prost(bytes="vec", tag="6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// The output of the application's logger (raw string). May be
    /// non-deterministic.
    #[prost(string, tag="7")]
    pub raw_log: ::prost::alloc::string::String,
    /// Time of the previous block.
    #[prost(string, tag="8")]
    pub timestamp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareTxRequest {
    /// Specify chainID for the target tx
    #[prost(uint64, tag="1")]
    pub chain_id: u64,
    /// Specify Ethereum address of a signer
    #[prost(string, tag="2")]
    pub signer_address: ::prost::alloc::string::String,
    /// Account sequence number (nonce) of signer
    #[prost(uint64, tag="3")]
    pub sequence: u64,
    /// Textual memo information to attach with tx
    #[prost(string, tag="4")]
    pub memo: ::prost::alloc::string::String,
    /// Block height until which the transaction is valid.
    #[prost(uint64, tag="5")]
    pub timeout_height: u64,
    /// Transaction fee details.
    #[prost(message, optional, tag="6")]
    pub fee: ::core::option::Option<CosmosTxFee>,
    /// List of Cosmos proto3-encoded Msgs to include in a single tx
    #[prost(bytes="vec", repeated, tag="7")]
    pub msgs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTxFee {
    /// Transaction gas price
    #[prost(message, repeated, tag="1")]
    pub price: ::prost::alloc::vec::Vec<CosmosCoin>,
    /// Transaction gas limit
    #[prost(uint64, tag="2")]
    pub gas: u64,
    /// Explicitly require fee delegation when set to true. Or don't care = false.
    /// Will be replaced by other flag in the next version.
    #[prost(bool, tag="3")]
    pub delegate_fee: bool,
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
pub struct PrepareTxResponse {
    /// EIP712-compatible message suitable for signing with eth_signTypedData_v4
    #[prost(string, tag="1")]
    pub data: ::prost::alloc::string::String,
    /// Account tx sequence (nonce)
    #[prost(uint64, tag="2")]
    pub sequence: u64,
    /// Sign mode for the resulting tx
    #[prost(string, tag="3")]
    pub sign_mode: ::prost::alloc::string::String,
    /// Specify proto-URL of a public key, which defines the signature format
    #[prost(string, tag="4")]
    pub pub_key_type: ::prost::alloc::string::String,
    /// Fee payer address provided by service
    #[prost(string, tag="5")]
    pub fee_payer: ::prost::alloc::string::String,
    /// Hex-encoded ethsecp256k1 signature bytes from fee payer
    #[prost(string, tag="6")]
    pub fee_payer_sig: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTxRequest {
    /// Specify Web3 chainID (from prepateTx) for the target Tx
    #[prost(uint64, tag="1")]
    pub chain_id: u64,
    /// Amino-encoded Tx JSON data (except Msgs)
    #[prost(bytes="vec", tag="2")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    /// List of Cosmos proto3-encoded Msgs from tx
    #[prost(bytes="vec", repeated, tag="3")]
    pub msgs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Specify ethsecp256k1 pubkey of the signer
    #[prost(message, optional, tag="4")]
    pub pub_key: ::core::option::Option<CosmosPubKey>,
    /// Hex-encoded ethsecp256k1 signature bytes
    #[prost(string, tag="5")]
    pub signature: ::prost::alloc::string::String,
    /// Fee payer address provided by service
    #[prost(string, tag="6")]
    pub fee_payer: ::prost::alloc::string::String,
    /// Hex-encoded ethsecp256k1 signature bytes from fee payer
    #[prost(string, tag="7")]
    pub fee_payer_sig: ::prost::alloc::string::String,
    /// Broadcast mode
    #[prost(string, tag="8")]
    pub mode: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosPubKey {
    /// Pubkey type URL
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Hex-encoded string of the public key
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTxResponse {
    /// Hex-encoded Tendermint transaction hash
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    /// The block height
    #[prost(sint64, tag="2")]
    pub height: i64,
    /// Tx index in the block
    #[prost(uint32, tag="3")]
    pub index: u32,
    /// Namespace for the resp code
    #[prost(string, tag="4")]
    pub codespace: ::prost::alloc::string::String,
    /// Response code
    #[prost(uint32, tag="5")]
    pub code: u32,
    /// Result bytes, if any
    #[prost(bytes="vec", tag="6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// The output of the application's logger (raw string). May be
    /// non-deterministic.
    #[prost(string, tag="7")]
    pub raw_log: ::prost::alloc::string::String,
    /// Time of the previous block.
    #[prost(string, tag="8")]
    pub timestamp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareCosmosTxRequest {
    /// Specify chainID for the target tx
    #[prost(uint64, tag="1")]
    pub chain_id: u64,
    /// sender address provided
    #[prost(string, tag="2")]
    pub sender_address: ::prost::alloc::string::String,
    /// Textual memo information to attach with tx
    #[prost(string, tag="3")]
    pub memo: ::prost::alloc::string::String,
    /// Block height until which the transaction is valid.
    #[prost(uint64, tag="4")]
    pub timeout_height: u64,
    /// Transaction fee details.
    #[prost(message, optional, tag="5")]
    pub fee: ::core::option::Option<CosmosTxFee>,
    /// List of Cosmos proto3-encoded Msgs to include in a single tx
    #[prost(bytes="vec", repeated, tag="6")]
    pub msgs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareCosmosTxResponse {
    /// proto encoded tx
    #[prost(bytes="vec", tag="1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    /// Sign mode for the resulting tx
    #[prost(string, tag="2")]
    pub sign_mode: ::prost::alloc::string::String,
    /// Specify proto-URL of a public key, which defines the signature format
    #[prost(string, tag="3")]
    pub pub_key_type: ::prost::alloc::string::String,
    /// Fee payer address provided by service
    #[prost(string, tag="4")]
    pub fee_payer: ::prost::alloc::string::String,
    /// Hex-encoded ethsecp256k1 signature bytes from fee payer
    #[prost(string, tag="5")]
    pub fee_payer_sig: ::prost::alloc::string::String,
    /// ethsecp256k1 feePayer pubkey
    #[prost(message, optional, tag="6")]
    pub fee_payer_pub_key: ::core::option::Option<CosmosPubKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastCosmosTxRequest {
    /// proto encoded tx
    #[prost(bytes="vec", tag="1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    /// Specify ethsecp256k1 sender pubkey
    #[prost(message, optional, tag="2")]
    pub pub_key: ::core::option::Option<CosmosPubKey>,
    /// Hex-encoded ethsecp256k1 sender signature bytes
    #[prost(string, tag="3")]
    pub signature: ::prost::alloc::string::String,
    /// sender address
    #[prost(string, tag="4")]
    pub sender_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastCosmosTxResponse {
    /// Hex-encoded Tendermint transaction hash
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    /// The block height
    #[prost(sint64, tag="2")]
    pub height: i64,
    /// Tx index in the block
    #[prost(uint32, tag="3")]
    pub index: u32,
    /// Namespace for the resp code
    #[prost(string, tag="4")]
    pub codespace: ::prost::alloc::string::String,
    /// Response code
    #[prost(uint32, tag="5")]
    pub code: u32,
    /// Result bytes, if any
    #[prost(bytes="vec", tag="6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// The output of the application's logger (raw string). May be
    /// non-deterministic.
    #[prost(string, tag="7")]
    pub raw_log: ::prost::alloc::string::String,
    /// Time of the previous block.
    #[prost(string, tag="8")]
    pub timestamp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeePayerRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeePayerResponse {
    /// Fee payer address provided by service
    #[prost(string, tag="1")]
    pub fee_payer: ::prost::alloc::string::String,
    /// ethsecp256k1 feePayer pubkey
    #[prost(message, optional, tag="2")]
    pub fee_payer_pub_key: ::core::option::Option<CosmosPubKey>,
}
include!("injective_exchange_rpc.tonic.rs");
// @@protoc_insertion_point(module)