use sea_orm::{ColumnTrait, EntityTrait, Iterable, QueryFilter, QuerySelect, Set};
use serde_json::{json, Value};

use crate::{
  components::user::model::{has_user, is_first_user, UserQueryBy},
  entities::{prelude::*, *},
  helpers::{email::extract_email_prefix, token},
  locales::{self, get_translation},
  AppState,
};

pub async fn user_register(
  state: &AppState,
  lang: String,
  display_name: String,
  email: String,
  password: String,
  url: String,
) -> Result<Value, String> {
  if has_user(UserQueryBy::Email(email.clone()), &state.conn).await {
    return Err(get_translation(&lang, "USER_REGISTERED"));
  }
  let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
  let mut model = wl_users::ActiveModel {
    display_name: Set(display_name),
    email: Set(email),
    url: Set(Some(url)),
    password: Set(hashed),
    ..Default::default()
  };
  if is_first_user(&state.conn).await {
    model.r#type = Set("administrator".to_string());
  } else {
    model.r#type = Set("guest".to_string());
  }
  let _ = WlUsers::insert(model).exec(&state.conn).await.unwrap();
  Ok(json! ({
    "data": {
      "verify": true
    }
  }))
}

pub async fn user_login(
  state: &AppState,
  _code: String,
  email: String,
  password: String,
) -> Result<Value, String> {
  let user = WlUsers::find()
    .filter(wl_users::Column::Email.eq(email))
    .one(&state.conn)
    .await
    .unwrap();
  match user {
    Some(user) => {
      // a time-consuming operation
      let result = bcrypt::verify(password, user.password.as_str());
      if result.is_err() {
        return Err("验证失败".to_string());
      }

      let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
        format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
      } else {
        state.anonymous_avatar.to_string()
      };

      let payload = token::Claims::new(user.email.clone(), 1);
      let token = token::sign(payload, "waline".to_string());
      let mail_md5 = format!("{:x}", md5::compute(user.email.clone()));
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
    None => Err("验证失败".to_string()),
  }
}

pub async fn user_logout() -> bool {
  true
}

pub async fn get_login_user_info(state: &AppState, token: String) -> Result<Value, String> {
  match token::verify(token, "waline".to_string()) {
    Ok(email) => {
      let user = WlUsers::find()
        .filter(wl_users::Column::Email.eq(email))
        .one(&state.conn)
        .await
        .unwrap();
      match user {
        Some(user) => {
          let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
            format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
          } else {
            state.anonymous_avatar.to_string()
          };
          let mail_md5 = format!("{:x}", md5::compute(user.email.clone()));
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
        None => Err("no this user".to_string()),
      }
    }
    Err(err) => Err(err),
  }
}

pub async fn set_user_profile(
  state: &AppState,
  display_name: Option<String>,
  label: Option<String>,
  url: Option<String>,
  _password: Option<String>,
) -> Result<bool, String> {
  // token::verify(value.to_string(), "waline".to_string());
  let model = wl_users::ActiveModel {
    display_name: Set(display_name.unwrap_or("".to_string())),
    label: Set(label),
    url: Set(url),
    ..Default::default()
  };
  match WlUsers::update(model).exec(&state.conn).await {
    Ok(_) => Ok(true),
    Err(err) => Err(err.to_string()),
  }
}

/// 设置用户类型（todo）
pub async fn set_user_type(state: &AppState, user_id: i32, r#type: String) -> Result<bool, String> {
  Err("todo".to_string())
}

/// 获取用户信息列表
pub async fn get_user_list(state: &AppState, _page: Option<u32>) -> Result<Vec<Value>, String> {
  let users = WlUsers::find()
    .select_only()
    .columns(wl_users::Column::iter().filter(|col| !matches!(col, wl_users::Column::Id)))
    .column_as(wl_users::Column::Id, "objectId")
    .into_json()
    .all(&state.conn)
    .await
    .unwrap();
  Ok(users)
}

pub async fn get_user(
  state: &AppState,
  lang: Option<String>,
  email: Option<String>,
) -> Result<Value, String> {
  if let Some(user) = WlUsers::find()
    .filter(wl_users::Column::Email.eq(email))
    .select_only()
    .columns(wl_users::Column::iter().filter(|col| !matches!(col, wl_users::Column::Id)))
    .column_as(wl_users::Column::Id, "objectId")
    .into_json()
    .one(&state.conn)
    .await
    .unwrap()
  {
    Ok(user)
  } else {
    Err(locales::get_translation(
      &lang.unwrap_or("en".to_owned()),
      "USER_NOT_EXIST",
    ))
  }
}

/// todo
pub async fn verification(state: &AppState, email: String, token: String) -> Result<bool, String> {
  let user = WlUsers::find()
    .filter(wl_users::Column::Email.eq(email))
    .one(&state.conn)
    .await
    .unwrap();

  if let Some(_) = user {
    // 用户已注册
    Err("todo".to_string())
  } else {
    // 用户未注册
    Err("todo".to_string())
  }
}

/// 设置 2fa（todo）
pub async fn set_2fa(_state: &AppState, _code: String, _secret: String) -> Result<bool, String> {
  Err("todo".to_string())
}

pub async fn get_2fa(
  state: &AppState,
  email: Option<String>,
  lang: String,
) -> Result<Value, String> {
  match email {
    Some(email) => {
      let res = WlUsers::find()
        .filter(wl_users::Column::Email.eq(email))
        .filter(wl_users::Column::TwoFactorAuth.is_not_null())
        .filter(wl_users::Column::TwoFactorAuth.ne(""))
        .one(&state.conn)
        .await
        .unwrap();
      match res {
        Some(res) => {
          println!(">>> {:?}", res.two_factor_auth);
          Ok(json!({
              "enable": true
          }))
        }
        None => Ok(json!({
            "enable": false
        })),
      }
    }
    None => Err("".to_string()),
  }
}
