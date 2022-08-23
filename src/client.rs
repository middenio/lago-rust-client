const DEFAULT_API_PATH: &str = "/api/v1/";
const DEFAULT_BASE_URL: &str = "https://api.getlago.com";
const DEFAULT_CONTENT_TYPE: &str = "application/json";
const ENV_LAGO_API_URI: &str = "LAGO_BASE_URI";

use crate::{ClientRequest, LagoApiKey};
use log::{debug, info};

use hyper::client::HttpConnector;
use hyper::{Body, Response};
use hyper_tls::HttpsConnector;

/// Lago Http Client
/// 
/// ```rust
/// 
/// ```
#[derive(Clone)]
pub struct Client {
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    api_key: String,
    pub base_uri: String,
}

impl Client {
    /// Create a new client with defaults
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let api_key = LagoApiKey::from_env()?;
        let base_uri = std::env::var(ENV_LAGO_API_URI).unwrap_or(DEFAULT_BASE_URL.to_owned());
        info!("setting lago api base uri to {}", base_uri);
        let https = HttpsConnector::new();
    
        Ok(Self {
            api_key: api_key,
            base_uri: format!("{}{}", base_uri, DEFAULT_API_PATH),
            client: hyper::Client::builder().build::<_, hyper::Body>(https),                
        })
    }

    /// Set client api key
    ///
    /// ```rust
    ///
    /// ```
    pub fn set_api_key(mut self, api_key: &str) -> Self {
        debug!("overriding api key from environment");
        self.api_key = api_key.to_owned();

        self
    }

    /// Set custom base uri for the client.  For use with self hosted.
    ///
    /// ```rust
    ///
    /// ```
    pub fn set_base_url(mut self, uri: &str) -> Self {
        info!("overriding default base uri to {}", uri);
        self.base_uri = uri.to_owned();

        self
    }

    /// Execute the `ClientRequest` against the requested lago api
    /// client.
    /// 
    /// ```rust
    /// 
    /// ```
    pub async fn send(
        self,
        req: ClientRequest,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        let request = hyper::Request::builder()
            .method(req.method)
            .uri(format!("{}{}", self.base_uri, req.path))
            .header("authorization", format!("Bearer {}", self.api_key))
            .header("content-type", DEFAULT_CONTENT_TYPE)
            .body(req.body)?;

        let response = self.client.request(request).await?;
        
        debug!("request to {} received status {}", req.path, response.status());

        Ok(response)
    }
}

#[cfg(test)]
mod lago_client_tests {
    use super::*;

    #[test]
    fn err_api_key_from_env() {
        std::env::remove_var("LAGO_API_KEY");
        let client = Client::new();

        assert!(client.is_err());
    }
}
