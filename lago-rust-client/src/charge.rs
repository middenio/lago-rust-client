// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;

///
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargeModel {
    Standard,
    Graduated,
    Package,
    Percentage,
}

///
#[derive(Deserialize, Serialize)]
pub struct Charge {
    #[serde(skip_serializing_if = "Option::is_none")]
    lago_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lago_billable_metric_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_model: Option<ChargeModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<String>>,
}
