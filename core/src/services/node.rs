use protos::node::{NodeRegisterResponse, NodeRegisterRequest};
use std::sync::Arc;
use grpcio::{EnvBuilder, ChannelBuilder};
use std::env;

pub struct NodeClient;

impl NodeClient {
    pub fn register_node(req: NodeRegisterRequest) -> Result<NodeRegisterResponse, grpcio::Error> {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(env::var("GATE_ADDR").unwrap().as_str());
        let client = protos::node_grpc::NodeClient::new(ch);

        client.register(&req)

    }
}
