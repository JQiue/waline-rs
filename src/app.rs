//! app
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  time::{Duration, Instant},
};

use crate::{
  components::{article, comment, db, ui, user},
  config::Config,
  error::AppError,
};
use actix_cors::Cors;
use actix_web::{
  middleware,
  web::{self, ServiceConfig},
  App, HttpResponse, HttpServer,
};
use sea_orm::{Database, DatabaseConnection};

#[derive(Debug)]
pub struct RateLimiter {
  counter: Mutex<HashMap<String, (usize, Instant)>>,
}

impl RateLimiter {
  fn new() -> Self {
    RateLimiter {
      counter: Mutex::new(HashMap::new()),
    }
  }
  pub fn check_and_update(&self, client_ip: &str, qps: u64, count: usize) -> bool {
    let mut counter = self.counter.lock().unwrap();
    counter.retain(|_, &mut (_, timestamp)| timestamp.elapsed() < Duration::from_secs(qps));
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
  pub anonymous_avatar: Arc<String>,
  pub jwt_key: String,
  pub levels: Option<String>,
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
      .configure(db::config)
      .route("/health", web::get().to(health_check)),
  );
  cfg.service(web::scope("/ui").configure(ui::config));
  #[cfg(feature = "leancloud")]
  cfg.route("/", web::get().to(health_check));
}

pub async fn start() -> Result<(), AppError> {
  let app_config = Config::from_env()?;
  let db = Database::connect(app_config.database_url).await?;
  db.ping().await?;
  let state = AppState {
    jwt_key: app_config.jwt_key,
    conn: db,
    anonymous_avatar: "https://seccdn.libravatar.org/avatar/d41d8cd98f00b204e9800998ecf8427e"
      .to_string()
      .into(),
    levels: app_config.levels,
    rate_limiter: Arc::new(RateLimiter::new()),
  };
  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .wrap(Cors::permissive())
      .app_data(web::Data::new(state.clone()))
      .configure(config_app)
  })
  .bind((app_config.host, app_config.port))?
  .workers(app_config.workers)
  .run()
  .await
  .map_err(AppError::from)
}
