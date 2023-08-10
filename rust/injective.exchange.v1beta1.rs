// @generated
/// spot authz messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpotLimitOrderAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpotMarketOrderAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateSpotLimitOrdersAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSpotOrderAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCancelSpotOrdersAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// derivative authz messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDerivativeLimitOrderAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDerivativeMarketOrderAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateDerivativeLimitOrdersAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelDerivativeOrderAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCancelDerivativeOrdersAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// common authz message used in both spot & derivative markets
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateOrdersAuthz {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub spot_markets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub derivative_markets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// spot_market_instant_listing_fee defines the expedited fee in INJ required
    /// to create a spot market by bypassing governance
    #[prost(message, optional, tag="1")]
    pub spot_market_instant_listing_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// derivative_market_instant_listing_fee defines the expedited fee in INJ
    /// required to create a derivative market by bypassing governance
    #[prost(message, optional, tag="2")]
    pub derivative_market_instant_listing_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// default_spot_maker_fee defines the default exchange trade fee for makers on
    /// a spot market
    #[prost(string, tag="3")]
    pub default_spot_maker_fee_rate: ::prost::alloc::string::String,
    /// default_spot_taker_fee_rate defines the default exchange trade fee rate for
    /// takers on a new spot market
    #[prost(string, tag="4")]
    pub default_spot_taker_fee_rate: ::prost::alloc::string::String,
    /// default_derivative_maker_fee defines the default exchange trade fee for
    /// makers on a new derivative market
    #[prost(string, tag="5")]
    pub default_derivative_maker_fee_rate: ::prost::alloc::string::String,
    /// default_derivative_taker_fee defines the default exchange trade fee for
    /// takers on a new derivative market
    #[prost(string, tag="6")]
    pub default_derivative_taker_fee_rate: ::prost::alloc::string::String,
    /// default_initial_margin_ratio defines the default initial margin ratio on a
    /// new derivative market
    #[prost(string, tag="7")]
    pub default_initial_margin_ratio: ::prost::alloc::string::String,
    /// default_maintenance_margin_ratio defines the default maintenance margin
    /// ratio on a new derivative market
    #[prost(string, tag="8")]
    pub default_maintenance_margin_ratio: ::prost::alloc::string::String,
    /// default_funding_interval defines the default funding interval on a
    /// derivative market
    #[prost(int64, tag="9")]
    pub default_funding_interval: i64,
    /// funding_multiple defines the timestamp multiple that the funding timestamp
    /// should be a multiple of
    #[prost(int64, tag="10")]
    pub funding_multiple: i64,
    /// relayer_fee_share_rate defines the trade fee share percentage that goes to
    /// relayers
    #[prost(string, tag="11")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// default_hourly_funding_rate_cap defines the default maximum absolute value
    /// of the hourly funding rate
    #[prost(string, tag="12")]
    pub default_hourly_funding_rate_cap: ::prost::alloc::string::String,
    /// hourly_interest_rate defines the hourly interest rate
    #[prost(string, tag="13")]
    pub default_hourly_interest_rate: ::prost::alloc::string::String,
    /// max_derivative_order_side_count defines the maximum number of derivative
    /// active orders a subaccount can have for a given orderbook side
    #[prost(uint32, tag="14")]
    pub max_derivative_order_side_count: u32,
    /// inj_reward_staked_requirement_threshold defines the threshold on INJ
    /// rewards after which one also needs staked INJ to receive more
    #[prost(string, tag="15")]
    pub inj_reward_staked_requirement_threshold: ::prost::alloc::string::String,
    /// the trading_rewards_vesting_duration defines the vesting times for trading
    /// rewards
    #[prost(int64, tag="16")]
    pub trading_rewards_vesting_duration: i64,
    /// liquidator_reward_share_rate defines the ratio of the split of the surplus
    /// collateral that goes to the liquidator
    #[prost(string, tag="17")]
    pub liquidator_reward_share_rate: ::prost::alloc::string::String,
    /// binary_options_market_instant_listing_fee defines the expedited fee in INJ
    /// required to create a derivative market by bypassing governance
    #[prost(message, optional, tag="18")]
    pub binary_options_market_instant_listing_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// atomic_market_order_access_level defines the required access permissions
    /// for executing atomic market orders
    #[prost(enumeration="AtomicMarketOrderAccessLevel", tag="19")]
    pub atomic_market_order_access_level: i32,
    /// spot_atomic_market_order_fee_multiplier defines the default multiplier for
    /// executing atomic market orders in spot markets
    #[prost(string, tag="20")]
    pub spot_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// derivative_atomic_market_order_fee_multiplier defines the default
    /// multiplier for executing atomic market orders in derivative markets
    #[prost(string, tag="21")]
    pub derivative_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// binary_options_atomic_market_order_fee_multiplier defines the default
    /// multiplier for executing atomic market orders in binary markets
    #[prost(string, tag="22")]
    pub binary_options_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// minimal_protocol_fee_rate defines the minimal protocol fee rate
    #[prost(string, tag="23")]
    pub minimal_protocol_fee_rate: ::prost::alloc::string::String,
    /// is_instant_derivative_market_launch_enabled defines whether instant
    /// derivative market launch is enabled
    #[prost(bool, tag="24")]
    pub is_instant_derivative_market_launch_enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketFeeMultiplier {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub fee_multiplier: ::prost::alloc::string::String,
}
/// An object describing a derivative market in the Injective Futures Protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarket {
    /// Ticker for the derivative contract.
    #[prost(string, tag="1")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="2")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="3")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="4")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="5")]
    pub oracle_scale_factor: u32,
    /// Address of the quote currency denomination for the derivative contract
    #[prost(string, tag="6")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag="7")]
    pub market_id: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio of a derivative
    /// market
    #[prost(string, tag="8")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio of a
    /// derivative market
    #[prost(string, tag="9")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the maker fee rate of a derivative market
    #[prost(string, tag="10")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the taker fee rate of a derivative market
    #[prost(string, tag="11")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag="12")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// true if the market is a perpetual market. false if the market is an expiry
    /// futures market
    #[prost(bool, tag="13")]
    pub is_perpetual: bool,
    /// Status of the market
    #[prost(enumeration="MarketStatus", tag="14")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag="15")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="16")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// An object describing a binary options market in Injective Protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarket {
    /// Ticker for the derivative contract.
    #[prost(string, tag="1")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag="2")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle Provider
    #[prost(string, tag="3")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="4")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="5")]
    pub oracle_scale_factor: u32,
    /// expiration timestamp
    #[prost(int64, tag="6")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag="7")]
    pub settlement_timestamp: i64,
    /// admin of the market
    #[prost(string, tag="8")]
    pub admin: ::prost::alloc::string::String,
    /// Address of the quote currency denomination for the binary options contract
    #[prost(string, tag="9")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag="10")]
    pub market_id: ::prost::alloc::string::String,
    /// maker_fee_rate defines the maker fee rate of a binary options market
    #[prost(string, tag="11")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the taker fee rate of a derivative market
    #[prost(string, tag="12")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag="13")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// Status of the market
    #[prost(enumeration="MarketStatus", tag="14")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag="15")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="16")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub settlement_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiryFuturesMarketInfo {
    /// market ID.
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// expiration_timestamp defines the expiration time for a time expiry futures
    /// market.
    #[prost(int64, tag="2")]
    pub expiration_timestamp: i64,
    /// expiration_twap_start_timestamp defines the start time of the TWAP
    /// calculation window
    #[prost(int64, tag="3")]
    pub twap_start_timestamp: i64,
    /// expiration_twap_start_price_cumulative defines the cumulative price for the
    /// start of the TWAP window
    #[prost(string, tag="4")]
    pub expiration_twap_start_price_cumulative: ::prost::alloc::string::String,
    /// settlement_price defines the settlement price for a time expiry futures
    /// market.
    #[prost(string, tag="5")]
    pub settlement_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketInfo {
    /// market ID.
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// hourly_funding_rate_cap defines the maximum absolute value of the hourly
    /// funding rate
    #[prost(string, tag="2")]
    pub hourly_funding_rate_cap: ::prost::alloc::string::String,
    /// hourly_interest_rate defines the hourly interest rate
    #[prost(string, tag="3")]
    pub hourly_interest_rate: ::prost::alloc::string::String,
    /// next_funding_timestamp defines the next funding timestamp in seconds of a
    /// perpetual market
    #[prost(int64, tag="4")]
    pub next_funding_timestamp: i64,
    /// funding_interval defines the next funding interval in seconds of a
    /// perpetual market.
    #[prost(int64, tag="5")]
    pub funding_interval: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketFunding {
    /// cumulative_funding defines the cumulative funding of a perpetual market.
    #[prost(string, tag="1")]
    pub cumulative_funding: ::prost::alloc::string::String,
    /// cumulative_price defines the cumulative price for the current hour up to
    /// the last timestamp
    #[prost(string, tag="2")]
    pub cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub last_timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketSettlementInfo {
    /// market ID.
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// settlement_price defines the settlement price
    #[prost(string, tag="2")]
    pub settlement_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextFundingTimestamp {
    #[prost(int64, tag="1")]
    pub next_timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MidPriceAndTob {
    /// mid price of the market
    #[prost(string, tag="1")]
    pub mid_price: ::prost::alloc::string::String,
    /// best buy price of the market
    #[prost(string, tag="2")]
    pub best_buy_price: ::prost::alloc::string::String,
    /// best sell price of the market
    #[prost(string, tag="3")]
    pub best_sell_price: ::prost::alloc::string::String,
}
/// An object describing trade pair of two assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarket {
    /// A name of the pair in format AAA/BBB, where AAA is base asset, BBB is quote
    /// asset.
    #[prost(string, tag="1")]
    pub ticker: ::prost::alloc::string::String,
    /// Coin denom used for the base asset
    #[prost(string, tag="2")]
    pub base_denom: ::prost::alloc::string::String,
    /// Coin used for the quote asset
    #[prost(string, tag="3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// maker_fee_rate defines the fee percentage makers pay when trading
    #[prost(string, tag="4")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the fee percentage takers pay when trading
    #[prost(string, tag="5")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag="6")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag="7")]
    pub market_id: ::prost::alloc::string::String,
    /// Status of the market
    #[prost(enumeration="MarketStatus", tag="8")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price required
    /// for orders in the market
    #[prost(string, tag="9")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="10")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// A subaccount's deposit for a given base currency
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposit {
    #[prost(string, tag="1")]
    pub available_balance: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub total_balance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountTradeNonce {
    #[prost(uint32, tag="1")]
    pub nonce: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderInfo {
    /// bytes32 subaccount ID that created the order
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// address fee_recipient address that will receive fees for the order
    #[prost(string, tag="2")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// price of the order
    #[prost(string, tag="3")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag="4")]
    pub quantity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrder {
    /// market_id represents the unique ID of the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// order_info contains the information of the order
    #[prost(message, optional, tag="2")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="3")]
    pub order_type: i32,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="4")]
    pub trigger_price: ::prost::alloc::string::String,
}
/// A valid Spot limit order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotLimitOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag="1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="2")]
    pub order_type: i32,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag="3")]
    pub fillable: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="4")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
/// A valid Spot market order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag="1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    #[prost(string, tag="2")]
    pub balance_hold: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    /// order types
    #[prost(enumeration="OrderType", tag="4")]
    pub order_type: i32,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="5")]
    pub trigger_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrder {
    /// market_id represents the unique ID of the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// order_info contains the information of the order
    #[prost(message, optional, tag="2")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="3")]
    pub order_type: i32,
    /// margin is the margin used by the limit order
    #[prost(string, tag="4")]
    pub margin: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="5")]
    pub trigger_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderbookMetadata {
    #[prost(uint32, tag="1")]
    pub vanilla_limit_order_count: u32,
    #[prost(uint32, tag="2")]
    pub reduce_only_limit_order_count: u32,
    /// AggregateReduceOnlyQuantity is the aggregate fillable quantity of the
    /// subaccount's reduce-only limit orders in the given direction.
    #[prost(string, tag="3")]
    pub aggregate_reduce_only_quantity: ::prost::alloc::string::String,
    /// AggregateVanillaQuantity is the aggregate fillable quantity of the
    /// subaccount's vanilla limit orders in the given direction.
    #[prost(string, tag="4")]
    pub aggregate_vanilla_quantity: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub vanilla_conditional_order_count: u32,
    #[prost(uint32, tag="6")]
    pub reduce_only_conditional_order_count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrder {
    /// price of the order
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_reduce_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderData {
    #[prost(message, optional, tag="1")]
    pub order: ::core::option::Option<SubaccountOrder>,
    #[prost(bytes="vec", tag="2")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
/// A valid Derivative limit order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeLimitOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag="1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="2")]
    pub order_type: i32,
    /// margin is the margin used by the limit order
    #[prost(string, tag="3")]
    pub margin: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag="4")]
    pub fillable: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="5")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
