// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioRequest {
    /// Account address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioResponse {
    /// The portfolio of this account
    #[prost(message, optional, tag="1")]
    pub portfolio: ::core::option::Option<AccountPortfolio>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPortfolio {
    /// The account's portfolio value in USD.
    #[prost(string, tag="1")]
    pub portfolio_value: ::prost::alloc::string::String,
    /// The account's available balance value in USD.
    #[prost(string, tag="2")]
    pub available_balance: ::prost::alloc::string::String,
    /// The account's locked balance value in USD.
    #[prost(string, tag="3")]
    pub locked_balance: ::prost::alloc::string::String,
    /// The account's total unrealized PnL value in USD.
    #[prost(string, tag="4")]
    pub unrealized_pnl: ::prost::alloc::string::String,
    /// List of all subaccounts' portfolio
    #[prost(message, repeated, tag="5")]
    pub subaccounts: ::prost::alloc::vec::Vec<SubaccountPortfolio>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountPortfolio {
    /// The ID of this subaccount
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The subaccount's available balance value in USD.
    #[prost(string, tag="2")]
    pub available_balance: ::prost::alloc::string::String,
    /// The subaccount's locked balance value in USD.
    #[prost(string, tag="3")]
    pub locked_balance: ::prost::alloc::string::String,
    /// The subaccount's total unrealized PnL value in USD.
    #[prost(string, tag="4")]
    pub unrealized_pnl: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStatesRequest {
    #[prost(string, repeated, tag="1")]
    pub spot_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub derivative_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStatesResponse {
    /// List of the spot order state records
    #[prost(message, repeated, tag="1")]
    pub spot_order_states: ::prost::alloc::vec::Vec<OrderStateRecord>,
    /// List of the derivative order state records
    #[prost(message, repeated, tag="2")]
    pub derivative_order_states: ::prost::alloc::vec::Vec<OrderStateRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStateRecord {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The Market ID of the order
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The type of the order
    #[prost(string, tag="4")]
    pub order_type: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="5")]
    pub order_side: ::prost::alloc::string::String,
    /// The state (status) of the order
    #[prost(string, tag="6")]
    pub state: ::prost::alloc::string::String,
    /// The filled quantity of the order
    #[prost(string, tag="7")]
    pub quantity_filled: ::prost::alloc::string::String,
    /// The filled quantity of the order
    #[prost(string, tag="8")]
    pub quantity_remaining: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="9")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="10")]
    pub updated_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountsListRequest {
    /// Account address, the subaccounts owner
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountsListResponse {
    #[prost(string, repeated, tag="1")]
    pub subaccounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalancesListRequest {
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Filter balances by denoms. If not set, the balances of all the denoms for
    /// the subaccount are provided.
    #[prost(string, repeated, tag="2")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalancesListResponse {
    /// List of subaccount balances
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<SubaccountBalance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalance {
    /// Related subaccount ID
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Account address, owner of this subaccount
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
    /// Coin denom on the chain.
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub deposit: ::core::option::Option<SubaccountDeposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountDeposit {
    #[prost(string, tag="1")]
    pub total_balance: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub available_balance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalanceRequest {
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Specify denom to get balance
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalanceResponse {
    /// Subaccount balance
    #[prost(message, optional, tag="1")]
    pub balance: ::core::option::Option<SubaccountBalance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamSubaccountBalanceRequest {
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Filter balances by denoms. If not set, the balances of all the denoms for
    /// the subaccount are provided.
    #[prost(string, repeated, tag="2")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamSubaccountBalanceResponse {
    /// Subaccount balance
    #[prost(message, optional, tag="1")]
    pub balance: ::core::option::Option<SubaccountBalance>,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="2")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountHistoryRequest {
    /// SubaccountId of the trader we want to get the history from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Filter history by denom
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// Filter history by transfer type
    #[prost(string, repeated, tag="3")]
    pub transfer_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="4")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned
    #[prost(sint32, tag="5")]
    pub limit: i32,
    /// Upper bound of account transfer history's executedAt
    #[prost(sint64, tag="6")]
    pub end_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountHistoryResponse {
    /// List of subaccount transfers
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<SubaccountBalanceTransfer>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalanceTransfer {
    /// Type of the subaccount balance transfer
    #[prost(string, tag="1")]
    pub transfer_type: ::prost::alloc::string::String,
    /// Subaccount ID of the sending side
    #[prost(string, tag="2")]
    pub src_subaccount_id: ::prost::alloc::string::String,
    /// Account address of the sending side
    #[prost(string, tag="3")]
    pub src_account_address: ::prost::alloc::string::String,
    /// Subaccount ID of the receiving side
    #[prost(string, tag="4")]
    pub dst_subaccount_id: ::prost::alloc::string::String,
    /// Account address of the receiving side
    #[prost(string, tag="5")]
    pub dst_account_address: ::prost::alloc::string::String,
    /// Coin amount of the transfer
    #[prost(message, optional, tag="6")]
    pub amount: ::core::option::Option<CosmosCoin>,
    /// Timestamp of the transfer in UNIX millis
    #[prost(sint64, tag="7")]
    pub executed_at: i64,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderSummaryRequest {
    /// SubaccountId of the trader we want to get the summary from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// MarketId is limiting order summary to specific market only
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Filter by direction of the orders
    #[prost(string, tag="3")]
    pub order_direction: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderSummaryResponse {
    /// Total count of subaccount's spot orders in given market and direction
    #[prost(sint64, tag="1")]
    pub spot_orders_total: i64,
    /// Total count of subaccount's derivative orders in given market and direction
    #[prost(sint64, tag="2")]
    pub derivative_orders_total: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsRequest {
    /// The distribution epoch sequence number. -1 for latest.
    #[prost(sint64, tag="1")]
    pub epoch: i64,
    /// Account address for the rewards distribution
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsResponse {
    /// The trading rewards distributed
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<Reward>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reward {
    /// Account address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// Reward coins distributed
    #[prost(message, repeated, tag="2")]
    pub rewards: ::prost::alloc::vec::Vec<Coin>,
    /// Rewards distribution timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub distributed_at: i64,
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
include!("injective_accounts_rpc.tonic.rs");
// @@protoc_insertion_point(module)