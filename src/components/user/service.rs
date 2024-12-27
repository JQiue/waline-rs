use helpers::{
  time::utc_now,
  uuid::{self, Alphabet},
};
use regex::Regex;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, Iterable, QueryFilter, QuerySelect,
  Set,
};
use serde_json::{json, Value};

use crate::{
  app::AppState,
  components::user::model::{has_user, is_first_user, UserQueryBy},
  config::Config,
  entities::*,
  error::AppError,
  helpers::email::{
    extract_email_prefix, send_email_notification, CommentNotification, NotifyType,
  },
  response::Code,
};

use super::model::get_user;

pub async fn user_register(
  state: &AppState,
  display_name: String,
  email: String,
  password: String,
  url: String,
  host: String,
  lang: String,
) -> Result<Value, Code> {
  if has_user(UserQueryBy::Email(email.clone()), &state.conn).await? {
    return Err(Code::UserRegistered);
  }
  let hashed = helpers::hash::bcrypt(password.as_bytes()).map_err(|_| Code::Error)?;
  let mut user = wl_users::ActiveModel {
    display_name: Set(display_name),
    email: Set(email.clone()),
    url: Set(Some(url)),
    password: Set(hashed),
    ..Default::default()
  };
  if is_first_user(&state.conn).await? {
    user.r#type = Set("administrator".to_string());
  } else {
    let app_config = Config::from_env().unwrap();
    let token = uuid::uuid(&Alphabet::NUMBERS, 4);
    user.r#type = Set(format!(
      "verify:{}:{}",
      token,
      utc_now().timestamp_millis() + 1 * 60 * 60 * 1000
    ));
    let url = format!(
      "http://{}/api/verification?token={}&email={}",
      host, token, email
    );
    send_email_notification(CommentNotification {
      sender_name: app_config.site_name,
      sender_email: email,
      comment_id: 0,
      comment: "".to_string(),
      url,
      notify_type: NotifyType::Notify,
      lang: Some(lang),
    });
  }
  match user.insert(&state.conn).await.map_err(AppError::from) {
    Ok(_) => Ok(json! ({
      "data": {
        "verify": true
      }
    })),
    Err(err) => Err(err.into()),
  }
}

pub async fn user_login(
  state: &AppState,
  _code: String,
  email: String,
  password: String,
) -> Result<Value, Code> {
  let user = get_user(UserQueryBy::Email(email.clone()), &state.conn).await?;
  let result =
    helpers::hash::verify_bcrypt(password.as_bytes(), user.password).map_err(AppError::from)?;
  if !result {
    return Err(Code::Error);
  }
  let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
    format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
  } else {
    state.anonymous_avatar.to_string()
  };
  let token =
    helpers::jwt::sign(user.email.clone(), state.jwt_key.clone(), 86400).map_err(AppError::from)?;
  let mail_md5 = helpers::hash::md5(user.email.as_bytes());
  let data = json!({
    "display_name": user.display_name,
    "email": user.email,
    "password": null,
    "type": user.r#type,
    "label": user.label,
    "url": user.url,
    "avatar": avatar,
    "github": user.github,
    "twitter": user.twitter,
    "facebook": user.facebook,
    "google": user.google,
    "weibo": user.weibo,
    "qq": user.qq,
    "2fa": user.two_factor_auth,
    "createdAt": user.created_at,
    "updatedAt": user.updated_at,
    "objectId": user.id,
    "mailMd5": mail_md5,
    "token": token
  });
  Ok(data)
}

pub async fn get_login_user_info(state: &AppState, token: String) -> Result<Value, Code> {
  let email = helpers::jwt::verify::<String>(token, state.jwt_key.clone())
    .map_err(AppError::from)?
    .claims
    .data;
  let user = get_user(UserQueryBy::Email(email), &state.conn).await?;
  let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
    format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
  } else {
    state.anonymous_avatar.to_string()
  };
  let mail_md5 = helpers::hash::md5(user.email.as_bytes());
  Ok(json! ({
      "display_name": user.display_name,
      "email": user.email,
      "type": user.r#type,
      "label": user.label,
      "url": user.url,
      "avatar": avatar,
      "github": user.github,
      "twitter": user.twitter,
      "facebook": user.facebook,
      "google": user.google,
      "weibo": user.weibo,
      "qq": user.qq,
      "2fa": user.two_factor_auth,
      "objectId": user.id,
      "mailMd5": mail_md5,
  }))
}

