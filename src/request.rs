use hyper::{Body, Method};
use std::collections::HashMap;

pub type QueryParams = HashMap<String, String>;

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
