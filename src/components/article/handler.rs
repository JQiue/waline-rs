use crate::{
  components::article::{model::*, service},
  AppState,
};

use actix_web::{
  get, post,
  web::{Data, Json, Query},
  HttpResponse,
};
use serde_json::json;

#[get("/article")]
async fn get_article(data: Data<AppState>, query: Query<GetArticleQuery>) -> HttpResponse {
  let Query(GetArticleQuery { path, r#type, lang }) = query;
  match service::get_article(&data, path, r#type, lang).await {
    Ok(data) => HttpResponse::Ok().json(json!({
      "data": data,
      "errmsg": "".to_string(),
      "errno": 0
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errmsg": "".to_string(),
      "errno": 1000
    })),
  }
}

#[post("/article")]
async fn update_article(
  data: Data<AppState>,
  query: Query<ApiArticleQuery>,
  body: Json<ApiArticleBody>,
) -> HttpResponse {
  let Json(ApiArticleBody {
    action,
    path,
    r#type,
  }) = body;
  let Query(ApiArticleQuery { lang }) = query;

  match service::update_article(&data, action, path, r#type, lang).await {
    Ok(data) => HttpResponse::Ok().json(json!({
      "data": data,
      "errmsg": "".to_string(),
      "errno": 0
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errmsg": "".to_string(),
      "errno": 1000
    })),
  }
}
