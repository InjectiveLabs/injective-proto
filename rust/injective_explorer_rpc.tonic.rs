// @generated
/// Generated client implementations.
pub mod injective_explorer_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InjectiveExplorerRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InjectiveExplorerRpcClient<tonic::transport::Channel> {
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
    impl<T> InjectiveExplorerRpcClient<T>
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
        ) -> InjectiveExplorerRpcClient<InterceptedService<T, F>>
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
            InjectiveExplorerRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_account_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountTxsResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetAccountTxs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetAccountTxs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_contract_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContractTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetContractTxsResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetContractTxs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetContractTxs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetBlocks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlockResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_validators(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidatorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorsResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetValidators",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetValidators",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_validator(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetValidator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetValidator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_validator_uptime(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidatorUptimeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorUptimeResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetValidatorUptime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetValidatorUptime",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTxsRequest>,
        ) -> std::result::Result<tonic::Response<super::GetTxsResponse>, tonic::Status> {
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetTxs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetTxs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_tx_by_tx_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTxByTxHashRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTxByTxHashResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetTxByTxHash",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetTxByTxHash",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_peggy_deposit_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPeggyDepositTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPeggyDepositTxsResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetPeggyDepositTxs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetPeggyDepositTxs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_peggy_withdrawal_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPeggyWithdrawalTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPeggyWithdrawalTxsResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetPeggyWithdrawalTxs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetPeggyWithdrawalTxs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ibc_transfer_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIbcTransferTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetIbcTransferTxsResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetIBCTransferTxs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetIBCTransferTxs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_wasm_codes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWasmCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWasmCodesResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetWasmCodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetWasmCodes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_wasm_code_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWasmCodeByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWasmCodeByIdResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetWasmCodeByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetWasmCodeByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_wasm_contracts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWasmContractsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWasmContractsResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetWasmContracts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetWasmContracts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_wasm_contract_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWasmContractByAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWasmContractByAddressResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetWasmContractByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetWasmContractByAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cw20_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCw20BalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCw20BalanceResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetCw20Balance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "GetCw20Balance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn relayers(
            &mut self,
            request: impl tonic::IntoRequest<super::RelayersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RelayersResponse>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/Relayers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "Relayers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamTxsResponse>>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/StreamTxs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "StreamTxs",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn stream_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamBlocksResponse>>,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/StreamBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_explorer_rpc.InjectiveExplorerRPC",
                        "StreamBlocks",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod injective_explorer_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InjectiveExplorerRpcServer.
    #[async_trait]
    pub trait InjectiveExplorerRpc: Send + Sync + 'static {
        async fn get_account_txs(
            &self,
            request: tonic::Request<super::GetAccountTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountTxsResponse>,
            tonic::Status,
        >;
        async fn get_contract_txs(
            &self,
            request: tonic::Request<super::GetContractTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetContractTxsResponse>,
            tonic::Status,
        >;
        async fn get_blocks(
            &self,
            request: tonic::Request<super::GetBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksResponse>,
            tonic::Status,
        >;
        async fn get_block(
            &self,
            request: tonic::Request<super::GetBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlockResponse>,
            tonic::Status,
        >;
        async fn get_validators(
            &self,
            request: tonic::Request<super::GetValidatorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorsResponse>,
            tonic::Status,
        >;
        async fn get_validator(
            &self,
            request: tonic::Request<super::GetValidatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorResponse>,
            tonic::Status,
        >;
        async fn get_validator_uptime(
            &self,
            request: tonic::Request<super::GetValidatorUptimeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorUptimeResponse>,
            tonic::Status,
        >;
        async fn get_txs(
            &self,
            request: tonic::Request<super::GetTxsRequest>,
        ) -> std::result::Result<tonic::Response<super::GetTxsResponse>, tonic::Status>;
        async fn get_tx_by_tx_hash(
            &self,
            request: tonic::Request<super::GetTxByTxHashRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTxByTxHashResponse>,
            tonic::Status,
        >;
        async fn get_peggy_deposit_txs(
            &self,
            request: tonic::Request<super::GetPeggyDepositTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPeggyDepositTxsResponse>,
            tonic::Status,
        >;
        async fn get_peggy_withdrawal_txs(
            &self,
            request: tonic::Request<super::GetPeggyWithdrawalTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPeggyWithdrawalTxsResponse>,
            tonic::Status,
        >;
        async fn get_ibc_transfer_txs(
            &self,
            request: tonic::Request<super::GetIbcTransferTxsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetIbcTransferTxsResponse>,
            tonic::Status,
        >;
        async fn get_wasm_codes(
            &self,
            request: tonic::Request<super::GetWasmCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWasmCodesResponse>,
            tonic::Status,
        >;
        async fn get_wasm_code_by_id(
            &self,
            request: tonic::Request<super::GetWasmCodeByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWasmCodeByIdResponse>,
            tonic::Status,
        >;
        async fn get_wasm_contracts(
            &self,
            request: tonic::Request<super::GetWasmContractsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWasmContractsResponse>,
            tonic::Status,
        >;
        async fn get_wasm_contract_by_address(
            &self,
            request: tonic::Request<super::GetWasmContractByAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWasmContractByAddressResponse>,
            tonic::Status,
        >;
        async fn get_cw20_balance(
            &self,
            request: tonic::Request<super::GetCw20BalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCw20BalanceResponse>,
            tonic::Status,
        >;
        async fn relayers(
            &self,
            request: tonic::Request<super::RelayersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RelayersResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamTxs method.
        type StreamTxsStream: futures_core::Stream<
                Item = std::result::Result<super::StreamTxsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_txs(
            &self,
            request: tonic::Request<super::StreamTxsRequest>,
        ) -> std::result::Result<tonic::Response<Self::StreamTxsStream>, tonic::Status>;
        /// Server streaming response type for the StreamBlocks method.
        type StreamBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::StreamBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_blocks(
            &self,
            request: tonic::Request<super::StreamBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamBlocksStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct InjectiveExplorerRpcServer<T: InjectiveExplorerRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InjectiveExplorerRpc> InjectiveExplorerRpcServer<T> {
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
    for InjectiveExplorerRpcServer<T>
    where
        T: InjectiveExplorerRpc,
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetAccountTxs" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountTxsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetAccountTxsRequest>
                    for GetAccountTxsSvc<T> {
                        type Response = super::GetAccountTxsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountTxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_txs(request).await
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
                        let method = GetAccountTxsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetContractTxs" => {
                    #[allow(non_camel_case_types)]
                    struct GetContractTxsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetContractTxsRequest>
                    for GetContractTxsSvc<T> {
                        type Response = super::GetContractTxsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetContractTxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_contract_txs(request).await
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
                        let method = GetContractTxsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetBlocksRequest>
                    for GetBlocksSvc<T> {
                        type Response = super::GetBlocksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_blocks(request).await };
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
                        let method = GetBlocksSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetBlockRequest>
                    for GetBlockSvc<T> {
                        type Response = super::GetBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_block(request).await };
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
                        let method = GetBlockSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetValidators" => {
                    #[allow(non_camel_case_types)]
                    struct GetValidatorsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetValidatorsRequest>
                    for GetValidatorsSvc<T> {
                        type Response = super::GetValidatorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetValidatorsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_validators(request).await
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
                        let method = GetValidatorsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetValidator" => {
                    #[allow(non_camel_case_types)]
                    struct GetValidatorSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetValidatorRequest>
                    for GetValidatorSvc<T> {
                        type Response = super::GetValidatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetValidatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_validator(request).await
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
                        let method = GetValidatorSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetValidatorUptime" => {
                    #[allow(non_camel_case_types)]
                    struct GetValidatorUptimeSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetValidatorUptimeRequest>
                    for GetValidatorUptimeSvc<T> {
                        type Response = super::GetValidatorUptimeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetValidatorUptimeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_validator_uptime(request).await
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
                        let method = GetValidatorUptimeSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetTxs" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetTxsRequest>
                    for GetTxsSvc<T> {
                        type Response = super::GetTxsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_txs(request).await };
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
                        let method = GetTxsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetTxByTxHash" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxByTxHashSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetTxByTxHashRequest>
                    for GetTxByTxHashSvc<T> {
                        type Response = super::GetTxByTxHashResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTxByTxHashRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_tx_by_tx_hash(request).await
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
                        let method = GetTxByTxHashSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetPeggyDepositTxs" => {
                    #[allow(non_camel_case_types)]
                    struct GetPeggyDepositTxsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetPeggyDepositTxsRequest>
                    for GetPeggyDepositTxsSvc<T> {
                        type Response = super::GetPeggyDepositTxsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPeggyDepositTxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_peggy_deposit_txs(request).await
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
                        let method = GetPeggyDepositTxsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetPeggyWithdrawalTxs" => {
                    #[allow(non_camel_case_types)]
                    struct GetPeggyWithdrawalTxsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetPeggyWithdrawalTxsRequest>
                    for GetPeggyWithdrawalTxsSvc<T> {
                        type Response = super::GetPeggyWithdrawalTxsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPeggyWithdrawalTxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_peggy_withdrawal_txs(request).await
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
                        let method = GetPeggyWithdrawalTxsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetIBCTransferTxs" => {
                    #[allow(non_camel_case_types)]
                    struct GetIBCTransferTxsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetIbcTransferTxsRequest>
                    for GetIBCTransferTxsSvc<T> {
                        type Response = super::GetIbcTransferTxsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIbcTransferTxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_ibc_transfer_txs(request).await
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
                        let method = GetIBCTransferTxsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetWasmCodes" => {
                    #[allow(non_camel_case_types)]
                    struct GetWasmCodesSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetWasmCodesRequest>
                    for GetWasmCodesSvc<T> {
                        type Response = super::GetWasmCodesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWasmCodesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_wasm_codes(request).await
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
                        let method = GetWasmCodesSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetWasmCodeByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetWasmCodeByIDSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetWasmCodeByIdRequest>
                    for GetWasmCodeByIDSvc<T> {
                        type Response = super::GetWasmCodeByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWasmCodeByIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_wasm_code_by_id(request).await
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
                        let method = GetWasmCodeByIDSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetWasmContracts" => {
                    #[allow(non_camel_case_types)]
                    struct GetWasmContractsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetWasmContractsRequest>
                    for GetWasmContractsSvc<T> {
                        type Response = super::GetWasmContractsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWasmContractsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_wasm_contracts(request).await
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
                        let method = GetWasmContractsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetWasmContractByAddress" => {
                    #[allow(non_camel_case_types)]
                    struct GetWasmContractByAddressSvc<T: InjectiveExplorerRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetWasmContractByAddressRequest>
                    for GetWasmContractByAddressSvc<T> {
                        type Response = super::GetWasmContractByAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetWasmContractByAddressRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_wasm_contract_by_address(request).await
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
                        let method = GetWasmContractByAddressSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/GetCw20Balance" => {
                    #[allow(non_camel_case_types)]
                    struct GetCw20BalanceSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::GetCw20BalanceRequest>
                    for GetCw20BalanceSvc<T> {
                        type Response = super::GetCw20BalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCw20BalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_cw20_balance(request).await
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
                        let method = GetCw20BalanceSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/Relayers" => {
                    #[allow(non_camel_case_types)]
                    struct RelayersSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::UnaryService<super::RelayersRequest>
                    for RelayersSvc<T> {
                        type Response = super::RelayersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RelayersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).relayers(request).await };
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
                        let method = RelayersSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/StreamTxs" => {
                    #[allow(non_camel_case_types)]
                    struct StreamTxsSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::ServerStreamingService<super::StreamTxsRequest>
                    for StreamTxsSvc<T> {
                        type Response = super::StreamTxsResponse;
                        type ResponseStream = T::StreamTxsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamTxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).stream_txs(request).await };
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
                        let method = StreamTxsSvc(inner);
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
                "/injective_explorer_rpc.InjectiveExplorerRPC/StreamBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct StreamBlocksSvc<T: InjectiveExplorerRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveExplorerRpc,
                    > tonic::server::ServerStreamingService<super::StreamBlocksRequest>
                    for StreamBlocksSvc<T> {
                        type Response = super::StreamBlocksResponse;
                        type ResponseStream = T::StreamBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamBlocksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_blocks(request).await
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
                        let method = StreamBlocksSvc(inner);
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
    impl<T: InjectiveExplorerRpc> Clone for InjectiveExplorerRpcServer<T> {
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
    impl<T: InjectiveExplorerRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InjectiveExplorerRpc> tonic::server::NamedService
    for InjectiveExplorerRpcServer<T> {
        const NAME: &'static str = "injective_explorer_rpc.InjectiveExplorerRPC";
    }
}
