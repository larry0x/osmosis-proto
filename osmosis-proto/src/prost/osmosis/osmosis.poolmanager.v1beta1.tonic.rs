// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
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
            Self {
                inner,
            }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self {
                inner,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
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
        pub async fn swap_exact_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSwapExactAmountIn>,
        ) -> std::result::Result<tonic::Response<super::MsgSwapExactAmountInResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Msg/SwapExactAmountIn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Msg", "SwapExactAmountIn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn swap_exact_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSwapExactAmountOut>,
        ) -> std::result::Result<tonic::Response<super::MsgSwapExactAmountOutResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Msg/SwapExactAmountOut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Msg", "SwapExactAmountOut"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn split_route_swap_exact_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSplitRouteSwapExactAmountIn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSplitRouteSwapExactAmountInResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Msg/SplitRouteSwapExactAmountIn",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Msg",
                "SplitRouteSwapExactAmountIn",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn split_route_swap_exact_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSplitRouteSwapExactAmountOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSplitRouteSwapExactAmountOutResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Msg/SplitRouteSwapExactAmountOut",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Msg",
                "SplitRouteSwapExactAmountOut",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_denom_pair_taker_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetDenomPairTakerFee>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetDenomPairTakerFeeResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Msg/SetDenomPairTakerFee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Msg", "SetDenomPairTakerFee"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_taker_fee_share_agreement_for_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetTakerFeeShareAgreementForDenom>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetTakerFeeShareAgreementForDenomResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Msg/SetTakerFeeShareAgreementForDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Msg",
                "SetTakerFeeShareAgreementForDenom",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_registered_alloyed_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetRegisteredAlloyedPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetRegisteredAlloyedPoolResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Msg/SetRegisteredAlloyedPool",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Msg",
                "SetRegisteredAlloyedPool",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        async fn swap_exact_amount_in(
            &self,
            request: tonic::Request<super::MsgSwapExactAmountIn>,
        ) -> std::result::Result<tonic::Response<super::MsgSwapExactAmountInResponse>, tonic::Status>;
        async fn swap_exact_amount_out(
            &self,
            request: tonic::Request<super::MsgSwapExactAmountOut>,
        ) -> std::result::Result<tonic::Response<super::MsgSwapExactAmountOutResponse>, tonic::Status>;
        async fn split_route_swap_exact_amount_in(
            &self,
            request: tonic::Request<super::MsgSplitRouteSwapExactAmountIn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSplitRouteSwapExactAmountInResponse>,
            tonic::Status,
        >;
        async fn split_route_swap_exact_amount_out(
            &self,
            request: tonic::Request<super::MsgSplitRouteSwapExactAmountOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSplitRouteSwapExactAmountOutResponse>,
            tonic::Status,
        >;
        async fn set_denom_pair_taker_fee(
            &self,
            request: tonic::Request<super::MsgSetDenomPairTakerFee>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetDenomPairTakerFeeResponse>,
            tonic::Status,
        >;
        async fn set_taker_fee_share_agreement_for_denom(
            &self,
            request: tonic::Request<super::MsgSetTakerFeeShareAgreementForDenom>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetTakerFeeShareAgreementForDenomResponse>,
            tonic::Status,
        >;
        async fn set_registered_alloyed_pool(
            &self,
            request: tonic::Request<super::MsgSetRegisteredAlloyedPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetRegisteredAlloyedPoolResponse>,
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
                "/osmosis.poolmanager.v1beta1.Msg/SwapExactAmountIn" => {
                    #[allow(non_camel_case_types)]
                    struct SwapExactAmountInSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSwapExactAmountIn> for SwapExactAmountInSvc<T> {
                        type Response = super::MsgSwapExactAmountInResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSwapExactAmountIn>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).swap_exact_amount_in(request).await };
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
                        let method = SwapExactAmountInSvc(inner);
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
                },
                "/osmosis.poolmanager.v1beta1.Msg/SwapExactAmountOut" => {
                    #[allow(non_camel_case_types)]
                    struct SwapExactAmountOutSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSwapExactAmountOut>
                        for SwapExactAmountOutSvc<T>
                    {
                        type Response = super::MsgSwapExactAmountOutResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSwapExactAmountOut>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).swap_exact_amount_out(request).await };
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
                        let method = SwapExactAmountOutSvc(inner);
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
                },
                "/osmosis.poolmanager.v1beta1.Msg/SplitRouteSwapExactAmountIn" => {
                    #[allow(non_camel_case_types)]
                    struct SplitRouteSwapExactAmountInSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSplitRouteSwapExactAmountIn>
                        for SplitRouteSwapExactAmountInSvc<T>
                    {
                        type Response = super::MsgSplitRouteSwapExactAmountInResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSplitRouteSwapExactAmountIn>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).split_route_swap_exact_amount_in(request).await
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
                        let method = SplitRouteSwapExactAmountInSvc(inner);
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
                },
                "/osmosis.poolmanager.v1beta1.Msg/SplitRouteSwapExactAmountOut" => {
                    #[allow(non_camel_case_types)]
                    struct SplitRouteSwapExactAmountOutSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSplitRouteSwapExactAmountOut>
                        for SplitRouteSwapExactAmountOutSvc<T>
                    {
                        type Response = super::MsgSplitRouteSwapExactAmountOutResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSplitRouteSwapExactAmountOut>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).split_route_swap_exact_amount_out(request).await
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
                        let method = SplitRouteSwapExactAmountOutSvc(inner);
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
                },
                "/osmosis.poolmanager.v1beta1.Msg/SetDenomPairTakerFee" => {
                    #[allow(non_camel_case_types)]
                    struct SetDenomPairTakerFeeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetDenomPairTakerFee>
                        for SetDenomPairTakerFeeSvc<T>
                    {
                        type Response = super::MsgSetDenomPairTakerFeeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetDenomPairTakerFee>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).set_denom_pair_taker_fee(request).await };
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
                        let method = SetDenomPairTakerFeeSvc(inner);
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
                },
                "/osmosis.poolmanager.v1beta1.Msg/SetTakerFeeShareAgreementForDenom" => {
                    #[allow(non_camel_case_types)]
                    struct SetTakerFeeShareAgreementForDenomSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgSetTakerFeeShareAgreementForDenom>
                        for SetTakerFeeShareAgreementForDenomSvc<T>
                    {
                        type Response = super::MsgSetTakerFeeShareAgreementForDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetTakerFeeShareAgreementForDenom>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_taker_fee_share_agreement_for_denom(request).await
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
                        let method = SetTakerFeeShareAgreementForDenomSvc(inner);
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
                },
                "/osmosis.poolmanager.v1beta1.Msg/SetRegisteredAlloyedPool" => {
                    #[allow(non_camel_case_types)]
                    struct SetRegisteredAlloyedPoolSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetRegisteredAlloyedPool>
                        for SetRegisteredAlloyedPoolSvc<T>
                    {
                        type Response = super::MsgSetRegisteredAlloyedPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetRegisteredAlloyedPool>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).set_registered_alloyed_pool(request).await };
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
                        let method = SetRegisteredAlloyedPoolSvc(inner);
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
                },
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
        const NAME: &'static str = "osmosis.poolmanager.v1beta1.Msg";
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
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
            Self {
                inner,
            }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self {
                inner,
            }
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
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
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
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::ParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::ParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.poolmanager.v1beta1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_swap_exact_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateSwapExactAmountInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountInResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "EstimateSwapExactAmountIn",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_swap_exact_amount_in_with_primitive_types(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateSwapExactAmountInWithPrimitiveTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountInResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountInWithPrimitiveTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "EstimateSwapExactAmountInWithPrimitiveTypes",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_single_pool_swap_exact_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateSinglePoolSwapExactAmountInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountInResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/EstimateSinglePoolSwapExactAmountIn",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "EstimateSinglePoolSwapExactAmountIn",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_swap_exact_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateSwapExactAmountOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountOutResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "EstimateSwapExactAmountOut",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_swap_exact_amount_out_with_primitive_types(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateSwapExactAmountOutWithPrimitiveTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountOutResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOutWithPrimitiveTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "EstimateSwapExactAmountOutWithPrimitiveTypes",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_single_pool_swap_exact_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateSinglePoolSwapExactAmountOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountOutResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/EstimateSinglePoolSwapExactAmountOut",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "EstimateSinglePoolSwapExactAmountOut",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn num_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::NumPoolsRequest>,
        ) -> std::result::Result<tonic::Response<super::NumPoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.poolmanager.v1beta1.Query/NumPools");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "NumPools"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool(
            &mut self,
            request: impl tonic::IntoRequest<super::PoolRequest>,
        ) -> std::result::Result<tonic::Response<super::PoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.poolmanager.v1beta1.Query/Pool");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "Pool"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::AllPoolsRequest>,
        ) -> std::result::Result<tonic::Response<super::AllPoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.poolmanager.v1beta1.Query/AllPools");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "AllPools"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_pools_by_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPoolsByDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::ListPoolsByDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/ListPoolsByDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "ListPoolsByDenom"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn spot_price(
            &mut self,
            request: impl tonic::IntoRequest<super::SpotPriceRequest>,
        ) -> std::result::Result<tonic::Response<super::SpotPriceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/SpotPrice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "SpotPrice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_pool_liquidity(
            &mut self,
            request: impl tonic::IntoRequest<super::TotalPoolLiquidityRequest>,
        ) -> std::result::Result<tonic::Response<super::TotalPoolLiquidityResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/TotalPoolLiquidity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "TotalPoolLiquidity"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_liquidity(
            &mut self,
            request: impl tonic::IntoRequest<super::TotalLiquidityRequest>,
        ) -> std::result::Result<tonic::Response<super::TotalLiquidityResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/TotalLiquidity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "TotalLiquidity"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_volume_for_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::TotalVolumeForPoolRequest>,
        ) -> std::result::Result<tonic::Response<super::TotalVolumeForPoolResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/TotalVolumeForPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.poolmanager.v1beta1.Query", "TotalVolumeForPool"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn trading_pair_taker_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::TradingPairTakerFeeRequest>,
        ) -> std::result::Result<tonic::Response<super::TradingPairTakerFeeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/TradingPairTakerFee",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "TradingPairTakerFee",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_trade_based_on_price_impact(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateTradeBasedOnPriceImpactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateTradeBasedOnPriceImpactResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/EstimateTradeBasedOnPriceImpact",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "EstimateTradeBasedOnPriceImpact",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_taker_fee_share_agreements(
            &mut self,
            request: impl tonic::IntoRequest<super::AllTakerFeeShareAgreementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllTakerFeeShareAgreementsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/AllTakerFeeShareAgreements",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "AllTakerFeeShareAgreements",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn taker_fee_share_agreement_from_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::TakerFeeShareAgreementFromDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TakerFeeShareAgreementFromDenomResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/TakerFeeShareAgreementFromDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "TakerFeeShareAgreementFromDenom",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn taker_fee_share_denoms_to_accrued_value(
            &mut self,
            request: impl tonic::IntoRequest<super::TakerFeeShareDenomsToAccruedValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TakerFeeShareDenomsToAccruedValueResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/TakerFeeShareDenomsToAccruedValue",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "TakerFeeShareDenomsToAccruedValue",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_taker_fee_share_accumulators(
            &mut self,
            request: impl tonic::IntoRequest<super::AllTakerFeeShareAccumulatorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllTakerFeeShareAccumulatorsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/AllTakerFeeShareAccumulators",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "AllTakerFeeShareAccumulators",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn registered_alloyed_pool_from_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisteredAlloyedPoolFromDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisteredAlloyedPoolFromDenomResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/RegisteredAlloyedPoolFromDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "RegisteredAlloyedPoolFromDenom",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn registered_alloyed_pool_from_pool_id(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisteredAlloyedPoolFromPoolIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisteredAlloyedPoolFromPoolIdResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/RegisteredAlloyedPoolFromPoolId",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "RegisteredAlloyedPoolFromPoolId",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_registered_alloyed_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::AllRegisteredAlloyedPoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllRegisteredAlloyedPoolsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.poolmanager.v1beta1.Query/AllRegisteredAlloyedPools",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "osmosis.poolmanager.v1beta1.Query",
                "AllRegisteredAlloyedPools",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        async fn params(
            &self,
            request: tonic::Request<super::ParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::ParamsResponse>, tonic::Status>;
        async fn estimate_swap_exact_amount_in(
            &self,
            request: tonic::Request<super::EstimateSwapExactAmountInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountInResponse>,
            tonic::Status,
        >;
        async fn estimate_swap_exact_amount_in_with_primitive_types(
            &self,
            request: tonic::Request<super::EstimateSwapExactAmountInWithPrimitiveTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountInResponse>,
            tonic::Status,
        >;
        async fn estimate_single_pool_swap_exact_amount_in(
            &self,
            request: tonic::Request<super::EstimateSinglePoolSwapExactAmountInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountInResponse>,
            tonic::Status,
        >;
        async fn estimate_swap_exact_amount_out(
            &self,
            request: tonic::Request<super::EstimateSwapExactAmountOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountOutResponse>,
            tonic::Status,
        >;
        async fn estimate_swap_exact_amount_out_with_primitive_types(
            &self,
            request: tonic::Request<super::EstimateSwapExactAmountOutWithPrimitiveTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountOutResponse>,
            tonic::Status,
        >;
        async fn estimate_single_pool_swap_exact_amount_out(
            &self,
            request: tonic::Request<super::EstimateSinglePoolSwapExactAmountOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateSwapExactAmountOutResponse>,
            tonic::Status,
        >;
        async fn num_pools(
            &self,
            request: tonic::Request<super::NumPoolsRequest>,
        ) -> std::result::Result<tonic::Response<super::NumPoolsResponse>, tonic::Status>;
        async fn pool(
            &self,
            request: tonic::Request<super::PoolRequest>,
        ) -> std::result::Result<tonic::Response<super::PoolResponse>, tonic::Status>;
        async fn all_pools(
            &self,
            request: tonic::Request<super::AllPoolsRequest>,
        ) -> std::result::Result<tonic::Response<super::AllPoolsResponse>, tonic::Status>;
        async fn list_pools_by_denom(
            &self,
            request: tonic::Request<super::ListPoolsByDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::ListPoolsByDenomResponse>, tonic::Status>;
        async fn spot_price(
            &self,
            request: tonic::Request<super::SpotPriceRequest>,
        ) -> std::result::Result<tonic::Response<super::SpotPriceResponse>, tonic::Status>;
        async fn total_pool_liquidity(
            &self,
            request: tonic::Request<super::TotalPoolLiquidityRequest>,
        ) -> std::result::Result<tonic::Response<super::TotalPoolLiquidityResponse>, tonic::Status>;
        async fn total_liquidity(
            &self,
            request: tonic::Request<super::TotalLiquidityRequest>,
        ) -> std::result::Result<tonic::Response<super::TotalLiquidityResponse>, tonic::Status>;
        async fn total_volume_for_pool(
            &self,
            request: tonic::Request<super::TotalVolumeForPoolRequest>,
        ) -> std::result::Result<tonic::Response<super::TotalVolumeForPoolResponse>, tonic::Status>;
        async fn trading_pair_taker_fee(
            &self,
            request: tonic::Request<super::TradingPairTakerFeeRequest>,
        ) -> std::result::Result<tonic::Response<super::TradingPairTakerFeeResponse>, tonic::Status>;
        async fn estimate_trade_based_on_price_impact(
            &self,
            request: tonic::Request<super::EstimateTradeBasedOnPriceImpactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateTradeBasedOnPriceImpactResponse>,
            tonic::Status,
        >;
        async fn all_taker_fee_share_agreements(
            &self,
            request: tonic::Request<super::AllTakerFeeShareAgreementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllTakerFeeShareAgreementsResponse>,
            tonic::Status,
        >;
        async fn taker_fee_share_agreement_from_denom(
            &self,
            request: tonic::Request<super::TakerFeeShareAgreementFromDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TakerFeeShareAgreementFromDenomResponse>,
            tonic::Status,
        >;
        async fn taker_fee_share_denoms_to_accrued_value(
            &self,
            request: tonic::Request<super::TakerFeeShareDenomsToAccruedValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TakerFeeShareDenomsToAccruedValueResponse>,
            tonic::Status,
        >;
        async fn all_taker_fee_share_accumulators(
            &self,
            request: tonic::Request<super::AllTakerFeeShareAccumulatorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllTakerFeeShareAccumulatorsResponse>,
            tonic::Status,
        >;
        async fn registered_alloyed_pool_from_denom(
            &self,
            request: tonic::Request<super::RegisteredAlloyedPoolFromDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisteredAlloyedPoolFromDenomResponse>,
            tonic::Status,
        >;
        async fn registered_alloyed_pool_from_pool_id(
            &self,
            request: tonic::Request<super::RegisteredAlloyedPoolFromPoolIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisteredAlloyedPoolFromPoolIdResponse>,
            tonic::Status,
        >;
        async fn all_registered_alloyed_pools(
            &self,
            request: tonic::Request<super::AllRegisteredAlloyedPoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllRegisteredAlloyedPoolsResponse>,
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
                "/osmosis.poolmanager.v1beta1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ParamsRequest>
                    for ParamsSvc<T> {
                        type Response = super::ParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).params(request).await };
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
                        let method = ParamsSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSwapExactAmountInSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::EstimateSwapExactAmountInRequest,
                    > for EstimateSwapExactAmountInSvc<T> {
                        type Response = super::EstimateSwapExactAmountInResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EstimateSwapExactAmountInRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).estimate_swap_exact_amount_in(request).await
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
                        let method = EstimateSwapExactAmountInSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountInWithPrimitiveTypes" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSwapExactAmountInWithPrimitiveTypesSvc<T: Query>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::EstimateSwapExactAmountInWithPrimitiveTypesRequest,
                    > for EstimateSwapExactAmountInWithPrimitiveTypesSvc<T> {
                        type Response = super::EstimateSwapExactAmountInResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EstimateSwapExactAmountInWithPrimitiveTypesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .estimate_swap_exact_amount_in_with_primitive_types(request)
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
                        let method = EstimateSwapExactAmountInWithPrimitiveTypesSvc(
                            inner,
                        );
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
                "/osmosis.poolmanager.v1beta1.Query/EstimateSinglePoolSwapExactAmountIn" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSinglePoolSwapExactAmountInSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::EstimateSinglePoolSwapExactAmountInRequest,
                    > for EstimateSinglePoolSwapExactAmountInSvc<T> {
                        type Response = super::EstimateSwapExactAmountInResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EstimateSinglePoolSwapExactAmountInRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .estimate_single_pool_swap_exact_amount_in(request)
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
                        let method = EstimateSinglePoolSwapExactAmountInSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSwapExactAmountOutSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::EstimateSwapExactAmountOutRequest,
                    > for EstimateSwapExactAmountOutSvc<T> {
                        type Response = super::EstimateSwapExactAmountOutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EstimateSwapExactAmountOutRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).estimate_swap_exact_amount_out(request).await
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
                        let method = EstimateSwapExactAmountOutSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOutWithPrimitiveTypes" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSwapExactAmountOutWithPrimitiveTypesSvc<T: Query>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::EstimateSwapExactAmountOutWithPrimitiveTypesRequest,
                    > for EstimateSwapExactAmountOutWithPrimitiveTypesSvc<T> {
                        type Response = super::EstimateSwapExactAmountOutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EstimateSwapExactAmountOutWithPrimitiveTypesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .estimate_swap_exact_amount_out_with_primitive_types(
                                        request,
                                    )
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
                        let method = EstimateSwapExactAmountOutWithPrimitiveTypesSvc(
                            inner,
                        );
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
                "/osmosis.poolmanager.v1beta1.Query/EstimateSinglePoolSwapExactAmountOut" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSinglePoolSwapExactAmountOutSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::EstimateSinglePoolSwapExactAmountOutRequest,
                    > for EstimateSinglePoolSwapExactAmountOutSvc<T> {
                        type Response = super::EstimateSwapExactAmountOutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EstimateSinglePoolSwapExactAmountOutRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .estimate_single_pool_swap_exact_amount_out(request)
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
                        let method = EstimateSinglePoolSwapExactAmountOutSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/NumPools" => {
                    #[allow(non_camel_case_types)]
                    struct NumPoolsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::NumPoolsRequest>
                    for NumPoolsSvc<T> {
                        type Response = super::NumPoolsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumPoolsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).num_pools(request).await };
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
                        let method = NumPoolsSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/Pool" => {
                    #[allow(non_camel_case_types)]
                    struct PoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::PoolRequest>
                    for PoolSvc<T> {
                        type Response = super::PoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PoolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).pool(request).await };
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
                        let method = PoolSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/AllPools" => {
                    #[allow(non_camel_case_types)]
                    struct AllPoolsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AllPoolsRequest>
                    for AllPoolsSvc<T> {
                        type Response = super::AllPoolsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AllPoolsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).all_pools(request).await };
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
                        let method = AllPoolsSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/ListPoolsByDenom" => {
                    #[allow(non_camel_case_types)]
                    struct ListPoolsByDenomSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::ListPoolsByDenomRequest>
                    for ListPoolsByDenomSvc<T> {
                        type Response = super::ListPoolsByDenomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPoolsByDenomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_pools_by_denom(request).await
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
                        let method = ListPoolsByDenomSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/SpotPrice" => {
                    #[allow(non_camel_case_types)]
                    struct SpotPriceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::SpotPriceRequest>
                    for SpotPriceSvc<T> {
                        type Response = super::SpotPriceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpotPriceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).spot_price(request).await };
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
                        let method = SpotPriceSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/TotalPoolLiquidity" => {
                    #[allow(non_camel_case_types)]
                    struct TotalPoolLiquiditySvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::TotalPoolLiquidityRequest>
                    for TotalPoolLiquiditySvc<T> {
                        type Response = super::TotalPoolLiquidityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TotalPoolLiquidityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_pool_liquidity(request).await
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
                        let method = TotalPoolLiquiditySvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/TotalLiquidity" => {
                    #[allow(non_camel_case_types)]
                    struct TotalLiquiditySvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::TotalLiquidityRequest>
                    for TotalLiquiditySvc<T> {
                        type Response = super::TotalLiquidityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TotalLiquidityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_liquidity(request).await
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
                        let method = TotalLiquiditySvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/TotalVolumeForPool" => {
                    #[allow(non_camel_case_types)]
                    struct TotalVolumeForPoolSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::TotalVolumeForPoolRequest>
                    for TotalVolumeForPoolSvc<T> {
                        type Response = super::TotalVolumeForPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TotalVolumeForPoolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_volume_for_pool(request).await
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
                        let method = TotalVolumeForPoolSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/TradingPairTakerFee" => {
                    #[allow(non_camel_case_types)]
                    struct TradingPairTakerFeeSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::TradingPairTakerFeeRequest>
                    for TradingPairTakerFeeSvc<T> {
                        type Response = super::TradingPairTakerFeeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TradingPairTakerFeeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trading_pair_taker_fee(request).await
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
                        let method = TradingPairTakerFeeSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/EstimateTradeBasedOnPriceImpact" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateTradeBasedOnPriceImpactSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::EstimateTradeBasedOnPriceImpactRequest,
                    > for EstimateTradeBasedOnPriceImpactSvc<T> {
                        type Response = super::EstimateTradeBasedOnPriceImpactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EstimateTradeBasedOnPriceImpactRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).estimate_trade_based_on_price_impact(request).await
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
                        let method = EstimateTradeBasedOnPriceImpactSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/AllTakerFeeShareAgreements" => {
                    #[allow(non_camel_case_types)]
                    struct AllTakerFeeShareAgreementsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::AllTakerFeeShareAgreementsRequest,
                    > for AllTakerFeeShareAgreementsSvc<T> {
                        type Response = super::AllTakerFeeShareAgreementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AllTakerFeeShareAgreementsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).all_taker_fee_share_agreements(request).await
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
                        let method = AllTakerFeeShareAgreementsSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/TakerFeeShareAgreementFromDenom" => {
                    #[allow(non_camel_case_types)]
                    struct TakerFeeShareAgreementFromDenomSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::TakerFeeShareAgreementFromDenomRequest,
                    > for TakerFeeShareAgreementFromDenomSvc<T> {
                        type Response = super::TakerFeeShareAgreementFromDenomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::TakerFeeShareAgreementFromDenomRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).taker_fee_share_agreement_from_denom(request).await
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
                        let method = TakerFeeShareAgreementFromDenomSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/TakerFeeShareDenomsToAccruedValue" => {
                    #[allow(non_camel_case_types)]
                    struct TakerFeeShareDenomsToAccruedValueSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::TakerFeeShareDenomsToAccruedValueRequest,
                    > for TakerFeeShareDenomsToAccruedValueSvc<T> {
                        type Response = super::TakerFeeShareDenomsToAccruedValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::TakerFeeShareDenomsToAccruedValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .taker_fee_share_denoms_to_accrued_value(request)
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
                        let method = TakerFeeShareDenomsToAccruedValueSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/AllTakerFeeShareAccumulators" => {
                    #[allow(non_camel_case_types)]
                    struct AllTakerFeeShareAccumulatorsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::AllTakerFeeShareAccumulatorsRequest,
                    > for AllTakerFeeShareAccumulatorsSvc<T> {
                        type Response = super::AllTakerFeeShareAccumulatorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AllTakerFeeShareAccumulatorsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).all_taker_fee_share_accumulators(request).await
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
                        let method = AllTakerFeeShareAccumulatorsSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/RegisteredAlloyedPoolFromDenom" => {
                    #[allow(non_camel_case_types)]
                    struct RegisteredAlloyedPoolFromDenomSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::RegisteredAlloyedPoolFromDenomRequest,
                    > for RegisteredAlloyedPoolFromDenomSvc<T> {
                        type Response = super::RegisteredAlloyedPoolFromDenomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RegisteredAlloyedPoolFromDenomRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).registered_alloyed_pool_from_denom(request).await
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
                        let method = RegisteredAlloyedPoolFromDenomSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/RegisteredAlloyedPoolFromPoolId" => {
                    #[allow(non_camel_case_types)]
                    struct RegisteredAlloyedPoolFromPoolIdSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::RegisteredAlloyedPoolFromPoolIdRequest,
                    > for RegisteredAlloyedPoolFromPoolIdSvc<T> {
                        type Response = super::RegisteredAlloyedPoolFromPoolIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RegisteredAlloyedPoolFromPoolIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).registered_alloyed_pool_from_pool_id(request).await
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
                        let method = RegisteredAlloyedPoolFromPoolIdSvc(inner);
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
                "/osmosis.poolmanager.v1beta1.Query/AllRegisteredAlloyedPools" => {
                    #[allow(non_camel_case_types)]
                    struct AllRegisteredAlloyedPoolsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::AllRegisteredAlloyedPoolsRequest,
                    > for AllRegisteredAlloyedPoolsSvc<T> {
                        type Response = super::AllRegisteredAlloyedPoolsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AllRegisteredAlloyedPoolsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).all_registered_alloyed_pools(request).await
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
                        let method = AllRegisteredAlloyedPoolsSvc(inner);
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
        const NAME: &'static str = "osmosis.poolmanager.v1beta1.Query";
    }
}
