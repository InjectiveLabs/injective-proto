// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundsResponse {
    #[prost(message, repeated, tag="1")]
    pub funds: ::prost::alloc::vec::Vec<InsuranceFund>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsuranceFund {
    /// Ticker of the derivative market.
    #[prost(string, tag="1")]
    pub market_ticker: ::prost::alloc::string::String,
    /// Derivative Market ID
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Coin denom used for the underwriting of the insurance fund.
    #[prost(string, tag="3")]
    pub deposit_denom: ::prost::alloc::string::String,
    /// Pool token denom
    #[prost(string, tag="4")]
    pub pool_token_denom: ::prost::alloc::string::String,
    /// Redemption notice period duration in seconds.
    #[prost(sint64, tag="5")]
    pub redemption_notice_period_duration: i64,
    #[prost(string, tag="6")]
    pub balance: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub total_share: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="8")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="9")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle Type
    #[prost(string, tag="10")]
    pub oracle_type: ::prost::alloc::string::String,
    /// Defines the expiry, if any
    #[prost(sint64, tag="11")]
    pub expiry: i64,
    /// Token metadata for the deposit asset, only for Ethereum-based assets
    #[prost(message, optional, tag="12")]
    pub deposit_token_meta: ::core::option::Option<TokenMeta>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMeta {
    /// Token full name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Token Ethereum contract address
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    /// Token symbol short name
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    /// URL to the logo image
    #[prost(string, tag="4")]
    pub logo: ::prost::alloc::string::String,
    /// Token decimals
    #[prost(sint32, tag="5")]
    pub decimals: i32,
    /// Token metadata fetched timestamp in UNIX millis.
    #[prost(sint64, tag="6")]
    pub updated_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedemptionsRequest {
    /// Account address of the redemption owner
    #[prost(string, tag="1")]
    pub redeemer: ::prost::alloc::string::String,
    /// Denom of the insurance pool token.
    #[prost(string, tag="2")]
    pub redemption_denom: ::prost::alloc::string::String,
    /// Status of the redemption. Either pending or disbursed.
    #[prost(string, tag="3")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedemptionsResponse {
    #[prost(message, repeated, tag="1")]
    pub redemption_schedules: ::prost::alloc::vec::Vec<RedemptionSchedule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedemptionSchedule {
    /// Redemption ID.
    #[prost(uint64, tag="1")]
    pub redemption_id: u64,
    /// Status of the redemption. Either pending or disbursed.
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
    /// Account address of the redemption owner
    #[prost(string, tag="3")]
    pub redeemer: ::prost::alloc::string::String,
    /// Claimable redemption time in seconds
    #[prost(sint64, tag="4")]
    pub claimable_redemption_time: i64,
    /// Amount of pool tokens being redeemed.
    #[prost(string, tag="5")]
    pub redemption_amount: ::prost::alloc::string::String,
    /// Pool token denom being redeemed.
    #[prost(string, tag="6")]
    pub redemption_denom: ::prost::alloc::string::String,
    /// Redemption request time in unix milliseconds.
    #[prost(sint64, tag="7")]
    pub requested_at: i64,
    /// Amount of quote tokens disbursed
    #[prost(string, tag="8")]
    pub disbursed_amount: ::prost::alloc::string::String,
    /// Denom of the quote tokens disbursed
    #[prost(string, tag="9")]
    pub disbursed_denom: ::prost::alloc::string::String,
    /// Redemption disbursement time in unix milliseconds.
    #[prost(sint64, tag="10")]
    pub disbursed_at: i64,
}
include!("injective_insurance_rpc.tonic.rs");
// @@protoc_insertion_point(module)