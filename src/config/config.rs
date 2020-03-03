use std::env;
use std::option::Option;

fn get_env_var(key: String) -> Option<String> {
    match env::var(key) {
        Ok(val) => Some(val),
        Err(_e) => None,
    }
}

pub fn health_port() -> String {
    get_env_var("HEALTH_PORT".to_string()).expect("HEALTH_PORT is not defined")
}

pub fn server_port() -> String {
    get_env_var("SERVER_PORT".to_string()).expect("SERVER_PORT is not defined")
}
