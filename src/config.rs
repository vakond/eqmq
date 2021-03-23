//! eqmq config module.

pub const EXCHANGE: &str = "equilibrium";
pub const QUEUE: &str = "events";
pub const ROUTE: &str = "test";

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents the publisher config.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Publisher {
    pub endpoint: String,
}

/// Represents the consumer config.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Consumer {
    pub endpoint: String,
}

/// Represents the main config.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EqMq {
    pub environment: String,
    pub publisher: Publisher,
    pub consumer: Consumer,
}

/// Implements the constructor of the config.
impl EqMq {
    pub fn load(filename: &Path) -> Self {
        let text = read(filename).expect("cannot read the config file");
        let cfg: EqMq = serde_json::from_str(&text).expect("invalid JSON in the config file");
        cfg
    }
}

/// Reads the main config from a file.
fn read(filename: &Path) -> anyhow::Result<String> {
    use anyhow::Context;
    let text = format!("{:?}", filename);
    Ok(std::fs::read_to_string(filename).context(text)?)
}
