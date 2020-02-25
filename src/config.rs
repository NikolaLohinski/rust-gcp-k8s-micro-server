pub mod config {
    use std::env;
    use std::option::Option;

    fn get_env_var(key: String) -> Option<String> {
        match env::var(key) {
            Ok(val) => Some(val),
            Err(_e) => None,
        }
    }

    pub fn server_port() -> String {
        get_env_var("SERVER_PORT".to_string()).expect("SERVER_PORT is not defined")
    }

    pub fn level_verbosity() -> u8 {
        let value:String = get_env_var("LEVEL_VERBOSITY".to_string()).expect("LEVEL_VERBOSITY is not defined");
        value.parse::<u8>().expect("LEVEL_VERBOSITY is not a <u8>")
    }
}