/// A valid Derivative market order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag="1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="2")]
    pub order_type: i32,
    #[prost(string, tag="3")]
    pub margin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub margin_hold: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="5")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(bool, tag="1")]
    pub is_long: bool,
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub entry_price: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub margin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub cumulative_funding_entry: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderIndicator {
    /// market_id represents the unique ID of the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_buy: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeLog {
    #[prost(string, tag="1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    /// bytes32 subaccount ID that executed the trade
    #[prost(bytes="vec", tag="3")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub fee: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub fee_recipient_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionDelta {
    #[prost(bool, tag="1")]
    pub is_long: bool,
    #[prost(string, tag="2")]
    pub execution_quantity: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub execution_margin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub execution_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeTradeLog {
    #[prost(bytes="vec", tag="1")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub position_delta: ::core::option::Option<PositionDelta>,
    #[prost(string, tag="3")]
    pub payout: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub fee: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub fee_recipient_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountPosition {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    #[prost(bytes="vec", tag="2")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountDeposit {
    #[prost(bytes="vec", tag="1")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub deposit: ::core::option::Option<Deposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositUpdate {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub deposits: ::prost::alloc::vec::Vec<SubaccountDeposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointsMultiplier {
    #[prost(string, tag="1")]
    pub maker_points_multiplier: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub taker_points_multiplier: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignBoostInfo {
    #[prost(string, repeated, tag="1")]
    pub boosted_spot_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub spot_market_multipliers: ::prost::alloc::vec::Vec<PointsMultiplier>,
    #[prost(string, repeated, tag="3")]
    pub boosted_derivative_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub derivative_market_multipliers: ::prost::alloc::vec::Vec<PointsMultiplier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignRewardPool {
    #[prost(int64, tag="1")]
    pub start_timestamp: i64,
    /// max_campaign_rewards are the maximum reward amounts to be disbursed at the
    /// end of the campaign
    #[prost(message, repeated, tag="2")]
    pub max_campaign_rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignInfo {
    /// number of seconds of the duration of each campaign
    #[prost(int64, tag="1")]
    pub campaign_duration_seconds: i64,
    /// the trading fee quote denoms which will be counted for the rewards
    #[prost(string, repeated, tag="2")]
    pub quote_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the optional boost info for markets
    #[prost(message, optional, tag="3")]
    pub trading_reward_boost_info: ::core::option::Option<TradingRewardCampaignBoostInfo>,
    /// the marketIDs which are disqualified from being rewarded
    #[prost(string, repeated, tag="4")]
    pub disqualified_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountTierInfo {
    #[prost(string, tag="1")]
    pub maker_discount_rate: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub taker_discount_rate: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub staked_amount: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub volume: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountSchedule {
    #[prost(uint64, tag="1")]
    pub bucket_count: u64,
    #[prost(int64, tag="2")]
    pub bucket_duration: i64,
    /// the trading fee quote denoms which will be counted for the fee paid
    /// contribution
    #[prost(string, repeated, tag="3")]
    pub quote_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the fee discount tiers
    #[prost(message, repeated, tag="4")]
    pub tier_infos: ::prost::alloc::vec::Vec<FeeDiscountTierInfo>,
    /// the marketIDs which are disqualified from contributing to the fee paid
    /// amount
    #[prost(string, repeated, tag="5")]
    pub disqualified_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountTierTtl {
    #[prost(uint64, tag="1")]
    pub tier: u64,
    #[prost(int64, tag="2")]
    pub ttl_timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeRecord {
    #[prost(string, tag="1")]
    pub maker_volume: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub taker_volume: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRewards {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeRecords {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub latest_trade_records: ::prost::alloc::vec::Vec<TradeRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountIDs {
    #[prost(bytes="vec", repeated, tag="1")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeRecord {
    #[prost(int64, tag="1")]
    pub timestamp: i64,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub quantity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Level {
    /// price
    #[prost(string, tag="1")]
    pub p: ::prost::alloc::string::String,
    /// quantity
    #[prost(string, tag="2")]
    pub q: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateSubaccountVolumeRecord {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateAccountVolumeRecord {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketVolume {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub volume: ::core::option::Option<VolumeRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomDecimals {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub decimals: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AtomicMarketOrderAccessLevel {
    Nobody = 0,
    /// currently unsupported
    BeginBlockerSmartContractsOnly = 1,
    SmartContractsOnly = 2,
    Everyone = 3,
}
impl AtomicMarketOrderAccessLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AtomicMarketOrderAccessLevel::Nobody => "Nobody",
            AtomicMarketOrderAccessLevel::BeginBlockerSmartContractsOnly => "BeginBlockerSmartContractsOnly",
            AtomicMarketOrderAccessLevel::SmartContractsOnly => "SmartContractsOnly",
            AtomicMarketOrderAccessLevel::Everyone => "Everyone",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Nobody" => Some(Self::Nobody),
            "BeginBlockerSmartContractsOnly" => Some(Self::BeginBlockerSmartContractsOnly),
            "SmartContractsOnly" => Some(Self::SmartContractsOnly),
            "Everyone" => Some(Self::Everyone),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarketStatus {
    Unspecified = 0,
    Active = 1,
    Paused = 2,
    Demolished = 3,
    Expired = 4,
}
impl MarketStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarketStatus::Unspecified => "Unspecified",
            MarketStatus::Active => "Active",
            MarketStatus::Paused => "Paused",
            MarketStatus::Demolished => "Demolished",
            MarketStatus::Expired => "Expired",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Active" => Some(Self::Active),
            "Paused" => Some(Self::Paused),
            "Demolished" => Some(Self::Demolished),
            "Expired" => Some(Self::Expired),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Unspecified = 0,
    Buy = 1,
    Sell = 2,
    StopBuy = 3,
    StopSell = 4,
    TakeBuy = 5,
    TakeSell = 6,
    BuyPo = 7,
    SellPo = 8,
    BuyAtomic = 9,
    SellAtomic = 10,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "UNSPECIFIED",
            OrderType::Buy => "BUY",
            OrderType::Sell => "SELL",
            OrderType::StopBuy => "STOP_BUY",
            OrderType::StopSell => "STOP_SELL",
            OrderType::TakeBuy => "TAKE_BUY",
            OrderType::TakeSell => "TAKE_SELL",
            OrderType::BuyPo => "BUY_PO",
            OrderType::SellPo => "SELL_PO",
            OrderType::BuyAtomic => "BUY_ATOMIC",
            OrderType::SellAtomic => "SELL_ATOMIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "BUY" => Some(Self::Buy),
            "SELL" => Some(Self::Sell),
            "STOP_BUY" => Some(Self::StopBuy),
            "STOP_SELL" => Some(Self::StopSell),
            "TAKE_BUY" => Some(Self::TakeBuy),
            "TAKE_SELL" => Some(Self::TakeSell),
            "BUY_PO" => Some(Self::BuyPo),
            "SELL_PO" => Some(Self::SellPo),
            "BUY_ATOMIC" => Some(Self::BuyAtomic),
            "SELL_ATOMIC" => Some(Self::SellAtomic),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionType {
    UnspecifiedExecutionType = 0,
    Market = 1,
    LimitFill = 2,
    LimitMatchRestingOrder = 3,
    LimitMatchNewOrder = 4,
    MarketLiquidation = 5,
    ExpiryMarketSettlement = 6,
}
impl ExecutionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionType::UnspecifiedExecutionType => "UnspecifiedExecutionType",
            ExecutionType::Market => "Market",
            ExecutionType::LimitFill => "LimitFill",
            ExecutionType::LimitMatchRestingOrder => "LimitMatchRestingOrder",
            ExecutionType::LimitMatchNewOrder => "LimitMatchNewOrder",
            ExecutionType::MarketLiquidation => "MarketLiquidation",
            ExecutionType::ExpiryMarketSettlement => "ExpiryMarketSettlement",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnspecifiedExecutionType" => Some(Self::UnspecifiedExecutionType),
            "Market" => Some(Self::Market),
            "LimitFill" => Some(Self::LimitFill),
            "LimitMatchRestingOrder" => Some(Self::LimitMatchRestingOrder),
            "LimitMatchNewOrder" => Some(Self::LimitMatchNewOrder),
            "MarketLiquidation" => Some(Self::MarketLiquidation),
            "ExpiryMarketSettlement" => Some(Self::ExpiryMarketSettlement),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderMask {
    Unused = 0,
    Any = 1,
    Regular = 2,
    Conditional = 4,
    /// for conditional orders means HIGHER
    DirectionBuyOrHigher = 8,
    /// for conditional orders means LOWER
    DirectionSellOrLower = 16,
    TypeMarket = 32,
    TypeLimit = 64,
}
impl OrderMask {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderMask::Unused => "UNUSED",
            OrderMask::Any => "ANY",
            OrderMask::Regular => "REGULAR",
            OrderMask::Conditional => "CONDITIONAL",
            OrderMask::DirectionBuyOrHigher => "DIRECTION_BUY_OR_HIGHER",
            OrderMask::DirectionSellOrLower => "DIRECTION_SELL_OR_LOWER",
            OrderMask::TypeMarket => "TYPE_MARKET",
            OrderMask::TypeLimit => "TYPE_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNUSED" => Some(Self::Unused),
            "ANY" => Some(Self::Any),
            "REGULAR" => Some(Self::Regular),
            "CONDITIONAL" => Some(Self::Conditional),
            "DIRECTION_BUY_OR_HIGHER" => Some(Self::DirectionBuyOrHigher),
            "DIRECTION_SELL_OR_LOWER" => Some(Self::DirectionSellOrLower),
            "TYPE_MARKET" => Some(Self::TypeMarket),
            "TYPE_LIMIT" => Some(Self::TypeLimit),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBatchSpotExecution {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_buy: bool,
    #[prost(enumeration="ExecutionType", tag="3")]
    pub execution_type: i32,
    #[prost(message, repeated, tag="4")]
    pub trades: ::prost::alloc::vec::Vec<TradeLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBatchDerivativeExecution {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_buy: bool,
    #[prost(bool, tag="3")]
    pub is_liquidation: bool,
    /// nil for time expiry futures
    #[prost(string, tag="4")]
    pub cumulative_funding: ::prost::alloc::string::String,
    #[prost(enumeration="ExecutionType", tag="5")]
    pub execution_type: i32,
    #[prost(message, repeated, tag="6")]
    pub trades: ::prost::alloc::vec::Vec<DerivativeTradeLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLostFundsFromLiquidation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub lost_funds_from_available_during_payout: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub lost_funds_from_order_cancels: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBatchDerivativePosition {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub positions: ::prost::alloc::vec::Vec<SubaccountPosition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDerivativeMarketPaused {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub settle_price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub total_missing_funds: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub missing_funds_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarketBeyondBankruptcy {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub settle_price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub missing_market_funds: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAllPositionsHaircut {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub settle_price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub missing_funds_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBinaryOptionsMarketUpdate {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<BinaryOptionsMarket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNewSpotOrders {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub buy_orders: ::prost::alloc::vec::Vec<SpotLimitOrder>,
    #[prost(message, repeated, tag="3")]
    pub sell_orders: ::prost::alloc::vec::Vec<SpotLimitOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNewDerivativeOrders {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub buy_orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
    #[prost(message, repeated, tag="3")]
    pub sell_orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelSpotOrder {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<SpotLimitOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSpotMarketUpdate {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<SpotMarket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventPerpetualMarketUpdate {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<DerivativeMarket>,
    #[prost(message, optional, tag="2")]
    pub perpetual_market_info: ::core::option::Option<PerpetualMarketInfo>,
    #[prost(message, optional, tag="3")]
    pub funding: ::core::option::Option<PerpetualMarketFunding>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventExpiryFuturesMarketUpdate {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<DerivativeMarket>,
    #[prost(message, optional, tag="3")]
    pub expiry_futures_market_info: ::core::option::Option<ExpiryFuturesMarketInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventPerpetualMarketFundingUpdate {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub funding: ::core::option::Option<PerpetualMarketFunding>,
    #[prost(bool, tag="3")]
    pub is_hourly_funding: bool,
    #[prost(string, tag="4")]
    pub funding_rate: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub mark_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubaccountDeposit {
    #[prost(string, tag="1")]
    pub src_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubaccountWithdraw {
    #[prost(bytes="vec", tag="1")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub dst_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubaccountBalanceTransfer {
    #[prost(string, tag="1")]
    pub src_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub dst_subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBatchDepositUpdate {
    #[prost(message, repeated, tag="1")]
    pub deposit_updates: ::prost::alloc::vec::Vec<DepositUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketOrderCancel {
    #[prost(message, optional, tag="1")]
    pub market_order: ::core::option::Option<DerivativeMarketOrder>,
    #[prost(string, tag="2")]
    pub cancel_quantity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelDerivativeOrder {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_limit_cancel: bool,
    #[prost(message, optional, tag="3")]
    pub limit_order: ::core::option::Option<DerivativeLimitOrder>,
    #[prost(message, optional, tag="4")]
    pub market_order_cancel: ::core::option::Option<DerivativeMarketOrderCancel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFeeDiscountSchedule {
    #[prost(message, optional, tag="1")]
    pub schedule: ::core::option::Option<FeeDiscountSchedule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTradingRewardCampaignUpdate {
    #[prost(message, optional, tag="1")]
    pub campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    #[prost(message, repeated, tag="2")]
    pub campaign_reward_pools: ::prost::alloc::vec::Vec<CampaignRewardPool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTradingRewardDistribution {
    #[prost(message, repeated, tag="1")]
    pub account_rewards: ::prost::alloc::vec::Vec<AccountRewards>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNewConditionalDerivativeOrder {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
    #[prost(bytes="vec", tag="3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="4")]
    pub is_market: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelConditionalDerivativeOrder {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_limit_cancel: bool,
    #[prost(message, optional, tag="3")]
    pub limit_order: ::core::option::Option<DerivativeLimitOrder>,
    #[prost(message, optional, tag="4")]
    pub market_order: ::core::option::Option<DerivativeMarketOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventConditionalDerivativeOrderTrigger {
    #[prost(bytes="vec", tag="1")]
    pub market_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="2")]
    pub is_limit_trigger: bool,
    #[prost(bytes="vec", tag="3")]
    pub triggered_order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub placed_order_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOrderFail {
    #[prost(bytes="vec", tag="1")]
    pub account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, tag="3")]
    pub flags: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAtomicMarketOrderFeeMultipliersUpdated {
    #[prost(message, repeated, tag="1")]
    pub market_fee_multipliers: ::prost::alloc::vec::Vec<MarketFeeMultiplier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOrderbookUpdate {
    #[prost(message, repeated, tag="1")]
    pub spot_updates: ::prost::alloc::vec::Vec<OrderbookUpdate>,
    #[prost(message, repeated, tag="2")]
    pub derivative_updates: ::prost::alloc::vec::Vec<OrderbookUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookUpdate {
    #[prost(uint64, tag="1")]
    pub seq: u64,
    #[prost(message, optional, tag="2")]
    pub orderbook: ::core::option::Option<Orderbook>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orderbook {
    #[prost(bytes="vec", tag="1")]
    pub market_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub buy_levels: ::prost::alloc::vec::Vec<Level>,
    #[prost(message, repeated, tag="3")]
    pub sell_levels: ::prost::alloc::vec::Vec<Level>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the exchange parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
/// MsgDeposit defines a SDK message for transferring coins from the sender's
/// bank balance into the subaccount's exchange deposits
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeposit {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// (Optional) bytes32 subaccount ID to deposit funds into. If empty, the coin
    /// will be deposited to the sender's default subaccount address.
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDepositResponse defines the Msg/Deposit response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {
}
/// MsgWithdraw defines a SDK message for withdrawing coins from a subaccount's
/// deposits to the user's bank balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdraw {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// bytes32 subaccount ID to withdraw funds from
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdraw defines the Msg/Withdraw response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {
}
/// MsgCreateSpotLimitOrder defines a SDK message for creating a new spot limit
/// order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotLimitOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<SpotOrder>,
}
/// MsgCreateSpotLimitOrderResponse defines the Msg/CreateSpotOrder response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotLimitOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
}
/// MsgBatchCreateSpotLimitOrders defines a SDK message for creating a new batch
/// of spot limit orders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateSpotLimitOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub orders: ::prost::alloc::vec::Vec<SpotOrder>,
}
/// MsgBatchCreateSpotLimitOrdersResponse defines the
/// Msg/BatchCreateSpotLimitOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateSpotLimitOrdersResponse {
    #[prost(string, repeated, tag="1")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgInstantSpotMarketLaunch defines a SDK message for creating a new spot
/// market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantSpotMarketLaunch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the spot market.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag="3")]
    pub base_denom: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag="4")]
    pub quote_denom: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price
    #[prost(string, tag="5")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="6")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// MsgInstantSpotMarketLaunchResponse defines the Msg/InstantSpotMarketLaunch
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantSpotMarketLaunchResponse {
}
/// MsgInstantPerpetualMarketLaunch defines a SDK message for creating a new
/// perpetual futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantPerpetualMarketLaunch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag="3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="6")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="7")]
    pub oracle_type: i32,
    /// maker_fee_rate defines the trade fee rate for makers on the perpetual
    /// market
    #[prost(string, tag="8")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the perpetual
    /// market
    #[prost(string, tag="9")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio for the perpetual
    /// market
    #[prost(string, tag="10")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// perpetual market
    #[prost(string, tag="11")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="12")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="13")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// MsgInstantPerpetualMarketLaunchResponse defines the
/// Msg/InstantPerpetualMarketLaunchResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantPerpetualMarketLaunchResponse {
}
/// MsgInstantBinaryOptionsMarketLaunch defines a SDK message for creating a new
/// perpetual futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantBinaryOptionsMarketLaunch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative contract.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag="3")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle Provider
    #[prost(string, tag="4")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="5")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="6")]
    pub oracle_scale_factor: u32,
    /// maker_fee_rate defines the trade fee rate for makers on the perpetual
    /// market
    #[prost(string, tag="7")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the perpetual
    /// market
    #[prost(string, tag="8")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// expiration timestamp
    #[prost(int64, tag="9")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag="10")]
    pub settlement_timestamp: i64,
    /// admin of the market
    #[prost(string, tag="11")]
    pub admin: ::prost::alloc::string::String,
    /// Address of the quote currency denomination for the binary options contract
    #[prost(string, tag="12")]
    pub quote_denom: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag="13")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="14")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// MsgInstantBinaryOptionsMarketLaunchResponse defines the
/// Msg/InstantBinaryOptionsMarketLaunchResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantBinaryOptionsMarketLaunchResponse {
}
/// MsgInstantExpiryFuturesMarketLaunch defines a SDK message for creating a new
/// expiry futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantExpiryFuturesMarketLaunch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag="3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="6")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="7")]
    pub oracle_scale_factor: u32,
    /// Expiration time of the market
    #[prost(int64, tag="8")]
    pub expiry: i64,
    /// maker_fee_rate defines the trade fee rate for makers on the expiry futures
    /// market
    #[prost(string, tag="9")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the expiry futures
    /// market
    #[prost(string, tag="10")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag="11")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag="12")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="13")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="14")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// MsgInstantExpiryFuturesMarketLaunchResponse defines the
/// Msg/InstantExpiryFuturesMarketLaunch response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantExpiryFuturesMarketLaunchResponse {
}
/// MsgCreateSpotMarketOrder defines a SDK message for creating a new spot market
/// order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotMarketOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<SpotOrder>,
}
/// MsgCreateSpotMarketOrderResponse defines the Msg/CreateSpotMarketLimitOrder
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotMarketOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub results: ::core::option::Option<SpotMarketOrderResults>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketOrderResults {
    #[prost(string, tag="1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub fee: ::prost::alloc::string::String,
}
/// A Cosmos-SDK MsgCreateDerivativeLimitOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeLimitOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgCreateDerivativeLimitOrderResponse defines the
/// Msg/CreateDerivativeMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeLimitOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
}
/// A Cosmos-SDK MsgCreateBinaryOptionsLimitOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsLimitOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgCreateBinaryOptionsLimitOrderResponse defines the
/// Msg/CreateBinaryOptionsLimitOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsLimitOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
}
/// A Cosmos-SDK MsgBatchCreateDerivativeLimitOrders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateDerivativeLimitOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub orders: ::prost::alloc::vec::Vec<DerivativeOrder>,
}
/// MsgBatchCreateDerivativeLimitOrdersResponse defines the
/// Msg/BatchCreateDerivativeLimitOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateDerivativeLimitOrdersResponse {
    #[prost(string, repeated, tag="1")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgCancelSpotOrder defines the Msg/CancelSpotOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSpotOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub order_hash: ::prost::alloc::string::String,
}
/// MsgCancelSpotOrderResponse defines the Msg/CancelSpotOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSpotOrderResponse {
}
/// MsgBatchCancelSpotOrders defines the Msg/BatchCancelSpotOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelSpotOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
/// MsgBatchCancelSpotOrdersResponse defines the Msg/BatchCancelSpotOrders
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelSpotOrdersResponse {
    #[prost(bool, repeated, tag="1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
/// MsgBatchCancelBinaryOptionsOrders defines the
/// Msg/BatchCancelBinaryOptionsOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelBinaryOptionsOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
/// BatchCancelBinaryOptionsOrdersResponse defines the
/// Msg/BatchCancelBinaryOptionsOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelBinaryOptionsOrdersResponse {
    #[prost(bool, repeated, tag="1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
/// MsgBatchUpdateOrders defines the Msg/BatchUpdateOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchUpdateOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// subaccount_id only used for the spot_market_ids_to_cancel_all and
    /// derivative_market_ids_to_cancel_all.
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub spot_market_ids_to_cancel_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub derivative_market_ids_to_cancel_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="5")]
    pub spot_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(message, repeated, tag="6")]
    pub derivative_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(message, repeated, tag="7")]
    pub spot_orders_to_create: ::prost::alloc::vec::Vec<SpotOrder>,
    #[prost(message, repeated, tag="8")]
    pub derivative_orders_to_create: ::prost::alloc::vec::Vec<DerivativeOrder>,
    #[prost(message, repeated, tag="9")]
    pub binary_options_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(string, repeated, tag="10")]
    pub binary_options_market_ids_to_cancel_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="11")]
    pub binary_options_orders_to_create: ::prost::alloc::vec::Vec<DerivativeOrder>,
}
/// MsgBatchUpdateOrdersResponse defines the Msg/BatchUpdateOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchUpdateOrdersResponse {
    #[prost(bool, repeated, tag="1")]
    pub spot_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(bool, repeated, tag="2")]
    pub derivative_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag="3")]
    pub spot_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub derivative_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, repeated, tag="5")]
    pub binary_options_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag="6")]
    pub binary_options_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Cosmos-SDK MsgCreateDerivativeMarketOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeMarketOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgCreateDerivativeMarketOrderResponse defines the
/// Msg/CreateDerivativeMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeMarketOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub results: ::core::option::Option<DerivativeMarketOrderResults>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketOrderResults {
    #[prost(string, tag="1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub position_delta: ::core::option::Option<PositionDelta>,
    #[prost(string, tag="5")]
    pub payout: ::prost::alloc::string::String,
}
/// A Cosmos-SDK MsgCreateBinaryOptionsMarketOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsMarketOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgCreateBinaryOptionsMarketOrderResponse defines the
/// Msg/CreateBinaryOptionsMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsMarketOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub results: ::core::option::Option<DerivativeMarketOrderResults>,
}
/// MsgCancelDerivativeOrder defines the Msg/CancelDerivativeOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelDerivativeOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag="5")]
    pub order_mask: i32,
}
/// MsgCancelDerivativeOrderResponse defines the
/// Msg/CancelDerivativeOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelDerivativeOrderResponse {
}
/// MsgCancelBinaryOptionsOrder defines the Msg/CancelBinaryOptionsOrder response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelBinaryOptionsOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag="5")]
    pub order_mask: i32,
}
/// MsgCancelBinaryOptionsOrderResponse defines the
/// Msg/CancelBinaryOptionsOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelBinaryOptionsOrderResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderData {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag="4")]
    pub order_mask: i32,
}
/// MsgBatchCancelDerivativeOrders defines the Msg/CancelDerivativeOrders
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelDerivativeOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
/// MsgBatchCancelDerivativeOrdersResponse defines the
/// Msg/CancelDerivativeOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelDerivativeOrdersResponse {
    #[prost(bool, repeated, tag="1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
/// A Cosmos-SDK MsgSubaccountTransfer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubaccountTransfer {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSubaccountTransferResponse defines the Msg/SubaccountTransfer response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubaccountTransferResponse {
}
/// A Cosmos-SDK MsgExternalTransfer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExternalTransfer {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgExternalTransferResponse defines the Msg/ExternalTransfer response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExternalTransferResponse {
}
/// A Cosmos-SDK MsgLiquidatePosition
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidatePosition {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// optional order to provide for liquidation
    #[prost(message, optional, tag="4")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgLiquidatePositionResponse defines the Msg/LiquidatePosition response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidatePositionResponse {
}
/// A Cosmos-SDK MsgIncreasePositionMargin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncreasePositionMargin {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub market_id: ::prost::alloc::string::String,
    /// amount defines the amount of margin to add to the position
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgIncreasePositionMarginResponse defines the Msg/IncreasePositionMargin
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncreasePositionMarginResponse {
}
/// MsgPrivilegedExecuteContract defines the Msg/Exec message type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPrivilegedExecuteContract {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// funds defines the user's bank coins used to fund the execution (e.g.
    /// 100inj).
    #[prost(string, tag="2")]
    pub funds: ::prost::alloc::string::String,
    /// contract_address defines the contract address to execute
    #[prost(string, tag="3")]
    pub contract_address: ::prost::alloc::string::String,
    /// data defines the call data used when executing the contract
    #[prost(string, tag="4")]
    pub data: ::prost::alloc::string::String,
}
/// MsgPrivilegedExecuteContractResponse defines the Msg/Exec response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPrivilegedExecuteContractResponse {
    #[prost(message, repeated, tag="1")]
    pub funds_diff: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketParamUpdateProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// maker_fee_rate defines the trade fee rate for makers on the spot market
    #[prost(string, tag="4")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the spot market
    #[prost(string, tag="5")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the relayer fee share rate for the spot
    /// market
    #[prost(string, tag="6")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="7")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="8")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    #[prost(enumeration="MarketStatus", tag="9")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeEnableProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration="ExchangeType", tag="3")]
    pub exchange_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchExchangeModificationProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub spot_market_param_update_proposals: ::prost::alloc::vec::Vec<SpotMarketParamUpdateProposal>,
    #[prost(message, repeated, tag="4")]
    pub derivative_market_param_update_proposals: ::prost::alloc::vec::Vec<DerivativeMarketParamUpdateProposal>,
    #[prost(message, repeated, tag="5")]
    pub spot_market_launch_proposals: ::prost::alloc::vec::Vec<SpotMarketLaunchProposal>,
    #[prost(message, repeated, tag="6")]
    pub perpetual_market_launch_proposals: ::prost::alloc::vec::Vec<PerpetualMarketLaunchProposal>,
    #[prost(message, repeated, tag="7")]
    pub expiry_futures_market_launch_proposals: ::prost::alloc::vec::Vec<ExpiryFuturesMarketLaunchProposal>,
    #[prost(message, optional, tag="8")]
    pub trading_reward_campaign_update_proposal: ::core::option::Option<TradingRewardCampaignUpdateProposal>,
    #[prost(message, repeated, tag="9")]
    pub binary_options_market_launch_proposals: ::prost::alloc::vec::Vec<BinaryOptionsMarketLaunchProposal>,
    #[prost(message, repeated, tag="10")]
    pub binary_options_param_update_proposals: ::prost::alloc::vec::Vec<BinaryOptionsMarketParamUpdateProposal>,
    #[prost(message, optional, tag="11")]
    pub denom_decimals_update_proposal: ::core::option::Option<UpdateDenomDecimalsProposal>,
}
/// SpotMarketLaunchProposal defines a SDK message for proposing a new spot
/// market through governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketLaunchProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Ticker for the spot market.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag="4")]
    pub base_denom: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag="5")]
    pub quote_denom: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price
    #[prost(string, tag="6")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="7")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// maker_fee_rate defines the fee percentage makers pay when trading
    #[prost(string, tag="8")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the fee percentage takers pay when trading
    #[prost(string, tag="9")]
    pub taker_fee_rate: ::prost::alloc::string::String,
}
/// PerpetualMarketLaunchProposal defines a SDK message for proposing a new
/// perpetual futures market through governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketLaunchProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag="4")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="5")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="6")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="7")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="8")]
    pub oracle_type: i32,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag="9")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag="10")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the exchange trade fee for makers for the derivative
    /// market
    #[prost(string, tag="11")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the exchange trade fee for takers for the derivative
    /// market
    #[prost(string, tag="12")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="13")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="14")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarketLaunchProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Ticker for the derivative contract.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag="4")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle Provider
    #[prost(string, tag="5")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="6")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="7")]
    pub oracle_scale_factor: u32,
    /// expiration timestamp
    #[prost(int64, tag="8")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag="9")]
    pub settlement_timestamp: i64,
    /// admin of the market
    #[prost(string, tag="10")]
    pub admin: ::prost::alloc::string::String,
    /// Address of the quote currency denomination for the binary options contract
    #[prost(string, tag="11")]
    pub quote_denom: ::prost::alloc::string::String,
    /// maker_fee_rate defines the maker fee rate of a binary options market
    #[prost(string, tag="12")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the taker fee rate of a derivative market
    #[prost(string, tag="13")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag="14")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="15")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// ExpiryFuturesMarketLaunchProposal defines a SDK message for proposing a new
/// expiry futures market through governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiryFuturesMarketLaunchProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag="4")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="5")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="6")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="7")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="8")]
    pub oracle_type: i32,
    /// Expiration time of the market
    #[prost(int64, tag="9")]
    pub expiry: i64,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag="10")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag="11")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the exchange trade fee for makers for the derivative
    /// market
    #[prost(string, tag="12")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the exchange trade fee for takers for the derivative
    /// market
    #[prost(string, tag="13")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="14")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="15")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketParamUpdateProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag="4")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag="5")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the exchange trade fee for makers for the derivative
    /// market
    #[prost(string, tag="6")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the exchange trade fee for takers for the derivative
    /// market
    #[prost(string, tag="7")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the relayer fee share rate for the
    /// derivative market
    #[prost(string, tag="8")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="9")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="10")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// hourly_interest_rate defines the hourly interest rate
    #[prost(string, tag="11")]
    pub hourly_interest_rate: ::prost::alloc::string::String,
    /// hourly_funding_rate_cap defines the maximum absolute value of the hourly
    /// funding rate
    #[prost(string, tag="12")]
    pub hourly_funding_rate_cap: ::prost::alloc::string::String,
    #[prost(enumeration="MarketStatus", tag="13")]
    pub status: i32,
    #[prost(message, optional, tag="14")]
    pub oracle_params: ::core::option::Option<OracleParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketForcedSettlementProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub settlement_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDenomDecimalsProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub denom_decimals: ::prost::alloc::vec::Vec<DenomDecimals>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarketParamUpdateProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// maker_fee_rate defines the exchange trade fee for makers for the derivative
    /// market
    #[prost(string, tag="4")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the exchange trade fee for takers for the derivative
    /// market
    #[prost(string, tag="5")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the relayer fee share rate for the
    /// derivative market
    #[prost(string, tag="6")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="7")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="8")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// expiration timestamp
    #[prost(int64, tag="9")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag="10")]
    pub settlement_timestamp: i64,
    /// new price at which market will be settled
    #[prost(string, tag="11")]
    pub settlement_price: ::prost::alloc::string::String,
    /// admin of the market
    #[prost(string, tag="12")]
    pub admin: ::prost::alloc::string::String,
    #[prost(enumeration="MarketStatus", tag="13")]
    pub status: i32,
    #[prost(message, optional, tag="14")]
    pub oracle_params: ::core::option::Option<ProviderOracleParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderOracleParams {
    /// Oracle base currency
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="2")]
    pub provider: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="3")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="4")]
    pub oracle_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleParams {
    /// Oracle base currency
    #[prost(string, tag="1")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="2")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="3")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="4")]
    pub oracle_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignLaunchProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    #[prost(message, repeated, tag="4")]
    pub campaign_reward_pools: ::prost::alloc::vec::Vec<CampaignRewardPool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignUpdateProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    #[prost(message, repeated, tag="4")]
    pub campaign_reward_pools_additions: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    #[prost(message, repeated, tag="5")]
    pub campaign_reward_pools_updates: ::prost::alloc::vec::Vec<CampaignRewardPool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardPointUpdate {
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// new_points overwrites the current trading reward points for the account
    #[prost(string, tag="12")]
    pub new_points: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardPendingPointsUpdateProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub pending_pool_timestamp: i64,
    #[prost(message, repeated, tag="4")]
    pub reward_point_updates: ::prost::alloc::vec::Vec<RewardPointUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub schedule: ::core::option::Option<FeeDiscountSchedule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommunityPoolSpendProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub proposals: ::prost::alloc::vec::Vec<super::super::super::cosmos::distribution::v1beta1::CommunityPoolSpendProposal>,
}
/// A Cosmos-SDK MsgRewardsOptOut
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRewardsOptOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgRewardsOptOutResponse defines the Msg/RewardsOptOut response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRewardsOptOutResponse {
}
/// A Cosmos-SDK MsgReclaimLockedFunds
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReclaimLockedFunds {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub locked_account_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// MsgReclaimLockedFundsResponse defines the Msg/ReclaimLockedFunds response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReclaimLockedFundsResponse {
}
/// MsgSignData defines an arbitrary, general-purpose, off-chain message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignData {
    /// Signer is the sdk.AccAddress of the message signer
    #[prost(bytes="vec", tag="1")]
    pub signer: ::prost::alloc::vec::Vec<u8>,
    /// Data represents the raw bytes of the content that is signed (text, json,
    /// etc)
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgSignDoc defines an arbitrary, general-purpose, off-chain message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignDoc {
    #[prost(string, tag="1")]
    pub sign_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<MsgSignData>,
}
/// MsgAdminUpdateBinaryOptionsMarket is used by the market Admin to operate the
/// market
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAdminUpdateBinaryOptionsMarket {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// new price at which market will be settled
    #[prost(string, tag="3")]
    pub settlement_price: ::prost::alloc::string::String,
    /// expiration timestamp
    #[prost(int64, tag="4")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag="5")]
    pub settlement_timestamp: i64,
    /// Status of the market
    #[prost(enumeration="MarketStatus", tag="6")]
    pub status: i32,
}
/// MsgAdminUpdateBinaryOptionsMarketResponse is the response for
/// AdminUpdateBinaryOptionsMarket rpc method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAdminUpdateBinaryOptionsMarketResponse {
}
/// AtomicMarketOrderFeeMultiplierScheduleProposal defines a SDK message for
/// proposing new atomic take fee multipliers for specified markets
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtomicMarketOrderFeeMultiplierScheduleProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub market_fee_multipliers: ::prost::alloc::vec::Vec<MarketFeeMultiplier>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExchangeType {
    ExchangeUnspecified = 0,
    Spot = 1,
    Derivatives = 2,
}
impl ExchangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExchangeType::ExchangeUnspecified => "EXCHANGE_UNSPECIFIED",
            ExchangeType::Spot => "SPOT",
            ExchangeType::Derivatives => "DERIVATIVES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXCHANGE_UNSPECIFIED" => Some(Self::ExchangeUnspecified),
            "SPOT" => Some(Self::Spot),
            "DERIVATIVES" => Some(Self::Derivatives),
            _ => None,
        }
    }
}
/// GenesisState defines the exchange module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of related to exchange.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// spot_markets is an array containing the genesis trade pairs
    #[prost(message, repeated, tag="2")]
    pub spot_markets: ::prost::alloc::vec::Vec<SpotMarket>,
    /// derivative_markets is an array containing the genesis derivative markets
    #[prost(message, repeated, tag="3")]
    pub derivative_markets: ::prost::alloc::vec::Vec<DerivativeMarket>,
    /// spot_orderbook defines the spot exchange limit orderbook active at genesis.
    #[prost(message, repeated, tag="4")]
    pub spot_orderbook: ::prost::alloc::vec::Vec<SpotOrderBook>,
    /// derivative_orderbook defines the derivative exchange limit orderbook active
    /// at genesis.
    #[prost(message, repeated, tag="5")]
    pub derivative_orderbook: ::prost::alloc::vec::Vec<DerivativeOrderBook>,
    /// balances defines the exchange users balances active at genesis.
    #[prost(message, repeated, tag="6")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
    /// positions defines the exchange derivative positions at genesis
    #[prost(message, repeated, tag="7")]
    pub positions: ::prost::alloc::vec::Vec<DerivativePosition>,
    /// subaccount_trade_nonces defines the subaccount trade nonces for the
    /// subaccounts at genesis
    #[prost(message, repeated, tag="8")]
    pub subaccount_trade_nonces: ::prost::alloc::vec::Vec<SubaccountNonce>,
    /// expiry_futures_market_info defines the market info for the expiry futures
    /// markets at genesis
    #[prost(message, repeated, tag="9")]
    pub expiry_futures_market_info_state: ::prost::alloc::vec::Vec<ExpiryFuturesMarketInfoState>,
    /// perpetual_market_info defines the market info for the perpetual derivative
    /// markets at genesis
    #[prost(message, repeated, tag="10")]
    pub perpetual_market_info: ::prost::alloc::vec::Vec<PerpetualMarketInfo>,
    /// perpetual_market_funding_state defines the funding state for the perpetual
    /// derivative markets at genesis
    #[prost(message, repeated, tag="11")]
    pub perpetual_market_funding_state: ::prost::alloc::vec::Vec<PerpetualMarketFundingState>,
    /// derivative_market_settlement_scheduled defines the scheduled markets for
    /// settlement at genesis
    #[prost(message, repeated, tag="12")]
    pub derivative_market_settlement_scheduled: ::prost::alloc::vec::Vec<DerivativeMarketSettlementInfo>,
    /// sets spot markets as enabled
    #[prost(bool, tag="13")]
    pub is_spot_exchange_enabled: bool,
    /// sets derivative markets as enabled
    #[prost(bool, tag="14")]
    pub is_derivatives_exchange_enabled: bool,
    /// the current trading reward campaign info
    #[prost(message, optional, tag="15")]
    pub trading_reward_campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    /// the current and upcoming trading reward campaign pools
    #[prost(message, repeated, tag="16")]
    pub trading_reward_pool_campaign_schedule: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    /// the current trading reward account points
    #[prost(message, repeated, tag="17")]
    pub trading_reward_campaign_account_points: ::prost::alloc::vec::Vec<TradingRewardCampaignAccountPoints>,
    /// the fee discount schedule
    #[prost(message, optional, tag="18")]
    pub fee_discount_schedule: ::core::option::Option<FeeDiscountSchedule>,
    /// the cached fee discount account tiers with TTL
    #[prost(message, repeated, tag="19")]
    pub fee_discount_account_tier_ttl: ::prost::alloc::vec::Vec<FeeDiscountAccountTierTtl>,
    /// the fee discount paid by accounts in all buckets
    #[prost(message, repeated, tag="20")]
    pub fee_discount_bucket_volume_accounts: ::prost::alloc::vec::Vec<FeeDiscountBucketVolumeAccounts>,
    /// sets the first fee cycle as finished
    #[prost(bool, tag="21")]
    pub is_first_fee_cycle_finished: bool,
    /// the current and upcoming trading reward campaign pending pools
    #[prost(message, repeated, tag="22")]
    pub pending_trading_reward_pool_campaign_schedule: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    /// the pending trading reward account points
    #[prost(message, repeated, tag="23")]
    pub pending_trading_reward_campaign_account_points: ::prost::alloc::vec::Vec<TradingRewardCampaignAccountPendingPoints>,
    /// the addresses opting out of trading rewards
    #[prost(string, repeated, tag="24")]
    pub rewards_opt_out_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="25")]
    pub historical_trade_records: ::prost::alloc::vec::Vec<TradeRecords>,
    /// binary_options_markets is an array containing the genesis binary options
    /// markets
    #[prost(message, repeated, tag="26")]
    pub binary_options_markets: ::prost::alloc::vec::Vec<BinaryOptionsMarket>,
    /// binary_options_markets_scheduled_for_settlement contains the marketIDs of
    /// binary options markets scheduled for next-block settlement
    #[prost(string, repeated, tag="27")]
    pub binary_options_market_ids_scheduled_for_settlement: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// spot_market_ids_scheduled_to_force_close defines the scheduled markets for
    /// forced closings at genesis
    #[prost(string, repeated, tag="28")]
    pub spot_market_ids_scheduled_to_force_close: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// denom_decimals defines the denom decimals for the exchange.
    #[prost(message, repeated, tag="29")]
    pub denom_decimals: ::prost::alloc::vec::Vec<DenomDecimals>,
    /// conditional_derivative_orderbook contains conditional orderbooks for all
    /// markets (both lmit and market conditional orders)
    #[prost(message, repeated, tag="30")]
    pub conditional_derivative_orderbooks: ::prost::alloc::vec::Vec<ConditionalDerivativeOrderBook>,
    /// market_fee_multipliers contains any non-default atomic order fee
    /// multipliers
    #[prost(message, repeated, tag="31")]
    pub market_fee_multipliers: ::prost::alloc::vec::Vec<MarketFeeMultiplier>,
    #[prost(message, repeated, tag="32")]
    pub orderbook_sequences: ::prost::alloc::vec::Vec<OrderbookSequence>,
    #[prost(message, repeated, tag="33")]
    pub subaccount_volumes: ::prost::alloc::vec::Vec<AggregateSubaccountVolumeRecord>,
    #[prost(message, repeated, tag="34")]
    pub market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookSequence {
    #[prost(uint64, tag="1")]
    pub sequence: u64,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountAccountTierTtl {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub tier_ttl: ::core::option::Option<FeeDiscountTierTtl>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountBucketVolumeAccounts {
    #[prost(int64, tag="1")]
    pub bucket_start_timestamp: i64,
    #[prost(message, repeated, tag="2")]
    pub account_volume: ::prost::alloc::vec::Vec<AccountVolume>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountVolume {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub volume: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignAccountPoints {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub points: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignAccountPendingPoints {
    #[prost(int64, tag="1")]
    pub reward_pool_start_timestamp: i64,
    #[prost(message, repeated, tag="2")]
    pub account_points: ::prost::alloc::vec::Vec<TradingRewardCampaignAccountPoints>,
}
/// Spot Exchange Limit Orderbook
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrderBook {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_buy_side: bool,
    #[prost(message, repeated, tag="3")]
    pub orders: ::prost::alloc::vec::Vec<SpotLimitOrder>,
}
/// Derivative Exchange Limit Orderbook
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrderBook {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_buy_side: bool,
    #[prost(message, repeated, tag="3")]
    pub orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
}
/// Orderbook containing limit & market conditional orders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionalDerivativeOrderBook {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub limit_buy_orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
    #[prost(message, repeated, tag="3")]
    pub market_buy_orders: ::prost::alloc::vec::Vec<DerivativeMarketOrder>,
    #[prost(message, repeated, tag="4")]
    pub limit_sell_orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
    #[prost(message, repeated, tag="5")]
    pub market_sell_orders: ::prost::alloc::vec::Vec<DerivativeMarketOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub deposits: ::core::option::Option<Deposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativePosition {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Position>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountNonce {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub subaccount_trade_nonce: ::core::option::Option<SubaccountTradeNonce>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiryFuturesMarketInfoState {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub market_info: ::core::option::Option<ExpiryFuturesMarketInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketFundingState {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub funding: ::core::option::Option<PerpetualMarketFunding>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subaccount {
    #[prost(string, tag="1")]
    pub trader: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub subaccount_nonce: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountOrdersRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountOrdersResponse {
    #[prost(message, repeated, tag="1")]
    pub buy_orders: ::prost::alloc::vec::Vec<SubaccountOrderData>,
    #[prost(message, repeated, tag="2")]
    pub sell_orders: ::prost::alloc::vec::Vec<SubaccountOrderData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderbookMetadataWithMarket {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<SubaccountOrderbookMetadata>,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_buy: bool,
}
/// QueryExchangeParamsRequest is the request type for the Query/ExchangeParams
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeParamsRequest {
}
/// QueryExchangeParamsRequest is the response type for the Query/ExchangeParams
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QuerySubaccountDepositsRequest is the request type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountDepositsRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub subaccount: ::core::option::Option<Subaccount>,
}
/// QuerySubaccountDepositsResponse is the response type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountDepositsResponse {
    #[prost(map="string, message", tag="1")]
    pub deposits: ::std::collections::HashMap<::prost::alloc::string::String, Deposit>,
}
/// QueryExchangeBalancesRequest is the request type for the
/// Query/ExchangeBalances RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeBalancesRequest {
}
/// QuerySubaccountDepositsResponse is the response type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeBalancesResponse {
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
}
/// QueryAggregateVolumeRequest is the request type for the Query/AggregateVolume
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVolumeRequest {
    /// can either be an address or a subaccount
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
}
/// QueryAggregateVolumeResponse is the response type for the
/// Query/AggregateVolume RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVolumeResponse {
    /// if an address is specified, then the aggregate_volumes will aggregate the
    /// volumes across all subaccounts for the address
    #[prost(message, repeated, tag="1")]
    pub aggregate_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
/// QueryAggregateVolumesRequest is the request type for the
/// Query/AggregateVolumes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVolumesRequest {
    #[prost(string, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryAggregateVolumesResponse is the response type for the
/// Query/AggregateVolumes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVolumesResponse {
    /// the aggregate volume records for the accounts specified
    #[prost(message, repeated, tag="1")]
    pub aggregate_account_volumes: ::prost::alloc::vec::Vec<AggregateAccountVolumeRecord>,
    /// the aggregate volumes for the markets specified
    #[prost(message, repeated, tag="2")]
    pub aggregate_market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
/// QueryAggregateMarketVolumeRequest is the request type for the
/// Query/AggregateMarketVolume RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateMarketVolumeRequest {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QueryAggregateMarketVolumeResponse is the response type for the
/// Query/AggregateMarketVolume RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateMarketVolumeResponse {
    #[prost(message, optional, tag="1")]
    pub volume: ::core::option::Option<VolumeRecord>,
}
/// QueryDenomDecimalRequest is the request type for the Query/DenomDecimal RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomDecimalRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomDecimalResponse is the response type for the Query/DenomDecimal RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomDecimalResponse {
    #[prost(uint64, tag="1")]
    pub decimal: u64,
}
/// QueryDenomDecimalsRequest is the request type for the Query/DenomDecimals RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomDecimalsRequest {
    /// denoms can be empty to query all denom decimals
    #[prost(string, repeated, tag="1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryDenomDecimalsRequest is the response type for the Query/DenomDecimals
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomDecimalsResponse {
    #[prost(message, repeated, tag="1")]
    pub denom_decimals: ::prost::alloc::vec::Vec<DenomDecimals>,
}
/// QueryAggregateMarketVolumesRequest is the request type for the
/// Query/AggregateMarketVolumes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateMarketVolumesRequest {
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryAggregateMarketVolumesResponse is the response type for the
/// Query/AggregateMarketVolumes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateMarketVolumesResponse {
    /// the aggregate volumes for the entire market
    #[prost(message, repeated, tag="1")]
    pub volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
/// QuerySubaccountDepositsRequest is the request type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountDepositRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
/// QuerySubaccountDepositsResponse is the response type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountDepositResponse {
    #[prost(message, optional, tag="1")]
    pub deposits: ::core::option::Option<Deposit>,
}
/// QuerySpotMarketsRequest is the request type for the Query/SpotMarkets RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMarketsRequest {
    /// Status of the market, for convenience it is set to string - not enum
    #[prost(string, tag="1")]
    pub status: ::prost::alloc::string::String,
    /// Filter by market IDs
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QuerySpotMarketsResponse is the response type for the Query/SpotMarkets RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMarketsResponse {
    #[prost(message, repeated, tag="1")]
    pub markets: ::prost::alloc::vec::Vec<SpotMarket>,
}
/// QuerySpotMarketRequest is the request type for the Query/SpotMarket RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMarketRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QuerySpotMarketResponse is the response type for the Query/SpotMarket RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMarketResponse {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<SpotMarket>,
}
/// QuerySpotOrderbookRequest is the request type for the Query/SpotOrderbook RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotOrderbookRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub limit: u64,
    #[prost(enumeration="OrderSide", tag="3")]
    pub order_side: i32,
    #[prost(string, tag="4")]
    pub limit_cumulative_notional: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub limit_cumulative_quantity: ::prost::alloc::string::String,
}
/// QuerySpotOrderbookResponse is the response type for the Query/SpotOrderbook
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotOrderbookResponse {
    #[prost(message, repeated, tag="1")]
    pub buys_price_level: ::prost::alloc::vec::Vec<Level>,
    #[prost(message, repeated, tag="2")]
    pub sells_price_level: ::prost::alloc::vec::Vec<Level>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullSpotMarket {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<SpotMarket>,
    /// mid_price_and_tob defines the mid price for this market and the best ask
    /// and bid orders
    #[prost(message, optional, tag="2")]
    pub mid_price_and_tob: ::core::option::Option<MidPriceAndTob>,
}
/// QueryFullSpotMarketsRequest is the request type for the Query/FullSpotMarkets
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFullSpotMarketsRequest {
    /// Status of the market, for convenience it is set to string - not enum
    #[prost(string, tag="1")]
    pub status: ::prost::alloc::string::String,
    /// Filter by market IDs
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Flag to return the markets mid price and top of the book buy and sell
    /// orders.
    #[prost(bool, tag="3")]
    pub with_mid_price_and_tob: bool,
}
/// QueryFullSpotMarketsResponse is the response type for the
/// Query/FullSpotMarkets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFullSpotMarketsResponse {
    #[prost(message, repeated, tag="1")]
    pub markets: ::prost::alloc::vec::Vec<FullSpotMarket>,
}
/// QuerySpotMarketRequest is the request type for the Query/SpotMarket RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFullSpotMarketRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Flag to return the markets mid price and top of the book buy and sell
    /// orders.
    #[prost(bool, tag="2")]
    pub with_mid_price_and_tob: bool,
}
/// QuerySpotMarketResponse is the response type for the Query/SpotMarket RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFullSpotMarketResponse {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<FullSpotMarket>,
}
/// QuerySpotOrdersByHashesRequest is the request type for the
/// Query/SpotOrdersByHashes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotOrdersByHashesRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// the order hashes
    #[prost(string, repeated, tag="3")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QuerySpotOrdersByHashesResponse is the response type for the
