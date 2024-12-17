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
}

impl Code {
  pub fn message(&self, lang: String) -> String {
    match self {
      Code::Success => "".to_owned(),
      Code::Error => "".to_owned(),
      Code::UserRegistered => get_translation(&lang, "USER_REGISTERED"),
      Code::DuplicateContent => get_translation(&lang, "Duplicate Content"),
      Code::Unauthorized => get_translation(&lang, "Unauthorized"),
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
      errmsg: Code::Success.message(lang.unwrap_or("en".to_owned())),
    }
  }

  pub fn error(code: Code, lang: Option<String>) -> Self {
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
