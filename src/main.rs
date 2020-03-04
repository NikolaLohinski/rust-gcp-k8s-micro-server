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

use config::config::{health_port, port, name, version};
use middlewares::log::LogIOMiddleware;
use routes::index;

fn logging() {
    let service = stackdriver_logger::Service{
        name: name(),
        version: version()
    };
    stackdriver_logger::init_with(Some(service), false);
}

fn health_check() {
    spawn(move || {
        gotham::start(format!("127.0.0.1:{}", health_port()), || {
            Ok(|state| (state, "OK"))
        })
    });
}

fn application() {
    let (chain, pipelines) = single_pipeline(new_pipeline().add(LogIOMiddleware).build());

    let router = build_router(chain, pipelines, |route| {
        route.get("/").to(index::handle);
    });

    gotham::start(format!("127.0.0.1:{}", port()), router);
}

fn main() {
    logging();
    health_check();
    application();
}
