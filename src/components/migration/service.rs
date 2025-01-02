use crate::components::migration::model::CommentData;
use crate::components::user::model::get_user;
use crate::prelude::*;

use crate::{
  app::AppState,
  components::{
    comment::model::{get_comment, CommentQueryBy},
    user::model::{has_user, UserQueryBy},
  },
  entities::{wl_comment, wl_counter, wl_users},
  error::AppError,
  response::Code,
};
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde_json::{json, Value};

use super::model::{CounterData, UserData};

pub async fn export_data(state: &AppState, _lang: String) -> Result<Value, String> {
  let comments = wl_comment::Entity::find()
    .into_partial_model::<CommentData>()
    .all(&state.conn)
    .await
    .log_err()
    .unwrap();
  let counters = wl_counter::Entity::find()
    .into_partial_model::<CounterData>()
    .all(&state.conn)
    .await
    .log_err()
    .unwrap();
  let users = wl_users::Entity::find()
    // .select_only()
    // .columns(wl_users::Column::iter().filter(|col| !matches!(col, wl_users::Column::Id)))
    // .column_as(wl_users::Column::Id, "objectId")
    // .into_json()
    // .all(&state.conn)
    // .await
    // .unwrap();
    .into_partial_model::<UserData>()
    .all(&state.conn)
    .await
    .log_err()
    .unwrap();
  let data = json!({
      "type": "waline",
      "version": 1,
      "time": Utc::now().timestamp_millis(),
      "tables": ["Comment", "Counter", "Users"],
      "data": {
        "Comment": comments,
        "Counter": counters,
        "Users": users,
      }
  });
  Ok(data)
}

pub async fn create_comment_data(
  state: &AppState,
  comment: Option<String>,
  ip: Option<String>,
  link: Option<String>,
  mail: Option<String>,
  nick: Option<String>,
  status: Option<String>,
  ua: Option<String>,
  url: Option<String>,
  create_at: Option<chrono::DateTime<Utc>>,
  updated_at: Option<chrono::DateTime<Utc>>,
  inserted_at: Option<chrono::DateTime<Utc>>,
) -> Result<Value, Code> {
  let comment = wl_comment::ActiveModel {
    comment: Set(comment),
    inserted_at: Set(inserted_at),
    ip: Set(ip),
    link: Set(link),
    mail: Set(mail),
    nick: Set(nick),
    status: Set(status.unwrap()),
    ua: Set(ua),
    url: Set(url),
    created_at: Set(create_at),
    updated_at: Set(updated_at),
    ..Default::default()
  }
  .insert(&state.conn)
  .await
  .map_err(AppError::from)?;
  Ok(json!({
    "objectId": comment.id,
    "comment": comment.comment,
    "ip": comment.ip,
    "link": comment.link,
    "mail": comment.mail,
    "nick": comment.nick,
    "status": comment.status,
    "ua": comment.ua,
    "url": comment.url,
    "insertedAt": comment.inserted_at,
    "createdAt": comment.created_at,
    "updatedAt": comment.updated_at,
  }))
}

pub async fn create_counter_data(
  state: &AppState,
  time: Option<i32>,
  url: Option<String>,
  reaction0: Option<i32>,
  reaction1: Option<i32>,
  reaction2: Option<i32>,
  reaction3: Option<i32>,
  reaction4: Option<i32>,
  reaction5: Option<i32>,
  reaction6: Option<i32>,
  reaction7: Option<i32>,
  reaction8: Option<i32>,
  created_at: Option<chrono::DateTime<Utc>>,
  updated_at: Option<chrono::DateTime<Utc>>,
) -> Result<wl_counter::Model, Code> {
  Ok(
    wl_counter::ActiveModel {
      time: Set(time),
      reaction0: Set(reaction0),
      reaction1: Set(reaction1),
      reaction2: Set(reaction2),
      reaction3: Set(reaction3),
      reaction4: Set(reaction4),
      reaction5: Set(reaction5),
      reaction6: Set(reaction6),
      reaction7: Set(reaction7),
      reaction8: Set(reaction8),
      url: Set(url.unwrap()),
      created_at: Set(created_at),
      updated_at: Set(updated_at),
      ..Default::default()
    }
    .insert(&state.conn)
    .await
    .map_err(AppError::from)?,
  )
}

