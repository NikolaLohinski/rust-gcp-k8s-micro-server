extern crate fern;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate futures;
extern crate hyper;
extern crate log;
extern crate stackdriver_logger;

use std::thread::spawn;

use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;

mod config;
mod middlewares;
mod routes;

use config::config::{health_port, server_port};
use middlewares::log::LogIOMiddleware;
use routes::index;

fn start_health_check() {
    spawn(move || {
        gotham::start(format!("127.0.0.1:{}", health_port()), || {
            Ok(|state| (state, "ok"))
        })
    });
}

fn start_application() {
    let (chain, pipelines) = single_pipeline(new_pipeline().add(LogIOMiddleware).build());

    let router = build_router(chain, pipelines, |route| {
        route.get("/").to(index::handle);
    });

    gotham::start(format!("127.0.0.1:{}", server_port()), router);
}

fn main() {
    stackdriver_logger::init_with_cargo!();
    start_health_check();
    start_application();
}
