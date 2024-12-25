use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::error::AppError;
use crate::{entities::wl_users, response::Code};

#[derive(Deserialize)]
pub struct UserRegisterQuery {
  pub lang: String,
}

#[derive(Deserialize)]
pub struct UserRegisterBody {
  pub display_name: String,
  pub email: String,
  pub password: String,
  pub url: String,
}

#[derive(Deserialize)]
pub struct UserLoginBody {
  pub code: String,
  pub email: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct SetUserProfileBody {
  pub display_name: Option<String>,
  pub label: Option<String>,
  pub url: Option<String>,
  pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct GetUserQuery {
  pub email: Option<String>,
  pub page: Option<u32>,
  pub lang: String,
}

#[derive(Deserialize)]
pub struct VerificationQuery {
  pub token: String,
  pub email: String,
}

#[derive(Deserialize)]
pub struct Set2faBody {
  pub code: String,
  pub secret: String,
}

#[derive(Deserialize)]
pub struct Get2faQuery {
  pub lang: String,
  pub email: Option<String>,
}

pub async fn is_first_user(conn: &DatabaseConnection) -> Result<bool, Code> {
  let users = wl_users::Entity::find()
    .all(conn)
    .await
    .map_err(AppError::from)?;
  Ok(users.is_empty())
}

pub async fn is_admin_user(email: String, conn: &DatabaseConnection) -> Result<bool, Code> {
  let user = wl_users::Entity::find()
    .filter(wl_users::Column::Email.eq(email))
    .filter(wl_users::Column::Type.eq("administrator"))
    .one(conn)
    .await
    .map_err(AppError::from)?;
  Ok(user.is_some())
}

#[derive(Debug, Clone)]
pub enum UserQueryBy {
  Id(u32),
  Email(String),
}

pub async fn has_user(query_by: UserQueryBy, conn: &DatabaseConnection) -> Result<bool, AppError> {
  let mut query = wl_users::Entity::find();
  match query_by {
    UserQueryBy::Id(id) => query = query.filter(wl_users::Column::Id.eq(id)),
    UserQueryBy::Email(email) => query = query.filter(wl_users::Column::Email.eq(email)),
  }
  let res = query.one(conn).await.map_err(AppError::from)?;
  Ok(res.is_some())
}

#[derive(Deserialize)]
pub struct SetUserTypeBody {
  pub r#type: String,
}

pub async fn get_user(
  query_by: UserQueryBy,
  conn: &DatabaseConnection,
) -> Result<wl_users::Model, AppError> {
  if !has_user(query_by.to_owned(), conn).await? {
    return Err(AppError::UserNotFound);
  }
  let mut query = wl_users::Entity::find();
  match query_by {
    UserQueryBy::Id(id) => query = query.filter(wl_users::Column::Id.eq(id)),
    UserQueryBy::Email(email) => query = query.filter(wl_users::Column::Email.eq(email)),
  }
  query
    .one(conn)
    .await
    .map_err(AppError::from)?
    .ok_or(AppError::UserNotFound)
}
