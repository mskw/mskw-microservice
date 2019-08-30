
use std::sync::Arc;
use std::error::Error;
use futures::sync::oneshot;
use std::{thread, io, env};
use std::io::Read;
use futures::Future;
use log::{info};
use std::str::FromStr;


pub fn start_grpc_server(services: Vec<grpcio::Service>, port: u16) -> grpcio::Server  {
    let env = Arc::new(grpcio::Environment::new(env_parse("NUM_THREADS")));
    let mut server_builder = grpcio::ServerBuilder::new(env)
        .bind("0.0.0.0", port);
    for service in services {
        server_builder = server_builder.register_service(service);
    }

    let mut server = server_builder.build().unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("gRPC server listening on {}:{}", host, port);
    }
    server
}

pub fn wait_for_enter() {
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
}

pub fn setup_server() {
    let _ = dotenv::dotenv();
    pretty_env_logger::init();

    info!("{} Starting...", env::var("SERVER_NAME").unwrap());
}

pub fn env_parse<T: FromStr>(name: &str) -> T {
    env::var(name).unwrap().parse().map_err(|e| drop(e)).unwrap()
}

//pub fn env_get(name: &str) -> &str {
//    env::var(name).unwrap().as_str()
//}
