


use protos::node::{NodeRegisterRequest, NodeRegisterResponse, HeartRequest, HeartResponse};
use protos::node_grpc::{create_node, Node};
use grpcio::{RpcContext, UnarySink};
use uuid::Uuid;
use crate::schedule::status::SystemStatus;
use std::sync::{Mutex, Arc};
//use core::utils::simple_resp;
use log::warn;
use futures::Future;

pub struct NodeInfo {
    pub name: String,
    pub uuid: String,
    pub ip: String,
    pub port: String,
    pub cluster: String,
}


#[derive(Clone)]
pub struct NodeService {
    status: Arc<Mutex<SystemStatus>>,
}

impl NodeService {
    pub fn new(status: Arc<Mutex<SystemStatus>>) -> Self {
        NodeService {
            status
        }
    }
}

impl Node for NodeService {
    fn register(&mut self, ctx: RpcContext, req: NodeRegisterRequest, sink: UnarySink<NodeRegisterResponse>) {
        let mut resp = NodeRegisterResponse::new();
        resp.set_err("".to_string());
        let uuid = Uuid::default().to_string();
        resp.set_node(uuid.to_owned());

        let mut status = self.status.clone();
        let mut status = status.lock().unwrap();
        let mut node_infos = status.nodes.get_mut();
        node_infos.insert(uuid.to_owned(), NodeInfo {
            name: req.name,
            uuid: uuid.to_owned(),
            ip: req.ip,
            port: req.port,
            cluster: req.cluster,
        });


        let f = sink
            .success(resp)
            .map_err(move |e| warn!("simple: ping {}", e));
        ctx.spawn(f);
    }

    fn heart(&mut self, ctx: RpcContext, req: HeartRequest, sink: UnarySink<HeartResponse>) {
        unimplemented!()
    }
}
