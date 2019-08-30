use protos::ping_pong;
use protos::ping_pong_grpc;

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use futures::sync::oneshot;

use ping_pong::{PingPongRequest, PingPongResponse};


fn main() {
    println!("Hello, world!");
}
