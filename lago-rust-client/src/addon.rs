// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

use std::io::Read;

use crate::{Client, ClientRequest, Currency, LagoResult};
use chrono::{DateTime, Utc};
use hyper::body::Buf;
use hyper::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const ADDON_API_PATH: &str = "addon";

///
#[derive(Deserialize, Serialize)]
pub struct AddOnParams {
    add_on: AddOnInput,
}

///
#[derive(Deserialize, Serialize)]
pub struct AddOnInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_cents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_currency: Option<Currency>,
}

///
#[derive(Deserialize, Serialize)]
pub struct AddOnListInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
}

///
#[derive(Deserialize, Serialize)]
pub struct AddOn {
    #[serde(skip_serializing_if = "Option::is_none")]
    lago_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_cents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,
}

///
#[derive(Deserialize, Serialize)]
pub struct AppliedAddOn {
    #[serde(skip_serializing_if = "Option::is_none")]
    lago_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lago_addon_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lago_customer_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    add_on_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_cents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_currency: Option<Currency>,
}

///
pub struct AddOnRequest {
    client: Client,
}

impl AddOnRequest {
    pub fn new(client: &Client) -> LagoResult<Self> {
        Ok(Self {
            client: client.clone(),
        })
    }

    ///
    pub async fn get(self, add_on_code: &str) -> LagoResult<AddOn> {
        let path = format!("{}/{}", ADDON_API_PATH, add_on_code);
        let request = ClientRequest::new(Method::GET, &path);
        let response = self.client.send(request).await?;
        let body = hyper::body::aggregate(response).await?;
        let addon = serde_json::from_reader(body.reader().by_ref())?;

        Ok(addon)
    }

    ///
    pub async fn get_list() {}

    ///
    pub async fn create() {}

    ///
    pub async fn delete() {}

    ///
    pub async fn apply_to_customer() {}
}

#[cfg(test)]
mod addon_tests {}
