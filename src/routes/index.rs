extern crate gotham;

use gotham::state::State;

pub fn handle(state: State) -> (State, &'static str) {
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
        let test_server = TestServer::new(|| Ok(handle)).unwrap();
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