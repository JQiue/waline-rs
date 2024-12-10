use std::fmt::Display;

use serde::Serialize;

use crate::locales::get_translation;

/// 响应状态码枚举
#[derive(Debug, Clone, Copy, Serialize)]
pub enum StatusCode {
  /// 成功
  Success,
  Error,
  UserRegistered,
}

impl StatusCode {
  pub fn message(&self, lang: String) -> String {
    match self {
      StatusCode::Success => "".to_owned(),
      StatusCode::Error => "失败".to_owned(),
      StatusCode::UserRegistered => get_translation(&lang, "USER_REGISTERED"),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct Response<T> {
  pub errno: i32,
  pub errmsg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

impl<T> Response<T> {
  pub fn success(data: Option<T>, lang: Option<String>) -> Self {
    Response {
      data,
      errno: 0,
      errmsg: StatusCode::Success.message(lang.unwrap_or("en".to_owned())),
    }
  }

  pub fn error(code: StatusCode, lang: Option<String>) -> Self {
    Response {
      data: None,
      errno: 1000,
      errmsg: code.message(lang.unwrap_or("en".to_owned())),
    }
  }
}

impl<T> Display for Response<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      r#"{{ "errno": {}, "errmsg": "{}" }}"#,
      self.errno, self.errmsg
    )
  }
}
