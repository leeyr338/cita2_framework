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

pub trait ConsensusService {
    fn verify_proof(&self, o: ::grpc::RequestOptions, p: super::blockchain::Proof) -> ::grpc::SingleResponse<super::common::RpcBool>;

    fn proc_consensus_message(&self, o: ::grpc::RequestOptions, p: super::consensus::ConsensusMessage) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn set_status(&self, o: ::grpc::RequestOptions, p: super::consensus::NewStatus) -> ::grpc::SingleResponse<super::common::RpcStatus>;
}

// client

pub struct ConsensusServiceClient {
    grpc_client: ::grpc::Client,
    method_VerifyProof: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::blockchain::Proof, super::common::RpcBool>>,
    method_ProcConsensusMessage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::consensus::ConsensusMessage, super::common::RpcStatus>>,
    method_SetStatus: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::consensus::NewStatus, super::common::RpcStatus>>,
}

impl ConsensusServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ConsensusServiceClient {
            grpc_client: grpc_client,
            method_VerifyProof: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ConsensusService/VerifyProof".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ProcConsensusMessage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ConsensusService/ProcConsensusMessage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SetStatus: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ConsensusService/SetStatus".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ConsensusServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ConsensusServiceClient::with_client(c)
        })
    }
}

impl ConsensusService for ConsensusServiceClient {
    fn verify_proof(&self, o: ::grpc::RequestOptions, p: super::blockchain::Proof) -> ::grpc::SingleResponse<super::common::RpcBool> {
        self.grpc_client.call_unary(o, p, self.method_VerifyProof.clone())
    }

    fn proc_consensus_message(&self, o: ::grpc::RequestOptions, p: super::consensus::ConsensusMessage) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_ProcConsensusMessage.clone())
    }

    fn set_status(&self, o: ::grpc::RequestOptions, p: super::consensus::NewStatus) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_SetStatus.clone())
    }
}

// server

pub struct ConsensusServiceServer;


impl ConsensusServiceServer {
    pub fn new_service_def<H : ConsensusService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/ConsensusService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ConsensusService/VerifyProof".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.verify_proof(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ConsensusService/ProcConsensusMessage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.proc_consensus_message(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ConsensusService/SetStatus".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.set_status(o, p))
                    },
                ),
            ],
        )
    }
}
