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
  m.insert("Duplicate Content", "发送的内容之前已经发过");
  m.insert("Unauthorized", "Unauthorized");
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
  m.insert("Duplicate Content", "發送的內容之前已經發過");
  m.insert("Unauthorized", "Unauthorized");
  m
}

fn en() -> TranslationMap {
  let mut m = HashMap::new();
  m.insert(
    "import data format not support!",
    "import data format not support!",
  );
  m.insert("USER_EXIST", "USER_EXIST");
  m.insert("USER_NOT_EXIST", "USER_NOT_EXIST");
  m.insert("USER_REGISTERED", "USER_REGISTERED");
  m.insert("TOKEN_EXPIRED", "密TOKEN_EXPIRED");
  m.insert(
    "TWO_FACTOR_AUTH_ERROR_DETAIL",
    "TWO_FACTOR_AUTH_ERROR_DETAIL",
  );
  m.insert("Duplicate Content", "Duplicate Content");
  m.insert("Unauthorized", "Unauthorized");
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
