
use protos::json_service::{JsonServiceRegisterRequest, JsonRequest, JsonResponse, JsonServiceRegisterResponse};
use protos::json_service_grpc::JsonService;

use grpcio;
use grpcio::{RpcContext, UnarySink, EnvBuilder, ChannelBuilder};
use uuid::Uuid;
use std::sync::{Mutex, Arc};
//use crate::utils::{simple_client, simple_resp};
use std::env;
use futures::Future;
use log::{warn};

#[derive(Clone)]
pub struct JsonMicroService;

impl JsonMicroService {
    pub fn new() -> Self {
        JsonMicroService { }
    }
}

impl JsonService for JsonMicroService {
    fn service_register(&mut self, ctx: RpcContext, req: JsonServiceRegisterRequest, sink: UnarySink<JsonServiceRegisterResponse>) {
        unimplemented!()
    }

    fn call(&mut self, ctx: RpcContext, req: JsonRequest, sink: UnarySink<JsonResponse>) {
        let mut resp = JsonResponse::new();
        resp.set_err("".to_owned());

        resp.set_json("{'error': 'not implemented yet.'}".to_string());

        let f = sink
            .success(resp)
            .map_err(move |e| warn!("simple_resp error: {}", e));
        ctx.spawn(f);

    }
}



pub struct JsonServiceClient;

impl JsonServiceClient {
    pub fn new() -> Self {
        JsonServiceClient {}
    }

    pub fn call(name: &str, json: &str) -> Result<String, grpcio::Error> {

        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(env::var("GATE_ADDR").unwrap().as_str());
        let client = protos::json_service_grpc::JsonServiceClient::new(ch);

        let mut req = JsonRequest::new();
        req.set_json(json.to_string());
        req.set_name(name.to_owned());
        client.call(&req)
            .map(|resp| resp.json)
    }
}
