use futures::future;
use futures::prelude::*;
use gotham::handler::{HandlerFuture, IntoResponse};
use gotham::state::State;
use log::info;

#[derive(Clone, NewMiddleware)]
pub struct Middleware;

impl gotham::middleware::Middleware for Middleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        info!("received request");

        let result = chain(state);

        let f = result.then(move |result| {
            info!("handled request");

            match result {
                Ok((state, response)) => future::ok((state, response)),
                Err((state, handler_error)) => {
                    let response = handler_error.into_response(&state);
                    future::ok((state, response))
                }
            }
        });

        Box::new(f)
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use self::tempfile::NamedTempFile;
    use gotham::pipeline::new_pipeline;
    use gotham::pipeline::single::single_pipeline;
    use gotham::router::builder::*;
    use gotham::test::TestServer;
    use hamcrest::prelude::*;
    use http::status::StatusCode;
    use log::LevelFilter;

    #[test]
    fn ensure_middleware_logs_input_and_output() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        simple_logging::log_to_file(path.clone(), LevelFilter::Info).unwrap();

        let (chain, pipelines) = single_pipeline(new_pipeline().add(Middleware).build());

        let router = build_router(chain, pipelines, |route| {
            route.get("/").to(|state| (state, "OK"));
        });

        let test_server = TestServer::new(router).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_that!(response.status(), is(equal_to(StatusCode::OK)));

        let file = File::open(path).unwrap();
        let mut lines = String::new();
        for line in BufReader::new(file).lines() {
            lines = lines + line.unwrap().as_ref();
        }
        assert_that!(lines.contains("received request"), is(true));
        assert_that!(lines.contains("handled request"), is(true));
    }
}
