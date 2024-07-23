use envy::from_env;
use serde::Deserialize;

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
}

impl Config {
  pub fn from_env() -> Config {
    dotenvy::dotenv_override().ok();
    from_env().expect("deserialize from env")
  }
}
