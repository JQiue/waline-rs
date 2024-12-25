use crate::{
  app::AppState,
  components::article::{model::*, service},
  response::Response,
};

use actix_web::{
  get, post,
  web::{Data, Json, Query},
  HttpResponse,
};

#[get("/article")]
async fn get_article(data: Data<AppState>, query: Query<GetArticleQuery>) -> HttpResponse {
  let Query(GetArticleQuery { path, r#type, lang }) = query;
  match service::get_article(&data, path, r#type).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(&lang))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(&lang))),
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
  match service::update_article(&data, action, path, r#type).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(&lang))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(&lang))),
  }
}
