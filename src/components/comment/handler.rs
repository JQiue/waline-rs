use actix_web::{
  delete, get, post, put,
  web::{Data, Json, Path, Query},
  HttpRequest, HttpResponse,
};
use serde_json::json;

use crate::{
  components::comment::{model::*, service},
  AppState,
};

/// get comment
#[get("/comment")]
async fn get_comment(
  state: Data<AppState>,
  _req: HttpRequest,
  query: Query<GetCommentQuery>,
) -> HttpResponse {
  let Query(GetCommentQuery {
    lang: _,
    path,
    page_size: _,
    page: _,
    sort_by: _,
    r#type: _,
    owner,
    status: _,
    keyword: _,
  }) = query;

  match service::get_comment(&state, path, owner).await {
    Ok(data) => HttpResponse::Ok().json(json!({
      "errno": 0,
      "errmsg": "".to_string(),
      "data": data
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "".to_string(),
    })),
  }
}

/// create comment
/// No user is created if the user is anonymous
#[post("/comment")]
async fn create_comment(
  state: Data<AppState>,
  _query: Query<CreateCommentQuery>,
  body: Json<CreateCommentBody>,
) -> HttpResponse {
  let Json(CreateCommentBody {
    comment,
    link,
    mail,
    nick,
    ua,
    url,
    pid,
    rid,
    at,
  }) = body;
  match service::create_comment(&state, comment, link, mail, nick, ua, url, pid, rid, at).await {
    Ok(data) => HttpResponse::Ok().json(json!({
      "errno": 0,
      "errmsg": "".to_string(),
      "data": data
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "".to_string(),
    })),
  }
}

/// delete comment
#[delete("/comment/{id}")]
pub async fn delete_comment(state: Data<AppState>, path: Path<u32>) -> HttpResponse {
  let id = path.into_inner();
  match service::delete_comment(&state, id).await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errno": 0,
      "errmsg": "".to_string(),
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "".to_string(),
    })),
  }
}

/// update comment
#[put("/comment/{id}")]
async fn update_comment(
  state: Data<AppState>,
  path: Path<u32>,
  body: Json<UpdateCommentBody>,
) -> HttpResponse {
  let actix_web::web::Json(UpdateCommentBody {
    status,
    like,
    comment,
    link,
    mail,
    nick,
    ua,
    url,
  }) = body;
  let id: u32 = path.into_inner();
  match service::update_comment(&state, id, status, like, comment, link, mail, nick, ua, url).await
  {
    Ok(data) => HttpResponse::Ok().json(json!({
      "data": data,
      "errno": 0,
      "errmsg": "".to_string(),
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "".to_string(),
    })),
  }
}
