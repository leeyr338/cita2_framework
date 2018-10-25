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

pub trait PoolService {
    fn add_unverified_transaction(&self, o: ::grpc::RequestOptions, p: super::blockchain::UnverifiedTransaction) -> ::grpc::SingleResponse<super::pool::AddUnverifiedTransactionRet>;

    fn add_batch_unverified_transactions(&self, o: ::grpc::RequestOptions, p: super::pool::UnverifiedTransactions) -> ::grpc::SingleResponse<super::common::RpcStatus>;

    fn get_unverified_transaction(&self, o: ::grpc::RequestOptions, p: super::pool::UnverifiedTransactionHashes) -> ::grpc::SingleResponse<super::pool::UnverifiedTransactions>;

    fn proposal_unverified_transaction_hashes(&self, o: ::grpc::RequestOptions, p: super::common::RpcNone) -> ::grpc::SingleResponse<super::pool::UnverifiedTransactionHashes>;

    fn check_unverified_transaction(&self, o: ::grpc::RequestOptions, p: super::pool::CheckUnverifiedTransactionReq) -> ::grpc::SingleResponse<super::common::RpcBool>;
}

// client

pub struct PoolServiceClient {
    grpc_client: ::grpc::Client,
    method_AddUnverifiedTransaction: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::blockchain::UnverifiedTransaction, super::pool::AddUnverifiedTransactionRet>>,
    method_AddBatchUnverifiedTransactions: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::pool::UnverifiedTransactions, super::common::RpcStatus>>,
    method_GetUnverifiedTransaction: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::pool::UnverifiedTransactionHashes, super::pool::UnverifiedTransactions>>,
    method_ProposalUnverifiedTransactionHashes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::common::RpcNone, super::pool::UnverifiedTransactionHashes>>,
    method_CheckUnverifiedTransaction: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::pool::CheckUnverifiedTransactionReq, super::common::RpcBool>>,
}

impl PoolServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        PoolServiceClient {
            grpc_client: grpc_client,
            method_AddUnverifiedTransaction: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/PoolService/AddUnverifiedTransaction".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddBatchUnverifiedTransactions: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/PoolService/AddBatchUnverifiedTransactions".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetUnverifiedTransaction: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/PoolService/GetUnverifiedTransaction".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ProposalUnverifiedTransactionHashes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/PoolService/ProposalUnverifiedTransactionHashes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CheckUnverifiedTransaction: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/PoolService/CheckUnverifiedTransaction".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            PoolServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            PoolServiceClient::with_client(c)
        })
    }
}

impl PoolService for PoolServiceClient {
    fn add_unverified_transaction(&self, o: ::grpc::RequestOptions, p: super::blockchain::UnverifiedTransaction) -> ::grpc::SingleResponse<super::pool::AddUnverifiedTransactionRet> {
        self.grpc_client.call_unary(o, p, self.method_AddUnverifiedTransaction.clone())
    }

    fn add_batch_unverified_transactions(&self, o: ::grpc::RequestOptions, p: super::pool::UnverifiedTransactions) -> ::grpc::SingleResponse<super::common::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_AddBatchUnverifiedTransactions.clone())
    }

    fn get_unverified_transaction(&self, o: ::grpc::RequestOptions, p: super::pool::UnverifiedTransactionHashes) -> ::grpc::SingleResponse<super::pool::UnverifiedTransactions> {
        self.grpc_client.call_unary(o, p, self.method_GetUnverifiedTransaction.clone())
    }

    fn proposal_unverified_transaction_hashes(&self, o: ::grpc::RequestOptions, p: super::common::RpcNone) -> ::grpc::SingleResponse<super::pool::UnverifiedTransactionHashes> {
        self.grpc_client.call_unary(o, p, self.method_ProposalUnverifiedTransactionHashes.clone())
    }

    fn check_unverified_transaction(&self, o: ::grpc::RequestOptions, p: super::pool::CheckUnverifiedTransactionReq) -> ::grpc::SingleResponse<super::common::RpcBool> {
        self.grpc_client.call_unary(o, p, self.method_CheckUnverifiedTransaction.clone())
    }
}

// server

pub struct PoolServiceServer;


impl PoolServiceServer {
    pub fn new_service_def<H : PoolService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/PoolService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/PoolService/AddUnverifiedTransaction".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_unverified_transaction(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/PoolService/AddBatchUnverifiedTransactions".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_batch_unverified_transactions(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/PoolService/GetUnverifiedTransaction".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_unverified_transaction(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/PoolService/ProposalUnverifiedTransactionHashes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.proposal_unverified_transaction_hashes(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/PoolService/CheckUnverifiedTransaction".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.check_unverified_transaction(o, p))
                    },
                ),
            ],
        )
    }
}
