use crate::entities::wl_comment;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};

#[derive(Debug, Clone)]
pub struct CommentRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl<'a> CommentRepository<'a> {
  pub async fn get_comments(&self) -> Result<Vec<wl_comment::Model>, DbErr> {
    wl_comment::Entity::find().all(self.db).await
  }
  pub async fn create_comment(
    &self,
    comment: wl_comment::ActiveModel,
  ) -> Result<wl_comment::Model, DbErr> {
    comment.insert(self.db).await
  }
  pub async fn update_comment(
    &self,
    comment: wl_comment::ActiveModel,
  ) -> Result<wl_comment::Model, DbErr> {
    comment.update(self.db).await
  }
}
