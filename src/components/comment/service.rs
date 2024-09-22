use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};
use serde_json::{json, Value};

use crate::{
  components::comment::model::*,
  entities::{prelude::*, *},
  helpers::{
    email::extract_email_prefix, markdown::render_md_to_html, time::get_current_utc_time, ua,
  },
  AppState,
};

pub async fn get_comment(
  state: &AppState,
  path: String,
  owner: Option<String>,
  page: i32,
  page_size: i32,
) -> Result<Value, String> {
  match owner.clone() {
    Some(owner) => {
      let mut data = vec![];
      if owner == "mine" {
        // let _header_value = req.headers().get(AUTHORIZATION).unwrap();
        let email = "";
        data = WlComment::find()
          .filter(wl_comment::Column::Mail.eq(email))
          .all(&state.conn)
          .await
          .unwrap();
      } else if owner == "all" {
        data = WlComment::find().all(&state.conn).await.unwrap();
      }

      return Ok(json!({
        "data": data,
        "page": page,
        "pageSize": page_size,
        "spamCount": 0,
        "totalPages": 5,
        "waitingCount": 0,
      }));
    }
    None => {
      println!(">>> None");
    }
  }

  // 根据 path 获取根评论
  let parrent_comments = wl_comment::Entity::find()
    .filter(wl_comment::Column::Url.contains(path.clone()))
    .filter(wl_comment::Column::Pid.is_null())
    .order_by(wl_comment::Column::InsertedAt, Order::Desc)
    .all(&state.conn)
    .await
    .unwrap();
  // Get comment count for articles
  let mut count = parrent_comments.len();
  let mut comments: Vec<DataEntry> = vec![];

  for parrent_comment in parrent_comments {
    let mut parrent_data_entry =
      build_data_entry(parrent_comment.clone(), state.anonymous_avatar.to_string());
    if let Some(user_id) = parrent_data_entry.user_id {
      let user = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await;
      parrent_data_entry.label = user.label;
      parrent_data_entry.r#type = Some(user.r#type);
    }
    let subcomments = wl_comment::Entity::find()
      .filter(wl_comment::Column::Url.contains(path.clone()))
      .filter(wl_comment::Column::Pid.eq(parrent_comment.id))
      .order_by(wl_comment::Column::InsertedAt, Order::Asc)
      .all(&state.conn)
      .await
      .unwrap();

    for subcomment in subcomments {
      count += 1;
      let mut subcomment_data_entry =
        build_data_entry(subcomment.clone(), state.anonymous_avatar.to_string());
      if let Some(user_id) = subcomment_data_entry.user_id {
        let user = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await;
        subcomment_data_entry.label = user.label;
        subcomment_data_entry.r#type = Some(user.r#type);
      }
      parrent_data_entry.children.push(subcomment_data_entry)
    }

    comments.push(parrent_data_entry)
  }
  Ok(json!({ "count": count, "data": comments, "page": 1, "pageSize": 10, "totalPages": 0 }))
}

/// create comment
/// No user is created if the user is anonymous

pub async fn create_comment(
  state: &AppState,
  comment: String,
  link: String,
  mail: String,
  nick: String,
  ua: String,
  url: String,
  pid: Option<i32>,
  rid: Option<i32>,
  _at: Option<String>,
) -> Result<Value, String> {
  let user = WlUsers::find()
    .filter(wl_users::Column::Email.eq(mail.clone()))
    .filter(wl_users::Column::DisplayName.eq(nick.clone()))
    .one(&state.conn)
    .await
    .unwrap();
  let html_output = render_md_to_html(&comment);
  let model = match user {
    Some(user) => create_comment_model(
      Some(user.id as i32),
      comment,
      link,
      mail.clone(),
      nick.clone(),
      ua,
      url,
      pid,
      rid,
    ),
    None => create_comment_model(
      None,
      comment,
      link,
      mail.clone(),
      nick.clone(),
      ua,
      url,
      pid,
      rid,
    ),
  };

  match WlComment::insert(model).exec(&state.conn).await {
    Ok(result) => {
      let comment = WlComment::find_by_id(result.last_insert_id)
        .one(&state.conn)
        .await
        .unwrap()
        .unwrap();
      let (browser, os) = ua::parse(comment.ua.unwrap());
      let like = comment.like.unwrap_or(0);
      let time = comment.created_at.unwrap().timestamp_millis();
      let pid = comment.pid;
      let rid = comment.rid;
      if nick == "匿名" {
        let mut data = json!({
          "addr":"",
          "avatar": state.anonymous_avatar.to_string(),
          "browser": browser,
          "comment": html_output,
          "like": like,
          "link": comment.link,
          "nick": comment.nick,
          "objectId": comment.id,
          "orig": comment.comment,
          "os": os,
          "status": comment.status,
          "time": time,
          "url": comment.url,
        });
        if let Some(pid) = pid {
          data["pid"] = json!(pid);
        }
        if let Some(rid) = rid {
          data["rid"] = json!(rid);
        };
        Ok(data)
      } else {
        let user = get_user(UserQueryBy::Email(mail.clone()), &state.conn).await;
        let avatar = if let Some(prefix) = extract_email_prefix(mail.clone()) {
          format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
        } else {
          state.anonymous_avatar.to_string()
        };
        let mut data = json!({
          "addr":"",
          "avatar": avatar,
          "browser": browser,
          "comment": html_output,
          "ip": comment.ip,
          "label": user.label,
          "mail": user.email,
          "type": user.r#type,
          "user_id": user.id,
          "like": like,
          "link": comment.link,
          "nick": comment.nick,
          "objectId": comment.id,
          "orig": comment.comment,
          "os": os,
          "status": comment.status,
          "time": time,
          "url": comment.url,
        });
        if let Some(pid) = pid {
          data["pid"] = json!(pid);
        }
        if let Some(rid) = rid {
          data["rid"] = json!(rid);
        };
        Ok(data)
      }
    }
    Err(err) => Err(err.to_string()),
  }
}

