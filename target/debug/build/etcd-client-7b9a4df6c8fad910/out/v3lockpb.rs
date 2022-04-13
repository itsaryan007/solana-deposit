#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockRequest {
    /// name is the identifier for the distributed shared lock to be acquired.
    #[prost(bytes = "vec", tag = "1")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    /// lease is the ID of the lease that will be attached to ownership of the
    /// lock. If the lease expires or is revoked and currently holds the lock,
    /// the lock is automatically released. Calls to Lock with the same lease will
    /// be treated as a single acquisition; locking twice with the same lease is a
    /// no-op.
    #[prost(int64, tag = "2")]
    pub lease: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::etcdserverpb::ResponseHeader>,
    /// key is a key that will exist on etcd for the duration that the Lock caller
    /// owns the lock. Users should not modify this key or the lock may exhibit
    /// undefined behavior.
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockRequest {
    /// key is the lock ownership key granted by Lock.
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::etcdserverpb::ResponseHeader>,
}
#[doc = r" Generated client implementations."]
pub mod lock_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The lock service exposes client-side locking facilities as a gRPC interface."]
    #[derive(Debug, Clone)]
    pub struct LockClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LockClient<tonic::transport::Channel> {
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
    impl<T> LockClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> LockClient<InterceptedService<T, F>>
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
            LockClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lock acquires a distributed shared lock on a given named lock."]
        #[doc = " On success, it will return a unique key that exists so long as the"]
        #[doc = " lock is held by the caller. This key can be used in conjunction with"]
        #[doc = " transactions to safely ensure updates to etcd only occur while holding"]
        #[doc = " lock ownership. The lock is held until Unlock is called on the key or the"]
        #[doc = " lease associate with the owner expires."]
        pub async fn lock(
            &mut self,
            request: impl tonic::IntoRequest<super::LockRequest>,
        ) -> Result<tonic::Response<super::LockResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/v3lockpb.Lock/Lock");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Unlock takes a key returned by Lock and releases the hold on lock. The"]
        #[doc = " next Lock caller waiting for the lock will then be woken up and given"]
        #[doc = " ownership of the lock."]
        pub async fn unlock(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlockRequest>,
        ) -> Result<tonic::Response<super::UnlockResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/v3lockpb.Lock/Unlock");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
