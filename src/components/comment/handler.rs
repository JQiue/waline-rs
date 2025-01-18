use actix_web::{
  delete, get, post, put,
  web::{Data, Json, Path, Query},
  HttpRequest, HttpResponse,
};
use helpers::jwt;

use crate::{
  app::AppState,
  components::{
    comment::{model::*, service},
    user::model::is_admin_user,
  },
  error::AppError,
  helpers::header::{extract_ip, extract_token},
  response::{Code, Response},
};

#[get("/comment")]
async fn get_comment_info(
  req: HttpRequest,
  state: Data<AppState>,
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
    status,
    keyword,
  }) = query.clone();
  if let Some(path) = path {
    let fields = query.validate_by_path();
    if fields.is_err() {
      return HttpResponse::Ok().json(Response::<()>::error(Code::Error, Some(&lang)));
    }
    let token = extract_token(&req);
    match service::get_comment_info(
      &state,
      path,
      page,
      page_size.unwrap(),
      sort_by.unwrap(),
      token,
    )
    .await
    {
      Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(&lang))),
      Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(&lang))),
    }
  } else {
    let fields = query.validate_by_admin();
    if fields.is_err() {
      tracing::error!("{:?}", fields.err().unwrap());
      return HttpResponse::Ok().json(Response::<()>::error(Code::Error, Some(&lang)));
    }
    let token = extract_token(&req).unwrap();
    let email = match jwt::verify::<String>(token, state.jwt_token.clone()).map_err(AppError::from)
    {
      Ok(token_data) => token_data.claims.data,
      Err(err) => return HttpResponse::Ok().json(Response::<()>::error(err.into(), Some(&lang))),
    };
    let is = match is_admin_user(email.clone(), &state.conn).await {
      Ok(value) => value,
      Err(err) => return HttpResponse::Ok().json(Response::<()>::error(err, Some(&lang))),
    };
    if !is {
      return HttpResponse::Ok().json(Response::<()>::error(Code::Unauthorized, Some(&lang)));
    }
    match service::get_comment_info_by_admin(
      &state,
      owner.unwrap(),
      email,
      keyword.unwrap(),
      status.unwrap(),
      page,
    )
    .await
    {
      Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(&lang))),
      Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(&lang))),
    }
  }
}

#[post("/comment")]
async fn create_comment(
  req: HttpRequest,
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
  let mut is_admin = false;
  let client_ip = extract_ip(&req);
  let pass = if let Ok(token) = extract_token(&req) {
    match jwt::verify::<String>(token, state.jwt_token.clone()) {
      Ok(verified_token) => {
        if is_admin_user(verified_token.claims.data, &state.conn)
          .await
          .unwrap()
        {
          is_admin = true;
          true
        } else {
          state.rate_limiter.check_and_update(&client_ip, 1)
        }
      }
      Err(err) => {
        tracing::error!("{}", err);
        return HttpResponse::Ok().json(Response::<()>::error(Code::Unauthorized, Some(&lang)));
      }
    }
  } else {
    if &state.login == "force" {
      return HttpResponse::Ok().json(Response::<()>::error(Code::Unauthorized, Some(&lang)));
    }
    state.rate_limiter.check_and_update(&client_ip, 1)
  };
  if !pass {
    return HttpResponse::Ok().json(Response::<()>::error(Code::FrequencyLimited, Some(&lang)));
  }
  if is_duplicate(&url, &mail, &nick, &link, &comment, &state.conn)
    .await
    .unwrap()
    && !is_admin
  {
    return HttpResponse::Ok().json(Response::<()>::error(Code::DuplicateContent, Some(&lang)));
  }
  match service::create_comment(
    &state,
    comment,
    link,
    mail,
    nick,
    ua,
    url,
    pid,
    rid,
    at,
    client_ip,
    Some(lang.clone()),
  )
  .await
  {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(&lang))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(&lang))),
  }
}

#[delete("/comment/{id}")]
pub async fn delete_comment(
  req: HttpRequest,
  state: Data<AppState>,
  path: Path<u32>,
) -> HttpResponse {
  let id = path.into_inner();
  if let Ok(token) = extract_token(&req) {
    match jwt::verify::<String>(token, state.jwt_token.clone()) {
      Ok(data) => match service::delete_comment(&state, id, data.claims.data).await {
        Ok(_) => HttpResponse::Ok().json(Response::success(Some(""), None)),
        Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
      },
      Err(_) => HttpResponse::Ok().json(Response::<()>::error(Code::Unauthorized, None)),
    }
  } else {
    HttpResponse::Ok().json(Response::<()>::error(Code::Unauthorized, None))
  }
}

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
    sticky,
  }) = body;
  let id: u32 = path.into_inner();
  match service::update_comment(
    &state, id, status, like, comment, link, mail, nick, ua, url, sticky,
  )
  .await
  {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), None)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}
