// @generated
/// Generated client implementations.
pub mod injective_spot_exchange_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InjectiveSpotExchangeRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InjectiveSpotExchangeRpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InjectiveSpotExchangeRpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InjectiveSpotExchangeRpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            InjectiveSpotExchangeRpcClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn markets(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarketsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Markets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "Markets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn market(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketRequest>,
        ) -> std::result::Result<tonic::Response<super::MarketResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Market",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "Market",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_markets(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamMarketsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamMarkets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "StreamMarkets",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn orderbook(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderbookResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Orderbook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "Orderbook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn orderbook_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderbookV2Request>,
        ) -> std::result::Result<
            tonic::Response<super::OrderbookV2Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/OrderbookV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "OrderbookV2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn orderbooks(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderbooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderbooksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Orderbooks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "Orderbooks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn orderbooks_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderbooksV2Request>,
        ) -> std::result::Result<
            tonic::Response<super::OrderbooksV2Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/OrderbooksV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "OrderbooksV2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_orderbook(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamOrderbookResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrderbook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "StreamOrderbook",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn stream_orderbook_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamOrderbookV2Request>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamOrderbookV2Response>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrderbookV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "StreamOrderbookV2",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn stream_orderbook_update(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamOrderbookUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamOrderbookUpdateResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrderbookUpdate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "StreamOrderbookUpdate",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn orders(
            &mut self,
            request: impl tonic::IntoRequest<super::OrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::OrdersResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Orders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "Orders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamOrdersResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "StreamOrders",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn trades(
            &mut self,
            request: impl tonic::IntoRequest<super::TradesRequest>,
        ) -> std::result::Result<tonic::Response<super::TradesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Trades",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "Trades",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_trades(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamTradesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamTradesResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamTrades",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "StreamTrades",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subaccount_orders_list(
            &mut self,
            request: impl tonic::IntoRequest<super::SubaccountOrdersListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountOrdersListResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/SubaccountOrdersList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "SubaccountOrdersList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_trades_list(
            &mut self,
            request: impl tonic::IntoRequest<super::SubaccountTradesListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountTradesListResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/SubaccountTradesList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "SubaccountTradesList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn orders_history(
            &mut self,
            request: impl tonic::IntoRequest<super::OrdersHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrdersHistoryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/OrdersHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "OrdersHistory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_orders_history(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamOrdersHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamOrdersHistoryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrdersHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC",
                        "StreamOrdersHistory",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod injective_spot_exchange_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InjectiveSpotExchangeRpcServer.
    #[async_trait]
    pub trait InjectiveSpotExchangeRpc: Send + Sync + 'static {
        async fn markets(
            &self,
            request: tonic::Request<super::MarketsRequest>,
        ) -> std::result::Result<tonic::Response<super::MarketsResponse>, tonic::Status>;
        async fn market(
            &self,
            request: tonic::Request<super::MarketRequest>,
        ) -> std::result::Result<tonic::Response<super::MarketResponse>, tonic::Status>;
        /// Server streaming response type for the StreamMarkets method.
        type StreamMarketsStream: futures_core::Stream<
                Item = std::result::Result<super::StreamMarketsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_markets(
            &self,
            request: tonic::Request<super::StreamMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamMarketsStream>,
            tonic::Status,
        >;
        async fn orderbook(
            &self,
            request: tonic::Request<super::OrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderbookResponse>,
            tonic::Status,
        >;
        async fn orderbook_v2(
            &self,
            request: tonic::Request<super::OrderbookV2Request>,
        ) -> std::result::Result<
            tonic::Response<super::OrderbookV2Response>,
            tonic::Status,
        >;
        async fn orderbooks(
            &self,
            request: tonic::Request<super::OrderbooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderbooksResponse>,
            tonic::Status,
        >;
        async fn orderbooks_v2(
            &self,
            request: tonic::Request<super::OrderbooksV2Request>,
        ) -> std::result::Result<
            tonic::Response<super::OrderbooksV2Response>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamOrderbook method.
        type StreamOrderbookStream: futures_core::Stream<
                Item = std::result::Result<super::StreamOrderbookResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_orderbook(
            &self,
            request: tonic::Request<super::StreamOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamOrderbookStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamOrderbookV2 method.
        type StreamOrderbookV2Stream: futures_core::Stream<
                Item = std::result::Result<
                    super::StreamOrderbookV2Response,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_orderbook_v2(
            &self,
            request: tonic::Request<super::StreamOrderbookV2Request>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamOrderbookV2Stream>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamOrderbookUpdate method.
        type StreamOrderbookUpdateStream: futures_core::Stream<
                Item = std::result::Result<
                    super::StreamOrderbookUpdateResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_orderbook_update(
            &self,
            request: tonic::Request<super::StreamOrderbookUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamOrderbookUpdateStream>,
            tonic::Status,
        >;
        async fn orders(
            &self,
            request: tonic::Request<super::OrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::OrdersResponse>, tonic::Status>;
        /// Server streaming response type for the StreamOrders method.
        type StreamOrdersStream: futures_core::Stream<
                Item = std::result::Result<super::StreamOrdersResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_orders(
            &self,
            request: tonic::Request<super::StreamOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamOrdersStream>,
            tonic::Status,
        >;
        async fn trades(
            &self,
            request: tonic::Request<super::TradesRequest>,
        ) -> std::result::Result<tonic::Response<super::TradesResponse>, tonic::Status>;
        /// Server streaming response type for the StreamTrades method.
        type StreamTradesStream: futures_core::Stream<
                Item = std::result::Result<super::StreamTradesResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_trades(
            &self,
            request: tonic::Request<super::StreamTradesRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamTradesStream>,
            tonic::Status,
        >;
        async fn subaccount_orders_list(
            &self,
            request: tonic::Request<super::SubaccountOrdersListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountOrdersListResponse>,
            tonic::Status,
        >;
        async fn subaccount_trades_list(
            &self,
            request: tonic::Request<super::SubaccountTradesListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountTradesListResponse>,
            tonic::Status,
        >;
        async fn orders_history(
            &self,
            request: tonic::Request<super::OrdersHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrdersHistoryResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamOrdersHistory method.
        type StreamOrdersHistoryStream: futures_core::Stream<
                Item = std::result::Result<
                    super::StreamOrdersHistoryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_orders_history(
            &self,
            request: tonic::Request<super::StreamOrdersHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamOrdersHistoryStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct InjectiveSpotExchangeRpcServer<T: InjectiveSpotExchangeRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InjectiveSpotExchangeRpc> InjectiveSpotExchangeRpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for InjectiveSpotExchangeRpcServer<T>
    where
        T: InjectiveSpotExchangeRpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Markets" => {
                    #[allow(non_camel_case_types)]
                    struct MarketsSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::MarketsRequest>
                    for MarketsSvc<T> {
                        type Response = super::MarketsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).markets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarketsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Market" => {
                    #[allow(non_camel_case_types)]
                    struct MarketSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::MarketRequest>
                    for MarketSvc<T> {
                        type Response = super::MarketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).market(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarketSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamMarkets" => {
                    #[allow(non_camel_case_types)]
                    struct StreamMarketsSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::ServerStreamingService<super::StreamMarketsRequest>
                    for StreamMarketsSvc<T> {
                        type Response = super::StreamMarketsResponse;
                        type ResponseStream = T::StreamMarketsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamMarketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_markets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamMarketsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Orderbook" => {
                    #[allow(non_camel_case_types)]
                    struct OrderbookSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::OrderbookRequest>
                    for OrderbookSvc<T> {
                        type Response = super::OrderbookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderbookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).orderbook(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderbookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/OrderbookV2" => {
                    #[allow(non_camel_case_types)]
                    struct OrderbookV2Svc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::OrderbookV2Request>
                    for OrderbookV2Svc<T> {
                        type Response = super::OrderbookV2Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderbookV2Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).orderbook_v2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderbookV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Orderbooks" => {
                    #[allow(non_camel_case_types)]
                    struct OrderbooksSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::OrderbooksRequest>
                    for OrderbooksSvc<T> {
                        type Response = super::OrderbooksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderbooksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).orderbooks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderbooksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/OrderbooksV2" => {
                    #[allow(non_camel_case_types)]
                    struct OrderbooksV2Svc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::OrderbooksV2Request>
                    for OrderbooksV2Svc<T> {
                        type Response = super::OrderbooksV2Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderbooksV2Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).orderbooks_v2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderbooksV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrderbook" => {
                    #[allow(non_camel_case_types)]
                    struct StreamOrderbookSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::ServerStreamingService<
                        super::StreamOrderbookRequest,
                    > for StreamOrderbookSvc<T> {
                        type Response = super::StreamOrderbookResponse;
                        type ResponseStream = T::StreamOrderbookStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamOrderbookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_orderbook(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamOrderbookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrderbookV2" => {
                    #[allow(non_camel_case_types)]
                    struct StreamOrderbookV2Svc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::ServerStreamingService<
                        super::StreamOrderbookV2Request,
                    > for StreamOrderbookV2Svc<T> {
                        type Response = super::StreamOrderbookV2Response;
                        type ResponseStream = T::StreamOrderbookV2Stream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamOrderbookV2Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_orderbook_v2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamOrderbookV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrderbookUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct StreamOrderbookUpdateSvc<T: InjectiveSpotExchangeRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::ServerStreamingService<
                        super::StreamOrderbookUpdateRequest,
                    > for StreamOrderbookUpdateSvc<T> {
                        type Response = super::StreamOrderbookUpdateResponse;
                        type ResponseStream = T::StreamOrderbookUpdateStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamOrderbookUpdateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_orderbook_update(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamOrderbookUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Orders" => {
                    #[allow(non_camel_case_types)]
                    struct OrdersSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::OrdersRequest>
                    for OrdersSvc<T> {
                        type Response = super::OrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).orders(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrdersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrders" => {
                    #[allow(non_camel_case_types)]
                    struct StreamOrdersSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::ServerStreamingService<super::StreamOrdersRequest>
                    for StreamOrdersSvc<T> {
                        type Response = super::StreamOrdersResponse;
                        type ResponseStream = T::StreamOrdersStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_orders(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamOrdersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/Trades" => {
                    #[allow(non_camel_case_types)]
                    struct TradesSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::TradesRequest>
                    for TradesSvc<T> {
                        type Response = super::TradesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TradesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).trades(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TradesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamTrades" => {
                    #[allow(non_camel_case_types)]
                    struct StreamTradesSvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::ServerStreamingService<super::StreamTradesRequest>
                    for StreamTradesSvc<T> {
                        type Response = super::StreamTradesResponse;
                        type ResponseStream = T::StreamTradesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamTradesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_trades(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamTradesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/SubaccountOrdersList" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountOrdersListSvc<T: InjectiveSpotExchangeRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::SubaccountOrdersListRequest>
                    for SubaccountOrdersListSvc<T> {
                        type Response = super::SubaccountOrdersListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubaccountOrdersListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_orders_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubaccountOrdersListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/SubaccountTradesList" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountTradesListSvc<T: InjectiveSpotExchangeRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::SubaccountTradesListRequest>
                    for SubaccountTradesListSvc<T> {
                        type Response = super::SubaccountTradesListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubaccountTradesListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_trades_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubaccountTradesListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/OrdersHistory" => {
                    #[allow(non_camel_case_types)]
                    struct OrdersHistorySvc<T: InjectiveSpotExchangeRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::UnaryService<super::OrdersHistoryRequest>
                    for OrdersHistorySvc<T> {
                        type Response = super::OrdersHistoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrdersHistoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).orders_history(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrdersHistorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/injective_spot_exchange_rpc.InjectiveSpotExchangeRPC/StreamOrdersHistory" => {
                    #[allow(non_camel_case_types)]
                    struct StreamOrdersHistorySvc<T: InjectiveSpotExchangeRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveSpotExchangeRpc,
                    > tonic::server::ServerStreamingService<
                        super::StreamOrdersHistoryRequest,
                    > for StreamOrdersHistorySvc<T> {
                        type Response = super::StreamOrdersHistoryResponse;
                        type ResponseStream = T::StreamOrdersHistoryStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamOrdersHistoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_orders_history(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamOrdersHistorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: InjectiveSpotExchangeRpc> Clone for InjectiveSpotExchangeRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: InjectiveSpotExchangeRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InjectiveSpotExchangeRpc> tonic::server::NamedService
    for InjectiveSpotExchangeRpcServer<T> {
        const NAME: &'static str = "injective_spot_exchange_rpc.InjectiveSpotExchangeRPC";
    }
}
