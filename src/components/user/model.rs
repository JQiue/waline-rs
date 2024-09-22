use crate::entities::{prelude::*, *};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

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
  pub lang: Option<String>,
  pub page: Option<u32>,
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

pub async fn is_first_user(conn: &DatabaseConnection) -> bool {
  let users = WlUsers::find().all(conn).await.unwrap();
  users.is_empty()
}

pub enum UserQueryBy {
  Id(u32),
  Email(String),
}

pub async fn has_user(query_by: UserQueryBy, conn: &DatabaseConnection) -> bool {
  let mut query = WlUsers::find();
  match query_by {
    UserQueryBy::Id(id) => query = query.filter(wl_users::Column::Id.eq(id)),
    UserQueryBy::Email(email) => query = query.filter(wl_users::Column::Email.eq(email)),
  }
  let res = query.one(conn).await.unwrap();
  res.is_some()
}

#[derive(Deserialize)]
pub struct SetUserTypeBody {
  pub r#type: String,
}