/// Query/SpotOrdersByHashes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotOrdersByHashesResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedSpotLimitOrder>,
}
/// QueryTraderSpotOrdersRequest is the request type for the
/// Query/TraderSpotOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderSpotOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
}
/// QueryAccountAddressSpotOrdersRequest is the request type for the
/// Query/AccountAddressSpotOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressSpotOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Account address of the trader
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrimmedSpotLimitOrder {
    /// price of the order
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag="3")]
    pub fillable: ::prost::alloc::string::String,
    /// true if the order is a buy
    #[prost(bool, tag="4")]
    pub is_buy: bool,
    #[prost(string, tag="5")]
    pub order_hash: ::prost::alloc::string::String,
}
/// QueryTraderSpotOrdersResponse is the response type for the
/// Query/TraderSpotOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderSpotOrdersResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedSpotLimitOrder>,
}
/// QueryAccountAddressSpotOrdersResponse is the response type for the
/// Query/AccountAddressSpotOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressSpotOrdersResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedSpotLimitOrder>,
}
/// QuerySpotMidPriceAndTOBRequest is the request type for the
/// Query/SpotMidPriceAndTOB RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMidPriceAndTobRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QuerySpotMidPriceAndTOBResponse is the response type for the
/// Query/SpotMidPriceAndTOB RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMidPriceAndTobResponse {
    /// mid price of the market
    #[prost(string, tag="1")]
    pub mid_price: ::prost::alloc::string::String,
    /// best buy price of the market
    #[prost(string, tag="2")]
    pub best_buy_price: ::prost::alloc::string::String,
    /// best sell price of the market
    #[prost(string, tag="3")]
    pub best_sell_price: ::prost::alloc::string::String,
}
/// QueryDerivativeMidPriceAndTOBRequest is the request type for the
/// Query/GetDerivativeMidPriceAndTOB RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMidPriceAndTobRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QueryDerivativeMidPriceAndTOBResponse is the response type for the
/// Query/GetDerivativeMidPriceAndTOB RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMidPriceAndTobResponse {
    /// mid price of the market
    #[prost(string, tag="1")]
    pub mid_price: ::prost::alloc::string::String,
    /// best buy price of the market
    #[prost(string, tag="2")]
    pub best_buy_price: ::prost::alloc::string::String,
    /// best sell price of the market
    #[prost(string, tag="3")]
    pub best_sell_price: ::prost::alloc::string::String,
}
/// QueryDerivativeOrderbookRequest is the request type for the
/// Query/DerivativeOrderbook RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeOrderbookRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub limit: u64,
    #[prost(string, tag="3")]
    pub limit_cumulative_notional: ::prost::alloc::string::String,
}
/// QueryDerivativeOrderbookResponse is the response type for the
/// Query/DerivativeOrderbook RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeOrderbookResponse {
    #[prost(message, repeated, tag="1")]
    pub buys_price_level: ::prost::alloc::vec::Vec<Level>,
    #[prost(message, repeated, tag="2")]
    pub sells_price_level: ::prost::alloc::vec::Vec<Level>,
}
/// QueryTraderSpotOrdersToCancelUpToAmountRequest is the request type for the
/// Query/TraderSpotOrdersToCancelUpToAmountRequest RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderSpotOrdersToCancelUpToAmountRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// the base amount to cancel (free up)
    #[prost(string, tag="3")]
    pub base_amount: ::prost::alloc::string::String,
    /// the quote amount to cancel (free up)
    #[prost(string, tag="4")]
    pub quote_amount: ::prost::alloc::string::String,
    /// The cancellation strategy
    #[prost(enumeration="CancellationStrategy", tag="5")]
    pub strategy: i32,
    /// The reference price for the cancellation strategy, e.g. mid price or mark
    /// price
    #[prost(string, tag="6")]
    pub reference_price: ::prost::alloc::string::String,
}
/// QueryTraderDerivativeOrdersToCancelUpToAmountRequest is the request type for
/// the Query/TraderDerivativeOrdersToCancelUpToAmountRequest RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeOrdersToCancelUpToAmountRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// the quote amount to cancel (free up)
    #[prost(string, tag="3")]
    pub quote_amount: ::prost::alloc::string::String,
    /// The cancellation strategy
    #[prost(enumeration="CancellationStrategy", tag="4")]
    pub strategy: i32,
    /// The reference price for the cancellation strategy, e.g. mid price or mark
    /// price
    #[prost(string, tag="5")]
    pub reference_price: ::prost::alloc::string::String,
}
/// QueryTraderDerivativeOrdersRequest is the request type for the
/// Query/TraderDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
}
/// QueryAccountAddressSpotOrdersRequest is the request type for the
/// Query/AccountAddressDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressDerivativeOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Account address of the trader
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrimmedDerivativeLimitOrder {
    /// price of the order
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    /// margin of the order
    #[prost(string, tag="3")]
    pub margin: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag="4")]
    pub fillable: ::prost::alloc::string::String,
    /// true if the order is a buy
    ///
    /// ensure omitempty is not in jsontag
    #[prost(bool, tag="5")]
    pub is_buy: bool,
    #[prost(string, tag="6")]
    pub order_hash: ::prost::alloc::string::String,
}
/// QueryTraderDerivativeOrdersResponse is the response type for the
/// Query/TraderDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeOrdersResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedDerivativeLimitOrder>,
}
/// QueryAccountAddressDerivativeOrdersResponse is the response type for the
/// Query/AccountAddressDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressDerivativeOrdersResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedDerivativeLimitOrder>,
}
/// QueryTraderDerivativeOrdersRequest is the request type for the
/// Query/TraderDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeOrdersByHashesRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// the order hashes
    #[prost(string, repeated, tag="3")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryDerivativeOrdersByHashesResponse is the response type for the
