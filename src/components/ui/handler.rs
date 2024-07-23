use actix_web::{get, http::header::ContentType, web::Query, HttpResponse};

use crate::components::ui::{model::*, service};

#[get("/profile")]
pub async fn ui_profile_page(_query: Query<UIProfilePageQuery>) -> HttpResponse {
  HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(service::ui_profile_page().await)
}

#[get("/login")]
pub async fn ui_login_page(query: Query<UiLoginPageQeury>) -> HttpResponse {
  HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(service::ui_login_page(query.redirect.clone()).await)
}

#[get("/migration")]
pub async fn ui_migration_page() -> HttpResponse {
  HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(service::ui_migration_page().await)
}
