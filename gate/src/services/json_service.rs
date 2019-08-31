extern crate uuid;
use protos::json_service::{JsonServiceRegisterRequest, JsonRequest, JsonResponse, JsonServiceRegisterResponse};
use protos::json_service_grpc::JsonService;

use grpcio;
use grpcio::{RpcContext, UnarySink, EnvBuilder, ChannelBuilder};
use uuid::Uuid;
use std::sync::{Mutex, Arc};
use crate::schedule::status::SystemStatus;
//use core::utils::{simple_resp, simple_client};
use futures::Future;

use log::{warn};

enum ServiceType {
    Json,
}

pub struct ServiceInfo {
    node: String,
    uuid: String,
    uri: String,
    name: String,
    service_type: ServiceType,
}


#[derive(Clone)]
pub struct GateJsonService {
    status: Arc<Mutex<SystemStatus>>,
}

impl GateJsonService {
    pub fn new(status: Arc<Mutex<SystemStatus>>) -> Self {
        GateJsonService {
            status,
        }
    }
}

impl JsonService for GateJsonService {
    fn service_register(&mut self, ctx: RpcContext, req: JsonServiceRegisterRequest, sink: UnarySink<JsonServiceRegisterResponse>) {
        let mut resp = JsonServiceRegisterResponse::new();
        resp.set_err("".to_owned());
        let uuid = Uuid::default().to_string();
        resp.set_uuid(uuid.to_owned());

        let mut status = self.status.clone();
        let mut status = status.lock().unwrap();
        let mut service_infos = status.services.get_mut();
        service_infos.insert(req.name.to_owned(), ServiceInfo {
            node: req.node,
            uuid,
            uri: req.uri,
            name: req.name,
            service_type: ServiceType::Json
        });

        let f = sink
            .success(resp)
            .map_err(move |e| warn!("PingPongService: ping {}", e));
        ctx.spawn(f);
    }

    fn call(&mut self, ctx: RpcContext, req: JsonRequest, sink: UnarySink<JsonResponse>) {
        let mut resp = JsonResponse::new();
        resp.set_err("".to_owned());

        let gate_client = GateJsonClient::new(self.status.clone());
        let json_resp = gate_client.call(req.get_name(), req.get_json()).unwrap();
        resp.set_json(json_resp);

        let f = sink
            .success(resp)
            .map_err(move |e| warn!("PingPongService: ping {}", e));
        ctx.spawn(f);


    }
}

pub struct GateJsonClient {
    status: Arc<Mutex<SystemStatus>>,
}


impl GateJsonClient {
    pub fn new(status: Arc<Mutex<SystemStatus>>) -> Self {
        GateJsonClient {
            status,
        }
    }

    pub fn call(&self, name: &str, json: &str) -> Result<String, grpcio::Error> {

        let mut status = self.status.clone();
        let mut status = status.lock().unwrap();
        let services = status.services.get_mut();
        let service_info = services.get(name).unwrap();

        let mut status = self.status.clone();
        let mut status = status.lock().unwrap();
        let nodes = status.nodes.get_mut();
        let node_info = nodes.get(service_info.uuid.as_str()).unwrap();

        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(format!("{}:{}", node_info.ip, node_info.port).as_str());
        let client = protos::json_service_grpc::JsonServiceClient::new(ch);


        let mut req = JsonRequest::new();
        req.set_name(name.to_string());
        req.set_json(json.to_string());

        client.call(&req)
            .map(|resp| resp.json)

    }
}

