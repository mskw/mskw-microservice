extern crate uuid;

use protos;
use core;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::str::FromStr;
use std::{env, time, thread};
use core::utils::env_parse;
use core::services::PingPongClient;
use std::thread::sleep;
use log::{trace, info, debug};
use std::process::exit;

mod services;
mod schedule;

fn main() {
    core::utils::setup_server();

    thread::spawn(move || {
        HttpServer::new(|| {
            App::new()
                .route("*", web::to(index))
        })
            .workers(env_parse("NUM_THREADS"))
            .bind(SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(),
                                  env_parse("WEB_PORT"))
            )
            .unwrap()
            .run()
            .unwrap()
    });


//    loop {
//        sleep(time::Duration::from_secs(1));
//        trace!("Pass 1s.");
//    }
    core::utils::wait_for_enter();

    exit(0);
}

fn index(req: HttpRequest) -> impl Responder {
    debug!("index");
    HttpResponse::Ok().body(req.uri().to_string())
}
