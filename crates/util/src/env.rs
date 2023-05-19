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
}

#[cfg(test)]
mod tests {
    use super::EnvManager;

    #[test]
    fn test_get_env_var() {
        let expect = EnvManager::get_env_var("HOME").unwrap_or("".into());
        let mut actual = "/home/".to_string();
        actual.push_str(
            EnvManager::get_env_var("USER")
                .unwrap_or("".into())
                .as_str(),
        );
        assert_eq!(actual, expect);
    }
}
