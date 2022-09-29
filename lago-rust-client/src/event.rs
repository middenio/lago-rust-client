///
use crate::{Client, ClientRequest, LagoResult};
use hyper::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const EVENT_API_PATH: &str = "events";

#[derive(Clone, Deserialize, Serialize)]
///
pub struct EventParams {
    event: EventInput,
}

#[derive(Clone, Deserialize, Serialize)]
///
pub struct EventInput {
    transaction_id: String,
    customer_id: String,
    code: String,
    timestamp: i64,
    properties: HashMap<String, String>,
}

/// Event API
///
/// ```rust
///
/// ```
pub struct EventRequest {
    client: Client,
}

impl EventRequest {
    /// Instantiate new Event client
    pub fn new(client: &Client) -> LagoResult<Self> {
        Ok(Self {
            client: client.clone(),
        })
    }

    /// Create a new event
    pub async fn create(self, event: &EventInput) -> LagoResult<()> {
        let params = EventParams {
            event: event.clone(),
        };

        let request = ClientRequest::new(Method::POST, EVENT_API_PATH)
            .with_body(serde_json::to_string(&params)?.into());

        let _ = self.client.send(request).await?;

        Ok(())
    }
}
