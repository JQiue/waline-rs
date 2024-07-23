use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::entities::{prelude::*, *};

#[derive(Debug, Deserialize)]
pub struct GetArticleQuery {
  pub path: String,
  pub r#type: String,
  pub lang: String,
}

#[derive(Deserialize)]
pub struct ApiArticleBody {
  pub action: Option<String>,
  pub path: String,
  pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiArticleQuery {
  pub lang: String,
}

pub async fn has_counter(url: String, conn: &DatabaseConnection) -> bool {
  let res = WlCounter::find()
    .filter(wl_counter::Column::Url.eq(url))
    .one(conn)
    .await
    .unwrap();
  res.is_some()
}
