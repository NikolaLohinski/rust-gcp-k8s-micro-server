use log::info;

use futures::future;
use futures::prelude::*;
use gotham::handler::{HandlerFuture, IntoResponse};
use gotham::middleware::Middleware;
use gotham::state::State;

#[derive(Clone, NewMiddleware)]
pub struct LogIOMiddleware;

impl Middleware for LogIOMiddleware {
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
