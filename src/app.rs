use crate::components::{
  article::ArticleComponent, comment::CommentComponent, db::DBComponent, ui::UIComponent,
  user::UserComponent,
};
use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse, Responder,
};

async fn health_check() -> impl Responder {
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