/// Query/DerivativeOrdersByHashes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeOrdersByHashesResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedDerivativeLimitOrder>,
}
/// QueryDerivativeMarketsRequest is the request type for the
/// Query/DerivativeMarkets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketsRequest {
    /// Status of the market, for convenience it is set to string - not enum
    #[prost(string, tag="1")]
    pub status: ::prost::alloc::string::String,
    /// Filter by market IDs
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Flag to return the markets mid price and top of the book buy and sell
    /// orders.
    #[prost(bool, tag="3")]
    pub with_mid_price_and_tob: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceLevel {
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// quantity
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketState {
    #[prost(message, optional, tag="1")]
    pub market_info: ::core::option::Option<PerpetualMarketInfo>,
    #[prost(message, optional, tag="2")]
    pub funding_info: ::core::option::Option<PerpetualMarketFunding>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullDerivativeMarket {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<DerivativeMarket>,
    #[prost(string, tag="4")]
    pub mark_price: ::prost::alloc::string::String,
    /// mid_price_and_tob defines the mid price for this market and the best ask
    /// and bid orders
    #[prost(message, optional, tag="5")]
    pub mid_price_and_tob: ::core::option::Option<MidPriceAndTob>,
    #[prost(oneof="full_derivative_market::Info", tags="2, 3")]
    pub info: ::core::option::Option<full_derivative_market::Info>,
}
/// Nested message and enum types in `FullDerivativeMarket`.
pub mod full_derivative_market {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Info {
        #[prost(message, tag="2")]
        PerpetualInfo(super::PerpetualMarketState),
        #[prost(message, tag="3")]
        FuturesInfo(super::ExpiryFuturesMarketInfo),
    }
}
/// QueryDerivativeMarketsResponse is the response type for the
/// Query/DerivativeMarkets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketsResponse {
    #[prost(message, repeated, tag="1")]
    pub markets: ::prost::alloc::vec::Vec<FullDerivativeMarket>,
}
/// QueryDerivativeMarketRequest is the request type for the
/// Query/DerivativeMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QueryDerivativeMarketResponse is the response type for the
/// Query/DerivativeMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketResponse {
    #[prost(message, optional, tag="1")]
    pub market: ::core::option::Option<FullDerivativeMarket>,
}
/// QueryDerivativeMarketAddressRequest is the request type for the
/// Query/DerivativeMarketAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketAddressRequest {
    /// Market ID for the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QueryDerivativeMarketAddressResponse is the response type for the
