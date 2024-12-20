//! app
use std::sync::Arc;

use crate::{
  components::{
    article::ArticleComponent, comment::CommentComponent, db::DBComponent, ui::UIComponent,
    user::UserComponent,
  },
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

#[derive(Debug, Clone)]
pub struct AppState {
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
      .configure(ArticleComponent::config)
      .configure(CommentComponent::config)
      .configure(UserComponent::config)
      .configure(DBComponent::config)
      .route("/health", web::get().to(health_check)),
  );
  cfg.service(web::scope("/ui").configure(UIComponent::config));
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
