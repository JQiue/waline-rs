//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "wl_Counter")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: u32,
  pub time: Option<i32>,
  pub reaction0: Option<i32>,
  pub reaction1: Option<i32>,
  pub reaction2: Option<i32>,
  pub reaction3: Option<i32>,
  pub reaction4: Option<i32>,
  pub reaction5: Option<i32>,
  pub reaction6: Option<i32>,
  pub reaction7: Option<i32>,
  pub reaction8: Option<i32>,
  pub url: String,
  #[sea_orm(column_name = "createdAt")]
  pub created_at: Option<DateTimeUtc>,
  #[sea_orm(column_name = "updatedAt")]
  pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
