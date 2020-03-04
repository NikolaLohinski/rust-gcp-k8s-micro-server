use std::env;
use std::option::Option;

fn get_env_var(key: String) -> Option<String> {
    match env::var(key) {
        Ok(val) => Some(val),
        Err(_e) => None,
    }
}

pub fn health_port() -> String {
    get_env_var("SERVICE_HEALTH_PORT".to_string()).expect("SERVICE_HEALTH_PORT is not defined")
}

pub fn port() -> String {
    get_env_var("SERVICE_PORT".to_string()).expect("SERVICE_PORT is not defined")
}

pub fn name() -> String {
    get_env_var("SERVICE_NAME".to_string()).expect("SERVICE_NAME is not defined")
}

pub fn version() -> String {
    get_env_var("SERVICE_VERSION".to_string()).expect("SERVICE_VERSION is not defined")
}
