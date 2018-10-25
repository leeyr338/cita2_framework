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

pub trait ChainService {
    fn add_block(&self, o: ::grpc::RequestOptions, p: super::blockchain::Block) -> ::grpc::SingleResponse<super::chain::AddBlockRet>;

    fn get_block(&self, o: ::grpc::RequestOptions, p: super::chain::GetBlockReq) -> ::grpc::SingleResponse<super::blockchain::Block>;

    fn get_receipt(&self, o: ::grpc::RequestOptions, p: super::chain::GetReceiptReq) -> ::grpc::SingleResponse<super::blockchain::Receipt>;

    fn get_transaction(&self, o: ::grpc::RequestOptions, p: super::chain::GetTransactionReq) -> ::grpc::SingleResponse<super::blockchain::SignedTransaction>;
}

// client

pub struct ChainServiceClient {
    grpc_client: ::grpc::Client,
    method_AddBlock: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::blockchain::Block, super::chain::AddBlockRet>>,
    method_GetBlock: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::chain::GetBlockReq, super::blockchain::Block>>,
    method_GetReceipt: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::chain::GetReceiptReq, super::blockchain::Receipt>>,
    method_GetTransaction: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::chain::GetTransactionReq, super::blockchain::SignedTransaction>>,
}

impl ChainServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ChainServiceClient {
            grpc_client: grpc_client,
            method_AddBlock: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ChainService/AddBlock".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetBlock: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ChainService/GetBlock".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetReceipt: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ChainService/GetReceipt".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetTransaction: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ChainService/GetTransaction".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ChainServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ChainServiceClient::with_client(c)
        })
    }
}

impl ChainService for ChainServiceClient {
    fn add_block(&self, o: ::grpc::RequestOptions, p: super::blockchain::Block) -> ::grpc::SingleResponse<super::chain::AddBlockRet> {
        self.grpc_client.call_unary(o, p, self.method_AddBlock.clone())
    }

    fn get_block(&self, o: ::grpc::RequestOptions, p: super::chain::GetBlockReq) -> ::grpc::SingleResponse<super::blockchain::Block> {
        self.grpc_client.call_unary(o, p, self.method_GetBlock.clone())
    }

    fn get_receipt(&self, o: ::grpc::RequestOptions, p: super::chain::GetReceiptReq) -> ::grpc::SingleResponse<super::blockchain::Receipt> {
        self.grpc_client.call_unary(o, p, self.method_GetReceipt.clone())
    }

    fn get_transaction(&self, o: ::grpc::RequestOptions, p: super::chain::GetTransactionReq) -> ::grpc::SingleResponse<super::blockchain::SignedTransaction> {
        self.grpc_client.call_unary(o, p, self.method_GetTransaction.clone())
    }
}

// server

pub struct ChainServiceServer;


impl ChainServiceServer {
    pub fn new_service_def<H : ChainService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/ChainService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ChainService/AddBlock".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_block(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ChainService/GetBlock".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_block(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ChainService/GetReceipt".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_receipt(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ChainService/GetTransaction".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_transaction(o, p))
                    },
                ),
            ],
        )
    }
}
