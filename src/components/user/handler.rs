use actix_web::{
  delete, get,
  http::header::{HeaderValue, AUTHORIZATION},
  post, put,
  web::{Data, Json, Path, Query},
  HttpRequest, HttpResponse,
};
use serde_json::json;

use crate::{
  app::AppState,
  components::user::{model::*, service},
  response::Response,
};

#[post("/user")]
pub async fn user_register(
  state: Data<AppState>,
  query: Query<UserRegisterQuery>,
  body: Json<UserRegisterBody>,
) -> HttpResponse {
  let Query(UserRegisterQuery { lang }) = query;
  let Json(UserRegisterBody {
    display_name,
    email,
    password,
    url,
  }) = body;
  match service::user_register(&state, display_name, email, password, url).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(lang))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(lang))),
  }
}

#[post("/token")]
pub async fn user_login(state: Data<AppState>, body: Json<UserLoginBody>) -> HttpResponse {
  let Json(UserLoginBody {
    code,
    email,
    password,
  }) = body;
  match service::user_login(&state, code, email, password).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), None)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}

#[delete("/token")]
pub async fn user_logout() -> HttpResponse {
  HttpResponse::Ok().json(Response::<()>::success(None, None))
}

fn extract_token_from_header(header_value: &Option<&HeaderValue>) -> Option<String> {
  header_value.and_then(|value| {
    let value = value.to_str().ok()?;
    if value.starts_with("Bearer ") {
      Some(value.split(' ').nth(1)?.to_string())
    } else {
      None
    }
  })
}

/// 获取登录用户信息
#[get("/token")]
async fn get_login_user_info(req: HttpRequest, state: Data<AppState>) -> HttpResponse {
  if let Some(token) = extract_token_from_header(&req.headers().get(AUTHORIZATION)) {
    match service::get_login_user_info(&state, token).await {
      Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), None)),
      Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
    }
  } else {
    HttpResponse::Ok().json(Response::<()>::error(
      crate::response::StatusCode::Error,
      None,
    ))
  }
}

/// set user profile
#[put("/user")]
pub async fn set_user_profile(
  state: Data<AppState>,
  body: Json<SetUserProfileBody>,
) -> HttpResponse {
  let Json(SetUserProfileBody {
    display_name,
    label,
    url,
    password,
  }) = body;
  match service::set_user_profile(&state, display_name, label, url, password).await {
    Ok(_) => HttpResponse::Ok().json(Response::<()>::success(None, None)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}

/// 设置用户类型（todo）
#[put("/token/{user_id}")]
pub async fn set_user_type(
  state: Data<AppState>,
  path: Path<i32>,
  body: Json<SetUserTypeBody>,
) -> HttpResponse {
  let user_id = path.into_inner();
  let Json(SetUserTypeBody { r#type }) = body;
  match service::set_user_type(&state, user_id, r#type).await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "",
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "",
    })),
  }
}

/// 获取用户信息
#[get("/user")]
pub async fn get_user_info(state: Data<AppState>, query: Query<GetUserQuery>) -> HttpResponse {
  let Query(GetUserQuery { email, lang, page }) = query;
  if page.is_some() {
    match service::get_user_info_list(&state, page).await {
      Ok(data) => HttpResponse::Ok().json(json!({
        "data": {
          "data": data,
          "page": 1,
          "pageSize": 10,
          "totalPages": 1
        },
        "errmsg": "",
        "errno": 0
      })),
      Err(err) => HttpResponse::Ok().json(json!({
        "errmsg": err,
        "errno": 1000
      })),
    }
  } else {
    match service::get_user_info(&state, email).await {
      Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), lang)),
      Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, lang)),
    }
  }
}

/// todo
#[post("/verification")]
pub async fn verification(state: Data<AppState>, query: Query<VerificationQuery>) -> HttpResponse {
  let Query(VerificationQuery { email, token }) = query;
  match service::verification(&state, email, token).await {
    Ok(_) => HttpResponse::Ok().json(Response::<()>::error(
      crate::response::StatusCode::UserRegistered,
      None,
    )),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}

/// 设置 2fa（todo）
#[post("/token/2fa")]
pub async fn set_2fa(state: Data<AppState>, body: Json<Set2faBody>) -> HttpResponse {
  let Json(Set2faBody { code, secret }) = body;
  match service::set_2fa(&state, code, secret).await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "二步验证失败"
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "二步验证失败"
    })),
  }
}

#[get("/token/2fa")]
pub async fn get_2fa(state: Data<AppState>, query: Query<Get2faQuery>) -> HttpResponse {
  let Query(Get2faQuery { lang, email }) = query;
  match service::get_2fa(&state, email).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(lang))),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "二部验证失败"
    })),
  }
}
