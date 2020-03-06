use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use http::response::Response;
use http::status::StatusCode;
use hyper::Body;

use middlewares::configuration::Configuration;

#[derive(Serialize)]
pub struct IndexResponse {
    from: String,
}

impl IntoResponse for IndexResponse {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serializable response"),
        )
    }
}

pub fn handle(mut state: State) -> (State, IndexResponse) {
    let configuration = Configuration::borrow_mut_from(&mut state);
    let response = IndexResponse {
        from: format!("{} (v{})", configuration.name, configuration.version),
    };
    (state, response)
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate http;
    use self::http::status::StatusCode;
    use gotham::test::TestServer;

    #[test]
    fn receive_default_response() {
        let test_server = TestServer::new(|| {
            Ok(|mut s: State| {
                s.put(Configuration {
                    name: String::from("test"),
                    version: String::from("α"),
                });
                handle(s)
            })
        })
        .unwrap();
        let response = test_server
            .client()
            .get("http://localhost:8080")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(
            std::str::from_utf8(&body[..]).unwrap(),
            r#"{"from":"test (vα)"}"#
        );
    }

    #[test]
    fn receive_response_without_state_configuration() {
        let test_server = TestServer::new(|| Ok(handle)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost:8080")
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
