use crate::LagoResult;
use log::info;

const ENV_API_KEY: &str = "LAGO_API_KEY";

/// Fetch API Key from the environment
///
/// # Example:
///
/// ```rust
/// use lago_rust_client::LagoApiKey;
///
/// let api_key = LagoApiKey::from_env();
///
/// ```
pub struct LagoApiKey {}

impl LagoApiKey {
    /// Returns lago key using default `LAGO_API_KEY` environment variable
    pub fn from_env() -> LagoResult<String> {
        LagoApiKey::with_env_var(ENV_API_KEY)
    }

    /// Returns lago key using a custom environment variable.
    pub fn with_env_var(var: &str) -> LagoResult<String> {
        info!("fetching api key from environment {}", var);
        std::env::var(var).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod api_key_tests {
    use super::*;

    const ENV_KEY: &str = "CUSTOM_LAGO_KEY";
    const KEY: &str = "secretkey!";

    #[test]
    #[ignore]
    fn custom_env_var_test() {
        std::env::set_var(ENV_KEY, KEY);

        let key = LagoApiKey::with_env_var(ENV_KEY).unwrap();

        std::env::remove_var(ENV_KEY);

        assert_eq!(key, KEY);
    }

    #[test]
    #[ignore]
    fn err_custom_env_var_test() {
        if std::env::var(ENV_KEY).is_ok() {
            std::env::remove_var(ENV_KEY);
        }

        let key = LagoApiKey::with_env_var(KEY);

        assert!(key.is_err());
    }
}
