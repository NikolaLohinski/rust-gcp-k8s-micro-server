use std::env;
use std::option::Option;

fn get_env_var(key: String) -> Option<String> {
    match env::var(key) {
        Ok(val) => Some(val),
        Err(_e) => None,
    }
}

pub const ENV_HEALTH_PORT: &str = "SERVICE_HEALTH_PORT";
pub fn health_port() -> String {
    get_env_var(ENV_HEALTH_PORT.to_string()).unwrap()
}

pub const ENV_PORT: &str = "SERVICE_PORT";
pub fn port() -> String {
    get_env_var(ENV_PORT.to_string()).unwrap()
}

pub const ENV_NAME: &str = "SERVICE_NAME";
pub fn name() -> String {
    get_env_var(ENV_NAME.to_string()).unwrap()
}

pub const ENV_VERSION: &str = "SERVICE_VERSION";
pub fn version() -> String {
    get_env_var(ENV_VERSION.to_string()).unwrap()
}
