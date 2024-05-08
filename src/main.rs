use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use services::config;
use std::env;
use tracing::info;

mod entities;
mod helpers;
mod services;

#[derive(Debug, Clone)]
struct AppState {
  conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  std::env::set_var("RUST_LOG", "info");

  tracing_subscriber::fmt::init();

  dotenvy::dotenv_override().ok();
  let workers = env::var("WORKERS")
    .unwrap_or("1".to_string())
    .parse()
    .unwrap();
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let host = env::var("HOST").expect("HOST is not set in .env file");
  let port = env::var("PORT").expect("PORT is not set in .env file");
  let server_url = format!("{host}:{port}");

  let conn = Database::connect(&db_url).await.unwrap();

  match conn.ping().await {
    Ok(_) => info!("Database is ok!"),
    Err(error) => panic!("{error}"),
  }

  info!("Server running at http://{server_url}");

  let state = AppState { conn };

  let _ = HttpServer::new(move || {
    let cors = Cors::permissive();
    App::new()
      .app_data(web::Data::new(state.clone()))
      .wrap(cors)
      .wrap(middleware::Logger::default())
      .configure(config)
  })
  .bind(server_url)?
  .workers(workers)
  .run()
  .await;
  Ok(())
}
