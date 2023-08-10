// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleListRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleListResponse {
    #[prost(message, repeated, tag="1")]
    pub oracles: ::prost::alloc::vec::Vec<Oracle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Oracle {
    /// The symbol of the oracle asset.
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="2")]
    pub base_symbol: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="3")]
    pub quote_symbol: ::prost::alloc::string::String,
    /// Oracle Type
    #[prost(string, tag="4")]
    pub oracle_type: ::prost::alloc::string::String,
    /// The price of the oracle asset
    #[prost(string, tag="5")]
    pub price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRequest {
    /// Oracle base currency
    #[prost(string, tag="1")]
    pub base_symbol: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="2")]
    pub quote_symbol: ::prost::alloc::string::String,
    /// Oracle Type
    #[prost(string, tag="3")]
    pub oracle_type: ::prost::alloc::string::String,
    /// OracleScaleFactor
    #[prost(uint32, tag="4")]
    pub oracle_scale_factor: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceResponse {
    /// The price of the oracle asset
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamPricesRequest {
    /// Oracle base currency
    #[prost(string, tag="1")]
    pub base_symbol: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="2")]
    pub quote_symbol: ::prost::alloc::string::String,
    /// Oracle Type
    #[prost(string, tag="3")]
    pub oracle_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamPricesResponse {
    /// The price of the oracle asset
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="2")]
    pub timestamp: i64,
}
include!("injective_oracle_rpc.tonic.rs");
// @@protoc_insertion_point(module)