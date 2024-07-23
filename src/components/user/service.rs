use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::{json, Value};

use crate::{
  components::user::model::{extract_email_prefix, has_user, is_first_user, UserQueryBy},
  entities::{prelude::*, *},
  helpers::token,
  locales::get_translation,
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
    return Err("用户已注册".to_string());
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
          return Ok(json! ({
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
          }));
        }
        None => return Err("no this user".to_string()),
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

/// 设置用户类型（未实现）
pub async fn set_user_type(state: &AppState, user_id: i32, r#type: String) -> Result<bool, String> {
  Err("未实现".to_string())
}

/// 获取用户信息（未实现）
pub async fn get_user_list(
  state: &AppState,
  email: Option<String>,
  lang: Option<String>,
) -> Result<bool, String> {
  Err("未实现".to_string())
}

/// 未实现
pub async fn verification(state: &AppState, email: String, token: String) -> Result<bool, String> {
  let user = WlUsers::find()
    .filter(wl_users::Column::Email.eq(email))
    .one(&state.conn)
    .await
    .unwrap();

  if let Some(_) = user {
    // 用户已注册
    Err("未实现".to_string())
  } else {
    // 用户未注册
    Err("未实现".to_string())
  }
}

/// 设置 2fa（未实现）
pub async fn set_2fa(state: &AppState, _code: String, secret: String) -> Result<bool, String> {
  Err("未实现".to_string())
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