pub async fn set_user_profile(
  state: &AppState,
  token: String,
  display_name: Option<String>,
  label: Option<String>,
  url: Option<String>,
  _password: Option<String>,
) -> Result<bool, Code> {
  let email = helpers::jwt::verify::<String>(token, state.jwt_key.to_string())
    .map_err(AppError::from)?
    .claims
    .data;
  let mut active_user = get_user(UserQueryBy::Email(email), &state.conn)
    .await?
    .into_active_model();
  active_user.display_name = Set(display_name.unwrap_or("".to_string()));
  active_user.label = Set(label);
  active_user.url = Set(url);
  let res = active_user.update(&state.conn).await;
  Ok(res.is_ok())
}

/// set user type（todo）
pub async fn set_user_type(
  _state: &AppState,
  _user_id: i32,
  _type: String,
) -> Result<bool, String> {
  Err("todo".to_string())
}

pub async fn get_user_info_list(state: &AppState, _page: Option<u32>) -> Result<Vec<Value>, Code> {
  let users = wl_users::Entity::find()
    .select_only()
    .columns(wl_users::Column::iter().filter(|col| !matches!(col, wl_users::Column::Id)))
    .column_as(wl_users::Column::Id, "objectId")
    .into_json()
    .all(&state.conn)
    .await
    .map_err(AppError::from)?;
  Ok(users)
}

pub async fn get_user_info(state: &AppState, email: Option<String>) -> Result<Value, Code> {
  match wl_users::Entity::find()
    .filter(wl_users::Column::Email.eq(email))
    .select_only()
    .columns(wl_users::Column::iter().filter(|col| !matches!(col, wl_users::Column::Id)))
    .column_as(wl_users::Column::Id, "objectId")
    .into_json()
    .one(&state.conn)
    .await
    .map_err(AppError::from)?
  {
    Some(data) => Ok(data),
    None => Err(Code::Error),
  }
}

pub async fn verification(state: &AppState, email: String, token: String) -> Result<bool, Code> {
  let user = get_user(UserQueryBy::Email(email), &state.conn)
    .await
    .map_err(AppError::from)?;
  tracing::debug!("type: {}", user.r#type);
  let reg = Regex::new(r"^verify:(\d{4}):(\d+)$").unwrap();
  tracing::debug!("reg {}", reg);
  let captures = reg.captures(&user.r#type).unwrap();
  tracing::debug!("captures {:#?}", captures);
  if token == captures.get(1).unwrap().as_str()
    && utc_now().timestamp_millis() < captures.get(2).unwrap().as_str().parse::<i64>().unwrap()
  {
    let mut active_user = user.into_active_model();
    active_user.r#type = Set("guest".to_string());
    active_user
      .update(&state.conn)
      .await
      .map_err(AppError::from)?;
    return Ok(true);
  }
  Err(Code::TokenExpired)
}

/// set 2fa（todo）
pub async fn set_2fa(_state: &AppState, _code: String, _secret: String) -> Result<bool, String> {
  Err("todo".to_string())
}

pub async fn get_2fa(state: &AppState, email: Option<String>) -> Result<Value, Code> {
  match email {
    Some(email) => {
      let user = wl_users::Entity::find()
        .filter(wl_users::Column::Email.eq(email))
        .filter(wl_users::Column::TwoFactorAuth.is_not_null())
        .filter(wl_users::Column::TwoFactorAuth.ne(""))
        .one(&state.conn)
        .await
        .map_err(AppError::from)?;
      match user {
        Some(_) => Ok(json!({
            "enable": true
        })),
        None => Ok(json!({
            "enable": false
        })),
      }
    }
    None => Err(Code::Error),
  }
}
