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

pub trait NetworkService {
    fn forword_unverified_transaction(&self, o: ::grpc::RequestOptions, p: super::blockchain::UnverifiedTransaction) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn send_unverified_transaction_hashes(&self, o: ::grpc::RequestOptions, p: super::network::UnverifiedTransactionHashes) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn broadcast_consensus_message(&self, o: ::grpc::RequestOptions, p: super::network::ConsensusMessage) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn broadcast_new_status(&self, o: ::grpc::RequestOptions, p: super::network::NewStatus) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn send_sync_request(&self, o: ::grpc::RequestOptions, p: super::network::SyncRequest) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn send_sync_response(&self, o: ::grpc::RequestOptions, p: super::network::SyncResponse) -> ::grpc::SingleResponse<super::common::RpcStatus>;
}

// client

pub struct NetworkServiceClient {
    grpc_client: ::grpc::Client,
    method_ForwordUnverifiedTransaction: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::blockchain::UnverifiedTransaction, super::common::RpcStatus>>,
    method_SendUnverifiedTransactionHashes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::network::UnverifiedTransactionHashes, super::common::RpcStatus>>,
    method_BroadcastConsensusMessage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::network::ConsensusMessage, super::common::RpcStatus>>,
    method_BroadcastNewStatus: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::network::NewStatus, super::common::RpcStatus>>,
    method_SendSyncRequest: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::network::SyncRequest, super::common::RpcStatus>>,
    method_SendSyncResponse: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::network::SyncResponse, super::common::RpcStatus>>,
}

impl NetworkServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        NetworkServiceClient {
            grpc_client: grpc_client,
            method_ForwordUnverifiedTransaction: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/NetworkService/ForwordUnverifiedTransaction".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendUnverifiedTransactionHashes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/NetworkService/SendUnverifiedTransactionHashes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BroadcastConsensusMessage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/NetworkService/BroadcastConsensusMessage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BroadcastNewStatus: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/NetworkService/BroadcastNewStatus".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendSyncRequest: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/NetworkService/SendSyncRequest".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendSyncResponse: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/NetworkService/SendSyncResponse".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            NetworkServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            NetworkServiceClient::with_client(c)
        })
    }
}

impl NetworkService for NetworkServiceClient {
    fn forword_unverified_transaction(&self, o: ::grpc::RequestOptions, p: super::blockchain::UnverifiedTransaction) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_ForwordUnverifiedTransaction.clone())
    }

    fn send_unverified_transaction_hashes(&self, o: ::grpc::RequestOptions, p: super::network::UnverifiedTransactionHashes) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_SendUnverifiedTransactionHashes.clone())
    }

    fn broadcast_consensus_message(&self, o: ::grpc::RequestOptions, p: super::network::ConsensusMessage) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_BroadcastConsensusMessage.clone())
    }

    fn broadcast_new_status(&self, o: ::grpc::RequestOptions, p: super::network::NewStatus) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_BroadcastNewStatus.clone())
    }

    fn send_sync_request(&self, o: ::grpc::RequestOptions, p: super::network::SyncRequest) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_SendSyncRequest.clone())
    }

    fn send_sync_response(&self, o: ::grpc::RequestOptions, p: super::network::SyncResponse) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_SendSyncResponse.clone())
    }
}

// server

pub struct NetworkServiceServer;


impl NetworkServiceServer {
    pub fn new_service_def<H : NetworkService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/NetworkService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/NetworkService/ForwordUnverifiedTransaction".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.forword_unverified_transaction(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/NetworkService/SendUnverifiedTransactionHashes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_unverified_transaction_hashes(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/NetworkService/BroadcastConsensusMessage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.broadcast_consensus_message(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/NetworkService/BroadcastNewStatus".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.broadcast_new_status(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/NetworkService/SendSyncRequest".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_sync_request(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/NetworkService/SendSyncResponse".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_sync_response(o, p))
                    },
                ),
            ],
        )
    }
}
