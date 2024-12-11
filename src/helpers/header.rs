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
    .ok_or(AppError::AuthorizationError)?
    .to_str()
    .map_err(AppError::from)?;
  if !auth_header.starts_with("Bearer ") {
    return Err(AppError::Error);
  }
  Ok(auth_header[7..].to_string()) // Skip "Bearer " prefix
}
