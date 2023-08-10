// @generated
/// EthAccount implements the authtypes.AccountI interface and embeds an
/// authtypes.BaseAccount type. It is compatible with the auth AccountKeeper.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthAccount {
    #[prost(message, optional, tag="1")]
    pub base_account: ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    #[prost(bytes="vec", tag="2")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionOptionsWeb3Tx {
    /// typedDataChainID used only in EIP712 Domain and should match
    /// Ethereum network ID in a Web3 provider (e.g. Metamask).
    #[prost(uint64, tag="1")]
    pub typed_data_chain_id: u64,
    /// feePayer is an account address for the fee payer. It will be validated
    /// during EIP712 signature checking.
    #[prost(string, tag="2")]
    pub fee_payer: ::prost::alloc::string::String,
    /// feePayerSig is a signature data from the fee paying account,
    /// allows to perform fee delegation when using EIP712 Domain.
    #[prost(bytes="vec", tag="3")]
    pub fee_payer_sig: ::prost::alloc::vec::Vec<u8>,
}
/// base header ak message type, we can cast the bytes into corresponding message
/// response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResponseGenericMessage {
    #[prost(string, tag="1")]
    pub header: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// improvised message to unpack length prefixed messages in tx response data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResponseData {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<TxResponseGenericMessage>,
}
// @@protoc_insertion_point(module)
