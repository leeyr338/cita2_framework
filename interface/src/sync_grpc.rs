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

pub trait SyncService {
    fn update_status(&self, o: ::grpc::RequestOptions, p: super::sync::StatusReq) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn proc_sync_request(&self, o: ::grpc::RequestOptions, p: super::sync::SyncRequest) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn proc_sync_response(&self, o: ::grpc::RequestOptions, p: super::sync::SyncResponse) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn get_peer_count(&self, o: ::grpc::RequestOptions, p: super::common::RpcNone) -> ::grpc::SingleResponse<super::sync::PeerCount>;
}

// client

pub struct SyncServiceClient {
    grpc_client: ::grpc::Client,
    method_UpdateStatus: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::sync::StatusReq, super::common::RpcStatus>>,
    method_ProcSyncRequest: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::sync::SyncRequest, super::common::RpcStatus>>,
    method_ProcSyncResponse: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::sync::SyncResponse, super::common::RpcStatus>>,
    method_GetPeerCount: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::common::RpcNone, super::sync::PeerCount>>,
}

impl SyncServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        SyncServiceClient {
            grpc_client: grpc_client,
            method_UpdateStatus: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/SyncService/UpdateStatus".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ProcSyncRequest: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/SyncService/ProcSyncRequest".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ProcSyncResponse: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/SyncService/ProcSyncResponse".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetPeerCount: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/SyncService/GetPeerCount".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            SyncServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            SyncServiceClient::with_client(c)
        })
    }
}

impl SyncService for SyncServiceClient {
    fn update_status(&self, o: ::grpc::RequestOptions, p: super::sync::StatusReq) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_UpdateStatus.clone())
    }

    fn proc_sync_request(&self, o: ::grpc::RequestOptions, p: super::sync::SyncRequest) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_ProcSyncRequest.clone())
    }

    fn proc_sync_response(&self, o: ::grpc::RequestOptions, p: super::sync::SyncResponse) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_ProcSyncResponse.clone())
    }

    fn get_peer_count(&self, o: ::grpc::RequestOptions, p: super::common::RpcNone) -> ::grpc::SingleResponse<super::sync::PeerCount> {
        self.grpc_client.call_unary(o, p, self.method_GetPeerCount.clone())
    }
}

// server

pub struct SyncServiceServer;


impl SyncServiceServer {
    pub fn new_service_def<H : SyncService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/SyncService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/SyncService/UpdateStatus".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_status(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/SyncService/ProcSyncRequest".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.proc_sync_request(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/SyncService/ProcSyncResponse".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.proc_sync_response(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/SyncService/GetPeerCount".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_peer_count(o, p))
                    },
                ),
            ],
        )
    }
}
