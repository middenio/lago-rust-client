///
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
///
pub enum Currency {
    Usd,
    Eur,
}

impl Default for Currency {
    fn default() -> Self {
        Self::Usd
    }
}