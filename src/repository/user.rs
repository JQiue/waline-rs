use crate::entities::wl_users;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

#[derive(Debug, Clone)]
pub struct UserRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl<'a> UserRepository<'a> {
  pub async fn get_users(&self) -> Result<Vec<wl_users::Model>, DbErr> {
    wl_users::Entity::find().all(self.db).await
  }
  pub async fn get_user_by_id(&self, id: u32) -> Result<Option<wl_users::Model>, DbErr> {
    wl_users::Entity::find_by_id(id).one(self.db).await
  }
  pub async fn get_user_by_email(&self, email: &str) -> Result<Option<wl_users::Model>, DbErr> {
    wl_users::Entity::find()
      .filter(wl_users::Column::Email.eq(email))
      .one(self.db)
      .await
  }
  pub async fn is_admin_user(&self, email: &str) -> Result<bool, DbErr> {
    let user = wl_users::Entity::find()
      .filter(wl_users::Column::Email.eq(email))
      .one(self.db)
      .await?;
    match user {
      Some(user) => Ok(user.user_type == "administrator"),
      None => Ok(false),
    }
  }
  pub async fn create_user(&self, user: wl_users::ActiveModel) -> Result<wl_users::Model, DbErr> {
    user.insert(self.db).await
  }
  pub async fn update_user(&self, user: wl_users::ActiveModel) -> Result<wl_users::Model, DbErr> {
    user.update(self.db).await
  }
}
