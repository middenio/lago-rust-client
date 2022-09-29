use crate::{BillableMetric, Charge, Client, ClientRequest, Currency, LagoResult, PaymentProvider};
///
use chrono::{DateTime, Utc};
use hyper::body::Buf;
use hyper::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const CUSTOMER_API_PATH: &str = "customers";

pub type CustomerPaymentProvider = String;

///
#[derive(Deserialize, Serialize)]
pub struct CustomerParams {
    customer: CustomerInput,
}

///
#[derive(Deserialize, Serialize)]
pub struct CustomerInput {
    customer_id: String,
    name: String,
    address_line_1: String,
    address_line_2: String,
    city: String,
    zipcode: String,
    country: String,
    legal_name: String,
    legal_number: String,
    phone: String,
    url: String,
    billing_configuration: CustomerBillingConfiguration,
    vat_rate: f32,
}

///
#[derive(Deserialize, Serialize)]
pub struct CustomerBillingConfiguration {
    provider: PaymentProvider,
    provider_customer_id: Option<String>,
}

///
#[derive(Deserialize, Serialize)]
pub struct CustomerChargeUsage {
    units: String,
    amount_cents: i32,
    amount_currency: Currency,
    charge: Charge,
    billable_metric: BillableMetric,
}

///
#[derive(Deserialize, Serialize)]
pub struct CustomerUsage {
    from_date: String,
    to_date: String,
    issuing_date: String,
    amount_cents: String,
    amount_currency: Currency,
    total_amount_cents: i32,
    total_amount_currency: Currency,
    vat_amount_cents: i32,
    vat_amount_currency: Currency,
    charges_usage: Vec<CustomerChargeUsage>,
}

///
#[derive(Deserialize, Serialize)]
pub struct Customer {
    lago_id: Uuid,
    sequential_id: String,
    slug: String,
    external_id: String,
    name: String,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zipcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    billing_configuration: CustomerBillingConfiguration,
    vat_rate: f32,
    created_at: DateTime<Utc>,
}

///
pub struct CustomerRequest {
    client: Client,
}

impl CustomerRequest {
    ///
    pub fn new(client: &Client) -> LagoResult<Self> {
        Ok(Self {
            client: client.clone(),
        })
    }

    ///
    pub async fn create(self) {}

    ///
    pub async fn get(self, external_id: &str) -> LagoResult<Customer> {
        let path = format!("{}/{}", CUSTOMER_API_PATH, external_id);
        let request = ClientRequest::new(Method::GET, &path);
        let response = self.client.send(request).await?;
        let body = hyper::body::aggregate(response).await?;

        let customer: Customer = serde_json::from_reader(body.reader())?;

        Ok(customer)
    }

    ///
    pub async fn get_list(self) -> LagoResult<Vec<Customer>> {
        let request = ClientRequest::new(Method::GET, CUSTOMER_API_PATH);
        let response = self.client.send(request).await?;
        let body = hyper::body::aggregate(response).await?;

        let customers: Vec<Customer> = serde_json::from_reader(body.reader())?;

        Ok(customers)
    }

    ///
    pub async fn update(self) {}

    ///
    pub async fn usage(self) {}
}

#[cfg(test)]
mod customer_tests {}
