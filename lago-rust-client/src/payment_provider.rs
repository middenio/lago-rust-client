// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentProvider {
    Stripe,
    #[serde(rename = "null")]
    Webhook,
}
