use std::env;

pub struct EnvManager;

impl EnvManager {
    pub fn get_env_var(key: &str) -> Option<String> {
        env::var(key).ok().map_or_else(
            || {
                log::error!("Environment variable `{}` is unset!", key);
                None
            },
            Some,
        )
    }
}

