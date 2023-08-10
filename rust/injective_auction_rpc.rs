// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionRequest {
    /// The auction round number. -1 for latest.
    #[prost(sint64, tag="1")]
    pub round: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionResponse {
    /// The auction
    #[prost(message, optional, tag="1")]
    pub auction: ::core::option::Option<Auction>,
    /// Bids of the auction
    #[prost(message, repeated, tag="2")]
    pub bids: ::prost::alloc::vec::Vec<Bid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Auction {
    /// Account address of the auction winner
    #[prost(string, tag="1")]
    pub winner: ::prost::alloc::string::String,
    /// Coins in the basket
    #[prost(message, repeated, tag="2")]
    pub basket: ::prost::alloc::vec::Vec<Coin>,
    #[prost(string, tag="3")]
    pub winning_bid_amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub round: u64,
    /// Auction end timestamp in UNIX millis.
    #[prost(sint64, tag="5")]
    pub end_timestamp: i64,
    /// UpdatedAt timestamp in UNIX millis.
    #[prost(sint64, tag="6")]
    pub updated_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    /// Denom of the coin
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    /// Account address of the bidder
    #[prost(string, tag="1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// Bid timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionsResponse {
    /// The historical auctions
    #[prost(message, repeated, tag="1")]
    pub auctions: ::prost::alloc::vec::Vec<Auction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamBidsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamBidsResponse {
    /// Account address of the bidder
    #[prost(string, tag="1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bid_amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub round: u64,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="4")]
    pub timestamp: i64,
}
include!("injective_auction_rpc.tonic.rs");
// @@protoc_insertion_point(module)