use helpers::time::utc_now;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::entities::*;
use crate::prelude::*;

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
) -> Result<wl_counter::Model, Code> {
  let counter = wl_counter::ActiveModel {
    time: Set(Some(0)),
    url: Set(url),
    created_at: Set(Some(utc_now())),
    ..Default::default()
  }
  .insert(conn)
  .await
  .map_err(AppError::from)?;
  Ok(counter)
}

pub async fn has_counter<'a>(
  query_by: &CounterQueryBy<'a>,
  conn: &DatabaseConnection,
) -> Result<bool, Code> {
  let mut query = wl_counter::Entity::find();
  match query_by {
    CounterQueryBy::Url(url) => query = query.filter(wl_counter::Column::Url.eq(*url)),
  }
  let res = query
    .one(conn)
    .await
    .map_err(AppError::from)?
    .ok_or(Code::Error);
  Ok(res.is_ok())
}

pub enum CounterQueryBy<'a> {
  Url(&'a str),
}

pub async fn get_counter<'a>(
  query_by: &CounterQueryBy<'a>,
  conn: &DatabaseConnection,
) -> Result<wl_counter::Model, Code> {
  if !has_counter(query_by, conn).await? {
    Err(Code::Error)
  } else {
    let mut query = wl_counter::Entity::find();
    match query_by {
      CounterQueryBy::Url(url) => query = query.filter(wl_counter::Column::Url.eq(*url)),
    }
    query
      .one(conn)
      .await
      .map_err(AppError::from)?
      .ok_or(Code::Error)
  }
}
