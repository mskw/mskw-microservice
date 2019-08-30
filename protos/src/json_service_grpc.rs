// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_JSON_SERVICE_SERVICE_REGISTER: ::grpcio::Method<super::json_service::JsonServiceRegisterRequest, super::json_service::JsonServiceRegisterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/JsonService/ServiceRegister",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_JSON_SERVICE_CALL: ::grpcio::Method<super::json_service::JsonRequest, super::json_service::JsonResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/JsonService/Call",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct JsonServiceClient {
    client: ::grpcio::Client,
}

impl JsonServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        JsonServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn service_register_opt(&self, req: &super::json_service::JsonServiceRegisterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::json_service::JsonServiceRegisterResponse> {
        self.client.unary_call(&METHOD_JSON_SERVICE_SERVICE_REGISTER, req, opt)
    }

    pub fn service_register(&self, req: &super::json_service::JsonServiceRegisterRequest) -> ::grpcio::Result<super::json_service::JsonServiceRegisterResponse> {
        self.service_register_opt(req, ::grpcio::CallOption::default())
    }

    pub fn service_register_async_opt(&self, req: &super::json_service::JsonServiceRegisterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::json_service::JsonServiceRegisterResponse>> {
        self.client.unary_call_async(&METHOD_JSON_SERVICE_SERVICE_REGISTER, req, opt)
    }

    pub fn service_register_async(&self, req: &super::json_service::JsonServiceRegisterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::json_service::JsonServiceRegisterResponse>> {
        self.service_register_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn call_opt(&self, req: &super::json_service::JsonRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::json_service::JsonResponse> {
        self.client.unary_call(&METHOD_JSON_SERVICE_CALL, req, opt)
    }

    pub fn call(&self, req: &super::json_service::JsonRequest) -> ::grpcio::Result<super::json_service::JsonResponse> {
        self.call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn call_async_opt(&self, req: &super::json_service::JsonRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::json_service::JsonResponse>> {
        self.client.unary_call_async(&METHOD_JSON_SERVICE_CALL, req, opt)
    }

    pub fn call_async(&self, req: &super::json_service::JsonRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::json_service::JsonResponse>> {
        self.call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait JsonService {
    fn service_register(&mut self, ctx: ::grpcio::RpcContext, req: super::json_service::JsonServiceRegisterRequest, sink: ::grpcio::UnarySink<super::json_service::JsonServiceRegisterResponse>);
    fn call(&mut self, ctx: ::grpcio::RpcContext, req: super::json_service::JsonRequest, sink: ::grpcio::UnarySink<super::json_service::JsonResponse>);
}

pub fn create_json_service<S: JsonService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_JSON_SERVICE_SERVICE_REGISTER, move |ctx, req, resp| {
        instance.service_register(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_JSON_SERVICE_CALL, move |ctx, req, resp| {
        instance.call(ctx, req, resp)
    });
    builder.build()
}
