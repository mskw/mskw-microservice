use protos::ping_pong::{PingPongRequest, PingPongResponse};
use protos::ping_pong_grpc::{PingPong, create_ping_pong};
use grpcio::{RpcContext, UnarySink, EnvBuilder, ChannelBuilder};
use log::{warn};
use futures::Future;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
//use crate::utils::simple_client;

#[derive(Clone)]
pub struct PingPongService;

impl PingPongService {
    pub fn create() -> grpcio::Service {
        create_ping_pong(PingPongService {})
    }
}

impl PingPong for PingPongService {
    fn ping(&mut self, ctx: RpcContext, req: PingPongRequest, sink: UnarySink<PingPongResponse>) {
        let mut resp = PingPongResponse::default();
        let msg = format!("{} - {} - pong", req.ping, env::var("SERVER_NAME").unwrap());
        resp.set_pong(msg);
        let f = sink
            .success(resp)
            .map_err(move |e| warn!("PingPongService: ping {}", e));
        ctx.spawn(f);
    }
}

pub struct PingPongClient;

impl PingPongClient {
    pub fn ping(addr: &str, ping: &str) -> Result<String, grpcio::Error> {

        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(addr);
        let client = protos::ping_pong_grpc::PingPongClient::new(ch);

        let mut req = PingPongRequest::default();
        req.set_ping(ping.to_owned());
        client.ping(&req)
            .map(|response| response.pong)
    }


}
