use helpers::time::utc_now;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::error::AppError;
use crate::{entities::*, response::StatusCode};

#[derive(Debug, Deserialize)]
pub struct GetArticleQuery {
  pub path: String,
  pub r#type: String,
  pub lang: String,
}

#[derive(Deserialize)]
pub struct UpdateArticleBody {
  pub action: Option<String>,
  pub path: String,
  pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleQuery {
  pub lang: String,
}

pub async fn create_counter(
  url: String,
  conn: &DatabaseConnection,
) -> Result<wl_counter::Model, StatusCode> {
  let counter = wl_counter::ActiveModel {
    time: Set(Some(1)),
    url: Set(url),
    created_at: Set(Some(utc_now())),
    ..Default::default()
  }
  .insert(conn)
  .await
  .map_err(AppError::from)?;
  Ok(counter)
}

pub async fn has_counter(
  query_by: CounterQueryBy,
  conn: &DatabaseConnection,
) -> Result<bool, StatusCode> {
  let mut query = wl_counter::Entity::find();
  match query_by {
    CounterQueryBy::Url(url) => query = query.filter(wl_counter::Column::Url.eq(url)),
  }
  let res = query
    .one(conn)
    .await
    .map_err(AppError::from)?
    .ok_or(StatusCode::Error);
  Ok(res.is_ok())
}

#[derive(Clone)]
pub enum CounterQueryBy {
  Url(String),
}

pub async fn get_counter(
  query_by: CounterQueryBy,
  conn: &DatabaseConnection,
) -> Result<wl_counter::Model, StatusCode> {
  if !has_counter(query_by.clone(), conn).await? {
    Err(StatusCode::Error)
  } else {
    let mut query = wl_counter::Entity::find();
    match query_by {
      CounterQueryBy::Url(url) => query = query.filter(wl_counter::Column::Url.eq(url)),
    }
    query
      .one(conn)
      .await
      .map_err(AppError::from)?
      .ok_or(StatusCode::Error)
  }
}
