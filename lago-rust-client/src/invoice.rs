// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

use crate::{Currency, Subscription, WebhookMessageType, WebhookObjectType};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

///
#[derive(Debug, Deserialize, Serialize)]
pub struct InvoiceData {
    lago_id: String,
    sequential_id: u32,
    number: String,
    // from_date: DateTime<Utc>,
    // to_date: DateTime<Utc>,
    issuing_date: NaiveDate,
    file_url: String,
    subscriptions: Vec<Subscription>,
    // fees: Vec<Fee>,
    credits: Vec<Credit>,
}

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CreditType {
    Coupon,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct CreditItem {
    r#type: CreditType,
    code: String,
    name: String,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct Credit {
    item: CreditItem,
    amount_cents: i32,
    amount_currency: Currency,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct Fee {
    item: FeeItem,
    amount_cents: u32,
    amount_currency: Currency,
    vat_amount_cents: u32,
    vat_amount_currency: Currency,
    units: String,
    events_count: Option<i32>,
}

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FeeType {
    Charge,
    Subscription,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct FeeItem {
    r#type: FeeType,
    amount_cents: i32,
    amount_currency: Currency,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct Invoice {
    webhook_type: WebhookMessageType,
    object_type: WebhookObjectType,
    invoice: InvoiceData,
}

#[cfg(test)]
mod invoice_tests {
    use super::*;

    const INVOICE_STR: &str = r#"
    {
      "webhook_type": "invoice.created",
      "object_type": "invoice",
      "invoice": {
        "lago_id": "5eb02857-a71e-4ea2-bcf9-57d3a41bc6ba",
        "sequential_id": 2,
        "number": "LAG-1234-001-002",
        "issuing_date": "2022-04-30",
        "status": "succeeded",
        "amount_cents": 100,
        "amount_currency": "EUR",
        "vat_amount_cents": 20,
        "vat_amount_currency": "EUR",
        "total_amount_cents": 120,
        "total_amount_currency": "EUR",
        "file_url": "https://getlago.com/invoice/file",
        "customer": {
          "lago_id": "99a6094e-199b-4101-896a-54e927ce7bd7",
          "external_id": "5eb02857-a71e-4ea2-bcf9-57d3a41bc6ba",
          "address_line1": "5230 Penfield Ave",
          "address_line2": null,
          "city": "Woodland Hills",
          "country": "US",
          "created_at": "2022-04-29T08:59:51Z",
          "email": "dinesh@piedpiper.test",
          "legal_name": "Coleman-Blair",
          "legal_number": "49-008-2965",
          "logo_url": "http://hooli.com/logo.png",
          "name": "Gavin Belson",
          "phone": "1-171-883-3711 x245",
          "state": "CA",
          "url": "http://hooli.com",
          "vat_rate": 20,
          "zipcode": "91364"
        },
        "subscriptions": [
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
          }
        ],
        "fees": [
          {
            "item": {
              "type": "subscription",
              "code": "plan_code",
              "name": "Plan"
            },
            "amount_cents": 100,
            "amount_currency": "EUR",
            "vat_amount_cents": 20,
            "vat_amount_currency": "EUR",
            "units": "0.32",
            "events_count": 23
          }
        ],
        "credits": [
          {
            "item": {
              "type": "coupon",
              "code": "coupon_code",
              "name": "Coupon"
            },
            "amount_cents": 100,
            "amount_currency": "EUR"
          }
        ]
      }
    }
    "#;

    #[test]
    fn invoice_serialize_test() {
        let invoice: Result<Invoice, serde_json::Error> = serde_json::from_str(&INVOICE_STR);

        println!("{:#?}", invoice);

        assert!(invoice.is_ok())
    }
}
