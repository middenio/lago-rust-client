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
    pub fn from_env() -> Result<String, std::env::VarError> {
        LagoApiKey::with_env_var(ENV_API_KEY)
    }

    pub fn with_env_var(var: &str) -> Result<String, std::env::VarError> {
        info!("fetching api key from environment {}", var);
        std::env::var(var)
    }
}

#[cfg(test)]
mod api_key_tests {
    use super::*;

    const ENV_KEY: &str = "CUSTOM_LAGO_KEY";
    const KEY: &str = "secretkey!";

    #[test]
    fn custom_env_var_test() {
        std::env::set_var(ENV_KEY, KEY);

        let key = LagoApiKey::with_env_var(ENV_KEY).unwrap();

        std::env::remove_var(ENV_KEY);

        assert_eq!(key, KEY);        
    }

    #[test]
    fn err_custom_env_var_test() {
        if std::env::var(ENV_KEY).is_ok() {
            std::env::remove_var(ENV_KEY);
        }

        let key = LagoApiKey::with_env_var(KEY);

        println!("{:#?}", key);

        assert!(key.is_err());        
    }

}