/// Query/DerivativeMarketAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketAddressResponse {
    /// address for the market
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// subaccountID for the market
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
}
/// QuerySubaccountTradeNonceRequest is the request type for the
/// Query/SubaccountTradeNonce RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountTradeNonceRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
}
/// QuerySubaccountPositionsRequest is the request type for the
/// Query/SubaccountPositions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountPositionsRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
}
/// QuerySubaccountPositionInMarketRequest is the request type for the
/// Query/SubaccountPositionInMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountPositionInMarketRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
}
/// QuerySubaccountEffectivePositionInMarketRequest is the request type for the
/// Query/SubaccountEffectivePositionInMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountEffectivePositionInMarketRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
}
/// QuerySubaccountOrderMetadataRequest is the request type for the
/// Query/SubaccountOrderMetadata RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountOrderMetadataRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
}
/// QuerySubaccountPositionsResponse is the response type for the
/// Query/SubaccountPositions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountPositionsResponse {
    #[prost(message, repeated, tag="1")]
    pub state: ::prost::alloc::vec::Vec<DerivativePosition>,
}
/// QuerySubaccountPositionInMarketResponse is the response type for the
/// Query/SubaccountPositionInMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountPositionInMarketResponse {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<Position>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectivePosition {
    #[prost(bool, tag="1")]
    pub is_long: bool,
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub entry_price: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub effective_margin: ::prost::alloc::string::String,
}
/// QuerySubaccountEffectivePositionInMarketResponse is the response type for the
/// Query/SubaccountEffectivePositionInMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountEffectivePositionInMarketResponse {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<EffectivePosition>,
}
/// QueryPerpetualMarketInfoRequest is the request type for the
/// Query/PerpetualMarketInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPerpetualMarketInfoRequest {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QueryPerpetualMarketInfoResponse is the response type for the
/// Query/PerpetualMarketInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPerpetualMarketInfoResponse {
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<PerpetualMarketInfo>,
}
/// QueryExpiryFuturesMarketInfoRequest is the request type for the Query/
/// ExpiryFuturesMarketInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExpiryFuturesMarketInfoRequest {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QueryExpiryFuturesMarketInfoResponse is the response type for the Query/
/// ExpiryFuturesMarketInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExpiryFuturesMarketInfoResponse {
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<ExpiryFuturesMarketInfo>,
}
/// QueryPerpetualMarketFundingRequest is the request type for the
/// Query/PerpetualMarketFunding RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPerpetualMarketFundingRequest {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
/// QueryPerpetualMarketFundingResponse is the response type for the
/// Query/PerpetualMarketFunding RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPerpetualMarketFundingResponse {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<PerpetualMarketFunding>,
}
/// QuerySubaccountOrderMetadataResponse is the response type for the
/// Query/SubaccountOrderMetadata RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountOrderMetadataResponse {
    #[prost(message, repeated, tag="1")]
    pub metadata: ::prost::alloc::vec::Vec<SubaccountOrderbookMetadataWithMarket>,
}
/// QuerySubaccountTradeNonceResponse is the response type for the
/// Query/SubaccountTradeNonce RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountTradeNonceResponse {
    #[prost(uint32, tag="1")]
    pub nonce: u32,
}
/// QueryModuleStateRequest is the request type for the Query/ExchangeModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {
}
/// QueryModuleStateResponse is the response type for the
/// Query/ExchangeModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<GenesisState>,
}
/// QueryPositionsRequest is the request type for the Query/Positions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPositionsRequest {
}
/// QueryPositionsResponse is the response type for the Query/Positions RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPositionsResponse {
    #[prost(message, repeated, tag="1")]
    pub state: ::prost::alloc::vec::Vec<DerivativePosition>,
}
/// QueryTradeRewardPointsRequest is the request type for the
/// Query/TradeRewardPoints RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTradeRewardPointsRequest {
    #[prost(string, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag="2")]
    pub pending_pool_timestamp: i64,
}
/// QueryTradeRewardPointsResponse is the response type for the
/// Query/TradeRewardPoints RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTradeRewardPointsResponse {
    #[prost(string, repeated, tag="1")]
    pub account_trade_reward_points: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryTradeRewardCampaignRequest is the request type for the
