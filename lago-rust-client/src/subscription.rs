// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

use crate::{Client, ClientRequest, LagoResult};
use chrono::{DateTime, NaiveDate, Utc};
use hyper::body::Buf;
use hyper::Method;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::io::Read;
use uuid::Uuid;

// use lago_macros::lago_client;

const SUBSCRIPTION_API_PATH: &str = "subscriptions";

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BillingTime {
    Anniversary,
    Calendar,
}

/// Subscription Status
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Pending,
    Terminated,
    Canceled,
}

///
#[derive(Deserialize, Serialize)]
pub struct SubscriptionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_time: Option<BillingTime>,
}

impl SubscriptionInput {
    pub fn new(customer_id: &str) -> Self {
        Self {
            customer_id: Some(customer_id.to_owned()),
            plan_code: None,
            billing_time: None,
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

///
#[derive(Debug, Deserialize, Serialize)]
pub struct Subscription {
    lago_id: Uuid,
    lago_customer_id: Uuid,
    external_id: String,
    external_customer_id: String,
    subscription_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    plan_code: String,
    status: SubscriptionStatus,
    billing_time: BillingTime,
    started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canceled_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    previous_plan_code: Option<String>,
    next_plan_code: Option<String>,
    downgrade_plan_date: Option<String>,
}

///
pub struct SubscriptionRequest {
    client: Client,
}

impl SubscriptionRequest {
    pub fn new(client: &Client) -> LagoResult<Self> {
        Ok(Self {
            client: client.clone(),
        })
    }

    /// Get all customer subscriptions
    pub async fn get_all(self, customer_id: &str) -> LagoResult<Subscriptions> {
        info!("requesting subscriptions for customer {}", customer_id);
        let body = serde_json::to_string(&SubscriptionInput::new(customer_id))?;
        let request = ClientRequest::new(Method::GET, SUBSCRIPTION_API_PATH).with_body(body.into());

        let response = self.client.send(request).await?;

        let body = hyper::body::aggregate(response).await?;

        let subscriptions: Subscriptions = serde_json::from_reader(body.reader().by_ref())?;

        debug!("\n\nRESPONSE:\n {:#?}", subscriptions);

        Ok(subscriptions)
    }

    ///
    pub async fn terminate(self, customer_id: &str) -> LagoResult<Subscription> {
        let body = serde_json::to_string(&SubscriptionInput::new(customer_id))?;
        let request =
            ClientRequest::new(Method::DELETE, SUBSCRIPTION_API_PATH).with_body(body.into());

        let response = self.client.send(request).await?;

        let body = hyper::body::aggregate(response).await?;
        let subscription = serde_json::from_reader(body.reader().by_ref())?;

        debug!("RESPONSE: {:#?}", subscription);

        Ok(subscription)
    }
}

#[cfg(test)]
mod subscription_tests {
    use super::*;

    const SUBSCRIPTION: &str = r#"
        {
            "lago_id": "e10843ca-7027-4a6a-8118-ade476d61582",
            "external_id": "0a25773d-09eb-47d3-ad21-6935f05e6025",
            "lago_customer_id": "3662d9fd-5e6e-414a-b011-7d9aa17201d4",
            "external_customer_id": "123456",
            "name": "Basic Plan Subscription",
            "plan_code": "plan_basic",
            "status": "active",
            "billing_time": "anniversary",
            "subscription_date": "2022-09-14",
            "started_at": "2022-09-14T09:59:59Z",
            "terminated_at": null,
            "canceled_at": null,
            "created_at": "2022-09-14T09:59:59Z"
        }"#;

    #[test]
    fn subscription_deserialize_test() {
        let s: Result<Subscription, serde_json::Error> = serde_json::from_str(&SUBSCRIPTION);

        assert!(s.is_ok());
    }
}
