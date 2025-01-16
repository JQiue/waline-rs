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

#[actix_web::main]
async fn main() -> Result<(), error::AppError> {
  std::env::set_var("RUST_LOG", "info");
  let filter = filter::Targets::new()
    .with_default(LevelFilter::INFO)
    .with_target("actix-web", LevelFilter::INFO)
    .with_target("waline-mini", LevelFilter::INFO)
    .with_target("sqlx::query", LevelFilter::OFF);
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .pretty()
        .with_timer(fmt::time::LocalTime::rfc_3339()),
    )
    .with(filter)
    .with(EnvFilter::from_default_env())
    .init();
  app::start().await
}
