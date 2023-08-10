// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketsRequest {
    /// Filter by market status
    #[prost(string, tag="1")]
    pub market_status: ::prost::alloc::string::String,
    /// Filter by the Coin denomination of the base currency
    #[prost(string, tag="2")]
    pub base_denom: ::prost::alloc::string::String,
    /// Filter by the Coin denomination of the quote currency
    #[prost(string, tag="3")]
    pub quote_denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketsResponse {
    /// Spot Markets list
    #[prost(message, repeated, tag="1")]
    pub markets: ::prost::alloc::vec::Vec<SpotMarketInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketInfo {
    /// SpotMarket ID is keccak265(baseDenom || quoteDenom)
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// The status of the market
    #[prost(string, tag="2")]
    pub market_status: ::prost::alloc::string::String,
    /// A name of the pair in format AAA/BBB, where AAA is base asset, BBB is quote
    /// asset.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// Coin denom used for the base asset.
    #[prost(string, tag="4")]
    pub base_denom: ::prost::alloc::string::String,
    /// Token metadata for base asset, only for Ethereum-based assets
    #[prost(message, optional, tag="5")]
    pub base_token_meta: ::core::option::Option<TokenMeta>,
    /// Coin denom used for the quote asset.
    #[prost(string, tag="6")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Token metadata for quote asset, only for Ethereum-based assets
    #[prost(message, optional, tag="7")]
    pub quote_token_meta: ::core::option::Option<TokenMeta>,
    /// Defines the fee percentage makers pay when trading (in quote asset)
    #[prost(string, tag="8")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// Defines the fee percentage takers pay when trading (in quote asset)
    #[prost(string, tag="9")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// Percentage of the transaction fee shared with the service provider
    #[prost(string, tag="10")]
    pub service_provider_fee: ::prost::alloc::string::String,
    /// Defines the minimum required tick size for the order's price
    #[prost(string, tag="11")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// Defines the minimum required tick size for the order's quantity
    #[prost(string, tag="12")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
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
pub struct MarketRequest {
    /// MarketId of the market we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketResponse {
    /// Info about particular spot market
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<SpotMarketInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMarketsRequest {
    /// List of market IDs for updates streaming, empty means 'ALL' spot markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMarketsResponse {
    /// Info about particular spot market
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<SpotMarketInfo>,
    /// Update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookRequest {
    /// MarketId of the market's orderbook we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookResponse {
    /// Orderbook of a particular spot market
    #[prost(message, optional, tag="1")]
    pub orderbook: ::core::option::Option<SpotLimitOrderbook>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotLimitOrderbook {
    /// Array of price levels for buys
    #[prost(message, repeated, tag="1")]
    pub buys: ::prost::alloc::vec::Vec<PriceLevel>,
    /// Array of price levels for sells
    #[prost(message, repeated, tag="2")]
    pub sells: ::prost::alloc::vec::Vec<PriceLevel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceLevel {
    /// Price number of the price level.
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// Quantity of the price level.
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    /// Price level last updated timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookV2Request {
    /// MarketId of the market's orderbook we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookV2Response {
    /// Orderbook of a particular spot market
    #[prost(message, optional, tag="1")]
    pub orderbook: ::core::option::Option<SpotLimitOrderbookV2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotLimitOrderbookV2 {
    /// Array of price levels for buys
    #[prost(message, repeated, tag="1")]
    pub buys: ::prost::alloc::vec::Vec<PriceLevel>,
    /// Array of price levels for sells
    #[prost(message, repeated, tag="2")]
    pub sells: ::prost::alloc::vec::Vec<PriceLevel>,
    /// market orderbook sequence
    #[prost(uint64, tag="3")]
    pub sequence: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbooksRequest {
    /// MarketIds of the markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbooksResponse {
    #[prost(message, repeated, tag="1")]
    pub orderbooks: ::prost::alloc::vec::Vec<SingleSpotLimitOrderbook>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleSpotLimitOrderbook {
    /// market's ID
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Orderbook of the market
    #[prost(message, optional, tag="2")]
    pub orderbook: ::core::option::Option<SpotLimitOrderbook>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbooksV2Request {
    /// MarketIds of the markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbooksV2Response {
    #[prost(message, repeated, tag="1")]
    pub orderbooks: ::prost::alloc::vec::Vec<SingleSpotLimitOrderbookV2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleSpotLimitOrderbookV2 {
    /// market's ID
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Orderbook of the market
    #[prost(message, optional, tag="2")]
    pub orderbook: ::core::option::Option<SpotLimitOrderbookV2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookRequest {
    /// List of market IDs for orderbook streaming, empty means 'ALL' spot markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookResponse {
    /// Orderbook of a Spot Market
    #[prost(message, optional, tag="1")]
    pub orderbook: ::core::option::Option<SpotLimitOrderbook>,
    /// Order update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
    /// MarketId of the market's orderbook
    #[prost(string, tag="4")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookV2Request {
    /// List of market IDs for orderbook streaming, empty means 'ALL' spot markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookV2Response {
    /// Orderbook of a Spot Market
    #[prost(message, optional, tag="1")]
    pub orderbook: ::core::option::Option<SpotLimitOrderbookV2>,
    /// Order update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
    /// MarketId of the market's orderbook
    #[prost(string, tag="4")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookUpdateRequest {
    /// List of market IDs for orderbook streaming, empty means 'ALL' spot markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookUpdateResponse {
    /// Orderbook level updates of a Spot Market
    #[prost(message, optional, tag="1")]
    pub orderbook_level_updates: ::core::option::Option<OrderbookLevelUpdates>,
    /// Order update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
    /// MarketId of the market's orderbook
    #[prost(string, tag="4")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookLevelUpdates {
    /// market's ID
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// orderbook update sequence
    #[prost(uint64, tag="2")]
    pub sequence: u64,
    /// buy levels
    #[prost(message, repeated, tag="3")]
    pub buys: ::prost::alloc::vec::Vec<PriceLevelUpdate>,
    /// sell levels
    #[prost(message, repeated, tag="4")]
    pub sells: ::prost::alloc::vec::Vec<PriceLevelUpdate>,
    /// updates timestamp
    #[prost(sint64, tag="5")]
    pub updated_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceLevelUpdate {
    /// Price number of the price level.
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// Quantity of the price level.
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    /// Price level status.
    #[prost(bool, tag="3")]
    pub is_active: bool,
    /// Price level last updated timestamp in UNIX millis.
    #[prost(sint64, tag="4")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdersRequest {
    /// MarketId of the market's orderbook we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Look for specific order side
    #[prost(string, tag="2")]
    pub order_side: ::prost::alloc::string::String,
    /// Look for specific subaccountId of an order
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="4")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned.
    #[prost(sint32, tag="5")]
    pub limit: i32,
    /// The starting timestamp in UNIX milliseconds that the trades must be equal or
    /// older than
    #[prost(sint64, tag="6")]
    pub start_time: i64,
    /// The ending timestamp in UNIX milliseconds that the trades must be equal or
    /// younger than
    #[prost(sint64, tag="7")]
    pub end_time: i64,
    /// MarketIds of the markets of which we want to get trades
    #[prost(string, repeated, tag="8")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Should include inactive orders
    #[prost(bool, tag="9")]
    pub include_inactive: bool,
    /// Choose to return subaccount total orders
    #[prost(bool, tag="10")]
    pub subaccount_total_orders: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdersResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<SpotLimitOrder>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotLimitOrder {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="2")]
    pub order_side: ::prost::alloc::string::String,
    /// Spot Market ID is keccak265(baseDenom + quoteDenom)
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Price of the order
    #[prost(string, tag="5")]
    pub price: ::prost::alloc::string::String,
    /// Quantity of the order
    #[prost(string, tag="6")]
    pub quantity: ::prost::alloc::string::String,
    /// The amount of the quantity remaining unfilled
    #[prost(string, tag="7")]
    pub unfilled_quantity: ::prost::alloc::string::String,
    /// Trigger price is the trigger price used by stop/take orders. 0 if the
    /// trigger price is not set.
    #[prost(string, tag="8")]
    pub trigger_price: ::prost::alloc::string::String,
    /// Fee recipient address
    #[prost(string, tag="9")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// Order state
    #[prost(string, tag="10")]
    pub state: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="11")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="12")]
    pub updated_at: i64,
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
pub struct StreamOrdersRequest {
    /// MarketId of the market's orderbook we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Look for specific order side
    #[prost(string, tag="2")]
    pub order_side: ::prost::alloc::string::String,
    /// Look for specific subaccountId of an order
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="4")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned.
    #[prost(sint32, tag="5")]
    pub limit: i32,
    /// The starting timestamp in UNIX milliseconds that the trades must be equal or
    /// older than
    #[prost(sint64, tag="6")]
    pub start_time: i64,
    /// The ending timestamp in UNIX milliseconds that the trades must be equal or
    /// younger than
    #[prost(sint64, tag="7")]
    pub end_time: i64,
    /// MarketIds of the markets of which we want to get trades
    #[prost(string, repeated, tag="8")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Should include inactive orders
    #[prost(bool, tag="9")]
    pub include_inactive: bool,
    /// Choose to return subaccount total orders
    #[prost(bool, tag="10")]
    pub subaccount_total_orders: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrdersResponse {
    /// Updated market order
    #[prost(message, optional, tag="1")]
    pub order: ::core::option::Option<SpotLimitOrder>,
    /// Order update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesRequest {
    /// MarketId of the market's orderbook we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Filter by execution side of the trade
    #[prost(string, tag="2")]
    pub execution_side: ::prost::alloc::string::String,
    /// Filter by direction the trade
    #[prost(string, tag="3")]
    pub direction: ::prost::alloc::string::String,
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the item result
    #[prost(uint64, tag="5")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned.
    #[prost(sint32, tag="6")]
    pub limit: i32,
    /// The starting timestamp in UNIX milliseconds that the trades must be equal or
    /// older than
    #[prost(sint64, tag="7")]
    pub start_time: i64,
    /// The ending timestamp in UNIX milliseconds that the trades must be equal or
    /// younger than
    #[prost(sint64, tag="8")]
    pub end_time: i64,
    /// MarketIds of the markets of which we want to get trades
    #[prost(string, repeated, tag="9")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Subaccount ids of traders we want to get trades
    #[prost(string, repeated, tag="10")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="11")]
    pub execution_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesResponse {
    /// Trades of a Spot Market
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<SpotTrade>,
    /// Paging indicates pages response is on
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotTrade {
    /// Maker order hash.
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The subaccountId that executed the trade
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The ID of the market that this trade is in
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The execution type of the trade
    #[prost(string, tag="4")]
    pub trade_execution_type: ::prost::alloc::string::String,
    /// The direction the trade
    #[prost(string, tag="5")]
    pub trade_direction: ::prost::alloc::string::String,
    /// Price level at which trade has been executed
    #[prost(message, optional, tag="6")]
    pub price: ::core::option::Option<PriceLevel>,
    /// The fee associated with the trade (quote asset denom)
    #[prost(string, tag="7")]
    pub fee: ::prost::alloc::string::String,
    /// Timestamp of trade execution in UNIX millis
    #[prost(sint64, tag="8")]
    pub executed_at: i64,
    /// Fee recipient address
    #[prost(string, tag="9")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// A unique string that helps differentiate between trades
    #[prost(string, tag="10")]
    pub trade_id: ::prost::alloc::string::String,
    /// Trade's execution side, marker/taker
    #[prost(string, tag="11")]
    pub execution_side: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTradesRequest {
    /// MarketId of the market's orderbook we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Filter by execution side of the trade
    #[prost(string, tag="2")]
    pub execution_side: ::prost::alloc::string::String,
    /// Filter by direction the trade
    #[prost(string, tag="3")]
    pub direction: ::prost::alloc::string::String,
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the item result
    #[prost(uint64, tag="5")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned.
    #[prost(sint32, tag="6")]
    pub limit: i32,
    /// The starting timestamp in UNIX milliseconds that the trades must be equal or
    /// older than
    #[prost(sint64, tag="7")]
    pub start_time: i64,
    /// The ending timestamp in UNIX milliseconds that the trades must be equal or
    /// younger than
    #[prost(sint64, tag="8")]
    pub end_time: i64,
    /// MarketIds of the markets of which we want to get trades
    #[prost(string, repeated, tag="9")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Subaccount ids of traders we want to get trades
    #[prost(string, repeated, tag="10")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="11")]
    pub execution_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTradesResponse {
    /// New spot market trade
    #[prost(message, optional, tag="1")]
    pub trade: ::core::option::Option<SpotTrade>,
    /// Executed trades update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrdersListRequest {
    /// subaccount ID to filter orders for specific subaccount
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Market ID to filter orders for specific market
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="3")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned
    #[prost(sint32, tag="4")]
    pub limit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrdersListResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<SpotLimitOrder>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountTradesListRequest {
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Filter trades by market ID
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Filter by execution type of trades
    #[prost(string, tag="3")]
    pub execution_type: ::prost::alloc::string::String,
    /// Filter by direction trades
    #[prost(string, tag="4")]
    pub direction: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="5")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned
    #[prost(sint32, tag="6")]
    pub limit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountTradesListResponse {
    /// List of spot market trades
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<SpotTrade>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdersHistoryRequest {
    /// subaccount ID to filter orders for specific subaccount
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Market ID to filter orders for specific market
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="3")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned
    #[prost(sint32, tag="4")]
    pub limit: i32,
    /// filter by order types
    #[prost(string, repeated, tag="5")]
    pub order_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// order side filter
    #[prost(string, tag="6")]
    pub direction: ::prost::alloc::string::String,
    /// Search for orders which createdAt >= startTime, time in millisecond
    #[prost(sint64, tag="7")]
    pub start_time: i64,
    /// Search for orders which createdAt <= endTime, time in millisecond
    #[prost(sint64, tag="8")]
    pub end_time: i64,
    /// Filter by order state
    #[prost(string, tag="9")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="10")]
    pub execution_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdersHistoryResponse {
    /// List of history spot orders
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<SpotOrderHistory>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrderHistory {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// Spot Market ID is keccak265(baseDenom + quoteDenom)
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// active state of the order
    #[prost(bool, tag="3")]
    pub is_active: bool,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The execution type
    #[prost(string, tag="5")]
    pub execution_type: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="6")]
    pub order_type: ::prost::alloc::string::String,
    /// Price of the order
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    /// Trigger price
    #[prost(string, tag="8")]
    pub trigger_price: ::prost::alloc::string::String,
    /// Quantity of the order
    #[prost(string, tag="9")]
    pub quantity: ::prost::alloc::string::String,
    /// Filled amount
    #[prost(string, tag="10")]
    pub filled_quantity: ::prost::alloc::string::String,
    /// Order state
    #[prost(string, tag="11")]
    pub state: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="12")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="13")]
    pub updated_at: i64,
    /// Order direction (order side)
    #[prost(string, tag="14")]
    pub direction: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrdersHistoryRequest {
    /// subaccount ID to filter orders for specific subaccount
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Market ID to filter orders for specific market
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// filter by order types
    #[prost(string, repeated, tag="3")]
    pub order_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// order side filter
    #[prost(string, tag="4")]
    pub direction: ::prost::alloc::string::String,
    /// Filter by order state
    #[prost(string, tag="5")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub execution_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrdersHistoryResponse {
    /// Updated order
    #[prost(message, optional, tag="1")]
    pub order: ::core::option::Option<SpotOrderHistory>,
    /// Order update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
include!("injective_spot_exchange_rpc.tonic.rs");
// @@protoc_insertion_point(module)