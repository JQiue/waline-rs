use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{
  entities::{prelude::*, *},
  helpers::{
    email::extract_email_prefix, markdown::render_md_to_html, time::get_current_utc_time, ua,
  },
};

pub async fn get_user(query_by: UserQueryBy, conn: &DatabaseConnection) -> wl_users::Model {
  let mut query = WlUsers::find();
  match query_by {
    UserQueryBy::Id(id) => query = query.filter(wl_users::Column::Id.eq(id)),
    UserQueryBy::Email(email) => query = query.filter(wl_users::Column::Email.eq(email)),
  }
  query.one(conn).await.unwrap().unwrap()
}

pub async fn is_anonymous(comment_id: u32, conn: &DatabaseConnection) -> bool {
  let res = WlComment::find_by_id(comment_id)
    .filter(wl_comment::Column::UserId.is_not_null())
    .filter(wl_comment::Column::UserId.ne(""))
    .one(conn)
    .await
    .unwrap();
  res.is_none()
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
  let (browser, os) = ua::parse(comment.ua.as_ref().unwrap().to_string());
  DataEntry {
    status: comment.status,
    like: comment.like,
    link: comment.link,
    mail: comment.mail.clone(),
    nick: comment.nick,
    user_id: comment.user_id,
    browser,
    os,
    r#type: None, // TODO: 获取用户类型
    object_id: comment.id,
    ip: comment.ip,
    orig: comment.comment.clone(),
    time: comment.created_at.unwrap().timestamp_millis(),
    pid: comment.pid,
    rid: comment.rid,
    comment: Some(render_md_to_html(comment.comment.as_ref().unwrap())),
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

pub enum UserQueryBy {
  Id(u32),
  Email(String),
}

#[derive(Deserialize)]
pub struct GetCommentQuery {
  pub lang: String,
  pub path: String,
  #[serde(rename = "pageSize")]
  pub page_size: i32,
  pub page: i32,
  #[serde(rename = "sortBy")]
  pub sort_by: String,
  pub r#type: Option<String>,
  pub owner: Option<String>,
  pub status: Option<String>,
  pub keyword: Option<String>,
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
  let utc_time = get_current_utc_time();
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
  //
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
