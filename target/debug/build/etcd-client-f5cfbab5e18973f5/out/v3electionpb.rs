#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignRequest {
    /// name is the election's identifier for the campaign.
    #[prost(bytes = "vec", tag = "1")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    /// lease is the ID of the lease attached to leadership of the election. If the
    /// lease expires or is revoked before resigning leadership, then the
    /// leadership is transferred to the next campaigner, if any.
    #[prost(int64, tag = "2")]
    pub lease: i64,
    /// value is the initial proclaimed value set when the campaigner wins the
    /// election.
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::etcdserverpb::ResponseHeader>,
    /// leader describes the resources used for holding leadereship of the election.
    #[prost(message, optional, tag = "2")]
    pub leader: ::core::option::Option<LeaderKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderKey {
    /// name is the election identifier that correponds to the leadership key.
    #[prost(bytes = "vec", tag = "1")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    /// key is an opaque key representing the ownership of the election. If the key
    /// is deleted, then leadership is lost.
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// rev is the creation revision of the key. It can be used to test for ownership
    /// of an election during transactions by testing the key's creation revision
    /// matches rev.
    #[prost(int64, tag = "3")]
    pub rev: i64,
    /// lease is the lease ID of the election leader.
    #[prost(int64, tag = "4")]
    pub lease: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderRequest {
    /// name is the election identifier for the leadership information.
    #[prost(bytes = "vec", tag = "1")]
    pub name: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::etcdserverpb::ResponseHeader>,
    /// kv is the key-value pair representing the latest leader update.
    #[prost(message, optional, tag = "2")]
    pub kv: ::core::option::Option<super::mvccpb::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResignRequest {
    /// leader is the leadership to relinquish by resignation.
    #[prost(message, optional, tag = "1")]
    pub leader: ::core::option::Option<LeaderKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResignResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::etcdserverpb::ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProclaimRequest {
    /// leader is the leadership hold on the election.
    #[prost(message, optional, tag = "1")]
    pub leader: ::core::option::Option<LeaderKey>,
    /// value is an update meant to overwrite the leader's current value.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProclaimResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::etcdserverpb::ResponseHeader>,
}
#[doc = r" Generated client implementations."]
pub mod election_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The election service exposes client-side election facilities as a gRPC interface."]
    #[derive(Debug, Clone)]
    pub struct ElectionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ElectionClient<tonic::transport::Channel> {
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
    impl<T> ElectionClient<T>
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
        ) -> ElectionClient<InterceptedService<T, F>>
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
            ElectionClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Campaign waits to acquire leadership in an election, returning a LeaderKey"]
        #[doc = " representing the leadership if successful. The LeaderKey can then be used"]
        #[doc = " to issue new values on the election, transactionally guard API requests on"]
        #[doc = " leadership still being held, and resign from the election."]
        pub async fn campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::CampaignRequest>,
        ) -> Result<tonic::Response<super::CampaignResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/v3electionpb.Election/Campaign");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Proclaim updates the leader's posted value with a new value."]
        pub async fn proclaim(
            &mut self,
            request: impl tonic::IntoRequest<super::ProclaimRequest>,
        ) -> Result<tonic::Response<super::ProclaimResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/v3electionpb.Election/Proclaim");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Leader returns the current election proclamation, if any."]
        pub async fn leader(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaderRequest>,
        ) -> Result<tonic::Response<super::LeaderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/v3electionpb.Election/Leader");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Observe streams election proclamations in-order as made by the election's"]
        #[doc = " elected leaders."]
        pub async fn observe(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaderRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::LeaderResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/v3electionpb.Election/Observe");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Resign releases election leadership so other campaigners may acquire"]
        #[doc = " leadership on the election."]
        pub async fn resign(
            &mut self,
            request: impl tonic::IntoRequest<super::ResignRequest>,
        ) -> Result<tonic::Response<super::ResignResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/v3electionpb.Election/Resign");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
