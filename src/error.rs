use crate::response::StatusCode;

#[derive(Debug)]
pub enum AppError {
  Error,
  DatabaseError,
  UserNotFound,
}

impl From<AppError> for StatusCode {
  fn from(err: AppError) -> Self {
    let status_code = match err {
      AppError::DatabaseError => StatusCode::Error,
      AppError::Error => StatusCode::Error,
      AppError::UserNotFound => StatusCode::Error,
    };
    tracing::error!("{:#?}", err);
    status_code
  }
}

impl From<sea_orm::DbErr> for AppError {
  fn from(err: sea_orm::DbErr) -> Self {
    tracing::error!("{:#?}", err);
    AppError::DatabaseError
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<envy::Error> for AppError {
  fn from(err: envy::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<helpers::jwt::Error> for AppError {
  fn from(err: helpers::jwt::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<helpers::hash::BcryptError> for AppError {
  fn from(err: helpers::hash::BcryptError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}
