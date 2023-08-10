// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestHeightRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestHeightResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<LatestBlockHeight>,
}
/// Latest block height from event provider db
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatestBlockHeight {
    #[prost(uint64, tag="1")]
    pub height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockEventsRpcRequest {
    /// Select backend processor
    #[prost(string, tag="1")]
    pub backend: ::prost::alloc::string::String,
    #[prost(sint32, tag="2")]
    pub height: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockEventsRpcResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<BlockEventsRpc>,
}
/// Processed block events for backend processor to consume
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockEventsRpc {
    /// Array of event types
    #[prost(string, repeated, tag="1")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Array of parsed events
    #[prost(bytes="vec", repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomEventsRpcRequest {
    /// Select backend processor
    #[prost(string, tag="1")]
    pub backend: ::prost::alloc::string::String,
    #[prost(sint32, tag="2")]
    pub height: i32,
    /// Specify custom events to get
    #[prost(string, tag="3")]
    pub events: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomEventsRpcResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<BlockEventsRpc>,
}
include!("event_provider_api.tonic.rs");
// @@protoc_insertion_point(module)