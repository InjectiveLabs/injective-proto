// @generated
/// Generated client implementations.
pub mod injective_accounts_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InjectiveAccountsRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InjectiveAccountsRpcClient<tonic::transport::Channel> {
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
    impl<T> InjectiveAccountsRpcClient<T>
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
        ) -> InjectiveAccountsRpcClient<InterceptedService<T, F>>
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
            InjectiveAccountsRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PortfolioResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/Portfolio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "Portfolio",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn order_states(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderStatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderStatesResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/OrderStates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "OrderStates",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccounts_list(
            &mut self,
            request: impl tonic::IntoRequest<super::SubaccountsListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountsListResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountsList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "SubaccountsList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_balances_list(
            &mut self,
            request: impl tonic::IntoRequest<super::SubaccountBalancesListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountBalancesListResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountBalancesList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "SubaccountBalancesList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_balance_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::SubaccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountBalanceResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountBalanceEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "SubaccountBalanceEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_subaccount_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamSubaccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamSubaccountBalanceResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/StreamSubaccountBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "StreamSubaccountBalance",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subaccount_history(
            &mut self,
            request: impl tonic::IntoRequest<super::SubaccountHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountHistoryResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "SubaccountHistory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subaccount_order_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::SubaccountOrderSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountOrderSummaryResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountOrderSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "SubaccountOrderSummary",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::RewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RewardsResponse>,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/Rewards",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_accounts_rpc.InjectiveAccountsRPC",
                        "Rewards",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod injective_accounts_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InjectiveAccountsRpcServer.
    #[async_trait]
    pub trait InjectiveAccountsRpc: Send + Sync + 'static {
        async fn portfolio(
            &self,
            request: tonic::Request<super::PortfolioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PortfolioResponse>,
            tonic::Status,
        >;
        async fn order_states(
            &self,
            request: tonic::Request<super::OrderStatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderStatesResponse>,
            tonic::Status,
        >;
        async fn subaccounts_list(
            &self,
            request: tonic::Request<super::SubaccountsListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountsListResponse>,
            tonic::Status,
        >;
        async fn subaccount_balances_list(
            &self,
            request: tonic::Request<super::SubaccountBalancesListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountBalancesListResponse>,
            tonic::Status,
        >;
        async fn subaccount_balance_endpoint(
            &self,
            request: tonic::Request<super::SubaccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountBalanceResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamSubaccountBalance method.
        type StreamSubaccountBalanceStream: futures_core::Stream<
                Item = std::result::Result<
                    super::StreamSubaccountBalanceResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_subaccount_balance(
            &self,
            request: tonic::Request<super::StreamSubaccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamSubaccountBalanceStream>,
            tonic::Status,
        >;
        async fn subaccount_history(
            &self,
            request: tonic::Request<super::SubaccountHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountHistoryResponse>,
            tonic::Status,
        >;
        async fn subaccount_order_summary(
            &self,
            request: tonic::Request<super::SubaccountOrderSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubaccountOrderSummaryResponse>,
            tonic::Status,
        >;
        async fn rewards(
            &self,
            request: tonic::Request<super::RewardsRequest>,
        ) -> std::result::Result<tonic::Response<super::RewardsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct InjectiveAccountsRpcServer<T: InjectiveAccountsRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InjectiveAccountsRpc> InjectiveAccountsRpcServer<T> {
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
    for InjectiveAccountsRpcServer<T>
    where
        T: InjectiveAccountsRpc,
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/Portfolio" => {
                    #[allow(non_camel_case_types)]
                    struct PortfolioSvc<T: InjectiveAccountsRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::UnaryService<super::PortfolioRequest>
                    for PortfolioSvc<T> {
                        type Response = super::PortfolioResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PortfolioRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).portfolio(request).await };
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
                        let method = PortfolioSvc(inner);
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/OrderStates" => {
                    #[allow(non_camel_case_types)]
                    struct OrderStatesSvc<T: InjectiveAccountsRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::UnaryService<super::OrderStatesRequest>
                    for OrderStatesSvc<T> {
                        type Response = super::OrderStatesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderStatesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).order_states(request).await
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
                        let method = OrderStatesSvc(inner);
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountsList" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountsListSvc<T: InjectiveAccountsRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::UnaryService<super::SubaccountsListRequest>
                    for SubaccountsListSvc<T> {
                        type Response = super::SubaccountsListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubaccountsListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccounts_list(request).await
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
                        let method = SubaccountsListSvc(inner);
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountBalancesList" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountBalancesListSvc<T: InjectiveAccountsRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::UnaryService<super::SubaccountBalancesListRequest>
                    for SubaccountBalancesListSvc<T> {
                        type Response = super::SubaccountBalancesListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubaccountBalancesListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_balances_list(request).await
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
                        let method = SubaccountBalancesListSvc(inner);
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountBalanceEndpoint" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountBalanceEndpointSvc<T: InjectiveAccountsRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::UnaryService<super::SubaccountBalanceRequest>
                    for SubaccountBalanceEndpointSvc<T> {
                        type Response = super::SubaccountBalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubaccountBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_balance_endpoint(request).await
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
                        let method = SubaccountBalanceEndpointSvc(inner);
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/StreamSubaccountBalance" => {
                    #[allow(non_camel_case_types)]
                    struct StreamSubaccountBalanceSvc<T: InjectiveAccountsRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::ServerStreamingService<
                        super::StreamSubaccountBalanceRequest,
                    > for StreamSubaccountBalanceSvc<T> {
                        type Response = super::StreamSubaccountBalanceResponse;
                        type ResponseStream = T::StreamSubaccountBalanceStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::StreamSubaccountBalanceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_subaccount_balance(request).await
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
                        let method = StreamSubaccountBalanceSvc(inner);
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountHistory" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountHistorySvc<T: InjectiveAccountsRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::UnaryService<super::SubaccountHistoryRequest>
                    for SubaccountHistorySvc<T> {
                        type Response = super::SubaccountHistoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubaccountHistoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_history(request).await
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
                        let method = SubaccountHistorySvc(inner);
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/SubaccountOrderSummary" => {
                    #[allow(non_camel_case_types)]
                    struct SubaccountOrderSummarySvc<T: InjectiveAccountsRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::UnaryService<super::SubaccountOrderSummaryRequest>
                    for SubaccountOrderSummarySvc<T> {
                        type Response = super::SubaccountOrderSummaryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubaccountOrderSummaryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subaccount_order_summary(request).await
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
                        let method = SubaccountOrderSummarySvc(inner);
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
                "/injective_accounts_rpc.InjectiveAccountsRPC/Rewards" => {
                    #[allow(non_camel_case_types)]
                    struct RewardsSvc<T: InjectiveAccountsRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveAccountsRpc,
                    > tonic::server::UnaryService<super::RewardsRequest>
                    for RewardsSvc<T> {
                        type Response = super::RewardsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RewardsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).rewards(request).await };
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
                        let method = RewardsSvc(inner);
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
    impl<T: InjectiveAccountsRpc> Clone for InjectiveAccountsRpcServer<T> {
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
    impl<T: InjectiveAccountsRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InjectiveAccountsRpc> tonic::server::NamedService
    for InjectiveAccountsRpcServer<T> {
        const NAME: &'static str = "injective_accounts_rpc.InjectiveAccountsRPC";
    }
}
