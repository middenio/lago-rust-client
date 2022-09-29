use std::io::Read;

///
use crate::{Charge, ChargeModel, Client, ClientRequest, Currency, LagoResult};
use hyper::{body::Buf, Method};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const PLANS_API_PATH: &str = "plans";

///
#[derive(Deserialize, Serialize)]
#[serde(rename = "lowercase")]
pub enum PlanInterval {
    Weekly,
    Monthly,
    Annually,
}

///
#[derive(Deserialize, Serialize)]
pub struct PlanParams {
    plan: PlanInput,
}

///
#[derive(Deserialize, Serialize)]
pub struct PlanChargeInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    lago_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billable_metric_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_model: Option<ChargeModel>,
    properties: Vec<String>,
}

///
#[derive(Deserialize, Serialize)]
pub struct PlanInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PlanInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_cents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_in_advance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bill_charge_monthly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charges: Option<Vec<Charge>>,
}

///
#[derive(Deserialize, Serialize)]
pub struct PlanListInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
}

///
#[derive(Deserialize, Serialize)]
pub struct Plan {
    lago_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PlanInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_cents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_in_advance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bill_charge_monthly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charges: Option<Vec<Charge>>,
}

/// Plan API
///
/// ```rust
///
/// ```
pub struct PlanRequest {
    client: Client,
}

impl PlanRequest {
    ///
    pub fn new(client: &Client) -> LagoResult<Self> {
        Ok(Self {
            client: client.clone(),
        })
    }

    ///
    pub async fn get(self, plan_code: &str) -> LagoResult<Plan> {
        let path = format!("{}/{}", PLANS_API_PATH, plan_code);

        let request = ClientRequest::new(Method::GET, &path);
        let response = self.client.send(request).await?;

        let body = hyper::body::aggregate(response).await?;

        let plan = serde_json::from_reader(body.reader().by_ref())?;

        Ok(plan)
    }

    ///
    pub async fn get_list() {}

    ///
    pub async fn create() {}

    ///
    pub async fn update() {}

    ///
    pub async fn delete() {}
}
