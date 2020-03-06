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
