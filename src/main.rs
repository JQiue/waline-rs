use tracing::Level;

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
  std::env::set_var("RUST_LOG", "error");
  tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .init();
  app::start().await
}
