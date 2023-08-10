// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPortfolioRequest {
    /// Account address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPortfolioResponse {
    /// The portfolio of this account
    #[prost(message, optional, tag="1")]
    pub portfolio: ::core::option::Option<Portfolio>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Portfolio {
    /// The account's portfolio address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// Account denom
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// Account available bank balances
    #[prost(message, repeated, tag="3")]
    pub bank_balances: ::prost::alloc::vec::Vec<Coin>,
    /// The subaccount balances of this account
    #[prost(message, repeated, tag="4")]
    pub subaccount_balances: ::prost::alloc::vec::Vec<SubaccountBalanceV2>,
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
pub struct SubaccountBalanceV2 {
    /// Related subaccount ID
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Available subaccount balances
    #[prost(message, repeated, tag="2")]
    pub available_balances: ::prost::alloc::vec::Vec<Coin>,
    /// Margin held by open orders
    #[prost(message, repeated, tag="3")]
    pub margin_hold: ::prost::alloc::vec::Vec<Coin>,
    /// unrealizedPNL of open positions
    #[prost(message, repeated, tag="4")]
    pub unrealized_pnl: ::prost::alloc::vec::Vec<Coin>,
}
include!("injective_portfolio_rpc.tonic.rs");
// @@protoc_insertion_point(module)