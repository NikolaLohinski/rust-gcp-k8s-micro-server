extern crate fern;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate stackdriver_logger;
extern crate log;
extern crate futures;
extern crate hyper;

use log::info;

use futures::future;
use futures::prelude::*;
use gotham::handler::{HandlerFuture, IntoResponse};
use gotham::middleware::Middleware;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

mod config;
use config::config::server_port;

#[derive(Clone, NewMiddleware)]
pub struct LogMiddleware;

impl Middleware for LogMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
        where
            Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        info!("received request");
        let result = chain(state);

        let f = result.then(move |result| {
            let (state, response) = match result {
                Ok((state, response)) => (state, response),
                Err((state, handler_error)) => {
                    let response = handler_error.into_response(&state);
                    (state, response)
                }
            };
            {
                info!("request handled");
            };
            future::ok((state, response))
        });
        Box::new(f)
    }
}

fn router() -> Router {
    let (chain, pipelines) = single_pipeline(new_pipeline().add(LogMiddleware).build());
    build_router(chain, pipelines, |route| {
        route.get("/").to(index);
    })
}


fn main() {
    stackdriver_logger::init_with_cargo!();
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
