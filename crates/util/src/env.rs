use std::env;

pub struct EnvManager;

impl EnvManager {
    pub fn get_env_var(key: &str) -> Option<String> {
        env::var(key).ok().map_or_else(
            || {
                log::warn!("Environment variable `{}` is unset!", key);
                None
            },
            Some,
        )
    }

    pub fn get_env_var_or(key: &str, default: impl Into<String>) -> String {
        match env::var(key) {
            Ok(v) => v,
            Err(_) => default.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::EnvManager;

    #[allow(deprecated)]
    #[test]
    fn test_get_env_var() {
        let actual = EnvManager::get_env_var("HOME").unwrap_or("".into());
        let expect = env::home_dir().unwrap().display().to_string();
        assert_eq!(actual, expect);
    }
}