pub async fn delete_comment(state: &AppState, id: u32) -> Result<bool, String> {
  match WlComment::delete_by_id(id).exec(&state.conn).await {
    Ok(_) => Ok(true),
    Err(err) => Err(err.to_string()),
  }
}

pub async fn update_comment(
  state: &AppState,
  id: u32,
  status: Option<String>,
  like: Option<bool>,
  comment: Option<String>,
  _link: Option<String>,
  _mail: Option<String>,
  _nick: Option<String>,
  ua: Option<String>,
  _url: Option<String>,
) -> Result<Value, String> {
  let updated_at = get_current_utc_time();
  let new_comment;
  if let Some(like) = like {
    let comment = wl_comment::Entity::find_by_id(id)
      .one(&state.conn)
      .await
      .unwrap()
      .unwrap();
    let model = if like {
      wl_comment::ActiveModel {
        id: Set(id),
        like: Set(Some(comment.like.unwrap_or(0) + 1)),
        updated_at: Set(Some(updated_at)),
        ..Default::default()
      }
    } else {
      wl_comment::ActiveModel {
        id: Set(id),
        like: Set(Some(comment.like.unwrap_or(0) - 1)),
        updated_at: Set(Some(updated_at)),
        ..Default::default()
      }
    };
    new_comment = WlComment::update(model).exec(&state.conn).await.unwrap();
  } else if let Some(status) = status {
    let model = wl_comment::ActiveModel {
      id: Set(id),
      status: Set(status),
      updated_at: Set(Some(updated_at)),
      ..Default::default()
    };
    new_comment = WlComment::update(model).exec(&state.conn).await.unwrap();
  } else {
    let model = wl_comment::ActiveModel {
      id: Set(id),
      comment: Set(comment),
      ua: Set(ua),
      ..Default::default()
    };
    new_comment = WlComment::update(model).exec(&state.conn).await.unwrap();
  }

  let (browser, os) = ua::parse(new_comment.ua.unwrap());
  let like = new_comment.like.unwrap_or(0);
  let time = new_comment.created_at.unwrap().timestamp_millis();
  let pid = new_comment.pid;
  let rid = new_comment.rid;
  let html_output = render_md_to_html(new_comment.comment.clone().unwrap().as_str());

  if is_anonymous(id, &state.conn).await {
    let data = json!({
      "addr":"",
      "avatar": state.anonymous_avatar.to_string(),
      "browser": browser,
      "comment": html_output,
      "ip": new_comment.ip,
      "mail": new_comment.mail,
      "user_id": new_comment.user_id,
      "like": like,
      "link": new_comment.link,
      "nick": new_comment.nick,
      "objectId": new_comment.id,
      "orig": new_comment.comment,
      "os": os,
      "status": new_comment.status,
      "time": time,
      "url": new_comment.url,
    });
    Ok(data)
  } else {
    let user = get_user(
      UserQueryBy::Id(new_comment.user_id.unwrap() as u32),
      &state.conn,
    )
    .await;
    let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
      format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
    } else {
      state.anonymous_avatar.to_string()
    };
    let mut data = json!({
      "addr":"",
      "avatar": avatar,
      "browser": browser,
      "comment": html_output,
      "ip": new_comment.ip,
      "label": user.label,
      "mail": user.email.clone(),
      "type": user.r#type,
      "user_id": new_comment.user_id,
      "like": like,
      "link": new_comment.link,
      "nick": new_comment.nick,
      "objectId": new_comment.id,
      "orig": new_comment.comment,
      "os": os,
      "status": new_comment.status,
      "time": time,
      "url": new_comment.url,
    });
    if let Some(pid) = pid {
      data["pid"] = json!(pid);
    }
    if let Some(rid) = rid {
      data["rid"] = json!(rid);
    };
    Ok(data)
  }
}
