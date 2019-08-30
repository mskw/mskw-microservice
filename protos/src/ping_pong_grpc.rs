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

const METHOD_PING_PONG_PING: ::grpcio::Method<super::ping_pong::PingPongRequest, super::ping_pong::PingPongResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/PingPong/Ping",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct PingPongClient {
    client: ::grpcio::Client,
}

impl PingPongClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PingPongClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn ping_opt(&self, req: &super::ping_pong::PingPongRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::ping_pong::PingPongResponse> {
        self.client.unary_call(&METHOD_PING_PONG_PING, req, opt)
    }

    pub fn ping(&self, req: &super::ping_pong::PingPongRequest) -> ::grpcio::Result<super::ping_pong::PingPongResponse> {
        self.ping_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ping_async_opt(&self, req: &super::ping_pong::PingPongRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::ping_pong::PingPongResponse>> {
        self.client.unary_call_async(&METHOD_PING_PONG_PING, req, opt)
    }

    pub fn ping_async(&self, req: &super::ping_pong::PingPongRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::ping_pong::PingPongResponse>> {
        self.ping_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait PingPong {
    fn ping(&mut self, ctx: ::grpcio::RpcContext, req: super::ping_pong::PingPongRequest, sink: ::grpcio::UnarySink<super::ping_pong::PingPongResponse>);
}

pub fn create_ping_pong<S: PingPong + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_PING_PONG_PING, move |ctx, req, resp| {
        instance.ping(ctx, req, resp)
    });
    builder.build()
}
