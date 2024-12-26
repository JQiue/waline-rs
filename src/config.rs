//! config

use serde::Deserialize;

use crate::error::AppError;

#[derive(Deserialize, Debug)]
pub struct Config {
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
  pub ipqps: Option<u64>,
  pub comment_audit: Option<bool>,
}

impl Config {
  pub fn from_env() -> Result<Config, AppError> {
    dotenvy::from_filename_override(".shuttle.env").ok();
    dotenvy::from_filename_override(".env").ok();
    envy::from_env().map_err(AppError::from)
  }
}
