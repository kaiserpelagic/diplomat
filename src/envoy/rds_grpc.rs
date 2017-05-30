// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait RouteDiscoveryService {
    fn stream_routes(&self, o: ::grpc::RequestOptions, p: super::rds::RouteDiscoveryRequest) -> ::grpc::StreamingResponse<super::rds::RouteDiscoveryResponse>;

    fn fetch_routes(&self, o: ::grpc::RequestOptions, p: super::rds::RouteDiscoveryRequest) -> ::grpc::SingleResponse<super::rds::RouteDiscoveryResponse>;
}

// client

pub struct RouteDiscoveryServiceClient {
    grpc_client: ::grpc::Client,
    method_StreamRoutes: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rds::RouteDiscoveryRequest, super::rds::RouteDiscoveryResponse>>,
    method_FetchRoutes: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rds::RouteDiscoveryRequest, super::rds::RouteDiscoveryResponse>>,
}

impl RouteDiscoveryServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        RouteDiscoveryServiceClient {
            grpc_client: grpc_client,
            method_StreamRoutes: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.RouteDiscoveryService/StreamRoutes".to_string(),
                streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FetchRoutes: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.RouteDiscoveryService/FetchRoutes".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new(host, port, tls, conf).map(|c| {
            RouteDiscoveryServiceClient::with_client(c)
        })
    }
}

impl RouteDiscoveryService for RouteDiscoveryServiceClient {
    fn stream_routes(&self, o: ::grpc::RequestOptions, p: super::rds::RouteDiscoveryRequest) -> ::grpc::StreamingResponse<super::rds::RouteDiscoveryResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_StreamRoutes.clone())
    }

    fn fetch_routes(&self, o: ::grpc::RequestOptions, p: super::rds::RouteDiscoveryRequest) -> ::grpc::SingleResponse<super::rds::RouteDiscoveryResponse> {
        self.grpc_client.call_unary(o, p, self.method_FetchRoutes.clone())
    }
}

// server

pub struct RouteDiscoveryServiceServer {
    pub grpc_server: ::grpc::Server,
}

impl ::std::ops::Deref for RouteDiscoveryServiceServer {
    type Target = ::grpc::Server;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl RouteDiscoveryServiceServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : RouteDiscoveryService + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H) -> Self {
        let service_definition = RouteDiscoveryServiceServer::new_service_def(h);
        RouteDiscoveryServiceServer {
            grpc_server: ::grpc::Server::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : RouteDiscoveryService + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = RouteDiscoveryServiceServer::new_service_def(h);
        RouteDiscoveryServiceServer {
            grpc_server: ::grpc::Server::new_plain_pool(addr, conf, service_definition, cpu_pool),
        }
    }

    pub fn new_service_def<H : RouteDiscoveryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.RouteDiscoveryService/StreamRoutes".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerServerStreaming::new(move |o, p| handler_copy.stream_routes(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.RouteDiscoveryService/FetchRoutes".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.fetch_routes(o, p))
                    },
                ),
            ],
        )
    }
}
