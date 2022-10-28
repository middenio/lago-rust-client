// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

const DEFAULT_API_PATH: &str = "/api/v1/";
const DEFAULT_BASE_URL: &str = "https://api.getlago.com";
const DEFAULT_CONTENT_TYPE: &str = "application/json";
const ENV_LAGO_API_URI: &str = "LAGO_BASE_URI";

use crate::LagoApiKey;
use hyper::client::HttpConnector;
use hyper::{Body, Method, Response};
use hyper_tls::HttpsConnector;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type LagoResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type QueryParams = HashMap<String, String>;

///
pub trait LagoClient {
    fn new(client: &Client) -> LagoResult<Self>
    where
        Self: Sized;
}

///
pub trait Json<'de, S: Serialize, D: Deserialize<'de>> {
    fn as_json(&self) -> String;
}

/// Request abstraction for http client
#[derive(Debug)]
pub struct ClientRequest {
    pub method: Method,
    pub path: String,
    pub query_params: Option<QueryParams>,
    pub body: Body,
}

impl ClientRequest {
    pub fn new(method: Method, path: &str) -> Self {
        Self {
            method,
            path: path.to_owned(),
            query_params: None,
            body: Body::empty(),
        }
    }

    pub fn with_body(mut self, body: Body) -> Self {
        self.body = body;

        self
    }

    pub fn with_query(mut self, query: QueryParams) -> Self {
        self.query_params = Some(query);

        self
    }
}

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
    pub fn new() -> LagoResult<Self> {
        let api_key = LagoApiKey::from_env()?;
        let base_uri =
            std::env::var(ENV_LAGO_API_URI).unwrap_or_else(|_| DEFAULT_BASE_URL.to_owned());
        info!("setting lago api base uri to {}", base_uri);
        let https = HttpsConnector::new();

        Ok(Self {
            api_key,
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
    pub async fn send(self, req: ClientRequest) -> LagoResult<Response<Body>> {
        let request = hyper::Request::builder()
            .method(req.method)
            .uri(format!("{}{}", self.base_uri, req.path))
            .header("authorization", format!("Bearer {}", self.api_key))
            .header("content-type", DEFAULT_CONTENT_TYPE)
            .body(req.body)?;

        let response = self.client.request(request).await?;

        debug!(
            "request to {} received status {}",
            req.path,
            response.status()
        );

        Ok(response)
    }
}

#[cfg(test)]
mod lago_client_tests {
    use super::*;

    #[test]
    #[ignore]
    fn err_api_key_from_env() {
        std::env::remove_var("LAGO_API_KEY");
        let client = Client::new();

        assert!(client.is_err());
    }
}
