use std::fmt::Display;

use serde::Serialize;

use crate::locales::get_translation;

/// Response code enumeration
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Code {
  Success,
  Error,
  UserRegistered,
  DuplicateContent,
  Unauthorized,
  FrequencyLimited,
  TokenExpired,
  Forbidden,
}

impl Code {
  pub fn message(&self, lang: &str) -> String {
    match self {
      Code::Success => "".to_owned(),
      Code::Error => "".to_owned(),
      Code::UserRegistered => get_translation(lang, "USER_REGISTERED"),
      Code::DuplicateContent => get_translation(lang, "Duplicate Content"),
      Code::Unauthorized => get_translation(lang, "Unauthorized"),
      Code::FrequencyLimited => get_translation(lang, "Comment too fast"),
      Code::TokenExpired => get_translation(lang, "TOKEN_EXPIRED"),
      Code::Forbidden => get_translation(lang, "FORBIDDEN"),
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
  pub fn success(data: Option<T>, lang: Option<&str>) -> Self {
    Response {
      data,
      errno: 0,
      errmsg: Code::Success.message(lang.unwrap_or("en")),
    }
  }

  pub fn error(code: Code, lang: Option<&str>) -> Self {
    let errno = match code {
      Code::Success => 0,
      Code::Error => 1000,
      Code::UserRegistered => 1000,
      Code::DuplicateContent => 1000,
      Code::Unauthorized => 401,
      Code::FrequencyLimited => 1000,
      Code::TokenExpired => 1000,
      Code::Forbidden => 403,
    };
    Response {
      data: None,
      errno,
      errmsg: code.message(lang.unwrap_or("en")),
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
