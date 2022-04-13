#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSlotConfirmationRequest {
    #[prost(uint64, tag = "1")]
    pub last_replicated_slot: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSlotConfirmationResponse {
    #[prost(uint64, repeated, tag = "1")]
    pub updated_slots: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaAccountsRequest {
    #[prost(uint64, tag = "1")]
    pub slot: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaAccountMeta {
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub lamports: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "4")]
    pub executable: bool,
    #[prost(uint64, tag = "5")]
    pub rent_epoch: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaAccountData {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaAccountInfo {
    #[prost(message, optional, tag = "1")]
    pub account_meta: ::core::option::Option<ReplicaAccountMeta>,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<ReplicaAccountData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaAccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<ReplicaAccountInfo>,
}
#[doc = r" Generated client implementations."]
pub mod accounts_db_repl_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct AccountsDbReplClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccountsDbReplClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AccountsDbReplClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AccountsDbReplClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            AccountsDbReplClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get_confirmed_slots(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplicaSlotConfirmationRequest>,
        ) -> Result<tonic::Response<super::ReplicaSlotConfirmationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/accountsdb_repl.AccountsDbRepl/get_confirmed_slots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_slot_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplicaAccountsRequest>,
        ) -> Result<tonic::Response<super::ReplicaAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/accountsdb_repl.AccountsDbRepl/get_slot_accounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod accounts_db_repl_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AccountsDbReplServer."]
    #[async_trait]
    pub trait AccountsDbRepl: Send + Sync + 'static {
        async fn get_confirmed_slots(
            &self,
            request: tonic::Request<super::ReplicaSlotConfirmationRequest>,
        ) -> Result<tonic::Response<super::ReplicaSlotConfirmationResponse>, tonic::Status>;
        async fn get_slot_accounts(
            &self,
            request: tonic::Request<super::ReplicaAccountsRequest>,
        ) -> Result<tonic::Response<super::ReplicaAccountsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AccountsDbReplServer<T: AccountsDbRepl> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AccountsDbRepl> AccountsDbReplServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccountsDbReplServer<T>
    where
        T: AccountsDbRepl,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/accountsdb_repl.AccountsDbRepl/get_confirmed_slots" => {
                    #[allow(non_camel_case_types)]
                    struct get_confirmed_slotsSvc<T: AccountsDbRepl>(pub Arc<T>);
                    impl<T: AccountsDbRepl>
                        tonic::server::UnaryService<super::ReplicaSlotConfirmationRequest>
                        for get_confirmed_slotsSvc<T>
                    {
                        type Response = super::ReplicaSlotConfirmationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReplicaSlotConfirmationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_confirmed_slots(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = get_confirmed_slotsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/accountsdb_repl.AccountsDbRepl/get_slot_accounts" => {
                    #[allow(non_camel_case_types)]
                    struct get_slot_accountsSvc<T: AccountsDbRepl>(pub Arc<T>);
                    impl<T: AccountsDbRepl>
                        tonic::server::UnaryService<super::ReplicaAccountsRequest>
                        for get_slot_accountsSvc<T>
                    {
                        type Response = super::ReplicaAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReplicaAccountsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_slot_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = get_slot_accountsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
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
    impl<T: AccountsDbRepl> Clone for AccountsDbReplServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AccountsDbRepl> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AccountsDbRepl> tonic::transport::NamedService for AccountsDbReplServer<T> {
        const NAME: &'static str = "accountsdb_repl.AccountsDbRepl";
    }
}
