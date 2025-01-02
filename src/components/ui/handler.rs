use actix_web::{
  get,
  http::{self, header::ContentType},
  web::{Data, Query},
  HttpResponse,
};
use helpers::jwt;

use crate::{
  app::AppState,
  components::ui::{model::*, service},
};

#[get("/profile")]
pub async fn ui_profile_page(
  state: Data<AppState>,
  query: Query<UIProfilePageQuery>,
) -> HttpResponse {
  if query.token.is_some() {
    if jwt::verify::<String>(query.token.clone().unwrap(), state.jwt_token.clone()).is_ok() {
      HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(service::admin_page().await)
    } else {
      HttpResponse::Found()
        .append_header((http::header::LOCATION, "/ui/login".to_string()))
        .finish()
    }
  } else {
    HttpResponse::Ok()
      .content_type(ContentType::html())
      .body(service::admin_page().await)
  }
}

#[get("/login")]
pub async fn ui_login_page(query: Query<UiLoginPageQeury>) -> HttpResponse {
  if query.redirect.is_some() {
    HttpResponse::Found()
      .append_header((http::header::LOCATION, query.redirect.clone().unwrap()))
      .finish()
  } else {
    HttpResponse::Ok()
      .content_type(ContentType::html())
      .body(service::admin_page().await)
  }
}

#[get("/migration")]
pub async fn ui_migration_page() -> HttpResponse {
  HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(service::admin_page().await)
}

pub async fn ui_page() -> HttpResponse {
  HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(service::admin_page().await)
}
