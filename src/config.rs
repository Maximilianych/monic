use std::{path::PathBuf, time::Duration};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ServiceConfig {
    pub name: String,
    pub service_type: String,
    pub target: String,
    #[serde(with = "humantime_serde")]
    pub interval: Duration,
    #[serde(with = "humantime_serde")]
    pub timeout: Duration
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub services: Vec<ServiceConfig>
}

impl Config {
    pub fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let raw_config = std::fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&raw_config)?;
        Ok(config)
    }
}