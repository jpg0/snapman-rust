use crate::log::LogLevel;
use std::path::PathBuf;
use figment::{Figment, providers::{Format, Toml}};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct SnapManConfig {
    pub snapraid_config: PathBuf,
    pub snapraid_binary: PathBuf,
    pub log_level: LogLevel,
}

pub fn load_config(path: PathBuf) -> Result<SnapManConfig, String> {
    return Figment::new()
        .merge(Toml::file(path))
        .extract::<SnapManConfig>().map_err(|e| e.to_string());
}