use actix_web::{http::header::HeaderValue, HttpRequest};

use crate::error::AppError;

pub fn extract_token_from_header(header_value: &Option<&HeaderValue>) -> Option<String> {
  header_value.and_then(|value| {
    let value = value.to_str().ok()?;
    if value.starts_with("Bearer ") {
      Some(value.split(' ').nth(1)?.to_string())
    } else {
      None
    }
  })
}

pub fn extract_token(req: &HttpRequest) -> Result<String, AppError> {
  let auth_header = req
    .headers()
    .get("Authorization")
    .ok_or(AppError::Authorization)?
    .to_str()
    .map_err(AppError::from)?;
  if !auth_header.starts_with("Bearer ") {
    return Err(AppError::Error);
  }
  Ok(auth_header[7..].to_string()) // Skip "Bearer " prefix
}

pub fn extract_ip(req: &HttpRequest) -> String {
  if let Some(h) = req.headers().get("X-Forwarded-For") {
    let s = h.to_str().unwrap_or("0.0.0.0").to_string();
    s
  } else if let Some(h) = req.headers().get("X-Real-IP") {
    let s = h.to_str().ok().unwrap_or("0.0.0.0").to_string();
    s
  } else {
    req
      .peer_addr()
      .map(|s| s.ip().to_string())
      .unwrap_or_default()
  }
}

pub fn extract_host(req: &HttpRequest) -> String {
  req
    .headers()
    .get("Host")
    .and_then(|h| h.to_str().ok())
    .unwrap_or_default()
    .to_string()
}

pub fn extract_referer(req: &HttpRequest) -> String {
  req
    .headers()
    .get("referer")
    .and_then(|h| h.to_str().ok())
    .unwrap_or_default()
    .to_string()
}
