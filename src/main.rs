extern crate fern;
extern crate gotham;
extern crate stackdriver_logger;
#[macro_use]
extern crate log;

use gotham::state::State;
use log::info;

mod config;
use config::config::server_port;

fn main() {
    stackdriver_logger::init_with_cargo!();
    gotham::start(format!("127.0.0.1:{}", server_port()), || Ok(index))
}

pub fn index(state: State) -> (State, &'static str) {
    info!("received request");
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
