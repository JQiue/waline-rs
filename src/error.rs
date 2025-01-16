use crate::response::Code;

#[derive(Debug)]
pub enum AppError {
  Error,
  Database,
  UserNotFound,
  Authorization,
  Akismet,
}

impl From<AppError> for Code {
  fn from(err: AppError) -> Self {
    let status_code = match err {
      AppError::Database => Code::Error,
      AppError::Error => Code::Error,
      AppError::UserNotFound => Code::Error,
      AppError::Authorization => Code::Error,
      AppError::Akismet => Code::Error,
    };
    tracing::error!("{:#?}", err);
    status_code
  }
}

impl From<sea_orm::DbErr> for AppError {
  fn from(err: sea_orm::DbErr) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Database
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
    AppError::Authorization
  }
}

impl From<helpers::hash::BcryptError> for AppError {
  fn from(err: helpers::hash::BcryptError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<actix_web::http::header::ToStrError> for AppError {
  fn from(err: actix_web::http::header::ToStrError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<instant_akismet::Error> for AppError {
  fn from(err: instant_akismet::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Akismet
  }
}

// impl ResponseError for AppError {
//   fn status_code(&self) -> StatusCode {
//     match self {
//       AppError::Error => todo!(),
//       AppError::DatabaseError => todo!(),
//       AppError::UserNotFound => todo!(),
//       AppError::AuthorizationError => todo!(),
//     }
//   }
//   fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
//     match self {
//       AppError::DatabaseError => HttpResponse::Ok().json(Response::error(self.status_code(), lang)),
//       AppError::Error => HttpResponse::Ok().json(Response::error(code, lang)),
//       AppError::UserNotFound => HttpResponse::Ok().json(Response::error(code, lang)),
//       AppError::AuthorizationError => HttpResponse::Ok().json(Response::error(code, lang)),
//     }
//   }
// }

// impl std::fmt::Display for AppError {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match self {
//       AppError::DatabaseError => write!(f, "Database error"),
//       AppError::Error => write!(f, "Internal server error"),
//       AppError::UserNotFound => write!(f, "User not found"),
//       AppError::AuthorizationError => write!(f, "Authorization error"),
//     }
//   }
// }
