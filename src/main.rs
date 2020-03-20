#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[cfg(test)]
#[macro_use]
extern crate assert_json_diff;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

extern crate fern;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate futures;
extern crate hyper;
extern crate log;
extern crate stackdriver_logger;
#[macro_use]
extern crate serde_derive;
extern crate alloc;
extern crate chrono;
extern crate http;
extern crate serde;

use std::thread::spawn;

use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;

mod config;
mod middlewares;
mod routes;

use config::config::{health_port, name, port, version};
use middlewares::{configuration, log_io};
use routes::index;

fn logging() {
    let service = stackdriver_logger::Service {
        name: name(),
        version: version(),
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
    let (chain, pipelines) = single_pipeline(
        new_pipeline()
            .add(log_io::Middleware)
            .add(configuration::Middleware)
            .build(),
    );

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
