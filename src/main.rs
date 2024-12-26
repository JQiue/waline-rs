use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

mod app;
mod components;
mod config;
mod entities;
mod error;
mod helpers;
mod locales;
mod response;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
  Ok(app::start().await.into())
}
