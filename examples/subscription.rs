// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

use lago_rust_client::{Client, SubscriptionRequest};
use log::{debug, info};

extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    info!("creating client");
    let lago = Client::new()?;

    let client = SubscriptionRequest::new(&lago)?;

    let response = client.get_all("123456").await?;

    debug!("{:#?}", response);

    Ok(())
}
