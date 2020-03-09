use gotham::handler::HandlerFuture;
use gotham::state::State;

use config::config::{name, version};

#[derive(StateData)]
pub struct Configuration {
    pub name: String,
    pub version: String,
}

#[derive(Clone, NewMiddleware)]
pub struct Middleware;

impl gotham::middleware::Middleware for Middleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        state.put(Configuration {
            name: name(),
            version: version(),
        });

        Box::new(chain(state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate http;

    use self::http::status::StatusCode;
    use gotham::pipeline::new_pipeline;
    use gotham::pipeline::single::single_pipeline;
    use gotham::router::builder::*;
    use gotham::state::FromState;
    use gotham::test::TestServer;

    use config::config::ENV_NAME;
    use config::config::ENV_VERSION;

    #[test]
    fn ensure_middleware_sets_state_correctly() {
        std::env::set_var(ENV_VERSION, "0.1");
        std::env::set_var(ENV_NAME, "test");
        let (chain, pipelines) = single_pipeline(new_pipeline().add(Middleware).build());

        let router = build_router(chain, pipelines, |route| {
            route.get("/").to(|mut state| {
                let configuration = Configuration::borrow_mut_from(&mut state);
                let message = format!(
                    "name={},version={}",
                    configuration.name, configuration.version
                );
                (state, message)
            });
        });

        let test_server = TestServer::new(router).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(
            std::str::from_utf8(&body[..]).unwrap(),
            "name=test,version=0.1"
        );
    }
}
