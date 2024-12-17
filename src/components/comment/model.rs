use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{
  entities::wl_comment,
  error::AppError,
  helpers::{email::extract_email_prefix, markdown::render_md_to_html, ua},
};

#[derive(Clone)]
pub enum CommentQueryBy {
  Id(u32),
}

pub async fn has_comment(
  query_by: CommentQueryBy,
  conn: &DatabaseConnection,
) -> Result<bool, AppError> {
  let mut query = wl_comment::Entity::find();
  match query_by {
    CommentQueryBy::Id(id) => query = query.filter(wl_comment::Column::Id.eq(id)),
  }
  let res = query.one(conn).await.map_err(AppError::from)?;
  Ok(res.is_some())
}

pub async fn get_comment(
  query_by: CommentQueryBy,
  conn: &DatabaseConnection,
) -> Result<wl_comment::Model, AppError> {
  if !has_comment(query_by.to_owned(), conn).await? {
    return Err(AppError::Error);
  }
  let mut query = wl_comment::Entity::find();
  match query_by {
    CommentQueryBy::Id(id) => query = query.filter(wl_comment::Column::Id.eq(id)),
  }
  query
    .one(conn)
    .await
    .map_err(AppError::from)?
    .ok_or(AppError::Error)
}

pub async fn is_anonymous(comment_id: u32, conn: &DatabaseConnection) -> Result<bool, AppError> {
  let res = wl_comment::Entity::find_by_id(comment_id)
    .filter(wl_comment::Column::UserId.is_not_null())
    .filter(wl_comment::Column::UserId.ne(""))
    .one(conn)
    .await
    .map_err(AppError::from)?;
  Ok(res.is_none())
}

pub async fn is_duplicate(
  comment_id: u32,
  url: String,
  mail: String,
  nick: String,
  link: String,
  comment: String,
  conn: &DatabaseConnection,
) -> Result<bool, AppError> {
  let res = wl_comment::Entity::find_by_id(comment_id)
    .filter(wl_comment::Column::Url.eq(url))
    .filter(wl_comment::Column::Mail.ne(mail))
    .filter(wl_comment::Column::Nick.ne(nick))
    .filter(wl_comment::Column::Link.ne(link))
    .filter(wl_comment::Column::Comment.ne(comment))
    .one(conn)
    .await
    .map_err(AppError::from)?;
  Ok(res.is_some())
}

#[derive(Serialize, Debug)]
pub struct DataEntry {
  pub status: String,
  pub like: Option<i32>,
  pub link: Option<String>,
  pub mail: Option<String>,
  pub nick: Option<String>,
  pub user_id: Option<i32>,
  pub browser: String,
  pub os: String,
  pub r#type: Option<String>,
  #[serde(rename = "objectId")]
  pub object_id: u32,
  pub ip: Option<String>,
  pub orig: Option<String>,
  pub url: Option<String>,
  pub pid: Option<i32>,
  pub rid: Option<i32>,
  pub time: i64,
  pub comment: Option<String>,
  pub avatar: String,
  pub level: i32,
  pub label: Option<String>,
  pub children: Vec<DataEntry>,
}

pub fn build_data_entry(comment: wl_comment::Model, anonymous_avatar: String) -> DataEntry {
  let (browser, os) = ua::parse(comment.ua.unwrap_or("".to_owned()));
  DataEntry {
    status: comment.status,
    like: comment.like,
    link: comment.link,
    mail: comment.mail.clone(),
    nick: comment.nick,
    user_id: comment.user_id,
    browser,
    os,
    url: comment.url,
    r#type: None,
    object_id: comment.id,
    ip: comment.ip,
    orig: comment.comment.clone(),
    time: comment.created_at.unwrap().timestamp_millis(),
    pid: comment.pid,
    rid: comment.rid,
    comment: Some(render_md_to_html(&comment.comment.unwrap_or("".to_owned()))),
    avatar: if comment.user_id.is_some() {
      format!(
        "https://q1.qlogo.cn/g?b=qq&nk={}&s=100",
        extract_email_prefix(comment.mail.unwrap()).unwrap()
      )
    } else {
      anonymous_avatar
    },
    level: 0,
    label: None,
    children: vec![],
  }
}

#[derive(Deserialize, Clone)]
pub struct GetCommentQuery {
  pub lang: Option<String>,
  pub path: Option<String>,
  #[serde(rename = "pageSize")]
  pub page_size: Option<i32>,
  pub page: i32,
  #[serde(rename = "sortBy")]
  pub sort_by: Option<String>,
  pub r#type: Option<String>,
  pub owner: Option<String>,
  pub status: Option<String>,
  pub keyword: Option<String>,
}

impl GetCommentQuery {
  pub fn validate_by_path(&self) -> Result<(), Vec<&'static str>> {
    let mut missing_fields = Vec::new();
    // 检查 Option 字段
    if self.path.is_none() {
      missing_fields.push("path");
    }
    if self.page_size.is_none() {
      missing_fields.push("pageSize");
    }
    if self.sort_by.is_none() {
      missing_fields.push("sortBy");
    }
    if missing_fields.is_empty() {
      Ok(())
    } else {
      Err(missing_fields)
    }
  }
  pub fn validate_by_admin(&self) -> Result<(), Vec<&'static str>> {
    let mut missing_fields = Vec::new();
    if self.lang.is_none() {
      missing_fields.push("lang");
    }
    if self.r#type.is_none() {
      missing_fields.push("type");
    }
    if self.owner.is_none() {
      missing_fields.push("owner");
    }
    if self.status.is_none() {
      missing_fields.push("status");
    }
    if self.keyword.is_none() {
      missing_fields.push("keyword");
    }
    if missing_fields.is_empty() {
      Ok(())
    } else {
      Err(missing_fields)
    }
  }
}

pub fn create_comment_model(
  user_id: Option<i32>,
  comment: String,
  link: String,
  mail: String,
  nick: String,
  ua: String,
  url: String,
  pid: Option<i32>,
  rid: Option<i32>,
) -> wl_comment::ActiveModel {
  let utc_time = helpers::time::utc_now();
  wl_comment::ActiveModel {
    user_id: Set(user_id),
    comment: Set(Some(comment)),
    link: Set(Some(link)),
    mail: Set(Some(mail)),
    nick: Set(Some(nick)),
    ua: Set(Some(ua)),
    url: Set(Some(url)),
    status: Set("approved".to_string()),
    pid: Set(pid),
    rid: Set(rid),
    inserted_at: Set(Some(utc_time)),
    created_at: Set(Some(utc_time)),
    updated_at: Set(Some(utc_time)),
    ..Default::default()
  }
}

#[derive(Deserialize)]
pub struct CreateCommentQuery {
  pub lang: String,
}

#[derive(Deserialize, Clone)]
pub struct CreateCommentBody {
  pub comment: String,
  // or ""
  pub link: String,
  // or ""
  pub mail: String,
  // or ""
  pub nick: String,
  // user-agent
  pub ua: String,
  // path
  pub url: String,
  // Parent comment ID
  pub pid: Option<i32>,
  // span id
  pub rid: Option<i32>,
  // at
  pub at: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateCommentBody {
  pub status: Option<String>,
  pub like: Option<bool>,
  pub comment: Option<String>,
  pub link: Option<String>,
  pub mail: Option<String>,
  pub nick: Option<String>,
  pub ua: Option<String>,
  pub url: Option<String>,
}
