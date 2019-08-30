use protos::ping_pong;
use protos::ping_pong_grpc;
use core;

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use futures::sync::oneshot;
use log::{info};

use futures::Future;
use core::utils::env_parse;

mod services;

fn main() {
    core::utils::setup_server();

    let mut service_list = Vec::new();

    service_list.push(core::services::PingPongService::create());

    let mut server = core::utils::start_grpc_server(service_list, env_parse("PORT"));

    core::utils::wait_for_enter();
    server.shutdown().wait();

}

