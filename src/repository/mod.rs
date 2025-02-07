mod comment;
mod counter;
mod user;

use sea_orm::DatabaseConnection;

pub use comment::CommentRepository;
pub use counter::CounterRepository;
pub use user::UserRepository;

#[derive(Debug, Clone)]
pub struct RepositoryManager {
  db: DatabaseConnection,
}

impl RepositoryManager {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db }
  }

  pub fn user(&self) -> UserRepository {
    UserRepository { db: &self.db }
  }

  pub fn comment(&self) -> CommentRepository {
    CommentRepository { db: &self.db }
  }

  pub fn counter(&self) -> CounterRepository {
    CounterRepository { db: &self.db }
  }
}
