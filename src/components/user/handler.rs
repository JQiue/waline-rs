use actix_web::{
  delete, get,
  http::header::{HeaderValue, AUTHORIZATION},
  post, put,
  web::{Data, Json, Path, Query},
  HttpRequest, HttpResponse,
};
use serde_json::json;

use crate::{
  components::user::{model::*, service},
  AppState,
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

  match service::user_register(&state, lang, display_name, email, password, url).await {
    Ok(data) => HttpResponse::Ok().json(json!({
     "data": data,
     "errmsg": "",
     "errno": 0,
    })),
    Err(err) => HttpResponse::Ok().json(json!({
     "errmsg": err,
     "errno": 1000,
    })),
  }
}

#[post("/token")]
pub async fn user_login(state: Data<AppState>, body: Json<ApiTokenBody>) -> HttpResponse {
  let Json(ApiTokenBody {
    code,
    email,
    password,
  }) = body;
  match service::user_login(&state, code, email, password).await {
    Ok(data) => HttpResponse::Ok().json(json!({
     "data": data,
     "errmsg": "",
     "errno": 0,
    })),
    Err(msg) => HttpResponse::Ok().json(json!({
     "errmsg": msg,
     "errno": 1000,
    })),
  }
}

#[delete("/token")]
pub async fn user_logout() -> HttpResponse {
  service::user_logout().await;
  HttpResponse::Ok().json(json! ({
    "errno": 0,
    "errmsg": "",
  }))
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
      Ok(data) => HttpResponse::Ok().json(json! ({
        "errno": 0,
        "errmsg": "",
        "data": data,
      })),
      Err(err) => HttpResponse::Ok().json(json! ({
        "errmsg": err,
        "errno":1000,
      })),
    }
  } else {
    HttpResponse::Ok().json(json! ({
      "errno":1000,
    }))
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
    Ok(_) => HttpResponse::Ok().json(json! ({
      "errno": 0,
      "errmsg": "",
    })),
    Err(err) => HttpResponse::Ok().json(json! ({
      "errno": 1000,
      "errmsg": err,
    })),
  }
}

/// 设置用户类型（未实现）
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

/// 获取用户信息（未实现）
#[get("/user")]
pub async fn get_user_list(state: Data<AppState>, query: Query<GetUserQuery>) -> HttpResponse {
  let Query(GetUserQuery { email, lang }) = query;
  match service::get_user_list(&state, email, lang).await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errmsg": "未实现",
      "errno": 1000
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errmsg": "未实现",
      "errno": 1000
    })),
  }
}

/// 未实现
#[post("/verification")]
pub async fn verification(state: Data<AppState>, query: Query<VerificationQuery>) -> HttpResponse {
  let Query(VerificationQuery { email, token }) = query;

  match service::verification(&state, email, token).await {
    Ok(_) => HttpResponse::Ok().json(json! ({
      "errmsg": "用户已注册",
      "errno": 1000,
    })),
    Err(_) => HttpResponse::Ok().json(json! ({
      "errmsg": "用户已注册",
      "errno": 1000,
    })),
  }
}

/// 设置 2fa（未实现）
#[post("/token/2fa")]
pub async fn set_2fa(state: Data<AppState>, body: Json<Set2faBody>) -> HttpResponse {
  let Json(Set2faBody { code, secret }) = body;
  match service::set_2fa(&state, code, secret).await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "二部验证失败"
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "二部验证失败"
    })),
  }
}

#[get("/token/2fa")]
pub async fn get_2fa(state: Data<AppState>, query: Query<Get2faQuery>) -> HttpResponse {
  let Query(Get2faQuery { lang, email }) = query;
  match service::get_2fa(&state, email, lang).await {
    Ok(data) => HttpResponse::Ok().json(json!({
      "errno": 0,
      "errmsg": "",
      "data": data
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "二部验证失败"
    })),
  }
}
