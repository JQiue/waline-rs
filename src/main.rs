mod app;
mod components;
mod config;
mod entities;
mod error;
mod helpers;
mod locales;
mod response;

#[actix_web::main]
async fn main() -> Result<(), error::AppError> {
  std::env::set_var("RUST_LOG", "debug");
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();
  app::start().await
}
