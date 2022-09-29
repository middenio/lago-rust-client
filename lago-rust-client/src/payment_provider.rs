use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentProvider {
    Stripe,
    #[serde(rename = "null")]
    Webhook,
}
