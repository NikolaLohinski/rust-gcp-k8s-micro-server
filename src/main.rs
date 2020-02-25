extern crate gotham;
extern crate fern;
#[macro_use]
extern crate log;

use gotham::state::State;

mod config;
use config::config::{level_verbosity, server_port};
mod logger;
use logger::logger::setup_logger;

fn main() {
    setup_logger(level_verbosity()).expect("Failed to setup logger");
    gotham::start(format!("127.0.0.1:{}", server_port()), || Ok(index))
}

pub fn index(state: State) -> (State, &'static str) {
    (state, "Hello World!")
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate http;

    use gotham::test::TestServer;
    use self::http::status::StatusCode;

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