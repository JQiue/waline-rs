//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "wl_Comment")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: u32,
  pub user_id: Option<i32>,
  #[sea_orm(column_type = "Text", nullable)]
  pub comment: Option<String>,
  #[sea_orm(column_name = "insertedAt")]
  pub inserted_at: Option<DateTimeUtc>,
  pub ip: Option<String>,
  pub link: Option<String>,
  pub mail: Option<String>,
  pub nick: Option<String>,
  pub pid: Option<i32>,
  pub rid: Option<i32>,
  pub sticky: Option<i8>,
  pub status: String,
  pub like: Option<i32>,
  #[sea_orm(column_type = "Text", nullable)]
  pub ua: Option<String>,
  pub url: Option<String>,
  #[sea_orm(column_name = "createdAt")]
  pub created_at: Option<DateTimeUtc>,
  #[sea_orm(column_name = "updatedAt")]
  pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
