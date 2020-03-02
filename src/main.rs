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
use gotham::router::Router;
use gotham::state::State;

mod config;
mod middleware;
use config::config::{health_port, server_port};
use middleware::middleware::LogIOMiddleware;

fn router() -> Router {
    let (chain, pipelines) = single_pipeline(new_pipeline().add(LogIOMiddleware).build());
    build_router(chain, pipelines, |route| {
        route.get("/").to(index);
    })
}

fn main() {
    stackdriver_logger::init_with_cargo!();

    // Health check
    spawn(move || {
        gotham::start(format!("127.0.0.1:{}", health_port()), || {
            Ok(|state| (state, "ok"))
        })
    });

    // Application
    gotham::start(format!("127.0.0.1:{}", server_port()), router());
}

pub fn index(state: State) -> (State, &'static str) {
    (state, "Hello World!")
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate http;

    use self::http::status::StatusCode;
    use gotham::test::TestServer;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(index)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost:8080")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
