// @generated
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        ) -> MsgClient<InterceptedService<T, F>>
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
            MsgClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn deposit(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeposit>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDepositResponse>,
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
                "/injective.exchange.v1beta1.Msg/Deposit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("injective.exchange.v1beta1.Msg", "Deposit"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdraw>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawResponse>,
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
                "/injective.exchange.v1beta1.Msg/Withdraw",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("injective.exchange.v1beta1.Msg", "Withdraw"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn instant_spot_market_launch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInstantSpotMarketLaunch>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantSpotMarketLaunchResponse>,
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
                "/injective.exchange.v1beta1.Msg/InstantSpotMarketLaunch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "InstantSpotMarketLaunch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn instant_perpetual_market_launch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInstantPerpetualMarketLaunch>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantPerpetualMarketLaunchResponse>,
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
                "/injective.exchange.v1beta1.Msg/InstantPerpetualMarketLaunch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "InstantPerpetualMarketLaunch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn instant_expiry_futures_market_launch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInstantExpiryFuturesMarketLaunch>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantExpiryFuturesMarketLaunchResponse>,
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
                "/injective.exchange.v1beta1.Msg/InstantExpiryFuturesMarketLaunch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "InstantExpiryFuturesMarketLaunch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_spot_limit_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateSpotLimitOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateSpotLimitOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CreateSpotLimitOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "CreateSpotLimitOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_create_spot_limit_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBatchCreateSpotLimitOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCreateSpotLimitOrdersResponse>,
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
                "/injective.exchange.v1beta1.Msg/BatchCreateSpotLimitOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "BatchCreateSpotLimitOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_spot_market_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateSpotMarketOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateSpotMarketOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CreateSpotMarketOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "CreateSpotMarketOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_spot_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelSpotOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCancelSpotOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CancelSpotOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Msg", "CancelSpotOrder"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_cancel_spot_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBatchCancelSpotOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCancelSpotOrdersResponse>,
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
                "/injective.exchange.v1beta1.Msg/BatchCancelSpotOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "BatchCancelSpotOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_update_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBatchUpdateOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchUpdateOrdersResponse>,
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
                "/injective.exchange.v1beta1.Msg/BatchUpdateOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "BatchUpdateOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn privileged_execute_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPrivilegedExecuteContract>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPrivilegedExecuteContractResponse>,
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
                "/injective.exchange.v1beta1.Msg/PrivilegedExecuteContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "PrivilegedExecuteContract",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_derivative_limit_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateDerivativeLimitOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateDerivativeLimitOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CreateDerivativeLimitOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "CreateDerivativeLimitOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_create_derivative_limit_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBatchCreateDerivativeLimitOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCreateDerivativeLimitOrdersResponse>,
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
                "/injective.exchange.v1beta1.Msg/BatchCreateDerivativeLimitOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "BatchCreateDerivativeLimitOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_derivative_market_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateDerivativeMarketOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateDerivativeMarketOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CreateDerivativeMarketOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "CreateDerivativeMarketOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_derivative_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelDerivativeOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCancelDerivativeOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CancelDerivativeOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "CancelDerivativeOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_cancel_derivative_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBatchCancelDerivativeOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCancelDerivativeOrdersResponse>,
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
                "/injective.exchange.v1beta1.Msg/BatchCancelDerivativeOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "BatchCancelDerivativeOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn instant_binary_options_market_launch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInstantBinaryOptionsMarketLaunch>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantBinaryOptionsMarketLaunchResponse>,
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
                "/injective.exchange.v1beta1.Msg/InstantBinaryOptionsMarketLaunch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "InstantBinaryOptionsMarketLaunch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_binary_options_limit_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateBinaryOptionsLimitOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateBinaryOptionsLimitOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CreateBinaryOptionsLimitOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "CreateBinaryOptionsLimitOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_binary_options_market_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateBinaryOptionsMarketOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateBinaryOptionsMarketOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CreateBinaryOptionsMarketOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "CreateBinaryOptionsMarketOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_binary_options_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelBinaryOptionsOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCancelBinaryOptionsOrderResponse>,
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
                "/injective.exchange.v1beta1.Msg/CancelBinaryOptionsOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "CancelBinaryOptionsOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_cancel_binary_options_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBatchCancelBinaryOptionsOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCancelBinaryOptionsOrdersResponse>,
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
                "/injective.exchange.v1beta1.Msg/BatchCancelBinaryOptionsOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "BatchCancelBinaryOptionsOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubaccountTransfer>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSubaccountTransferResponse>,
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
                "/injective.exchange.v1beta1.Msg/SubaccountTransfer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "SubaccountTransfer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn external_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExternalTransfer>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExternalTransferResponse>,
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
                "/injective.exchange.v1beta1.Msg/ExternalTransfer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Msg", "ExternalTransfer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn liquidate_position(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLiquidatePosition>,
        ) -> std::result::Result<
            tonic::Response<super::MsgLiquidatePositionResponse>,
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
                "/injective.exchange.v1beta1.Msg/LiquidatePosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "LiquidatePosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn increase_position_margin(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgIncreasePositionMargin>,
        ) -> std::result::Result<
            tonic::Response<super::MsgIncreasePositionMarginResponse>,
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
                "/injective.exchange.v1beta1.Msg/IncreasePositionMargin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "IncreasePositionMargin",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn rewards_opt_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRewardsOptOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRewardsOptOutResponse>,
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
                "/injective.exchange.v1beta1.Msg/RewardsOptOut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Msg", "RewardsOptOut"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn admin_update_binary_options_market(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAdminUpdateBinaryOptionsMarket>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAdminUpdateBinaryOptionsMarketResponse>,
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
                "/injective.exchange.v1beta1.Msg/AdminUpdateBinaryOptionsMarket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "AdminUpdateBinaryOptionsMarket",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reclaim_locked_funds(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgReclaimLockedFunds>,
        ) -> std::result::Result<
            tonic::Response<super::MsgReclaimLockedFundsResponse>,
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
                "/injective.exchange.v1beta1.Msg/ReclaimLockedFunds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Msg",
                        "ReclaimLockedFunds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
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
                "/injective.exchange.v1beta1.Msg/UpdateParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Msg", "UpdateParams"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        async fn deposit(
            &self,
            request: tonic::Request<super::MsgDeposit>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDepositResponse>,
            tonic::Status,
        >;
        async fn withdraw(
            &self,
            request: tonic::Request<super::MsgWithdraw>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawResponse>,
            tonic::Status,
        >;
        async fn instant_spot_market_launch(
            &self,
            request: tonic::Request<super::MsgInstantSpotMarketLaunch>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantSpotMarketLaunchResponse>,
            tonic::Status,
        >;
        async fn instant_perpetual_market_launch(
            &self,
            request: tonic::Request<super::MsgInstantPerpetualMarketLaunch>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantPerpetualMarketLaunchResponse>,
            tonic::Status,
        >;
        async fn instant_expiry_futures_market_launch(
            &self,
            request: tonic::Request<super::MsgInstantExpiryFuturesMarketLaunch>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantExpiryFuturesMarketLaunchResponse>,
            tonic::Status,
        >;
        async fn create_spot_limit_order(
            &self,
            request: tonic::Request<super::MsgCreateSpotLimitOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateSpotLimitOrderResponse>,
            tonic::Status,
        >;
        async fn batch_create_spot_limit_orders(
            &self,
            request: tonic::Request<super::MsgBatchCreateSpotLimitOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCreateSpotLimitOrdersResponse>,
            tonic::Status,
        >;
        async fn create_spot_market_order(
            &self,
            request: tonic::Request<super::MsgCreateSpotMarketOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateSpotMarketOrderResponse>,
            tonic::Status,
        >;
        async fn cancel_spot_order(
            &self,
            request: tonic::Request<super::MsgCancelSpotOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCancelSpotOrderResponse>,
            tonic::Status,
        >;
        async fn batch_cancel_spot_orders(
            &self,
            request: tonic::Request<super::MsgBatchCancelSpotOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCancelSpotOrdersResponse>,
            tonic::Status,
        >;
        async fn batch_update_orders(
            &self,
            request: tonic::Request<super::MsgBatchUpdateOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchUpdateOrdersResponse>,
            tonic::Status,
        >;
        async fn privileged_execute_contract(
            &self,
            request: tonic::Request<super::MsgPrivilegedExecuteContract>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPrivilegedExecuteContractResponse>,
            tonic::Status,
        >;
        async fn create_derivative_limit_order(
            &self,
            request: tonic::Request<super::MsgCreateDerivativeLimitOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateDerivativeLimitOrderResponse>,
            tonic::Status,
        >;
        async fn batch_create_derivative_limit_orders(
            &self,
            request: tonic::Request<super::MsgBatchCreateDerivativeLimitOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCreateDerivativeLimitOrdersResponse>,
            tonic::Status,
        >;
        async fn create_derivative_market_order(
            &self,
            request: tonic::Request<super::MsgCreateDerivativeMarketOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateDerivativeMarketOrderResponse>,
            tonic::Status,
        >;
        async fn cancel_derivative_order(
            &self,
            request: tonic::Request<super::MsgCancelDerivativeOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCancelDerivativeOrderResponse>,
            tonic::Status,
        >;
        async fn batch_cancel_derivative_orders(
            &self,
            request: tonic::Request<super::MsgBatchCancelDerivativeOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCancelDerivativeOrdersResponse>,
            tonic::Status,
        >;
        async fn instant_binary_options_market_launch(
            &self,
            request: tonic::Request<super::MsgInstantBinaryOptionsMarketLaunch>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantBinaryOptionsMarketLaunchResponse>,
            tonic::Status,
        >;
        async fn create_binary_options_limit_order(
            &self,
            request: tonic::Request<super::MsgCreateBinaryOptionsLimitOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateBinaryOptionsLimitOrderResponse>,
            tonic::Status,
        >;
        async fn create_binary_options_market_order(
            &self,
            request: tonic::Request<super::MsgCreateBinaryOptionsMarketOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateBinaryOptionsMarketOrderResponse>,
            tonic::Status,
        >;
        async fn cancel_binary_options_order(
            &self,
            request: tonic::Request<super::MsgCancelBinaryOptionsOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCancelBinaryOptionsOrderResponse>,
            tonic::Status,
        >;
        async fn batch_cancel_binary_options_orders(
            &self,
            request: tonic::Request<super::MsgBatchCancelBinaryOptionsOrders>,
        ) -> std::result::Result<
            tonic::Response<super::MsgBatchCancelBinaryOptionsOrdersResponse>,
            tonic::Status,
        >;
        async fn subaccount_transfer(
            &self,
            request: tonic::Request<super::MsgSubaccountTransfer>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSubaccountTransferResponse>,
            tonic::Status,
        >;
        async fn external_transfer(
            &self,
            request: tonic::Request<super::MsgExternalTransfer>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExternalTransferResponse>,
            tonic::Status,
        >;
        async fn liquidate_position(
            &self,
            request: tonic::Request<super::MsgLiquidatePosition>,
        ) -> std::result::Result<
            tonic::Response<super::MsgLiquidatePositionResponse>,
            tonic::Status,
        >;
        async fn increase_position_margin(
            &self,
            request: tonic::Request<super::MsgIncreasePositionMargin>,
        ) -> std::result::Result<
            tonic::Response<super::MsgIncreasePositionMarginResponse>,
            tonic::Status,
        >;
        async fn rewards_opt_out(
            &self,
            request: tonic::Request<super::MsgRewardsOptOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRewardsOptOutResponse>,
            tonic::Status,
        >;
        async fn admin_update_binary_options_market(
            &self,
            request: tonic::Request<super::MsgAdminUpdateBinaryOptionsMarket>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAdminUpdateBinaryOptionsMarketResponse>,
            tonic::Status,
        >;
        async fn reclaim_locked_funds(
            &self,
            request: tonic::Request<super::MsgReclaimLockedFunds>,
        ) -> std::result::Result<
            tonic::Response<super::MsgReclaimLockedFundsResponse>,
            tonic::Status,
        >;
        async fn update_params(
            &self,
            request: tonic::Request<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
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
                "/injective.exchange.v1beta1.Msg/Deposit" => {
                    #[allow(non_camel_case_types)]
                    struct DepositSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeposit>
                    for DepositSvc<T> {
                        type Response = super::MsgDepositResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeposit>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).deposit(request).await };
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
                        let method = DepositSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/Withdraw" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdraw>
                    for WithdrawSvc<T> {
                        type Response = super::MsgWithdrawResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdraw>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).withdraw(request).await };
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
                        let method = WithdrawSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/InstantSpotMarketLaunch" => {
                    #[allow(non_camel_case_types)]
                    struct InstantSpotMarketLaunchSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgInstantSpotMarketLaunch>
                    for InstantSpotMarketLaunchSvc<T> {
                        type Response = super::MsgInstantSpotMarketLaunchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgInstantSpotMarketLaunch>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).instant_spot_market_launch(request).await
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
                        let method = InstantSpotMarketLaunchSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/InstantPerpetualMarketLaunch" => {
                    #[allow(non_camel_case_types)]
                    struct InstantPerpetualMarketLaunchSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgInstantPerpetualMarketLaunch>
                    for InstantPerpetualMarketLaunchSvc<T> {
                        type Response = super::MsgInstantPerpetualMarketLaunchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgInstantPerpetualMarketLaunch,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).instant_perpetual_market_launch(request).await
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
                        let method = InstantPerpetualMarketLaunchSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/InstantExpiryFuturesMarketLaunch" => {
                    #[allow(non_camel_case_types)]
                    struct InstantExpiryFuturesMarketLaunchSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgInstantExpiryFuturesMarketLaunch,
                    > for InstantExpiryFuturesMarketLaunchSvc<T> {
                        type Response = super::MsgInstantExpiryFuturesMarketLaunchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgInstantExpiryFuturesMarketLaunch,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).instant_expiry_futures_market_launch(request).await
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
                        let method = InstantExpiryFuturesMarketLaunchSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CreateSpotLimitOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSpotLimitOrderSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgCreateSpotLimitOrder>
                    for CreateSpotLimitOrderSvc<T> {
                        type Response = super::MsgCreateSpotLimitOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateSpotLimitOrder>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_spot_limit_order(request).await
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
                        let method = CreateSpotLimitOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/BatchCreateSpotLimitOrders" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateSpotLimitOrdersSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgBatchCreateSpotLimitOrders>
                    for BatchCreateSpotLimitOrdersSvc<T> {
                        type Response = super::MsgBatchCreateSpotLimitOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBatchCreateSpotLimitOrders>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).batch_create_spot_limit_orders(request).await
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
                        let method = BatchCreateSpotLimitOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CreateSpotMarketOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSpotMarketOrderSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgCreateSpotMarketOrder>
                    for CreateSpotMarketOrderSvc<T> {
                        type Response = super::MsgCreateSpotMarketOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateSpotMarketOrder>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_spot_market_order(request).await
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
                        let method = CreateSpotMarketOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CancelSpotOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CancelSpotOrderSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCancelSpotOrder>
                    for CancelSpotOrderSvc<T> {
                        type Response = super::MsgCancelSpotOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelSpotOrder>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).cancel_spot_order(request).await
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
                        let method = CancelSpotOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/BatchCancelSpotOrders" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCancelSpotOrdersSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgBatchCancelSpotOrders>
                    for BatchCancelSpotOrdersSvc<T> {
                        type Response = super::MsgBatchCancelSpotOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBatchCancelSpotOrders>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).batch_cancel_spot_orders(request).await
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
                        let method = BatchCancelSpotOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/BatchUpdateOrders" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUpdateOrdersSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBatchUpdateOrders>
                    for BatchUpdateOrdersSvc<T> {
                        type Response = super::MsgBatchUpdateOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBatchUpdateOrders>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).batch_update_orders(request).await
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
                        let method = BatchUpdateOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/PrivilegedExecuteContract" => {
                    #[allow(non_camel_case_types)]
                    struct PrivilegedExecuteContractSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgPrivilegedExecuteContract>
                    for PrivilegedExecuteContractSvc<T> {
                        type Response = super::MsgPrivilegedExecuteContractResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPrivilegedExecuteContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).privileged_execute_contract(request).await
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
                        let method = PrivilegedExecuteContractSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CreateDerivativeLimitOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDerivativeLimitOrderSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgCreateDerivativeLimitOrder>
                    for CreateDerivativeLimitOrderSvc<T> {
                        type Response = super::MsgCreateDerivativeLimitOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateDerivativeLimitOrder>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_derivative_limit_order(request).await
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
                        let method = CreateDerivativeLimitOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/BatchCreateDerivativeLimitOrders" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateDerivativeLimitOrdersSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgBatchCreateDerivativeLimitOrders,
                    > for BatchCreateDerivativeLimitOrdersSvc<T> {
                        type Response = super::MsgBatchCreateDerivativeLimitOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgBatchCreateDerivativeLimitOrders,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).batch_create_derivative_limit_orders(request).await
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
                        let method = BatchCreateDerivativeLimitOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CreateDerivativeMarketOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDerivativeMarketOrderSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgCreateDerivativeMarketOrder>
                    for CreateDerivativeMarketOrderSvc<T> {
                        type Response = super::MsgCreateDerivativeMarketOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgCreateDerivativeMarketOrder,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_derivative_market_order(request).await
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
                        let method = CreateDerivativeMarketOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CancelDerivativeOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CancelDerivativeOrderSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgCancelDerivativeOrder>
                    for CancelDerivativeOrderSvc<T> {
                        type Response = super::MsgCancelDerivativeOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelDerivativeOrder>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).cancel_derivative_order(request).await
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
                        let method = CancelDerivativeOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/BatchCancelDerivativeOrders" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCancelDerivativeOrdersSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgBatchCancelDerivativeOrders>
                    for BatchCancelDerivativeOrdersSvc<T> {
                        type Response = super::MsgBatchCancelDerivativeOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgBatchCancelDerivativeOrders,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).batch_cancel_derivative_orders(request).await
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
                        let method = BatchCancelDerivativeOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/InstantBinaryOptionsMarketLaunch" => {
                    #[allow(non_camel_case_types)]
                    struct InstantBinaryOptionsMarketLaunchSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgInstantBinaryOptionsMarketLaunch,
                    > for InstantBinaryOptionsMarketLaunchSvc<T> {
                        type Response = super::MsgInstantBinaryOptionsMarketLaunchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgInstantBinaryOptionsMarketLaunch,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).instant_binary_options_market_launch(request).await
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
                        let method = InstantBinaryOptionsMarketLaunchSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CreateBinaryOptionsLimitOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBinaryOptionsLimitOrderSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgCreateBinaryOptionsLimitOrder,
                    > for CreateBinaryOptionsLimitOrderSvc<T> {
                        type Response = super::MsgCreateBinaryOptionsLimitOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgCreateBinaryOptionsLimitOrder,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_binary_options_limit_order(request).await
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
                        let method = CreateBinaryOptionsLimitOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CreateBinaryOptionsMarketOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBinaryOptionsMarketOrderSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgCreateBinaryOptionsMarketOrder,
                    > for CreateBinaryOptionsMarketOrderSvc<T> {
                        type Response = super::MsgCreateBinaryOptionsMarketOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgCreateBinaryOptionsMarketOrder,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_binary_options_market_order(request).await
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
                        let method = CreateBinaryOptionsMarketOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/CancelBinaryOptionsOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CancelBinaryOptionsOrderSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgCancelBinaryOptionsOrder>
                    for CancelBinaryOptionsOrderSvc<T> {
                        type Response = super::MsgCancelBinaryOptionsOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelBinaryOptionsOrder>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).cancel_binary_options_order(request).await
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
                        let method = CancelBinaryOptionsOrderSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/BatchCancelBinaryOptionsOrders" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCancelBinaryOptionsOrdersSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgBatchCancelBinaryOptionsOrders,
                    > for BatchCancelBinaryOptionsOrdersSvc<T> {
                        type Response = super::MsgBatchCancelBinaryOptionsOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgBatchCancelBinaryOptionsOrders,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).batch_cancel_binary_options_orders(request).await
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
                        let method = BatchCancelBinaryOptionsOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/SubaccountTransfer" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountTransferSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSubaccountTransfer>
                    for SubaccountTransferSvc<T> {
                        type Response = super::MsgSubaccountTransferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSubaccountTransfer>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_transfer(request).await
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
                        let method = SubaccountTransferSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/ExternalTransfer" => {
                    #[allow(non_camel_case_types)]
                    struct ExternalTransferSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExternalTransfer>
                    for ExternalTransferSvc<T> {
                        type Response = super::MsgExternalTransferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExternalTransfer>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).external_transfer(request).await
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
                        let method = ExternalTransferSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/LiquidatePosition" => {
                    #[allow(non_camel_case_types)]
                    struct LiquidatePositionSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgLiquidatePosition>
                    for LiquidatePositionSvc<T> {
                        type Response = super::MsgLiquidatePositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLiquidatePosition>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).liquidate_position(request).await
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
                        let method = LiquidatePositionSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/IncreasePositionMargin" => {
                    #[allow(non_camel_case_types)]
                    struct IncreasePositionMarginSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgIncreasePositionMargin>
                    for IncreasePositionMarginSvc<T> {
                        type Response = super::MsgIncreasePositionMarginResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgIncreasePositionMargin>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).increase_position_margin(request).await
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
                        let method = IncreasePositionMarginSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/RewardsOptOut" => {
                    #[allow(non_camel_case_types)]
                    struct RewardsOptOutSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRewardsOptOut>
                    for RewardsOptOutSvc<T> {
                        type Response = super::MsgRewardsOptOutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRewardsOptOut>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).rewards_opt_out(request).await
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
                        let method = RewardsOptOutSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/AdminUpdateBinaryOptionsMarket" => {
                    #[allow(non_camel_case_types)]
                    struct AdminUpdateBinaryOptionsMarketSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgAdminUpdateBinaryOptionsMarket,
                    > for AdminUpdateBinaryOptionsMarketSvc<T> {
                        type Response = super::MsgAdminUpdateBinaryOptionsMarketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgAdminUpdateBinaryOptionsMarket,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).admin_update_binary_options_market(request).await
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
                        let method = AdminUpdateBinaryOptionsMarketSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/ReclaimLockedFunds" => {
                    #[allow(non_camel_case_types)]
                    struct ReclaimLockedFundsSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgReclaimLockedFunds>
                    for ReclaimLockedFundsSvc<T> {
                        type Response = super::MsgReclaimLockedFundsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgReclaimLockedFunds>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).reclaim_locked_funds(request).await
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
                        let method = ReclaimLockedFundsSvc(inner);
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
                "/injective.exchange.v1beta1.Msg/UpdateParams" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateParamsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateParams>
                    for UpdateParamsSvc<T> {
                        type Response = super::MsgUpdateParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_params(request).await
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
                        let method = UpdateParamsSvc(inner);
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
    impl<T: Msg> Clone for MsgServer<T> {
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
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "injective.exchange.v1beta1.Msg";
    }
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
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
        ) -> QueryClient<InterceptedService<T, F>>
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
            QueryClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn query_exchange_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryExchangeParamsResponse>,
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
                "/injective.exchange.v1beta1.Query/QueryExchangeParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "QueryExchangeParams",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_deposits(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySubaccountDepositsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountDepositsResponse>,
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
                "/injective.exchange.v1beta1.Query/SubaccountDeposits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SubaccountDeposits",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_deposit(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySubaccountDepositRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountDepositResponse>,
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
                "/injective.exchange.v1beta1.Query/SubaccountDeposit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SubaccountDeposit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn exchange_balances(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeBalancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryExchangeBalancesResponse>,
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
                "/injective.exchange.v1beta1.Query/ExchangeBalances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "ExchangeBalances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn aggregate_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVolumeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregateVolumeResponse>,
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
                "/injective.exchange.v1beta1.Query/AggregateVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "AggregateVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn aggregate_volumes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVolumesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregateVolumesResponse>,
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
                "/injective.exchange.v1beta1.Query/AggregateVolumes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "AggregateVolumes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn aggregate_market_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateMarketVolumeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregateMarketVolumeResponse>,
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
                "/injective.exchange.v1beta1.Query/AggregateMarketVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "AggregateMarketVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn aggregate_market_volumes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateMarketVolumesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregateMarketVolumesResponse>,
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
                "/injective.exchange.v1beta1.Query/AggregateMarketVolumes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "AggregateMarketVolumes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn denom_decimal(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomDecimalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDenomDecimalResponse>,
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
                "/injective.exchange.v1beta1.Query/DenomDecimal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Query", "DenomDecimal"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn denom_decimals(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomDecimalsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDenomDecimalsResponse>,
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
                "/injective.exchange.v1beta1.Query/DenomDecimals",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Query", "DenomDecimals"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn spot_markets(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySpotMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotMarketsResponse>,
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
                "/injective.exchange.v1beta1.Query/SpotMarkets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Query", "SpotMarkets"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn spot_market(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySpotMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotMarketResponse>,
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
                "/injective.exchange.v1beta1.Query/SpotMarket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Query", "SpotMarket"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn full_spot_markets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFullSpotMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFullSpotMarketsResponse>,
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
                "/injective.exchange.v1beta1.Query/FullSpotMarkets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "FullSpotMarkets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn full_spot_market(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFullSpotMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFullSpotMarketResponse>,
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
                "/injective.exchange.v1beta1.Query/FullSpotMarket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Query", "FullSpotMarket"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn spot_orderbook(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySpotOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotOrderbookResponse>,
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
                "/injective.exchange.v1beta1.Query/SpotOrderbook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Query", "SpotOrderbook"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn trader_spot_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTraderSpotOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderSpotOrdersResponse>,
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
                "/injective.exchange.v1beta1.Query/TraderSpotOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "TraderSpotOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn account_address_spot_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountAddressSpotOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAccountAddressSpotOrdersResponse>,
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
                "/injective.exchange.v1beta1.Query/AccountAddressSpotOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "AccountAddressSpotOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn spot_orders_by_hashes(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySpotOrdersByHashesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotOrdersByHashesResponse>,
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
                "/injective.exchange.v1beta1.Query/SpotOrdersByHashes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SpotOrdersByHashes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySubaccountOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountOrdersResponse>,
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
                "/injective.exchange.v1beta1.Query/SubaccountOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SubaccountOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn trader_spot_transient_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTraderSpotOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderSpotOrdersResponse>,
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
                "/injective.exchange.v1beta1.Query/TraderSpotTransientOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "TraderSpotTransientOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn spot_mid_price_and_tob(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySpotMidPriceAndTobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotMidPriceAndTobResponse>,
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
                "/injective.exchange.v1beta1.Query/SpotMidPriceAndTOB",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SpotMidPriceAndTOB",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn derivative_mid_price_and_tob(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDerivativeMidPriceAndTobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeMidPriceAndTobResponse>,
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
                "/injective.exchange.v1beta1.Query/DerivativeMidPriceAndTOB",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "DerivativeMidPriceAndTOB",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn derivative_orderbook(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDerivativeOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeOrderbookResponse>,
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
                "/injective.exchange.v1beta1.Query/DerivativeOrderbook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "DerivativeOrderbook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn trader_derivative_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTraderDerivativeOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderDerivativeOrdersResponse>,
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
                "/injective.exchange.v1beta1.Query/TraderDerivativeOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "TraderDerivativeOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn account_address_derivative_orders(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryAccountAddressDerivativeOrdersRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryAccountAddressDerivativeOrdersResponse>,
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
                "/injective.exchange.v1beta1.Query/AccountAddressDerivativeOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "AccountAddressDerivativeOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn derivative_orders_by_hashes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDerivativeOrdersByHashesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeOrdersByHashesResponse>,
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
                "/injective.exchange.v1beta1.Query/DerivativeOrdersByHashes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "DerivativeOrdersByHashes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn trader_derivative_transient_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTraderDerivativeOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderDerivativeOrdersResponse>,
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
                "/injective.exchange.v1beta1.Query/TraderDerivativeTransientOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "TraderDerivativeTransientOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn derivative_markets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDerivativeMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeMarketsResponse>,
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
                "/injective.exchange.v1beta1.Query/DerivativeMarkets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "DerivativeMarkets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn derivative_market(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDerivativeMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeMarketResponse>,
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
                "/injective.exchange.v1beta1.Query/DerivativeMarket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "DerivativeMarket",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn derivative_market_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDerivativeMarketAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeMarketAddressResponse>,
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
                "/injective.exchange.v1beta1.Query/DerivativeMarketAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "DerivativeMarketAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_trade_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySubaccountTradeNonceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountTradeNonceResponse>,
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
                "/injective.exchange.v1beta1.Query/SubaccountTradeNonce",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SubaccountTradeNonce",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn exchange_module_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryModuleStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryModuleStateResponse>,
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
                "/injective.exchange.v1beta1.Query/ExchangeModuleState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "ExchangeModuleState",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn positions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPositionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPositionsResponse>,
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
                "/injective.exchange.v1beta1.Query/Positions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Query", "Positions"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySubaccountPositionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountPositionsResponse>,
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
                "/injective.exchange.v1beta1.Query/SubaccountPositions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SubaccountPositions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_position_in_market(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QuerySubaccountPositionInMarketRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountPositionInMarketResponse>,
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
                "/injective.exchange.v1beta1.Query/SubaccountPositionInMarket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SubaccountPositionInMarket",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_effective_position_in_market(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QuerySubaccountEffectivePositionInMarketRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountEffectivePositionInMarketResponse>,
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
                "/injective.exchange.v1beta1.Query/SubaccountEffectivePositionInMarket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SubaccountEffectivePositionInMarket",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn perpetual_market_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPerpetualMarketInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPerpetualMarketInfoResponse>,
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
                "/injective.exchange.v1beta1.Query/PerpetualMarketInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "PerpetualMarketInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn expiry_futures_market_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExpiryFuturesMarketInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryExpiryFuturesMarketInfoResponse>,
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
                "/injective.exchange.v1beta1.Query/ExpiryFuturesMarketInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "ExpiryFuturesMarketInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn perpetual_market_funding(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPerpetualMarketFundingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPerpetualMarketFundingResponse>,
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
                "/injective.exchange.v1beta1.Query/PerpetualMarketFunding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "PerpetualMarketFunding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_order_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySubaccountOrderMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountOrderMetadataResponse>,
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
                "/injective.exchange.v1beta1.Query/SubaccountOrderMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "SubaccountOrderMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn trade_reward_points(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTradeRewardPointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTradeRewardPointsResponse>,
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
                "/injective.exchange.v1beta1.Query/TradeRewardPoints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "TradeRewardPoints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn pending_trade_reward_points(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTradeRewardPointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTradeRewardPointsResponse>,
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
                "/injective.exchange.v1beta1.Query/PendingTradeRewardPoints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "PendingTradeRewardPoints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn trade_reward_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTradeRewardCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTradeRewardCampaignResponse>,
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
                "/injective.exchange.v1beta1.Query/TradeRewardCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "TradeRewardCampaign",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn fee_discount_account_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFeeDiscountAccountInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFeeDiscountAccountInfoResponse>,
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
                "/injective.exchange.v1beta1.Query/FeeDiscountAccountInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "FeeDiscountAccountInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn fee_discount_schedule(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFeeDiscountScheduleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFeeDiscountScheduleResponse>,
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
                "/injective.exchange.v1beta1.Query/FeeDiscountSchedule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "FeeDiscountSchedule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn balance_mismatches(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBalanceMismatchesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBalanceMismatchesResponse>,
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
                "/injective.exchange.v1beta1.Query/BalanceMismatches",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "BalanceMismatches",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn balance_with_balance_holds(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBalanceWithBalanceHoldsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBalanceWithBalanceHoldsResponse>,
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
                "/injective.exchange.v1beta1.Query/BalanceWithBalanceHolds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "BalanceWithBalanceHolds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn fee_discount_tier_statistics(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryFeeDiscountTierStatisticsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryFeeDiscountTierStatisticsResponse>,
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
                "/injective.exchange.v1beta1.Query/FeeDiscountTierStatistics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "FeeDiscountTierStatistics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn mito_vault_infos(
            &mut self,
            request: impl tonic::IntoRequest<super::MitoVaultInfosRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MitoVaultInfosResponse>,
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
                "/injective.exchange.v1beta1.Query/MitoVaultInfos",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("injective.exchange.v1beta1.Query", "MitoVaultInfos"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_market_id_from_vault(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMarketIdFromVaultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryMarketIdFromVaultResponse>,
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
                "/injective.exchange.v1beta1.Query/QueryMarketIDFromVault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "QueryMarketIDFromVault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn historical_trade_records(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryHistoricalTradeRecordsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryHistoricalTradeRecordsResponse>,
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
                "/injective.exchange.v1beta1.Query/HistoricalTradeRecords",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "HistoricalTradeRecords",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_opted_out_of_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIsOptedOutOfRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryIsOptedOutOfRewardsResponse>,
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
                "/injective.exchange.v1beta1.Query/IsOptedOutOfRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "IsOptedOutOfRewards",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn opted_out_of_rewards_accounts(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryOptedOutOfRewardsAccountsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryOptedOutOfRewardsAccountsResponse>,
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
                "/injective.exchange.v1beta1.Query/OptedOutOfRewardsAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "OptedOutOfRewardsAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_volatility(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMarketVolatilityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryMarketVolatilityResponse>,
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
                "/injective.exchange.v1beta1.Query/MarketVolatility",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "MarketVolatility",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn binary_options_markets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBinaryMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBinaryMarketsResponse>,
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
                "/injective.exchange.v1beta1.Query/BinaryOptionsMarkets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "BinaryOptionsMarkets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn trader_derivative_conditional_orders(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryTraderDerivativeConditionalOrdersRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderDerivativeConditionalOrdersResponse>,
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
                "/injective.exchange.v1beta1.Query/TraderDerivativeConditionalOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "TraderDerivativeConditionalOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_atomic_execution_fee_multiplier(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryMarketAtomicExecutionFeeMultiplierRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryMarketAtomicExecutionFeeMultiplierResponse>,
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
                "/injective.exchange.v1beta1.Query/MarketAtomicExecutionFeeMultiplier",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective.exchange.v1beta1.Query",
                        "MarketAtomicExecutionFeeMultiplier",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        async fn query_exchange_params(
            &self,
            request: tonic::Request<super::QueryExchangeParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryExchangeParamsResponse>,
            tonic::Status,
        >;
        async fn subaccount_deposits(
            &self,
            request: tonic::Request<super::QuerySubaccountDepositsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountDepositsResponse>,
            tonic::Status,
        >;
        async fn subaccount_deposit(
            &self,
            request: tonic::Request<super::QuerySubaccountDepositRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountDepositResponse>,
            tonic::Status,
        >;
        async fn exchange_balances(
            &self,
            request: tonic::Request<super::QueryExchangeBalancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryExchangeBalancesResponse>,
            tonic::Status,
        >;
        async fn aggregate_volume(
            &self,
            request: tonic::Request<super::QueryAggregateVolumeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregateVolumeResponse>,
            tonic::Status,
        >;
        async fn aggregate_volumes(
            &self,
            request: tonic::Request<super::QueryAggregateVolumesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregateVolumesResponse>,
            tonic::Status,
        >;
        async fn aggregate_market_volume(
            &self,
            request: tonic::Request<super::QueryAggregateMarketVolumeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregateMarketVolumeResponse>,
            tonic::Status,
        >;
        async fn aggregate_market_volumes(
            &self,
            request: tonic::Request<super::QueryAggregateMarketVolumesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregateMarketVolumesResponse>,
            tonic::Status,
        >;
        async fn denom_decimal(
            &self,
            request: tonic::Request<super::QueryDenomDecimalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDenomDecimalResponse>,
            tonic::Status,
        >;
        async fn denom_decimals(
            &self,
            request: tonic::Request<super::QueryDenomDecimalsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDenomDecimalsResponse>,
            tonic::Status,
        >;
        async fn spot_markets(
            &self,
            request: tonic::Request<super::QuerySpotMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotMarketsResponse>,
            tonic::Status,
        >;
        async fn spot_market(
            &self,
            request: tonic::Request<super::QuerySpotMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotMarketResponse>,
            tonic::Status,
        >;
        async fn full_spot_markets(
            &self,
            request: tonic::Request<super::QueryFullSpotMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFullSpotMarketsResponse>,
            tonic::Status,
        >;
        async fn full_spot_market(
            &self,
            request: tonic::Request<super::QueryFullSpotMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFullSpotMarketResponse>,
            tonic::Status,
        >;
        async fn spot_orderbook(
            &self,
            request: tonic::Request<super::QuerySpotOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotOrderbookResponse>,
            tonic::Status,
        >;
        async fn trader_spot_orders(
            &self,
            request: tonic::Request<super::QueryTraderSpotOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderSpotOrdersResponse>,
            tonic::Status,
        >;
        async fn account_address_spot_orders(
            &self,
            request: tonic::Request<super::QueryAccountAddressSpotOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAccountAddressSpotOrdersResponse>,
            tonic::Status,
        >;
        async fn spot_orders_by_hashes(
            &self,
            request: tonic::Request<super::QuerySpotOrdersByHashesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotOrdersByHashesResponse>,
            tonic::Status,
        >;
        async fn subaccount_orders(
            &self,
            request: tonic::Request<super::QuerySubaccountOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountOrdersResponse>,
            tonic::Status,
        >;
        async fn trader_spot_transient_orders(
            &self,
            request: tonic::Request<super::QueryTraderSpotOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderSpotOrdersResponse>,
            tonic::Status,
        >;
        async fn spot_mid_price_and_tob(
            &self,
            request: tonic::Request<super::QuerySpotMidPriceAndTobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotMidPriceAndTobResponse>,
            tonic::Status,
        >;
        async fn derivative_mid_price_and_tob(
            &self,
            request: tonic::Request<super::QueryDerivativeMidPriceAndTobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeMidPriceAndTobResponse>,
            tonic::Status,
        >;
        async fn derivative_orderbook(
            &self,
            request: tonic::Request<super::QueryDerivativeOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeOrderbookResponse>,
            tonic::Status,
        >;
        async fn trader_derivative_orders(
            &self,
            request: tonic::Request<super::QueryTraderDerivativeOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderDerivativeOrdersResponse>,
            tonic::Status,
        >;
        async fn account_address_derivative_orders(
            &self,
            request: tonic::Request<super::QueryAccountAddressDerivativeOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAccountAddressDerivativeOrdersResponse>,
            tonic::Status,
        >;
        async fn derivative_orders_by_hashes(
            &self,
            request: tonic::Request<super::QueryDerivativeOrdersByHashesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeOrdersByHashesResponse>,
            tonic::Status,
        >;
        async fn trader_derivative_transient_orders(
            &self,
            request: tonic::Request<super::QueryTraderDerivativeOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderDerivativeOrdersResponse>,
            tonic::Status,
        >;
        async fn derivative_markets(
            &self,
            request: tonic::Request<super::QueryDerivativeMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeMarketsResponse>,
            tonic::Status,
        >;
        async fn derivative_market(
            &self,
            request: tonic::Request<super::QueryDerivativeMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeMarketResponse>,
            tonic::Status,
        >;
        async fn derivative_market_address(
            &self,
            request: tonic::Request<super::QueryDerivativeMarketAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDerivativeMarketAddressResponse>,
            tonic::Status,
        >;
        async fn subaccount_trade_nonce(
            &self,
            request: tonic::Request<super::QuerySubaccountTradeNonceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountTradeNonceResponse>,
            tonic::Status,
        >;
        async fn exchange_module_state(
            &self,
            request: tonic::Request<super::QueryModuleStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryModuleStateResponse>,
            tonic::Status,
        >;
        async fn positions(
            &self,
            request: tonic::Request<super::QueryPositionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPositionsResponse>,
            tonic::Status,
        >;
        async fn subaccount_positions(
            &self,
            request: tonic::Request<super::QuerySubaccountPositionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountPositionsResponse>,
            tonic::Status,
        >;
        async fn subaccount_position_in_market(
            &self,
            request: tonic::Request<super::QuerySubaccountPositionInMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountPositionInMarketResponse>,
            tonic::Status,
        >;
        async fn subaccount_effective_position_in_market(
            &self,
            request: tonic::Request<
                super::QuerySubaccountEffectivePositionInMarketRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountEffectivePositionInMarketResponse>,
            tonic::Status,
        >;
        async fn perpetual_market_info(
            &self,
            request: tonic::Request<super::QueryPerpetualMarketInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPerpetualMarketInfoResponse>,
            tonic::Status,
        >;
        async fn expiry_futures_market_info(
            &self,
            request: tonic::Request<super::QueryExpiryFuturesMarketInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryExpiryFuturesMarketInfoResponse>,
            tonic::Status,
        >;
        async fn perpetual_market_funding(
            &self,
            request: tonic::Request<super::QueryPerpetualMarketFundingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPerpetualMarketFundingResponse>,
            tonic::Status,
        >;
        async fn subaccount_order_metadata(
            &self,
            request: tonic::Request<super::QuerySubaccountOrderMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySubaccountOrderMetadataResponse>,
            tonic::Status,
        >;
        async fn trade_reward_points(
            &self,
            request: tonic::Request<super::QueryTradeRewardPointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTradeRewardPointsResponse>,
            tonic::Status,
        >;
        async fn pending_trade_reward_points(
            &self,
            request: tonic::Request<super::QueryTradeRewardPointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTradeRewardPointsResponse>,
            tonic::Status,
        >;
        async fn trade_reward_campaign(
            &self,
            request: tonic::Request<super::QueryTradeRewardCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTradeRewardCampaignResponse>,
            tonic::Status,
        >;
        async fn fee_discount_account_info(
            &self,
            request: tonic::Request<super::QueryFeeDiscountAccountInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFeeDiscountAccountInfoResponse>,
            tonic::Status,
        >;
        async fn fee_discount_schedule(
            &self,
            request: tonic::Request<super::QueryFeeDiscountScheduleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFeeDiscountScheduleResponse>,
            tonic::Status,
        >;
        async fn balance_mismatches(
            &self,
            request: tonic::Request<super::QueryBalanceMismatchesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBalanceMismatchesResponse>,
            tonic::Status,
        >;
        async fn balance_with_balance_holds(
            &self,
            request: tonic::Request<super::QueryBalanceWithBalanceHoldsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBalanceWithBalanceHoldsResponse>,
            tonic::Status,
        >;
        async fn fee_discount_tier_statistics(
            &self,
            request: tonic::Request<super::QueryFeeDiscountTierStatisticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFeeDiscountTierStatisticsResponse>,
            tonic::Status,
        >;
        async fn mito_vault_infos(
            &self,
            request: tonic::Request<super::MitoVaultInfosRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MitoVaultInfosResponse>,
            tonic::Status,
        >;
        async fn query_market_id_from_vault(
            &self,
            request: tonic::Request<super::QueryMarketIdFromVaultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryMarketIdFromVaultResponse>,
            tonic::Status,
        >;
        async fn historical_trade_records(
            &self,
            request: tonic::Request<super::QueryHistoricalTradeRecordsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryHistoricalTradeRecordsResponse>,
            tonic::Status,
        >;
        async fn is_opted_out_of_rewards(
            &self,
            request: tonic::Request<super::QueryIsOptedOutOfRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryIsOptedOutOfRewardsResponse>,
            tonic::Status,
        >;
        async fn opted_out_of_rewards_accounts(
            &self,
            request: tonic::Request<super::QueryOptedOutOfRewardsAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryOptedOutOfRewardsAccountsResponse>,
            tonic::Status,
        >;
        async fn market_volatility(
            &self,
            request: tonic::Request<super::QueryMarketVolatilityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryMarketVolatilityResponse>,
            tonic::Status,
        >;
        async fn binary_options_markets(
            &self,
            request: tonic::Request<super::QueryBinaryMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBinaryMarketsResponse>,
            tonic::Status,
        >;
        async fn trader_derivative_conditional_orders(
            &self,
            request: tonic::Request<super::QueryTraderDerivativeConditionalOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraderDerivativeConditionalOrdersResponse>,
            tonic::Status,
        >;
        async fn market_atomic_execution_fee_multiplier(
            &self,
            request: tonic::Request<
                super::QueryMarketAtomicExecutionFeeMultiplierRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryMarketAtomicExecutionFeeMultiplierResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
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
                "/injective.exchange.v1beta1.Query/QueryExchangeParams" => {
                    #[allow(non_camel_case_types)]
                    struct QueryExchangeParamsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryExchangeParamsRequest>
                    for QueryExchangeParamsSvc<T> {
                        type Response = super::QueryExchangeParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryExchangeParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_exchange_params(request).await
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
                        let method = QueryExchangeParamsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SubaccountDeposits" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountDepositsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySubaccountDepositsRequest>
                    for SubaccountDepositsSvc<T> {
                        type Response = super::QuerySubaccountDepositsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySubaccountDepositsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_deposits(request).await
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
                        let method = SubaccountDepositsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SubaccountDeposit" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountDepositSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySubaccountDepositRequest>
                    for SubaccountDepositSvc<T> {
                        type Response = super::QuerySubaccountDepositResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySubaccountDepositRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_deposit(request).await
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
                        let method = SubaccountDepositSvc(inner);
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
                "/injective.exchange.v1beta1.Query/ExchangeBalances" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeBalancesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryExchangeBalancesRequest>
                    for ExchangeBalancesSvc<T> {
                        type Response = super::QueryExchangeBalancesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryExchangeBalancesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).exchange_balances(request).await
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
                        let method = ExchangeBalancesSvc(inner);
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
                "/injective.exchange.v1beta1.Query/AggregateVolume" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateVolumeSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryAggregateVolumeRequest>
                    for AggregateVolumeSvc<T> {
                        type Response = super::QueryAggregateVolumeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregateVolumeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aggregate_volume(request).await
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
                        let method = AggregateVolumeSvc(inner);
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
                "/injective.exchange.v1beta1.Query/AggregateVolumes" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateVolumesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryAggregateVolumesRequest>
                    for AggregateVolumesSvc<T> {
                        type Response = super::QueryAggregateVolumesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregateVolumesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aggregate_volumes(request).await
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
                        let method = AggregateVolumesSvc(inner);
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
                "/injective.exchange.v1beta1.Query/AggregateMarketVolume" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateMarketVolumeSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryAggregateMarketVolumeRequest,
                    > for AggregateMarketVolumeSvc<T> {
                        type Response = super::QueryAggregateMarketVolumeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryAggregateMarketVolumeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aggregate_market_volume(request).await
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
                        let method = AggregateMarketVolumeSvc(inner);
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
                "/injective.exchange.v1beta1.Query/AggregateMarketVolumes" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateMarketVolumesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryAggregateMarketVolumesRequest,
                    > for AggregateMarketVolumesSvc<T> {
                        type Response = super::QueryAggregateMarketVolumesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryAggregateMarketVolumesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aggregate_market_volumes(request).await
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
                        let method = AggregateMarketVolumesSvc(inner);
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
                "/injective.exchange.v1beta1.Query/DenomDecimal" => {
                    #[allow(non_camel_case_types)]
                    struct DenomDecimalSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryDenomDecimalRequest>
                    for DenomDecimalSvc<T> {
                        type Response = super::QueryDenomDecimalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDenomDecimalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).denom_decimal(request).await
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
                        let method = DenomDecimalSvc(inner);
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
                "/injective.exchange.v1beta1.Query/DenomDecimals" => {
                    #[allow(non_camel_case_types)]
                    struct DenomDecimalsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryDenomDecimalsRequest>
                    for DenomDecimalsSvc<T> {
                        type Response = super::QueryDenomDecimalsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDenomDecimalsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).denom_decimals(request).await
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
                        let method = DenomDecimalsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SpotMarkets" => {
                    #[allow(non_camel_case_types)]
                    struct SpotMarketsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySpotMarketsRequest>
                    for SpotMarketsSvc<T> {
                        type Response = super::QuerySpotMarketsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySpotMarketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).spot_markets(request).await
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
                        let method = SpotMarketsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SpotMarket" => {
                    #[allow(non_camel_case_types)]
                    struct SpotMarketSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySpotMarketRequest>
                    for SpotMarketSvc<T> {
                        type Response = super::QuerySpotMarketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySpotMarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).spot_market(request).await };
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
                        let method = SpotMarketSvc(inner);
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
                "/injective.exchange.v1beta1.Query/FullSpotMarkets" => {
                    #[allow(non_camel_case_types)]
                    struct FullSpotMarketsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryFullSpotMarketsRequest>
                    for FullSpotMarketsSvc<T> {
                        type Response = super::QueryFullSpotMarketsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFullSpotMarketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).full_spot_markets(request).await
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
                        let method = FullSpotMarketsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/FullSpotMarket" => {
                    #[allow(non_camel_case_types)]
                    struct FullSpotMarketSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryFullSpotMarketRequest>
                    for FullSpotMarketSvc<T> {
                        type Response = super::QueryFullSpotMarketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFullSpotMarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).full_spot_market(request).await
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
                        let method = FullSpotMarketSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SpotOrderbook" => {
                    #[allow(non_camel_case_types)]
                    struct SpotOrderbookSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySpotOrderbookRequest>
                    for SpotOrderbookSvc<T> {
                        type Response = super::QuerySpotOrderbookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySpotOrderbookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).spot_orderbook(request).await
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
                        let method = SpotOrderbookSvc(inner);
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
                "/injective.exchange.v1beta1.Query/TraderSpotOrders" => {
                    #[allow(non_camel_case_types)]
                    struct TraderSpotOrdersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTraderSpotOrdersRequest>
                    for TraderSpotOrdersSvc<T> {
                        type Response = super::QueryTraderSpotOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTraderSpotOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trader_spot_orders(request).await
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
                        let method = TraderSpotOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Query/AccountAddressSpotOrders" => {
                    #[allow(non_camel_case_types)]
                    struct AccountAddressSpotOrdersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryAccountAddressSpotOrdersRequest,
                    > for AccountAddressSpotOrdersSvc<T> {
                        type Response = super::QueryAccountAddressSpotOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryAccountAddressSpotOrdersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).account_address_spot_orders(request).await
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
                        let method = AccountAddressSpotOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SpotOrdersByHashes" => {
                    #[allow(non_camel_case_types)]
                    struct SpotOrdersByHashesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySpotOrdersByHashesRequest>
                    for SpotOrdersByHashesSvc<T> {
                        type Response = super::QuerySpotOrdersByHashesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySpotOrdersByHashesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).spot_orders_by_hashes(request).await
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
                        let method = SpotOrdersByHashesSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SubaccountOrders" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountOrdersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySubaccountOrdersRequest>
                    for SubaccountOrdersSvc<T> {
                        type Response = super::QuerySubaccountOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySubaccountOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_orders(request).await
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
                        let method = SubaccountOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Query/TraderSpotTransientOrders" => {
                    #[allow(non_camel_case_types)]
                    struct TraderSpotTransientOrdersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTraderSpotOrdersRequest>
                    for TraderSpotTransientOrdersSvc<T> {
                        type Response = super::QueryTraderSpotOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTraderSpotOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trader_spot_transient_orders(request).await
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
                        let method = TraderSpotTransientOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SpotMidPriceAndTOB" => {
                    #[allow(non_camel_case_types)]
                    struct SpotMidPriceAndTOBSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySpotMidPriceAndTobRequest>
                    for SpotMidPriceAndTOBSvc<T> {
                        type Response = super::QuerySpotMidPriceAndTobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySpotMidPriceAndTobRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).spot_mid_price_and_tob(request).await
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
                        let method = SpotMidPriceAndTOBSvc(inner);
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
                "/injective.exchange.v1beta1.Query/DerivativeMidPriceAndTOB" => {
                    #[allow(non_camel_case_types)]
                    struct DerivativeMidPriceAndTOBSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryDerivativeMidPriceAndTobRequest,
                    > for DerivativeMidPriceAndTOBSvc<T> {
                        type Response = super::QueryDerivativeMidPriceAndTobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryDerivativeMidPriceAndTobRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).derivative_mid_price_and_tob(request).await
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
                        let method = DerivativeMidPriceAndTOBSvc(inner);
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
                "/injective.exchange.v1beta1.Query/DerivativeOrderbook" => {
                    #[allow(non_camel_case_types)]
                    struct DerivativeOrderbookSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryDerivativeOrderbookRequest>
                    for DerivativeOrderbookSvc<T> {
                        type Response = super::QueryDerivativeOrderbookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryDerivativeOrderbookRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).derivative_orderbook(request).await
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
                        let method = DerivativeOrderbookSvc(inner);
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
                "/injective.exchange.v1beta1.Query/TraderDerivativeOrders" => {
                    #[allow(non_camel_case_types)]
                    struct TraderDerivativeOrdersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTraderDerivativeOrdersRequest,
                    > for TraderDerivativeOrdersSvc<T> {
                        type Response = super::QueryTraderDerivativeOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTraderDerivativeOrdersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trader_derivative_orders(request).await
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
                        let method = TraderDerivativeOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Query/AccountAddressDerivativeOrders" => {
                    #[allow(non_camel_case_types)]
                    struct AccountAddressDerivativeOrdersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryAccountAddressDerivativeOrdersRequest,
                    > for AccountAddressDerivativeOrdersSvc<T> {
                        type Response = super::QueryAccountAddressDerivativeOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryAccountAddressDerivativeOrdersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).account_address_derivative_orders(request).await
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
                        let method = AccountAddressDerivativeOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Query/DerivativeOrdersByHashes" => {
                    #[allow(non_camel_case_types)]
                    struct DerivativeOrdersByHashesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryDerivativeOrdersByHashesRequest,
                    > for DerivativeOrdersByHashesSvc<T> {
                        type Response = super::QueryDerivativeOrdersByHashesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryDerivativeOrdersByHashesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).derivative_orders_by_hashes(request).await
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
                        let method = DerivativeOrdersByHashesSvc(inner);
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
                "/injective.exchange.v1beta1.Query/TraderDerivativeTransientOrders" => {
                    #[allow(non_camel_case_types)]
                    struct TraderDerivativeTransientOrdersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTraderDerivativeOrdersRequest,
                    > for TraderDerivativeTransientOrdersSvc<T> {
                        type Response = super::QueryTraderDerivativeOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTraderDerivativeOrdersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trader_derivative_transient_orders(request).await
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
                        let method = TraderDerivativeTransientOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Query/DerivativeMarkets" => {
                    #[allow(non_camel_case_types)]
                    struct DerivativeMarketsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryDerivativeMarketsRequest>
                    for DerivativeMarketsSvc<T> {
                        type Response = super::QueryDerivativeMarketsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDerivativeMarketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).derivative_markets(request).await
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
                        let method = DerivativeMarketsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/DerivativeMarket" => {
                    #[allow(non_camel_case_types)]
                    struct DerivativeMarketSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryDerivativeMarketRequest>
                    for DerivativeMarketSvc<T> {
                        type Response = super::QueryDerivativeMarketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDerivativeMarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).derivative_market(request).await
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
                        let method = DerivativeMarketSvc(inner);
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
                "/injective.exchange.v1beta1.Query/DerivativeMarketAddress" => {
                    #[allow(non_camel_case_types)]
                    struct DerivativeMarketAddressSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryDerivativeMarketAddressRequest,
                    > for DerivativeMarketAddressSvc<T> {
                        type Response = super::QueryDerivativeMarketAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryDerivativeMarketAddressRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).derivative_market_address(request).await
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
                        let method = DerivativeMarketAddressSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SubaccountTradeNonce" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountTradeNonceSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QuerySubaccountTradeNonceRequest,
                    > for SubaccountTradeNonceSvc<T> {
                        type Response = super::QuerySubaccountTradeNonceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySubaccountTradeNonceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_trade_nonce(request).await
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
                        let method = SubaccountTradeNonceSvc(inner);
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
                "/injective.exchange.v1beta1.Query/ExchangeModuleState" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeModuleStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryModuleStateRequest>
                    for ExchangeModuleStateSvc<T> {
                        type Response = super::QueryModuleStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryModuleStateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).exchange_module_state(request).await
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
                        let method = ExchangeModuleStateSvc(inner);
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
                "/injective.exchange.v1beta1.Query/Positions" => {
                    #[allow(non_camel_case_types)]
                    struct PositionsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryPositionsRequest>
                    for PositionsSvc<T> {
                        type Response = super::QueryPositionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPositionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).positions(request).await };
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
                        let method = PositionsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SubaccountPositions" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountPositionsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySubaccountPositionsRequest>
                    for SubaccountPositionsSvc<T> {
                        type Response = super::QuerySubaccountPositionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySubaccountPositionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_positions(request).await
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
                        let method = SubaccountPositionsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SubaccountPositionInMarket" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountPositionInMarketSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QuerySubaccountPositionInMarketRequest,
                    > for SubaccountPositionInMarketSvc<T> {
                        type Response = super::QuerySubaccountPositionInMarketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySubaccountPositionInMarketRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_position_in_market(request).await
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
                        let method = SubaccountPositionInMarketSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SubaccountEffectivePositionInMarket" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountEffectivePositionInMarketSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QuerySubaccountEffectivePositionInMarketRequest,
                    > for SubaccountEffectivePositionInMarketSvc<T> {
                        type Response = super::QuerySubaccountEffectivePositionInMarketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySubaccountEffectivePositionInMarketRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .subaccount_effective_position_in_market(request)
                                    .await
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
                        let method = SubaccountEffectivePositionInMarketSvc(inner);
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
                "/injective.exchange.v1beta1.Query/PerpetualMarketInfo" => {
                    #[allow(non_camel_case_types)]
                    struct PerpetualMarketInfoSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryPerpetualMarketInfoRequest>
                    for PerpetualMarketInfoSvc<T> {
                        type Response = super::QueryPerpetualMarketInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryPerpetualMarketInfoRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).perpetual_market_info(request).await
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
                        let method = PerpetualMarketInfoSvc(inner);
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
                "/injective.exchange.v1beta1.Query/ExpiryFuturesMarketInfo" => {
                    #[allow(non_camel_case_types)]
                    struct ExpiryFuturesMarketInfoSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryExpiryFuturesMarketInfoRequest,
                    > for ExpiryFuturesMarketInfoSvc<T> {
                        type Response = super::QueryExpiryFuturesMarketInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryExpiryFuturesMarketInfoRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).expiry_futures_market_info(request).await
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
                        let method = ExpiryFuturesMarketInfoSvc(inner);
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
                "/injective.exchange.v1beta1.Query/PerpetualMarketFunding" => {
                    #[allow(non_camel_case_types)]
                    struct PerpetualMarketFundingSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryPerpetualMarketFundingRequest,
                    > for PerpetualMarketFundingSvc<T> {
                        type Response = super::QueryPerpetualMarketFundingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryPerpetualMarketFundingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).perpetual_market_funding(request).await
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
                        let method = PerpetualMarketFundingSvc(inner);
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
                "/injective.exchange.v1beta1.Query/SubaccountOrderMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountOrderMetadataSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QuerySubaccountOrderMetadataRequest,
                    > for SubaccountOrderMetadataSvc<T> {
                        type Response = super::QuerySubaccountOrderMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySubaccountOrderMetadataRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_order_metadata(request).await
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
                        let method = SubaccountOrderMetadataSvc(inner);
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
                "/injective.exchange.v1beta1.Query/TradeRewardPoints" => {
                    #[allow(non_camel_case_types)]
                    struct TradeRewardPointsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTradeRewardPointsRequest>
                    for TradeRewardPointsSvc<T> {
                        type Response = super::QueryTradeRewardPointsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTradeRewardPointsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trade_reward_points(request).await
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
                        let method = TradeRewardPointsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/PendingTradeRewardPoints" => {
                    #[allow(non_camel_case_types)]
                    struct PendingTradeRewardPointsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTradeRewardPointsRequest>
                    for PendingTradeRewardPointsSvc<T> {
                        type Response = super::QueryTradeRewardPointsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTradeRewardPointsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).pending_trade_reward_points(request).await
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
                        let method = PendingTradeRewardPointsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/TradeRewardCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct TradeRewardCampaignSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTradeRewardCampaignRequest>
                    for TradeRewardCampaignSvc<T> {
                        type Response = super::QueryTradeRewardCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTradeRewardCampaignRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trade_reward_campaign(request).await
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
                        let method = TradeRewardCampaignSvc(inner);
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
                "/injective.exchange.v1beta1.Query/FeeDiscountAccountInfo" => {
                    #[allow(non_camel_case_types)]
                    struct FeeDiscountAccountInfoSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryFeeDiscountAccountInfoRequest,
                    > for FeeDiscountAccountInfoSvc<T> {
                        type Response = super::QueryFeeDiscountAccountInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryFeeDiscountAccountInfoRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).fee_discount_account_info(request).await
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
                        let method = FeeDiscountAccountInfoSvc(inner);
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
                "/injective.exchange.v1beta1.Query/FeeDiscountSchedule" => {
                    #[allow(non_camel_case_types)]
                    struct FeeDiscountScheduleSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryFeeDiscountScheduleRequest>
                    for FeeDiscountScheduleSvc<T> {
                        type Response = super::QueryFeeDiscountScheduleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryFeeDiscountScheduleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).fee_discount_schedule(request).await
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
                        let method = FeeDiscountScheduleSvc(inner);
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
                "/injective.exchange.v1beta1.Query/BalanceMismatches" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceMismatchesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryBalanceMismatchesRequest>
                    for BalanceMismatchesSvc<T> {
                        type Response = super::QueryBalanceMismatchesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBalanceMismatchesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).balance_mismatches(request).await
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
                        let method = BalanceMismatchesSvc(inner);
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
                "/injective.exchange.v1beta1.Query/BalanceWithBalanceHolds" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceWithBalanceHoldsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryBalanceWithBalanceHoldsRequest,
                    > for BalanceWithBalanceHoldsSvc<T> {
                        type Response = super::QueryBalanceWithBalanceHoldsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryBalanceWithBalanceHoldsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).balance_with_balance_holds(request).await
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
                        let method = BalanceWithBalanceHoldsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/FeeDiscountTierStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct FeeDiscountTierStatisticsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryFeeDiscountTierStatisticsRequest,
                    > for FeeDiscountTierStatisticsSvc<T> {
                        type Response = super::QueryFeeDiscountTierStatisticsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryFeeDiscountTierStatisticsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).fee_discount_tier_statistics(request).await
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
                        let method = FeeDiscountTierStatisticsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/MitoVaultInfos" => {
                    #[allow(non_camel_case_types)]
                    struct MitoVaultInfosSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::MitoVaultInfosRequest>
                    for MitoVaultInfosSvc<T> {
                        type Response = super::MitoVaultInfosResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MitoVaultInfosRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mito_vault_infos(request).await
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
                        let method = MitoVaultInfosSvc(inner);
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
                "/injective.exchange.v1beta1.Query/QueryMarketIDFromVault" => {
                    #[allow(non_camel_case_types)]
                    struct QueryMarketIDFromVaultSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryMarketIdFromVaultRequest>
                    for QueryMarketIDFromVaultSvc<T> {
                        type Response = super::QueryMarketIdFromVaultResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryMarketIdFromVaultRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_market_id_from_vault(request).await
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
                        let method = QueryMarketIDFromVaultSvc(inner);
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
                "/injective.exchange.v1beta1.Query/HistoricalTradeRecords" => {
                    #[allow(non_camel_case_types)]
                    struct HistoricalTradeRecordsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryHistoricalTradeRecordsRequest,
                    > for HistoricalTradeRecordsSvc<T> {
                        type Response = super::QueryHistoricalTradeRecordsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryHistoricalTradeRecordsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).historical_trade_records(request).await
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
                        let method = HistoricalTradeRecordsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/IsOptedOutOfRewards" => {
                    #[allow(non_camel_case_types)]
                    struct IsOptedOutOfRewardsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryIsOptedOutOfRewardsRequest>
                    for IsOptedOutOfRewardsSvc<T> {
                        type Response = super::QueryIsOptedOutOfRewardsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryIsOptedOutOfRewardsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).is_opted_out_of_rewards(request).await
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
                        let method = IsOptedOutOfRewardsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/OptedOutOfRewardsAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct OptedOutOfRewardsAccountsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryOptedOutOfRewardsAccountsRequest,
                    > for OptedOutOfRewardsAccountsSvc<T> {
                        type Response = super::QueryOptedOutOfRewardsAccountsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryOptedOutOfRewardsAccountsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).opted_out_of_rewards_accounts(request).await
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
                        let method = OptedOutOfRewardsAccountsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/MarketVolatility" => {
                    #[allow(non_camel_case_types)]
                    struct MarketVolatilitySvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryMarketVolatilityRequest>
                    for MarketVolatilitySvc<T> {
                        type Response = super::QueryMarketVolatilityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryMarketVolatilityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).market_volatility(request).await
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
                        let method = MarketVolatilitySvc(inner);
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
                "/injective.exchange.v1beta1.Query/BinaryOptionsMarkets" => {
                    #[allow(non_camel_case_types)]
                    struct BinaryOptionsMarketsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryBinaryMarketsRequest>
                    for BinaryOptionsMarketsSvc<T> {
                        type Response = super::QueryBinaryMarketsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBinaryMarketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).binary_options_markets(request).await
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
                        let method = BinaryOptionsMarketsSvc(inner);
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
                "/injective.exchange.v1beta1.Query/TraderDerivativeConditionalOrders" => {
                    #[allow(non_camel_case_types)]
                    struct TraderDerivativeConditionalOrdersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTraderDerivativeConditionalOrdersRequest,
                    > for TraderDerivativeConditionalOrdersSvc<T> {
                        type Response = super::QueryTraderDerivativeConditionalOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTraderDerivativeConditionalOrdersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trader_derivative_conditional_orders(request).await
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
                        let method = TraderDerivativeConditionalOrdersSvc(inner);
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
                "/injective.exchange.v1beta1.Query/MarketAtomicExecutionFeeMultiplier" => {
                    #[allow(non_camel_case_types)]
                    struct MarketAtomicExecutionFeeMultiplierSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryMarketAtomicExecutionFeeMultiplierRequest,
                    > for MarketAtomicExecutionFeeMultiplierSvc<T> {
                        type Response = super::QueryMarketAtomicExecutionFeeMultiplierResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryMarketAtomicExecutionFeeMultiplierRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .market_atomic_execution_fee_multiplier(request)
                                    .await
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
                        let method = MarketAtomicExecutionFeeMultiplierSvc(inner);
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
    impl<T: Query> Clone for QueryServer<T> {
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
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "injective.exchange.v1beta1.Query";
    }
}
