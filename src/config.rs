//! config

use serde::Deserialize;

use crate::error::AppError;

fn default_workers() -> usize {
  1
}

fn default_port() -> u16 {
  #[cfg(feature = "leancloud")]
  {
    std::env::var("LEANCLOUD_APP_PORT")
      .unwrap_or_else(|_| "8360".to_string())
      .parse()
      .unwrap_or(8360)
  }
  #[cfg(not(feature = "leancloud"))]
  {
    8360
  }
}

fn default_ipqps() -> u64 {
  60
}

fn default_host() -> String {
  "127.0.0.1".to_string()
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

fn default_disable_authore_notify() -> bool {
  false
}

#[derive(Deserialize)]
pub struct EnvConfig {
  #[serde(default = "default_workers")]
  pub workers: usize,
  #[serde(default = "default_host")]
  pub host: String,
  #[serde(default = "default_port")]
  pub port: u16,
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
  #[serde(default = "default_disable_authore_notify")]
  pub disable_author_notify: bool,
}

impl EnvConfig {
  pub fn load_env() -> Result<EnvConfig, AppError> {
    dotenvy::dotenv_override().ok();
    envy::from_env().map_err(AppError::from)
  }
}
