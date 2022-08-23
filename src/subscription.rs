use crate::{Client, ClientRequest};
use chrono::{DateTime, Utc};
use hyper::body::Buf;
use hyper::Method;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::io::{Read};
use uuid::Uuid;

const SUBSCRIPTION_API_PATH: &str = "subscriptions";

/// Subscription Status
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Pending,
    Terminated,
    Canceled,
}

#[derive(Deserialize, Serialize)]
pub struct SubscriptionInput {
    customer_id: String,
    plan_code: Option<String>,
}

impl SubscriptionInput {
    pub fn new(customer_id: &str) -> Self {
        Self {
            customer_id: customer_id.to_owned(),
            plan_code: None,
        }
    }

    pub fn with_plan(mut self, plan_code: &str) -> Self {
        self.plan_code = Some(plan_code.to_owned());

        self
    }
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct Subscriptions {
    subscriptions: Vec<Subscription>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscription {
    lago_id: Uuid,
    lago_customer_id: Uuid,
    customer_id: String,
    name: String,
    unique_id: Uuid,
    plan_code: String,
    status: SubscriptionStatus,
    started_at: DateTime<Utc>,
    terminated_at: Option<DateTime<Utc>>,
    canceled_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
}

pub struct SubscriptionClient {
    client: Client,
}

impl SubscriptionClient {
    pub fn new(client: &Client) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self {
            client: client.clone(),
        })
    }

    /// Get all customer subscriptions
    pub async fn get_all(
        self,
        customer_id: &str,
    ) -> Result<Subscriptions, Box<dyn std::error::Error + Send + Sync>> {
        info!("requesting subscriptions for customer {}", customer_id);
        let body = serde_json::to_string(&SubscriptionInput::new(customer_id))?;
        let request = ClientRequest::new(Method::GET, SUBSCRIPTION_API_PATH).with_body(body.into());

        let response = self.client.send(request).await?;

        let body = hyper::body::aggregate(response).await?;

        let subscriptions: Subscriptions = serde_json::from_reader(body.reader().by_ref())?;

        debug!("\n\nRESPONSE:\n {:#?}", subscriptions);

        Ok(subscriptions)
    }

    pub async fn terminate(
        self,
        customer_id: &str,
    ) -> Result<Subscription, Box<dyn std::error::Error + Send + Sync>> {
        let body = serde_json::to_string(&SubscriptionInput::new(customer_id))?;
        let request =
            ClientRequest::new(Method::DELETE, SUBSCRIPTION_API_PATH).with_body(body.into());

        let response = self.client.send(request).await?;

        let body = hyper::body::aggregate(response).await?;
        let subscription = serde_json::from_reader(body.reader())?;

        debug!("RESPONSE: {:#?}", subscription);

        Ok(subscription)
    }
}

#[cfg(test)]
mod subscription_tests {
    use super::*;

    const B_SUBSCRIPTIONS: &[u8] = br#" 
    {
        "subscriptions": [
            {
                "lago_id": "30dec4e0-d390-4a60-84b4-01a7116c2250",
                "lago_customer_id": "54f0a14c-0d59-41b9-b6cd-689f55a3dbab",
                "customer_id": "123456",
                "name": "",
                "unique_id": "da4bbba8-eb7a-432f-b3c4-2090383762de",
                "plan_code": "plan_test",
                "status": "active",
                "started_at": "2022-08-20T12:29:37Z",
                "terminated_at": null,
                "canceled_at": null,
                "created_at": "2022-08-20T12:29:37Z"
            }
        ],
        "meta": {
            "current_page": 1,
            "next_page": null,
            "prev_page": null,
            "total_pages": 1,
            "total_count": 1
        }
    }"#;

    const SUBSCRIPTION: &str = r#"
    {
        "subscriptions": [
            {
                "lago_id": "30dec4e0-d390-4a60-84b4-01a7116c2250",
                "lago_customer_id": "54f0a14c-0d59-41b9-b6cd-689f55a3dbab",
                "customer_id": "123456",
                "name": "",
                "unique_id": "da4bbba8-eb7a-432f-b3c4-2090383762de",
                "plan_code": "plan_test",
                "status": "active",
                "started_at": "2022-08-20T12:29:37Z",
                "terminated_at": null,
                "canceled_at": null,
                "created_at": "2022-08-20T12:29:37Z"
            }
        ],
        "meta": {
            "current_page": 1,
            "next_page": null,
            "prev_page": null,
            "total_pages": 1,
            "total_count": 1
        }
    }"#;

    #[test]
    fn subscription_deserialize_test() {
        let s: Result<Subscriptions, serde_json::Error> = serde_json::from_str(&SUBSCRIPTION);

        assert!(s.is_ok());
    }

    #[test]
    fn subscription_deserialize_reader_test() {
        let v = B_SUBSCRIPTIONS.to_vec();
        let temp = std::str::from_utf8(&v).unwrap();

        let s: Result<Subscriptions, serde_json::Error> =
            serde_json::from_reader(B_SUBSCRIPTIONS.reader().by_ref());
        let s2: Result<Subscriptions, serde_json::Error> = serde_json::from_str(temp);

        assert!(s.is_ok());
        assert!(s2.is_ok());
    }
}
