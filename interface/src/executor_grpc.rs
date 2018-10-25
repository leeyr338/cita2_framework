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

pub trait ExecutorService {
    fn apply(&self, o: ::grpc::RequestOptions, p: super::executor::ApplyReq) -> ::grpc::SingleResponse<super::executor::ApplyRet>;
}

// client

pub struct ExecutorServiceClient {
    grpc_client: ::grpc::Client,
    method_Apply: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::executor::ApplyReq, super::executor::ApplyRet>>,
}

impl ExecutorServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ExecutorServiceClient {
            grpc_client: grpc_client,
            method_Apply: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ExecutorService/Apply".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ExecutorServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ExecutorServiceClient::with_client(c)
        })
    }
}

impl ExecutorService for ExecutorServiceClient {
    fn apply(&self, o: ::grpc::RequestOptions, p: super::executor::ApplyReq) -> ::grpc::SingleResponse<super::executor::ApplyRet> {
        self.grpc_client.call_unary(o, p, self.method_Apply.clone())
    }
}

// server

pub struct ExecutorServiceServer;


impl ExecutorServiceServer {
    pub fn new_service_def<H : ExecutorService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/ExecutorService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ExecutorService/Apply".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.apply(o, p))
                    },
                ),
            ],
        )
    }
}
