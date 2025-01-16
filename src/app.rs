//! app
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  time::{Duration, Instant},
};

use crate::{
  components::{
    article, comment, migration,
    ui::{self, handler::ui_page},
    user,
  },
  config::EnvConfig,
  error::AppError,
};

use actix_cors::Cors;
use actix_web::{
  middleware,
  web::{self, ServiceConfig},
  App, HttpResponse, HttpServer,
};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

#[derive(Debug)]
pub struct RateLimiter {
  qps: u64,
  counter: Mutex<HashMap<String, (usize, Instant)>>,
}

impl RateLimiter {
  fn new(qps: u64) -> Self {
    RateLimiter {
      qps,
      counter: Mutex::new(HashMap::new()),
    }
  }
  pub fn check_and_update(&self, client_ip: &str, count: usize) -> bool {
    let mut counter = self.counter.lock().unwrap();
    counter.retain(|_, &mut (_, timestamp)| timestamp.elapsed() < Duration::from_secs(self.qps));
    match counter.get_mut(client_ip) {
      Some((cnt, timestamp)) => {
        if *cnt >= count {
          false
        } else {
          *cnt += 1;
          *timestamp = Instant::now();
          true
        }
      }
      None => {
        counter.insert(client_ip.to_string(), (1, Instant::now()));
        true
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct AppState {
  pub rate_limiter: Arc<RateLimiter>,
  pub conn: DatabaseConnection,
  pub jwt_token: String,
  pub levels: Option<String>,
  pub comment_audit: bool,
  pub login: String,
}

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().json(serde_json::json!({"status": "OK"}))
}

pub fn config_app(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/api")
      .configure(article::config)
      .configure(comment::config)
      .configure(user::config)
      .configure(migration::config)
      .route("/health", web::get().to(health_check)),
  );
  cfg.route("/ui", web::get().to(ui_page));
  cfg.service(web::scope("/ui").configure(ui::config));
  #[cfg(feature = "leancloud")]
  cfg.route("/", web::get().to(health_check));
}

pub async fn start() -> Result<(), AppError> {
  let EnvConfig {
    workers,
    host,
    port,
    database_url,
    jwt_token,
    levels,
    akismet_key,
    ipqps,
    comment_audit,
    login,
    ..
  } = EnvConfig::load_env()?;
  let conn = Database::connect(database_url).await?;
  conn.ping().await?;
  if akismet_key != "false" {
    info!("The anti-spam system has been activated")
  }
  let state = AppState {
    jwt_token,
    conn,
    levels,
    login,
    comment_audit,
    rate_limiter: Arc::new(RateLimiter::new(ipqps)),
  };
  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .wrap(Cors::permissive())
      .app_data(web::Data::new(state.clone()))
      .configure(config_app)
  })
  .bind((host, port))?
  .workers(workers)
  .run()
  .await
  .map_err(AppError::from)
}
