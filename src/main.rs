use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpServer};
use app::config_app;
use config::Config;
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

mod app;
mod components;
mod config;
mod entities;
mod helpers;

#[derive(Debug, Clone)]
struct AppState {
  conn: DatabaseConnection,
  anonymous_avatar: Arc<String>,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
  std::env::set_var("RUST_LOG", "info");

  let app_config = Config::from_env();

  tracing_subscriber::fmt::init();

  let conn = Database::connect(app_config.database_url).await.unwrap();

  match conn.ping().await {
    Ok(_) => info!("Database is ok!"),
    Err(error) => panic!("{error}"),
  }

  info!(
    "Server running at http://{}:{}",
    app_config.host, app_config.port
  );

  let state = AppState {
    conn,
    anonymous_avatar: "https://seccdn.libravatar.org/avatar/d41d8cd98f00b204e9800998ecf8427e"
      .to_string()
      .into(),
  };

  let _ = HttpServer::new(move || {
    let cors = Cors::permissive();
    App::new()
      .app_data(web::Data::new(state.clone()))
      .wrap(cors)
      .wrap(middleware::Logger::default())
      .configure(config_app)
  })
  .bind((app_config.host, app_config.port))?
  .workers(app_config.workers)
  .run()
  .await;
  Ok(())
}
