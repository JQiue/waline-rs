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
  pub smtp_service: String,
  pub smtp_user: String,
  pub smtp_pass: String,
  pub author_email: String,
  pub site_name: String,
  pub site_url: String,
}

impl Config {
  pub fn from_env() -> Result<Config, AppError> {
    dotenvy::dotenv_override().ok();
    envy::from_env().map_err(AppError::from)
  }
}