/// Query/TradeRewardCampaign RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTradeRewardCampaignRequest {
}
/// QueryTradeRewardCampaignResponse is the response type for the
/// Query/TradeRewardCampaign RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTradeRewardCampaignResponse {
    #[prost(message, optional, tag="1")]
    pub trading_reward_campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    #[prost(message, repeated, tag="2")]
    pub trading_reward_pool_campaign_schedule: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    #[prost(string, tag="3")]
    pub total_trade_reward_points: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub pending_trading_reward_pool_campaign_schedule: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    #[prost(string, repeated, tag="5")]
    pub pending_total_trade_reward_points: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryIsRegisteredDMMRequest is the request type for the Query/IsRegisteredDMM
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsOptedOutOfRewardsRequest {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
}
/// QueryIsRegisteredDMMResponse is the response type for the
/// Query/IsRegisteredDMM RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsOptedOutOfRewardsResponse {
    #[prost(bool, tag="1")]
    pub is_opted_out: bool,
}
/// QueryRegisteredDMMsRequest is the request type for the Query/RegisteredDMMs
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOptedOutOfRewardsAccountsRequest {
}
/// QueryRegisteredDMMsResponse is the response type for the Query/RegisteredDMMs
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOptedOutOfRewardsAccountsResponse {
    #[prost(string, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryFeeDiscountAccountInfoRequest is the request type for the
/// Query/FeeDiscountAccountInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountAccountInfoRequest {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
}
/// QueryFeeDiscountAccountInfoResponse is the response type for the
/// Query/FeeDiscountAccountInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountAccountInfoResponse {
    #[prost(uint64, tag="1")]
    pub tier_level: u64,
    #[prost(message, optional, tag="2")]
    pub account_info: ::core::option::Option<FeeDiscountTierInfo>,
    #[prost(message, optional, tag="3")]
    pub account_ttl: ::core::option::Option<FeeDiscountTierTtl>,
}
/// QueryFeeDiscountScheduleRequest is the request type for the
/// Query/FeeDiscountSchedule RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountScheduleRequest {
}
/// QueryFeeDiscountScheduleResponse is the response type for the
/// Query/FeeDiscountSchedule RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountScheduleResponse {
    #[prost(message, optional, tag="1")]
    pub fee_discount_schedule: ::core::option::Option<FeeDiscountSchedule>,
}
/// QueryBalanceMismatchesRequest is the request type for the
/// Query/QueryBalanceMismatches RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceMismatchesRequest {
    #[prost(int64, tag="1")]
    pub dust_factor: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceMismatch {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub available: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub total: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub balance_hold: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub expected_total: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub difference: ::prost::alloc::string::String,
}
/// QueryBalanceMismatchesResponse is the response type for the
/// Query/QueryBalanceMismatches RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceMismatchesResponse {
    #[prost(message, repeated, tag="1")]
    pub balance_mismatches: ::prost::alloc::vec::Vec<BalanceMismatch>,
}
/// QueryBalanceWithBalanceHoldsRequest is the request type for the
/// Query/QueryBalanceWithBalanceHolds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceWithBalanceHoldsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceWithMarginHold {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub available: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub total: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub balance_hold: ::prost::alloc::string::String,
}
/// QueryBalanceWithBalanceHoldsResponse is the response type for the
/// Query/QueryBalanceWithBalanceHolds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceWithBalanceHoldsResponse {
    #[prost(message, repeated, tag="1")]
    pub balance_with_balance_holds: ::prost::alloc::vec::Vec<BalanceWithMarginHold>,
}
/// QueryFeeDiscountTierStatisticsRequest is the request type for the
/// Query/QueryFeeDiscountTierStatistics RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountTierStatisticsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TierStatistic {
    #[prost(uint64, tag="1")]
    pub tier: u64,
    #[prost(uint64, tag="2")]
    pub count: u64,
}
/// QueryFeeDiscountTierStatisticsResponse is the response type for the
/// Query/QueryFeeDiscountTierStatistics RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountTierStatisticsResponse {
    #[prost(message, repeated, tag="1")]
    pub statistics: ::prost::alloc::vec::Vec<TierStatistic>,
}
/// MitoVaultInfosRequest is the request type for the Query/MitoVaultInfos RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MitoVaultInfosRequest {
}
/// MitoVaultInfosResponse is the response type for the Query/MitoVaultInfos RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MitoVaultInfosResponse {
    #[prost(string, repeated, tag="1")]
    pub master_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub derivative_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub spot_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub cw20_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryMarketIDFromVaultRequest is the request type for the Query/QueryMarketIDFromVault RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketIdFromVaultRequest {
    #[prost(string, tag="1")]
    pub vault_address: ::prost::alloc::string::String,
}
/// QueryMarketIDFromVaultResponse is the response type for the
/// Query/QueryMarketIDFromVault RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketIdFromVaultResponse {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalTradeRecordsRequest {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalTradeRecordsResponse {
    #[prost(message, repeated, tag="1")]
    pub trade_records: ::prost::alloc::vec::Vec<TradeRecords>,
}
/// TradeHistoryOptions are the optional params for Query/MarketVolatility RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeHistoryOptions {
    /// TradeGroupingSec of 0 means use the chain's default grouping
    #[prost(uint64, tag="1")]
    pub trade_grouping_sec: u64,
    /// MaxAge restricts the trade records oldest age in seconds from the current block time to consider.
    /// A value of 0 means use all the records present on the chain.
    #[prost(uint64, tag="2")]
    pub max_age: u64,
    /// If IncludeRawHistory is true, the raw underlying data used for the
    /// computation is included in the response
    #[prost(bool, tag="4")]
    pub include_raw_history: bool,
    /// If IncludeMetadata is true, metadata on the computation is included in the
    /// response
    #[prost(bool, tag="5")]
    pub include_metadata: bool,
}
/// QueryMarketVolatilityRequest are the request params for the
/// Query/MarketVolatility RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketVolatilityRequest {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub trade_history_options: ::core::option::Option<TradeHistoryOptions>,
}
/// QueryMarketVolatilityResponse is the response type for the
/// Query/MarketVolatility RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketVolatilityResponse {
    #[prost(string, tag="1")]
    pub volatility: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub history_metadata: ::core::option::Option<super::super::oracle::v1beta1::MetadataStatistics>,
    #[prost(message, repeated, tag="3")]
    pub raw_history: ::prost::alloc::vec::Vec<TradeRecord>,
}
/// QuerBinaryMarketsRequest is the request type for the Query/BinaryMarkets RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBinaryMarketsRequest {
    /// Status of the market, for convenience it is set to string - not enum
    #[prost(string, tag="1")]
    pub status: ::prost::alloc::string::String,
}
/// QueryBinaryMarketsResponse is the response type for the Query/BinaryMarkets
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBinaryMarketsResponse {
    #[prost(message, repeated, tag="1")]
    pub markets: ::prost::alloc::vec::Vec<BinaryOptionsMarket>,
}
/// QueryConditionalOrdersRequest is the request type for the
/// Query/ConditionalOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeConditionalOrdersRequest {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrimmedDerivativeConditionalOrder {
    /// price of the order
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    /// margin of the order
    #[prost(string, tag="3")]
    pub margin: ::prost::alloc::string::String,
    /// price to trigger the order
    #[prost(string, tag="4")]
    pub trigger_price: ::prost::alloc::string::String,
    /// true if the order is a buy
    ///
    /// ensure omitempty is not in jsontag
    #[prost(bool, tag="5")]
    pub is_buy: bool,
    #[prost(bool, tag="6")]
    pub is_limit: bool,
    #[prost(string, tag="7")]
    pub order_hash: ::prost::alloc::string::String,
}
/// QueryTraderDerivativeOrdersResponse is the response type for the
/// Query/TraderDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeConditionalOrdersResponse {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedDerivativeConditionalOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketAtomicExecutionFeeMultiplierRequest {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketAtomicExecutionFeeMultiplierResponse {
    #[prost(string, tag="1")]
    pub multiplier: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSide {
    /// will return both
    SideUnspecified = 0,
    Buy = 1,
    Sell = 2,
}
impl OrderSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderSide::SideUnspecified => "Side_Unspecified",
            OrderSide::Buy => "Buy",
            OrderSide::Sell => "Sell",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Side_Unspecified" => Some(Self::SideUnspecified),
            "Buy" => Some(Self::Buy),
            "Sell" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// CancellationStrategy is the list of cancellation strategies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CancellationStrategy {
    /// just cancelling in random order in most efficient way
    UnspecifiedOrder = 0,
    /// e.g. for buy orders from lowest to highest price
    FromWorstToBest = 1,
    /// e.g. for buy orders from higest to lowest price
    FromBestToWorst = 2,
}
impl CancellationStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CancellationStrategy::UnspecifiedOrder => "UnspecifiedOrder",
            CancellationStrategy::FromWorstToBest => "FromWorstToBest",
            CancellationStrategy::FromBestToWorst => "FromBestToWorst",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnspecifiedOrder" => Some(Self::UnspecifiedOrder),
            "FromWorstToBest" => Some(Self::FromWorstToBest),
            "FromBestToWorst" => Some(Self::FromBestToWorst),
            _ => None,
        }
    }
}
include!("injective.exchange.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)