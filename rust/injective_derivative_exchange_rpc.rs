// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketsRequest {
    /// Filter by market status
    #[prost(string, tag="1")]
    pub market_status: ::prost::alloc::string::String,
    /// Filter by the Coin denomination of the quote currency
    #[prost(string, tag="2")]
    pub quote_denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketsResponse {
    /// Derivative Markets list
    #[prost(message, repeated, tag="1")]
    pub markets: ::prost::alloc::vec::Vec<DerivativeMarketInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketInfo {
    /// DerivativeMarket ID is crypto.Keccak256Hash([]byte((oracleType.String() +
    /// ticker + quoteDenom + oracleBase + oracleQuote))) for perpetual markets and
    /// crypto.Keccak256Hash([]byte((oracleType.String() + ticker + quoteDenom +
    /// oracleBase + oracleQuote + strconv.Itoa(int(expiry))))) for expiry futures
    /// markets
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// The status of the market
    #[prost(string, tag="2")]
    pub market_status: ::prost::alloc::string::String,
    /// A name of the pair in format AAA/BBB, where AAA is base asset, BBB is quote
    /// asset.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle Type
    #[prost(string, tag="6")]
    pub oracle_type: ::prost::alloc::string::String,
    /// OracleScaleFactor
    #[prost(uint32, tag="7")]
    pub oracle_scale_factor: u32,
    /// Defines the initial margin ratio of a derivative market
    #[prost(string, tag="8")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// Defines the maintenance margin ratio of a derivative market
    #[prost(string, tag="9")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// Coin denom used for the quote asset.
    #[prost(string, tag="10")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Token metadata for quote asset, only for Ethereum-based assets
    #[prost(message, optional, tag="11")]
    pub quote_token_meta: ::core::option::Option<TokenMeta>,
    /// Defines the fee percentage makers pay when trading (in quote asset)
    #[prost(string, tag="12")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// Defines the fee percentage takers pay when trading (in quote asset)
    #[prost(string, tag="13")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// Percentage of the transaction fee shared with the service provider
    #[prost(string, tag="14")]
    pub service_provider_fee: ::prost::alloc::string::String,
    /// True if the market is a perpetual swap market
    #[prost(bool, tag="15")]
    pub is_perpetual: bool,
    /// Defines the minimum required tick size for the order's price
    #[prost(string, tag="16")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// Defines the minimum required tick size for the order's quantity
    #[prost(string, tag="17")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    #[prost(message, optional, tag="18")]
    pub perpetual_market_info: ::core::option::Option<PerpetualMarketInfo>,
    #[prost(message, optional, tag="19")]
    pub perpetual_market_funding: ::core::option::Option<PerpetualMarketFunding>,
    #[prost(message, optional, tag="20")]
    pub expiry_futures_market_info: ::core::option::Option<ExpiryFuturesMarketInfo>,
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
pub struct PerpetualMarketInfo {
    /// Defines the default maximum absolute value of the hourly funding rate of the
    /// perpetual market.
    #[prost(string, tag="1")]
    pub hourly_funding_rate_cap: ::prost::alloc::string::String,
    /// Defines the hourly interest rate of the perpetual market.
    #[prost(string, tag="2")]
    pub hourly_interest_rate: ::prost::alloc::string::String,
    /// Defines the next funding timestamp in seconds of a perpetual market in UNIX
    /// seconds.
    #[prost(sint64, tag="3")]
    pub next_funding_timestamp: i64,
    /// Defines the funding interval in seconds of a perpetual market in seconds.
    #[prost(sint64, tag="4")]
    pub funding_interval: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketFunding {
    /// Defines the cumulative funding of a perpetual market.
    #[prost(string, tag="1")]
    pub cumulative_funding: ::prost::alloc::string::String,
    /// Defines defines the cumulative price for the current hour up to the last
    /// timestamp.
    #[prost(string, tag="2")]
    pub cumulative_price: ::prost::alloc::string::String,
    /// Defines the last funding timestamp in seconds of a perpetual market in UNIX
    /// seconds.
    #[prost(sint64, tag="3")]
    pub last_timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiryFuturesMarketInfo {
    /// Defines the expiration time for a time expiry futures market in UNIX seconds.
    #[prost(sint64, tag="1")]
    pub expiration_timestamp: i64,
    /// Defines the settlement price for a time expiry futures market.
    #[prost(string, tag="2")]
    pub settlement_price: ::prost::alloc::string::String,
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
    /// Info about particular derivative market
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<DerivativeMarketInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMarketRequest {
    /// List of market IDs for updates streaming, empty means 'ALL' derivative
    /// markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMarketResponse {
    /// Info about particular derivative market
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<DerivativeMarketInfo>,
    /// Update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarketsRequest {
    /// Filter by market status
    #[prost(string, tag="1")]
    pub market_status: ::prost::alloc::string::String,
    /// Filter by the Coin denomination of the quote currency
    #[prost(string, tag="2")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="3")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned.
    #[prost(sint32, tag="4")]
    pub limit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarketsResponse {
    /// Binary Options Markets list
    #[prost(message, repeated, tag="1")]
    pub markets: ::prost::alloc::vec::Vec<BinaryOptionsMarketInfo>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarketInfo {
    /// Binary Options Market ID is crypto.Keccak256Hash([]byte((oracleType.String()
    /// + ticker + quoteDenom + oracleSymbol + oracleProvider)))
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// The status of the market
    #[prost(string, tag="2")]
    pub market_status: ::prost::alloc::string::String,
    /// A name of the binary options market.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag="4")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle provider
    #[prost(string, tag="5")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle Type
    #[prost(string, tag="6")]
    pub oracle_type: ::prost::alloc::string::String,
    /// OracleScaleFactor
    #[prost(uint32, tag="7")]
    pub oracle_scale_factor: u32,
    /// Defines the expiration time for the market in UNIX seconds.
    #[prost(sint64, tag="8")]
    pub expiration_timestamp: i64,
    /// Defines the settlement time for the market in UNIX seconds.
    #[prost(sint64, tag="9")]
    pub settlement_timestamp: i64,
    /// Coin denom used for the quote asset.
    #[prost(string, tag="10")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Token metadata for quote asset, only for Ethereum-based assets
    #[prost(message, optional, tag="11")]
    pub quote_token_meta: ::core::option::Option<TokenMeta>,
    /// Defines the fee percentage makers pay when trading (in quote asset)
    #[prost(string, tag="12")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// Defines the fee percentage takers pay when trading (in quote asset)
    #[prost(string, tag="13")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// Percentage of the transaction fee shared with the service provider
    #[prost(string, tag="14")]
    pub service_provider_fee: ::prost::alloc::string::String,
    /// Defines the minimum required tick size for the order's price
    #[prost(string, tag="15")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// Defines the minimum required tick size for the order's quantity
    #[prost(string, tag="16")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// Defines the settlement price of the market
    #[prost(string, tag="17")]
    pub settlement_price: ::prost::alloc::string::String,
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
pub struct BinaryOptionsMarketRequest {
    /// MarketId of the market we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarketResponse {
    /// Info about particular derivative market
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<BinaryOptionsMarketInfo>,
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
    /// Orderbook of a particular derivative market
    #[prost(message, optional, tag="1")]
    pub orderbook: ::core::option::Option<DerivativeLimitOrderbook>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeLimitOrderbook {
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
    /// Orderbook of a particular derivative market
    #[prost(message, optional, tag="1")]
    pub orderbook: ::core::option::Option<DerivativeLimitOrderbookV2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeLimitOrderbookV2 {
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
    pub orderbooks: ::prost::alloc::vec::Vec<SingleDerivativeLimitOrderbook>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleDerivativeLimitOrderbook {
    /// market's ID
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Orderbook of the market
    #[prost(message, optional, tag="2")]
    pub orderbook: ::core::option::Option<DerivativeLimitOrderbook>,
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
    pub orderbooks: ::prost::alloc::vec::Vec<SingleDerivativeLimitOrderbookV2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleDerivativeLimitOrderbookV2 {
    /// market's ID
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Orderbook of the market
    #[prost(message, optional, tag="2")]
    pub orderbook: ::core::option::Option<DerivativeLimitOrderbookV2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookRequest {
    /// List of market IDs for orderbook streaming, empty means 'ALL' derivative
    /// markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookResponse {
    /// Orderbook of a Derivative Market
    #[prost(message, optional, tag="1")]
    pub orderbook: ::core::option::Option<DerivativeLimitOrderbook>,
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
    /// List of market IDs for orderbook streaming, empty means 'ALL' derivative
    /// markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookV2Response {
    /// Orderbook of a Derivative Market
    #[prost(message, optional, tag="1")]
    pub orderbook: ::core::option::Option<DerivativeLimitOrderbookV2>,
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
    /// List of market IDs for orderbook streaming, empty means 'ALL' derivative
    /// markets
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrderbookUpdateResponse {
    /// Orderbook level updates of a Derivative Market
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
    /// Limit is used to specify the maximum number of items to be returned
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
    /// Only search for conditional/non-conditional orders
    #[prost(string, tag="9")]
    pub is_conditional: ::prost::alloc::string::String,
    /// Search for specific order type
    #[prost(string, tag="10")]
    pub order_type: ::prost::alloc::string::String,
    /// Should include inactive orders
    #[prost(bool, tag="11")]
    pub include_inactive: bool,
    /// Choose to return subaccount total orders
    #[prost(bool, tag="12")]
    pub subaccount_total_orders: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdersResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeLimitOrder {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="2")]
    pub order_side: ::prost::alloc::string::String,
    /// Derivative Market ID
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// True if the order is a reduce-only order
    #[prost(bool, tag="5")]
    pub is_reduce_only: bool,
    /// Margin of the order
    #[prost(string, tag="6")]
    pub margin: ::prost::alloc::string::String,
    /// Price of the order
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    /// Quantity of the order
    #[prost(string, tag="8")]
    pub quantity: ::prost::alloc::string::String,
    /// The amount of the quantity remaining unfilled
    #[prost(string, tag="9")]
    pub unfilled_quantity: ::prost::alloc::string::String,
    /// Trigger price is the trigger price used by stop/take orders
    #[prost(string, tag="10")]
    pub trigger_price: ::prost::alloc::string::String,
    /// Fee recipient address
    #[prost(string, tag="11")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// Order state
    #[prost(string, tag="12")]
    pub state: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="13")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="14")]
    pub updated_at: i64,
    /// Order number of subaccount
    #[prost(sint64, tag="15")]
    pub order_number: i64,
    /// Order type
    #[prost(string, tag="16")]
    pub order_type: ::prost::alloc::string::String,
    /// Order type
    #[prost(bool, tag="17")]
    pub is_conditional: bool,
    /// Trigger timestamp, only exists for conditional orders
    #[prost(uint64, tag="18")]
    pub trigger_at: u64,
    /// OrderHash of order that is triggered by this conditional order
    #[prost(string, tag="19")]
    pub placed_order_hash: ::prost::alloc::string::String,
    /// Execution type of conditional order
    #[prost(string, tag="20")]
    pub execution_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsRequest {
    /// SubaccountId of the trader we want to get the positions from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// MarketId of the position we want to fetch
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="3")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned
    #[prost(sint32, tag="4")]
    pub limit: i32,
    /// The starting timestamp in UNIX milliseconds that the trades must be equal or
    /// older than
    #[prost(sint64, tag="5")]
    pub start_time: i64,
    /// The ending timestamp in UNIX milliseconds that the trades must be equal or
    /// younger than
    #[prost(sint64, tag="6")]
    pub end_time: i64,
    /// MarketIds of the markets of which we want to get trades
    #[prost(string, repeated, tag="7")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// filter by direction of the position
    #[prost(string, tag="8")]
    pub direction: ::prost::alloc::string::String,
    /// Choose to return subaccount total positions
    #[prost(bool, tag="9")]
    pub subaccount_total_positions: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsResponse {
    #[prost(message, repeated, tag="1")]
    pub positions: ::prost::alloc::vec::Vec<DerivativePosition>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativePosition {
    /// Ticker of the derivative market
    #[prost(string, tag="1")]
    pub ticker: ::prost::alloc::string::String,
    /// Derivative Market ID
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// The subaccountId that the position belongs to
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Direction of the position
    #[prost(string, tag="4")]
    pub direction: ::prost::alloc::string::String,
    /// Quantity of the position
    #[prost(string, tag="5")]
    pub quantity: ::prost::alloc::string::String,
    /// Price of the position
    #[prost(string, tag="6")]
    pub entry_price: ::prost::alloc::string::String,
    /// Margin of the position
    #[prost(string, tag="7")]
    pub margin: ::prost::alloc::string::String,
    /// LiquidationPrice of the position
    #[prost(string, tag="8")]
    pub liquidation_price: ::prost::alloc::string::String,
    /// MarkPrice of the position
    #[prost(string, tag="9")]
    pub mark_price: ::prost::alloc::string::String,
    /// Aggregate Quantity of the Reduce Only orders associated with the position
    #[prost(string, tag="11")]
    pub aggregate_reduce_only_quantity: ::prost::alloc::string::String,
    /// Position updated timestamp in UNIX millis.
    #[prost(sint64, tag="12")]
    pub updated_at: i64,
    /// Position created timestamp in UNIX millis.
    #[prost(sint64, tag="13")]
    pub created_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidablePositionsRequest {
    /// Market ID to filter orders for specific market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="2")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned.
    #[prost(sint32, tag="3")]
    pub limit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidablePositionsResponse {
    /// List of derivative positions
    #[prost(message, repeated, tag="1")]
    pub positions: ::prost::alloc::vec::Vec<DerivativePosition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingPaymentsRequest {
    /// SubaccountId of the trader we want to get the positions from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// MarketId of the position we want to fetch
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="3")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned.
    #[prost(sint32, tag="4")]
    pub limit: i32,
    /// Upper bound of funding payment updatedAt
    #[prost(sint64, tag="5")]
    pub end_time: i64,
    /// Filter by market ids
    #[prost(string, repeated, tag="6")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingPaymentsResponse {
    /// List of funding payments
    #[prost(message, repeated, tag="1")]
    pub payments: ::prost::alloc::vec::Vec<FundingPayment>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingPayment {
    /// Derivative Market ID
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// The subaccountId that the position belongs to
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Amount of the funding payment
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// Timestamp of funding payment in UNIX millis
    #[prost(sint64, tag="4")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingRatesRequest {
    /// MarketId of the position we want to fetch
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="2")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned.
    #[prost(sint32, tag="3")]
    pub limit: i32,
    /// Upper bound of funding timestamp
    #[prost(sint64, tag="4")]
    pub end_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingRatesResponse {
    /// List of funding rates
    #[prost(message, repeated, tag="1")]
    pub funding_rates: ::prost::alloc::vec::Vec<FundingRate>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingRate {
    /// Derivative Market ID
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Value of the funding rate
    #[prost(string, tag="2")]
    pub rate: ::prost::alloc::string::String,
    /// Timestamp of funding rate in UNIX millis
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamPositionsRequest {
    /// SubaccountId of the trader we want to get the positions from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Backward compat single market ID of position we want to stream
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// List of market IDs of the positions we want to stream
    #[prost(string, repeated, tag="3")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Subaccount ids of traders we want to get positions
    #[prost(string, repeated, tag="4")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamPositionsResponse {
    /// Updated Derivative Position
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<DerivativePosition>,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="2")]
    pub timestamp: i64,
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
    /// Limit is used to specify the maximum number of items to be returned
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
    /// Only search for conditional/non-conditional orders
    #[prost(string, tag="9")]
    pub is_conditional: ::prost::alloc::string::String,
    /// Search for specific order type
    #[prost(string, tag="10")]
    pub order_type: ::prost::alloc::string::String,
    /// Should include inactive orders
    #[prost(bool, tag="11")]
    pub include_inactive: bool,
    /// Choose to return subaccount total orders
    #[prost(bool, tag="12")]
    pub subaccount_total_orders: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOrdersResponse {
    /// Updated market order
    #[prost(message, optional, tag="1")]
    pub order: ::core::option::Option<DerivativeLimitOrder>,
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
    /// Skip will skip the first n item from the result
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
    /// Trades of a Derivative Market
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<DerivativeTrade>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeTrade {
    /// Order hash.
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
    /// True if the trade is a liquidation
    #[prost(bool, tag="5")]
    pub is_liquidation: bool,
    /// Position Delta from the trade
    #[prost(message, optional, tag="6")]
    pub position_delta: ::core::option::Option<PositionDelta>,
    /// The payout associated with the trade
    #[prost(string, tag="7")]
    pub payout: ::prost::alloc::string::String,
    /// The fee associated with the trade
    #[prost(string, tag="8")]
    pub fee: ::prost::alloc::string::String,
    /// Timestamp of trade execution in UNIX millis
    #[prost(sint64, tag="9")]
    pub executed_at: i64,
    /// Fee recipient address
    #[prost(string, tag="10")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// A unique string that helps differentiate between trades
    #[prost(string, tag="11")]
    pub trade_id: ::prost::alloc::string::String,
    /// Trade's execution side, marker/taker
    #[prost(string, tag="12")]
    pub execution_side: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionDelta {
    /// The direction the trade
    #[prost(string, tag="1")]
    pub trade_direction: ::prost::alloc::string::String,
    /// Execution Price of the trade.
    #[prost(string, tag="2")]
    pub execution_price: ::prost::alloc::string::String,
    /// Execution Quantity of the trade.
    #[prost(string, tag="3")]
    pub execution_quantity: ::prost::alloc::string::String,
    /// Execution Margin of the trade.
    #[prost(string, tag="4")]
    pub execution_margin: ::prost::alloc::string::String,
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
    /// Skip will skip the first n item from the result
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
    /// New derivative market trade
    #[prost(message, optional, tag="1")]
    pub trade: ::core::option::Option<DerivativeTrade>,
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
    /// List of derivative orders
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
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
    /// List of derivative market trades
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<DerivativeTrade>,
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
    /// Only search for conditional/non-conditional orders
    #[prost(string, tag="9")]
    pub is_conditional: ::prost::alloc::string::String,
    /// filter by order type
    #[prost(string, tag="10")]
    pub order_type: ::prost::alloc::string::String,
    /// Filter by order state
    #[prost(string, tag="11")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="12")]
    pub execution_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdersHistoryResponse {
    /// List of historical derivative orders
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<DerivativeOrderHistory>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrderHistory {
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
    /// True if an order is reduce only
    #[prost(bool, tag="14")]
    pub is_reduce_only: bool,
    /// Order direction (order side)
    #[prost(string, tag="15")]
    pub direction: ::prost::alloc::string::String,
    /// True if this is conditional order, otherwise false
    #[prost(bool, tag="16")]
    pub is_conditional: bool,
    /// Trigger timestamp in unix milli
    #[prost(uint64, tag="17")]
    pub trigger_at: u64,
    /// Order hash placed when this triggers
    #[prost(string, tag="18")]
    pub placed_order_hash: ::prost::alloc::string::String,
    /// Order's margin
    #[prost(string, tag="19")]
    pub margin: ::prost::alloc::string::String,
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
    pub order: ::core::option::Option<DerivativeOrderHistory>,
    /// Order update type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
include!("injective_derivative_exchange_rpc.tonic.rs");
// @@protoc_insertion_point(module)