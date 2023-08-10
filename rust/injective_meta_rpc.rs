// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionResponse {
    /// injective-exchange code version.
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    /// Additional build meta info.
    #[prost(map="string, string", tag="2")]
    pub build: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {
    /// Provide current system UNIX timestamp in millis
    #[prost(sint64, tag="1")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    /// The original timestamp value in millis.
    #[prost(sint64, tag="1")]
    pub timestamp: i64,
    /// UNIX time on the server in millis.
    #[prost(sint64, tag="2")]
    pub server_time: i64,
    /// injective-exchange code version.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// Additional build meta info.
    #[prost(map="string, string", tag="4")]
    pub build: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Server's location region
    #[prost(string, tag="5")]
    pub region: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamKeepaliveRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamKeepaliveResponse {
    /// Server event
    #[prost(string, tag="1")]
    pub event: ::prost::alloc::string::String,
    /// New conection endpoint for the gRPC API
    #[prost(string, tag="2")]
    pub new_endpoint: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
include!("injective_meta_rpc.tonic.rs");
// @@protoc_insertion_point(module)