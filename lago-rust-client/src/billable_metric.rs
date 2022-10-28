// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

use crate::{Client, LagoResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

///
#[derive(Deserialize, Serialize)]
pub enum AggregationType {
    #[serde(rename = "count_agg")]
    Count,
    #[serde(rename = "sum_agg")]
    Sum,
    #[serde(rename = "max_agg")]
    Max,
    #[serde(rename = "unique_count_agg")]
    UniqueCount,
}

///
#[derive(Deserialize, Serialize)]
pub struct BillableMetric {
    lago_id: Uuid,
}

///
pub struct BillableMetricRequest {
    client: Client,
}

impl BillableMetricRequest {
    ///
    pub fn new(client: &Client) -> LagoResult<Self> {
        Ok(Self {
            client: client.clone(),
        })
    }

    ///
    pub async fn get() {}

    ///
    pub async fn get_list() {}

    ///
    pub async fn create() {}

    ///
    pub async fn update() {}

    ///
    pub async fn delete() {}
}
