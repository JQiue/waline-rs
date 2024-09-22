//! locales

use std::collections::HashMap;

type TranslationMap = HashMap<&'static str, &'static str>;

fn zh_cn() -> TranslationMap {
  let mut m = HashMap::new();
  m.insert("import data format not support!", "文件格式不支持");
  m.insert("USER_EXIST", "用户已存在");
  m.insert("USER_NOT_EXIST", "用户不存在");
  m.insert("USER_REGISTERED", "用户已注册");
  m.insert("TOKEN_EXPIRED", "密钥已过期");
  m.insert("TWO_FACTOR_AUTH_ERROR_DETAIL", "二步验证失败");
  m
}

fn zh_tw() -> TranslationMap {
  let mut m = HashMap::new();
  m.insert("import data format not support!", "文件格式不支持");
  m.insert("USER_EXIST", "用戶已存在");
  m.insert("USER_NOT_EXIST", "用戶不存在");
  m.insert("USER_REGISTERED", "用戶已註冊");
  m.insert("TOKEN_EXPIRED", "密鑰已過期");
  m.insert("TWO_FACTOR_AUTH_ERROR_DETAIL", "二步驗證失敗");
  m
}

fn en() -> TranslationMap {
  let mut m = HashMap::new();
  m.insert("import data format not support!", "文件格式不支持");
  m.insert("USER_EXIST", "USER_EXIST");
  m.insert("USER_NOT_EXIST", "USER_NOT_EXIST");
  m.insert("USER_REGISTERED", "USER_REGISTERED");
  m.insert("TOKEN_EXPIRED", "密鑰已TOKEN_EXPIRED過期");
  m.insert(
    "TWO_FACTOR_AUTH_ERROR_DETAIL",
    "TWO_FACTOR_AUTH_ERROR_DETAIL",
  );
  m
}

/// Gets the corresponding text translation according to lang (Default in English)
pub fn get_translation(lang: &str, key: &str) -> String {
  let translations = match lang {
    "zh" | "zh-cn" | "zh-CN" => zh_cn(),
    "zh-tw" | "zh-TW" => zh_tw(),
    "en" | "en-us" | "en-US" => en(),
    _ => en(),
  };

  translations.get(key).copied().unwrap_or(key).to_string()
}

pub fn get_language(accept_language: &str) -> &'static str {
  accept_language
    .split(',')
    .next()
    .and_then(|lang| lang.split('-').next())
    .map(|lang| lang.to_lowercase())
    .map(|lang| match lang.as_str() {
      "zh" => "zh-cn",
      "en" => "en",
      "jp" => "jp",
      _ => "en",
    })
    .unwrap_or("en")
}
