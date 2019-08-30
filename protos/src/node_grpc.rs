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

const METHOD_NODE_REGISTER_REGISTER: ::grpcio::Method<super::node::NodeRegisterRequest, super::node::NodeRegisterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/NodeRegister/Register",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NODE_REGISTER_HEART: ::grpcio::Method<super::node::HeartRequest, super::node::HeratResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/NodeRegister/Heart",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct NodeRegisterClient {
    client: ::grpcio::Client,
}

impl NodeRegisterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        NodeRegisterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn register_opt(&self, req: &super::node::NodeRegisterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::node::NodeRegisterResponse> {
        self.client.unary_call(&METHOD_NODE_REGISTER_REGISTER, req, opt)
    }

    pub fn register(&self, req: &super::node::NodeRegisterRequest) -> ::grpcio::Result<super::node::NodeRegisterResponse> {
        self.register_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_async_opt(&self, req: &super::node::NodeRegisterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::node::NodeRegisterResponse>> {
        self.client.unary_call_async(&METHOD_NODE_REGISTER_REGISTER, req, opt)
    }

    pub fn register_async(&self, req: &super::node::NodeRegisterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::node::NodeRegisterResponse>> {
        self.register_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn heart_opt(&self, req: &super::node::HeartRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::node::HeratResponse> {
        self.client.unary_call(&METHOD_NODE_REGISTER_HEART, req, opt)
    }

    pub fn heart(&self, req: &super::node::HeartRequest) -> ::grpcio::Result<super::node::HeratResponse> {
        self.heart_opt(req, ::grpcio::CallOption::default())
    }

    pub fn heart_async_opt(&self, req: &super::node::HeartRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::node::HeratResponse>> {
        self.client.unary_call_async(&METHOD_NODE_REGISTER_HEART, req, opt)
    }

    pub fn heart_async(&self, req: &super::node::HeartRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::node::HeratResponse>> {
        self.heart_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait NodeRegister {
    fn register(&mut self, ctx: ::grpcio::RpcContext, req: super::node::NodeRegisterRequest, sink: ::grpcio::UnarySink<super::node::NodeRegisterResponse>);
    fn heart(&mut self, ctx: ::grpcio::RpcContext, req: super::node::HeartRequest, sink: ::grpcio::UnarySink<super::node::HeratResponse>);
}

pub fn create_node_register<S: NodeRegister + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NODE_REGISTER_REGISTER, move |ctx, req, resp| {
        instance.register(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_NODE_REGISTER_HEART, move |ctx, req, resp| {
        instance.heart(ctx, req, resp)
    });
    builder.build()
}
