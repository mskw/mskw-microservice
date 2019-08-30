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

const METHOD_SERVICE_DISCOVERY_DISCOVERY: ::grpcio::Method<super::service_discovery::DiscoveryRequest, super::service_discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ServiceDiscovery/Discovery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_DISCOVERY_DISCOVERY_NODE: ::grpcio::Method<super::service_discovery::DiscoveryNodeRequest, super::service_discovery::DiscoveryNodeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ServiceDiscovery/DiscoveryNode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ServiceDiscoveryClient {
    client: ::grpcio::Client,
}

impl ServiceDiscoveryClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ServiceDiscoveryClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn discovery_opt(&self, req: &super::service_discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service_discovery::DiscoveryResponse> {
        self.client.unary_call(&METHOD_SERVICE_DISCOVERY_DISCOVERY, req, opt)
    }

    pub fn discovery(&self, req: &super::service_discovery::DiscoveryRequest) -> ::grpcio::Result<super::service_discovery::DiscoveryResponse> {
        self.discovery_opt(req, ::grpcio::CallOption::default())
    }

    pub fn discovery_async_opt(&self, req: &super::service_discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_discovery::DiscoveryResponse>> {
        self.client.unary_call_async(&METHOD_SERVICE_DISCOVERY_DISCOVERY, req, opt)
    }

    pub fn discovery_async(&self, req: &super::service_discovery::DiscoveryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_discovery::DiscoveryResponse>> {
        self.discovery_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn discovery_node_opt(&self, req: &super::service_discovery::DiscoveryNodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service_discovery::DiscoveryNodeResponse> {
        self.client.unary_call(&METHOD_SERVICE_DISCOVERY_DISCOVERY_NODE, req, opt)
    }

    pub fn discovery_node(&self, req: &super::service_discovery::DiscoveryNodeRequest) -> ::grpcio::Result<super::service_discovery::DiscoveryNodeResponse> {
        self.discovery_node_opt(req, ::grpcio::CallOption::default())
    }

    pub fn discovery_node_async_opt(&self, req: &super::service_discovery::DiscoveryNodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_discovery::DiscoveryNodeResponse>> {
        self.client.unary_call_async(&METHOD_SERVICE_DISCOVERY_DISCOVERY_NODE, req, opt)
    }

    pub fn discovery_node_async(&self, req: &super::service_discovery::DiscoveryNodeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_discovery::DiscoveryNodeResponse>> {
        self.discovery_node_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ServiceDiscovery {
    fn discovery(&mut self, ctx: ::grpcio::RpcContext, req: super::service_discovery::DiscoveryRequest, sink: ::grpcio::UnarySink<super::service_discovery::DiscoveryResponse>);
    fn discovery_node(&mut self, ctx: ::grpcio::RpcContext, req: super::service_discovery::DiscoveryNodeRequest, sink: ::grpcio::UnarySink<super::service_discovery::DiscoveryNodeResponse>);
}

pub fn create_service_discovery<S: ServiceDiscovery + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_DISCOVERY_DISCOVERY, move |ctx, req, resp| {
        instance.discovery(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SERVICE_DISCOVERY_DISCOVERY_NODE, move |ctx, req, resp| {
        instance.discovery_node(ctx, req, resp)
    });
    builder.build()
}
