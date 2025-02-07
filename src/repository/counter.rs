use crate::entities::wl_counter;
use helpers::time::utc_now;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
  QueryFilter, Set,
};

#[derive(Debug, Clone)]
pub struct CounterRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl<'a> CounterRepository<'a> {
  pub async fn has_counter(&self, url: &str) -> Result<bool, DbErr> {
    wl_counter::Entity::find()
      .filter(wl_counter::Column::Url.eq(url))
      .count(self.db)
      .await
      .map(|x| x > 0)
  }

  pub async fn get_counter(&self, url: &str) -> Result<Option<wl_counter::Model>, DbErr> {
    wl_counter::Entity::find()
      .filter(wl_counter::Column::Url.eq(url))
      .one(self.db)
      .await
  }
  pub async fn create_counter(&self, url: String) -> Result<wl_counter::Model, DbErr> {
    wl_counter::ActiveModel {
      time: Set(Some(0)),
      url: Set(url),
      created_at: Set(Some(utc_now())),
      ..Default::default()
    }
    .insert(self.db)
    .await
  }
  pub async fn update_counter(&self, url: &str, time: i32) {
    //   let counter = self.get_counter(url).await?;
    //   let mut active_counter = counter.unwrap_or_else(|| wl_counter::ActiveModel {
    //     time: Set(Some(0)),
    //     url: Set(url.to_string()),
    //     created_at: Set(Some(utc_now())),
    //     ..Default::default()
    //   });
    //   active_counter.time = Set(Some(
    //     active_counter.time.take().unwrap_or(Some(0)).unwrap_or(0) + time,
    //   ));
    //   active_counter.update(self.db).await
  }
}
