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
  pub site_name: String,
  pub site_url: String,
  pub smtp_service: Option<String>,
  pub smtp_host: Option<String>,
  pub smtp_port: Option<u16>,
  pub smtp_user: Option<String>,
  pub smtp_pass: Option<String>,
  pub author_email: Option<String>,
  pub levels: Option<String>,
  pub comment_audit: Option<bool>,
}

impl Config {
  pub fn from_env() -> Result<Config, AppError> {
    dotenvy::dotenv_override().ok();
    envy::from_env().map_err(AppError::from)
  }
}
