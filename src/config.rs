//! config

use serde::Deserialize;

use crate::error::AppError;

fn default_ipqps() -> u64 {
  60
}

fn default_akismet_key() -> String {
  "86fe49f5ea50".to_string()
}

fn default_comment_audit() -> bool {
  false
}

fn default_login() -> String {
  "no".to_string()
}

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
  pub database_url: String,
  pub jwt_token: String,
  pub site_name: String,
  pub site_url: String,
  pub smtp_service: Option<String>,
  pub smtp_host: Option<String>,
  pub smtp_port: Option<u16>,
  pub smtp_user: Option<String>,
  pub smtp_pass: Option<String>,
  pub author_email: Option<String>,
  pub levels: Option<String>,
  #[serde(default = "default_ipqps")]
  pub ipqps: u64,
  #[serde(default = "default_comment_audit")]
  pub comment_audit: bool,
  #[serde(default = "default_akismet_key")]
  pub akismet_key: String,
  #[serde(default = "default_login")]
  pub login: String,
}

impl EnvConfig {
  pub fn load_env() -> Result<EnvConfig, AppError> {
    dotenvy::from_filename_override(".shuttle.env").ok();
    dotenvy::from_filename_override(".env").ok();
    envy::from_env().map_err(AppError::from)
  }
}
