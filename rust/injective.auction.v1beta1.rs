// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// auction_period_duration defines the auction period duration
    #[prost(int64, tag="1")]
    pub auction_period: i64,
    /// min_next_bid_increment_rate defines the minimum increment rate for new bids
    #[prost(string, tag="2")]
    pub min_next_bid_increment_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    #[prost(string, tag="1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBid {
    /// bidder describes the address of bidder
    #[prost(string, tag="1")]
    pub bidder: ::prost::alloc::string::String,
    /// amount describes the amount the bidder put on the auction
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag="3")]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAuctionResult {
    /// winner describes the address of the winner
    #[prost(string, tag="1")]
    pub winner: ::prost::alloc::string::String,
    /// amount describes the amount the winner get from the auction
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag="3")]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAuctionStart {
    /// round defines the round number of auction
    #[prost(uint64, tag="1")]
    pub round: u64,
    /// ending_timestamp describes auction end time
    #[prost(int64, tag="2")]
    pub ending_timestamp: i64,
    /// new_basket describes auction module balance at the time of new auction
    /// start
    #[prost(message, repeated, tag="3")]
    pub new_basket: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the auction module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of related to auction.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// current auction round
    #[prost(uint64, tag="2")]
    pub auction_round: u64,
    /// current highest bid
    #[prost(message, optional, tag="3")]
    pub highest_bid: ::core::option::Option<Bid>,
    /// auction ending timestamp
    #[prost(int64, tag="4")]
    pub auction_ending_timestamp: i64,
}
/// QueryAuctionParamsRequest is the request type for the Query/AuctionParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionParamsRequest {
}
/// QueryAuctionParamsRequest is the response type for the Query/AuctionParams
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryCurrentAuctionBasketRequest is the request type for the
/// Query/CurrentAuctionBasket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentAuctionBasketRequest {
}
/// QueryCurrentAuctionBasketResponse is the response type for the
/// Query/CurrentAuctionBasket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentAuctionBasketResponse {
    /// amount describes the amount put on auction
    #[prost(message, repeated, tag="1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// auctionRound describes current auction round
    #[prost(uint64, tag="2")]
    pub auction_round: u64,
    /// auctionClosingTime describes auction close time for the round
    #[prost(int64, tag="3")]
    pub auction_closing_time: i64,
    /// highestBidder describes highest bidder on current round
    #[prost(string, tag="4")]
    pub highest_bidder: ::prost::alloc::string::String,
    /// highestBidAmount describes highest bid amount on current round
    #[prost(string, tag="5")]
    pub highest_bid_amount: ::prost::alloc::string::String,
}
/// QueryModuleStateRequest is the request type for the Query/AuctionModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {
}
/// QueryModuleStateResponse is the response type for the
/// Query/AuctionModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<GenesisState>,
}
/// Bid defines a SDK message for placing a bid for an auction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBid {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// amount of the bid in INJ tokens
    #[prost(message, optional, tag="2")]
    pub bid_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// the current auction round being bid on
    #[prost(uint64, tag="3")]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBidResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the ocr parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
include!("injective.auction.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)