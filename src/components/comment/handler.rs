use actix_web::{
  delete, get, post, put,
  web::{Data, Json, Path, Query},
  HttpRequest, HttpResponse,
};

use crate::{
  app::AppState,
  components::comment::{model::*, service},
  response::Response,
};

/// get comment
#[get("/comment")]
async fn get_comment_info(
  state: Data<AppState>,
  _req: HttpRequest,
  query: Query<GetCommentQuery>,
) -> HttpResponse {
  let Query(GetCommentQuery {
    lang,
    path,
    page_size,
    page,
    sort_by,
    r#type: _,
    owner,
    status: _,
    keyword: _,
  }) = query;
  match service::get_comment_info(&state, path, owner, page, page_size, sort_by).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(lang))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(lang))),
  }
}

/// create comment
/// No user is created if the user is anonymous
#[post("/comment")]
async fn create_comment(
  _req: HttpRequest,
  state: Data<AppState>,
  query: Query<CreateCommentQuery>,
  body: Json<CreateCommentBody>,
) -> HttpResponse {
  let Query(CreateCommentQuery { lang }) = query;
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
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(lang))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(lang))),
  }
}

/// delete comment
#[delete("/comment/{id}")]
pub async fn delete_comment(state: Data<AppState>, path: Path<u32>) -> HttpResponse {
  let id = path.into_inner();
  match service::delete_comment(&state, id).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), None)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
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
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), None)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}
