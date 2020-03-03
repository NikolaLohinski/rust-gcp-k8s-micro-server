extern crate gotham;

use gotham::state::State;

pub fn handle(state: State) -> (State, &'static str) {
    (state, "Hello World!")
}
