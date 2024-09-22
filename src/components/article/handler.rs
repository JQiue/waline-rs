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
      "errmsg": "",
      "errno": 0
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errmsg": "",
      "errno": 1000
    })),
  }
}

#[post("/article")]
async fn update_article(
  data: Data<AppState>,
  query: Query<UpdateArticleQuery>,
  body: Json<UpdateArticleBody>,
) -> HttpResponse {
  let Json(UpdateArticleBody {
    action,
    path,
    r#type,
  }) = body;
  let Query(UpdateArticleQuery { lang }) = query;

  match service::update_article(&data, action, path, r#type, lang).await {
    Ok(data) => HttpResponse::Ok().json(json!({
      "data": data,
      "errmsg": "",
      "errno": 0
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errmsg": "",
      "errno": 1000
    })),
  }
}
