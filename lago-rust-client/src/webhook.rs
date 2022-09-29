use crate::{payment_provider::PaymentProvider, Client, ClientRequest, LagoResult};
use hyper::body::Buf;
use hyper::Method;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

const WEBHOOK_API_PATH: &str = "webhooks";
const WEBHOOK_DEFAULT_ISS: &str = "https://api.getlago.com";
const WEBHOOK_SIGNATURE_HTTP_HEADER: &str = "X-Lago-Signature";
const WEBHOOK_SIGNATURE_VALIDATOR: &str = r#"
{
    "algorithm": "RS256",
    "iss": "https://api.getlago.com",
    "verify_iss": true,
}
"#;

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookObjectType {
    Customer,
    EventError,
    Invoice,
    PaymentProviderCustomerError,
    PaymentProviderInvoicePaymentError,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub enum WebhookMessageType {
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
    #[serde(rename = "invoice.addon_added")]
    AddonAdded,
    #[serde(rename = "invoice.payment_failure")]
    PaymentFailure,
    #[serde(rename = "event.error")]
    EventError,
    #[serde(rename = "customer.payment_provider_created")]
    PaymentProviderCreated,
    #[serde(rename = "invoice.payment_failure")]
    PaymentProviderError,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderError {
    code: String,
    message: String,
}

///
#[derive(Deserialize, Serialize)]
pub struct PaymentProviderInvoicePaymentError {
    lago_invoice_id: String,
    lago_customer_id: String,
    external_customer_id: String,
    provider_customer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_provider: Option<PaymentProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_error: Option<ProviderError>,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookMessage<T> {
    webhook_type: WebhookMessageType,
    object_type: WebhookObjectType,
    #[serde(flatten)]
    data: T,
}

///
pub struct WebhookRequest {
    client: Client,
    public_key: DecodingKey,
}

impl WebhookRequest {
    /// Create a new webhook request client
    ///
    /// ```rust
    /// ```
    pub async fn new(client: &Client) -> LagoResult<Self> {
        let path = format!("{}/public_key", WEBHOOK_API_PATH);
        let request = ClientRequest::new(Method::GET, &path);
        let response = client.clone().send(request).await?;
        let mut raw = [];

        hyper::body::aggregate(response)
            .await?
            .copy_to_slice(&mut raw);

        let key = DecodingKey::from_rsa_der(&raw);

        Ok(WebhookRequest {
            client: client.clone(),
            public_key: key,
        })
    }

    ///
    pub fn is_valid(self, signature: &str) -> bool {
        // let mut validation = Validation::new(Algorithm::RS256);
        // validation.set_issuer(&vec![WEBHOOK_DEFAULT_ISS]);
        // match decode(signature, &self.public_key, &validation) {
        //     Ok(_) => true,
        //     _ => false,
        // }

        true
    }

    ///
    pub fn validator(_iss: &str) -> String {
        WEBHOOK_SIGNATURE_VALIDATOR.to_string()
    }
}

#[cfg(test)]
mod webhook_tests {
    use super::*;

    const PUBLIC_KEY_B64: &str = r#"LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FR
OEFNSUlCQ2dLQ0FRRUFyMk9MQk9WTHROWk12Q09Xb2hkTwoxVjJaRzBYZUMzWE8xZTY3NWNCNXl4
cVoyR28yRWdvRVNKWllpdEFWdkFoZXNrZFM0Y0Zpd2NlVDdvbGRWSC9FCnNDVHlFZ1hpRkpkNXF0
M21FMkJRL3U3SzIvOVVuUXpUV0Q2MG0rOUNPL1hUYkxCWDIwZFRUQUJEa2lJWnhQZ3MKQ2ZhYzRT
V3MxUkFJbmpRNUFDQW1MdmxNK3pSWk10VlI2ZTBUcGNOQ2dGRzhkV3ppcUcyNTVOYlAyeG1LbzI5
TgpNYmtPUnhyZWIvNkg4MS94UWdHNzMxeEJOWWMxYjh5eDBWYmtnYjZwWGYrbHB1dGdBbkJyd0g2
VXFabVFHZHUyCkdpNVdLVWp2NWpGU2dldkNDNWxPNkVYYTlpaENzZWNZUXc2TExEakZaY3NLV2F6
OHV2a3R1STBtb2MxenUyZ3UKM3dJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg=="#;

    const PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAr2OLBOVLtNZMvCOWohdO
1V2ZG0XeC3XO1e675cB5yxqZ2Go2EgoESJZYitAVvAheskdS4cFiwceT7oldVH/E
sCTyEgXiFJd5qt3mE2BQ/u7K2/9UnQzTWD60m+9CO/XTbLBX20dTTABDkiIZxPgs
Cfac4SWs1RAInjQ5ACAmLvlM+zRZMtVR6e0TpcNCgFG8dWziqG255NbP2xmKo29N
MbkORxreb/6H81/xQgG731xBNYc1b8yx0Vbkgb6pXf+lputgAnBrwH6UqZmQGdu2
Gi5WKUjv5jFSgevCC5lO6EXa9ihCsecYQw6LLDjFZcsKWaz8uvktuI0moc1zu2gu
3wIDAQAB
-----END PUBLIC KEY-----
"#;

    #[test]
    #[ignore]
    fn b64_decode_test() {
        let txt = base64::decode(PUBLIC_KEY_B64.trim());
        println!("{:#?}", txt);
        assert!(txt.is_ok())
    }

    #[test]
    fn serialize_object_type_test() {}

    #[test]
    fn serialize_message_type_test() {}
}
