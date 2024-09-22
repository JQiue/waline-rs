use crate::{
  components::user::model::{has_user, UserQueryBy},
  entities::{prelude::*, *},
  AppState,
};
use chrono::{DateTime, Utc};
use sea_orm::{EntityTrait, Iterable, QuerySelect, Set};
use serde_json::{json, Value};

pub async fn export_data(state: &AppState, _lang: String) -> Result<Value, String> {
  let comments = WlComment::find()
    .select_only()
    .columns(wl_comment::Column::iter().filter(|col| !matches!(col, wl_comment::Column::Id)))
    .column_as(wl_comment::Column::Id, "objectId")
    .into_json()
    .all(&state.conn)
    .await
    .unwrap();
  let counters = WlCounter::find()
    .select_only()
    .columns(wl_counter::Column::iter().filter(|col| !matches!(col, wl_counter::Column::Id)))
    .column_as(wl_counter::Column::Id, "objectId")
    .into_json()
    .all(&state.conn)
    .await
    .unwrap();
  let users = WlUsers::find()
    .select_only()
    .columns(wl_users::Column::iter().filter(|col| !matches!(col, wl_users::Column::Id)))
    .column_as(wl_users::Column::Id, "objectId")
    .into_json()
    .all(&state.conn)
    .await
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
) -> Result<bool, String> {
  let model = wl_comment::ActiveModel {
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
  };
  match WlComment::insert(model).exec(&state.conn).await {
    Ok(res) => {
      println!("{:?}", res);
      Ok(true)
    }
    Err(err) => Err(err.to_string()),
  }
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
) -> Result<bool, String> {
  let model = wl_counter::ActiveModel {
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
  };
  match WlCounter::insert(model).exec(&state.conn).await {
    Ok(_) => Ok(true),
    Err(err) => Err(err.to_string()),
  }
}

// todo
pub async fn update_comment_data(
  state: &AppState,
  lang: String,
  object_id: Option<u32>,
  user_id: Option<u32>,
) -> Result<bool, String> {
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
  match WlUsers::insert(model).exec(&state.conn).await {
    Ok(_) => Ok(true),
    Err(err) => Err(err.to_string()),
  }
}

pub async fn update_user_data(
  state: &AppState,
  lang: String,
  object_id: Option<u32>,
  display_name: Option<String>,
  password: Option<String>,
  email: Option<String>,
  url: Option<String>,
  label: Option<String>,
  r#type: Option<String>,
  two_factor_auth: Option<String>,
  created_at: Option<DateTime<Utc>>,
  updated_at: Option<DateTime<Utc>>,
) -> Result<bool, String> {
  if has_user(
    UserQueryBy::Email(email.clone().unwrap_or("".to_string())),
    &state.conn,
  )
  .await
  {
    let model = wl_users::ActiveModel {
      id: Set(object_id.unwrap()),
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
    match WlUsers::update(model).exec(&state.conn).await {
      Ok(_) => Ok(true),
      Err(err) => Err(err.to_string()),
    }
  } else {
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
    match WlUsers::insert(model).exec(&state.conn).await {
      Ok(_) => Ok(true),
      Err(err) => Err(err.to_string()),
    }
  }
}

pub async fn delete_data(state: &AppState, table: &str, lang: String) -> Result<bool, String> {
  match table {
    "Comment" => {
      let res = WlComment::delete_many().exec(&state.conn).await.unwrap();
      println!("Delete Result >>> {:?}", res);
      Ok(true)
    }
    "Counter" => {
      let res = WlCounter::delete_many().exec(&state.conn).await.unwrap();
      println!("Delete Result >>> {:?}", res);
      Ok(true)
    }
    "User" => Ok(true),
    _ => Err("".to_string()),
  }
}
