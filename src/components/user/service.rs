use sea_orm::{
  ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, Iterable, QueryFilter, QuerySelect,
  Set,
};
use serde_json::{json, Value};

use crate::{
  app::AppState,
  components::user::model::{has_user, is_first_user, UserQueryBy},
  entities::*,
  error::AppError,
  helpers::email::extract_email_prefix,
  response::Code,
};

use super::model::get_user;

pub async fn user_register(
  state: &AppState,
  display_name: String,
  email: String,
  password: String,
  url: String,
) -> Result<Value, Code> {
  if has_user(UserQueryBy::Email(email.clone()), &state.conn).await? {
    return Err(Code::UserRegistered);
  }

  let hashed = helpers::hash::bcrypt(password.as_bytes()).map_err(|_| Code::Error)?;
  let mut model = wl_users::ActiveModel {
    display_name: Set(display_name),
    email: Set(email),
    url: Set(Some(url)),
    password: Set(hashed),
    ..Default::default()
  };
  if is_first_user(&state.conn).await? {
    model.r#type = Set("administrator".to_string());
  } else {
    model.r#type = Set("guest".to_string());
  }
  match model.insert(&state.conn).await.map_err(AppError::from) {
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

/// 设置用户类型（todo）
pub async fn set_user_type(
  _state: &AppState,
  _user_id: i32,
  _type: String,
) -> Result<bool, String> {
  Err("todo".to_string())
}

/// 获取用户信息列表
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
  if let Some(user) = wl_users::Entity::find()
    .filter(wl_users::Column::Email.eq(email))
    .select_only()
    .columns(wl_users::Column::iter().filter(|col| !matches!(col, wl_users::Column::Id)))
    .column_as(wl_users::Column::Id, "objectId")
    .into_json()
    .one(&state.conn)
    .await
    .map_err(AppError::from)?
  {
    Ok(user)
  } else {
    Err(Code::UserRegistered)
  }
}

/// todo
pub async fn verification(state: &AppState, email: String, _token: String) -> Result<bool, Code> {
  let user = get_user(UserQueryBy::Email(email), &state.conn)
    .await
    .map_err(AppError::from);
  Ok(user.is_ok())
}

/// 设置 2fa（todo）
pub async fn set_2fa(_state: &AppState, _code: String, _secret: String) -> Result<bool, String> {
  Err("todo".to_string())
}

pub async fn get_2fa(state: &AppState, email: Option<String>) -> Result<Value, Code> {
  match email {
    Some(email) => {
      let res = wl_users::Entity::find()
        .filter(wl_users::Column::Email.eq(email))
        .filter(wl_users::Column::TwoFactorAuth.is_not_null())
        .filter(wl_users::Column::TwoFactorAuth.ne(""))
        .one(&state.conn)
        .await
        .map_err(AppError::from)?;
      match res {
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
