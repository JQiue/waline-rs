//! config

use serde::Deserialize;

use crate::error::AppError;

fn default_workers() -> usize {
  1
}

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(default = "default_workers")]
  pub workers: usize,
  pub host: String,
  pub port: u16,
  pub database_url: String,
  pub jwt_key: String,
  pub levels: Option<String>,
}

impl Config {
  pub fn from_env() -> Result<Config, AppError> {
    dotenvy::dotenv_override().ok();
    envy::from_env().map_err(AppError::from)
  }
}
