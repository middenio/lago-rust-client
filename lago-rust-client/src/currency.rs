///
use serde::{Deserialize, Serialize};
use std::default::Default;

///
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    Usd,
    Eur,
}

impl Default for Currency {
    /// Set default currency to USD
    fn default() -> Self {
        Self::Usd
    }
}
