use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app;
mod components;
mod config;
mod entities;
mod error;
mod helpers;
mod locales;
mod prelude;
mod response;
mod traits;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
  Ok(app::start().await.into())
}