pub async fn update_comment_data(
  state: &AppState,
  object_id: u32,
  pid: Option<i32>,
  rid: Option<i32>,
) -> Result<bool, Code> {
  let mut comment = get_comment(CommentQueryBy::Id(object_id), &state.conn)
    .await?
    .into_active_model();
  comment.pid = Set(pid);
  comment.rid = Set(rid);
  comment.update(&state.conn).await.map_err(AppError::from)?;
  Ok(true)
}

pub async fn create_user_data(
  state: &AppState,
  _object_id: Option<u32>,
  display_name: Option<String>,
  password: Option<String>,
  email: Option<String>,
  r#type: Option<String>,
  label: Option<String>,
  url: Option<String>,
  two_factor_auth: Option<String>,
  created_at: Option<DateTime<Utc>>,
  updated_at: Option<DateTime<Utc>>,
) -> Result<bool, String> {
  let model = wl_users::ActiveModel {
    display_name: Set(display_name.unwrap()),
    email: Set(email.unwrap()),
    password: Set(password.unwrap()),
    r#type: Set(r#type.unwrap()),
    label: Set(label),
    url: Set(url),
    two_factor_auth: Set(two_factor_auth),
    created_at: Set(created_at),
    updated_at: Set(updated_at),
    ..Default::default()
  };
  match wl_users::Entity::insert(model).exec(&state.conn).await {
    Ok(_) => Ok(true),
    Err(err) => Err(err.to_string()),
  }
}

pub async fn update_user_data(
  state: &AppState,
  _object_id: Option<u32>,
  display_name: Option<String>,
  password: Option<String>,
  email: Option<String>,
  url: Option<String>,
  label: Option<String>,
  r#type: Option<String>,
  two_factor_auth: Option<String>,
  created_at: Option<DateTime<Utc>>,
  updated_at: Option<DateTime<Utc>>,
) -> Result<(), Code> {
  if has_user(
    UserQueryBy::Email(email.clone().unwrap_or("".to_string())),
    &state.conn,
  )
  .await?
  {
    let mut active_user = get_user(
      UserQueryBy::Email(email.clone().unwrap_or("".to_string())),
      &state.conn,
    )
    .await?
    .into_active_model();
    active_user.display_name = Set(display_name.unwrap());
    active_user.email = Set(email.unwrap());
    active_user.password = Set(password.unwrap());
    active_user.r#type = Set(r#type.unwrap());
    active_user.label = Set(label);
    active_user.url = Set(url);
    active_user.two_factor_auth = Set(two_factor_auth);
    active_user.created_at = Set(created_at);
    active_user.updated_at = Set(updated_at);
    match active_user
      .update(&state.conn)
      .await
      .log_err()
      .map_err(AppError::from)
    {
      Ok(_) => Ok(()),
      Err(_) => Err(Code::Error),
    }
  } else {
    match (wl_users::ActiveModel {
      display_name: Set(display_name.unwrap()),
      email: Set(email.unwrap()),
      password: Set(password.unwrap()),
      r#type: Set(r#type.unwrap()),
      label: Set(label),
      url: Set(url),
      two_factor_auth: Set(two_factor_auth),
      created_at: Set(created_at),
      updated_at: Set(updated_at),
      ..Default::default()
    }
    .insert(&state.conn)
    .await
    .log_err())
    {
      Ok(_) => Ok(()),
      Err(_) => Err(Code::Error),
    }
  }
}

pub async fn delete_data(state: &AppState, table: &str) -> Result<bool, Code> {
  match table {
    "Comment" => {
      wl_comment::Entity::delete_many()
        .exec(&state.conn)
        .await
        .map_err(AppError::from)?;
      Ok(true)
    }
    "Counter" => {
      wl_counter::Entity::delete_many()
        .exec(&state.conn)
        .await
        .map_err(AppError::from)?;
      Ok(true)
    }
    "User" => Ok(true),
    _ => Err(Code::Error),
  }
}
