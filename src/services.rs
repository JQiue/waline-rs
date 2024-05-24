#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{
  entities::{prelude::*, *},
  helpers::{token, ua},
  AppState,
};
use actix_web::{
  delete, get,
  http::header::{ContentType, HeaderValue, AUTHORIZATION},
  post, put,
  web::{resource, Data, Json, Path, Query, ServiceConfig},
  HttpRequest, HttpResponse, Responder,
};
use chrono::Utc;
use pulldown_cmark::{self, Event};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

const ANONYMOUS_AVATAR: &str =
  "https://seccdn.libravatar.org/avatar/d41d8cd98f00b204e9800998ecf8427e";

fn extract_email_prefix(email: String) -> Option<String> {
  let mut res = email.split('@');
  res.next().map(|prefix| prefix.to_string())
}

fn render_markdown(markdown: &str) -> String {
  let parser = pulldown_cmark::Parser::new(markdown);
  let parser = parser.map(|event| match event {
    Event::SoftBreak => Event::HardBreak,
    _ => event,
  });
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);
  html_output
}

#[derive(Serialize, Deserialize)]
struct ResponseModel<T> {
  data: T,
  errmsg: String,
  errno: i8,
}

#[derive(Serialize, Debug)]
struct DataEntry {
  status: String,
  like: Option<i32>,
  link: Option<String>,
  mail: Option<String>,
  nick: Option<String>,
  user_id: Option<i32>,
  browser: String,
  os: String,
  r#type: Option<String>,
  objectId: u32,
  ip: Option<String>,
  orig: Option<String>,
  pid: Option<i32>,
  rid: Option<i32>,
  time: i64,
  comment: Option<String>,
  avatar: String,
  level: i32,
  label: Option<String>,
  children: Vec<DataEntry>,
}

fn build_data_entry(comment: wl_comment::Model) -> DataEntry {
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
    objectId: comment.id,
    ip: comment.ip,
    orig: comment.comment.clone(),
    time: comment.created_at.unwrap().timestamp_millis(),
    pid: comment.pid,
    rid: comment.rid,
    comment: Some(render_markdown(comment.comment.as_ref().unwrap())),
    avatar: if comment.user_id.is_some() {
      format!(
        "https://q1.qlogo.cn/g?b=qq&nk={}&s=100",
        extract_email_prefix(comment.mail.unwrap()).unwrap()
      )
    } else {
      ANONYMOUS_AVATAR.to_string()
    },
    level: 0,
    label: None,
    children: vec![],
  }
}

enum UserQueryBy {
  Id(u32),
  Email(String),
}

async fn get_user(query_by: UserQueryBy, conn: &DatabaseConnection) -> wl_users::Model {
  let mut query = WlUsers::find();
  match query_by {
    UserQueryBy::Id(id) => query = query.filter(wl_users::Column::Id.eq(id)),
    UserQueryBy::Email(email) => query = query.filter(wl_users::Column::Email.eq(email)),
  }
  query.one(conn).await.unwrap().unwrap()
}

async fn is_anonymous(comment_id: u32, conn: &DatabaseConnection) -> bool {
  let res = WlComment::find_by_id(comment_id)
    .filter(wl_comment::Column::UserId.is_not_null())
    .filter(wl_comment::Column::UserId.ne(""))
    .one(conn)
    .await
    .unwrap();
  res.is_none()
}

#[derive(Deserialize)]
struct GetCommentQuery {
  lang: String,
  path: String,
  pageSize: i32,
  page: i32,
  sortBy: String,
  r#type: Option<String>,
  owner: Option<String>,
  status: Option<String>,
  keyword: Option<String>,
}

/// get comment
#[get("/api/comment")]
async fn get_comment(
  req: HttpRequest,
  data: Data<AppState>,
  query: Query<GetCommentQuery>,
) -> impl Responder {
  let conn = &data.conn;
  match query.owner.clone() {
    Some(owner) => {
      let mut data = vec![];
      if owner == "mine" {
        let _header_value = req.headers().get(AUTHORIZATION).unwrap();
        let email = "";
        data = WlComment::find()
          .filter(wl_comment::Column::Mail.eq(email))
          .all(conn)
          .await
          .unwrap();
      } else if owner == "all" {
        data = WlComment::find().all(conn).await.unwrap();
      }

      return HttpResponse::Ok().json(json!({
        "data": {
          "data": data,
          "page": 1,
          "pageSize": 10,
          "spamCount": 0,
          "totalPages": 5,
          "waitingCount": 0,
        },
        "errmsg": "",
        "errno": 0
      }));
    }
    None => {
      println!(">>> None");
    }
  }

  // 根据 path 获取根评论
  let parrent_comments = wl_comment::Entity::find()
    .filter(wl_comment::Column::Url.contains(query.path.clone()))
    .filter(wl_comment::Column::Pid.is_null())
    .order_by(wl_comment::Column::InsertedAt, Order::Desc)
    .all(conn)
    .await
    .unwrap();
  // Get comment count for articles
  let mut count = parrent_comments.len();
  let mut comments: Vec<DataEntry> = vec![];

  for parrent_comment in parrent_comments {
    let mut parrent_data_entry = build_data_entry(parrent_comment.clone());
    if let Some(user_id) = parrent_data_entry.user_id {
      let user = get_user(UserQueryBy::Id(user_id as u32), conn).await;
      parrent_data_entry.label = user.label;
      parrent_data_entry.r#type = Some(user.r#type);
    }
    let subcomments = wl_comment::Entity::find()
      .filter(wl_comment::Column::Url.contains(query.path.clone()))
      .filter(wl_comment::Column::Pid.eq(parrent_comment.id))
      .order_by(wl_comment::Column::InsertedAt, Order::Asc)
      .all(conn)
      .await
      .unwrap();

    for subcomment in subcomments {
      count += 1;
      let mut subcomment_data_entry = build_data_entry(subcomment.clone());
      if let Some(user_id) = subcomment_data_entry.user_id {
        let user = get_user(UserQueryBy::Id(user_id as u32), conn).await;
        subcomment_data_entry.label = user.label;
        subcomment_data_entry.r#type = Some(user.r#type);
      }
      parrent_data_entry.children.push(subcomment_data_entry)
    }

    comments.push(parrent_data_entry)
  }
  let data =
    json!({ "count": count, "data": comments, "page": 1, "pageSize": 10, "totalPages": 0 });
  HttpResponse::Ok().json(ResponseModel {
    errno: 0,
    errmsg: "".to_string(),
    data,
  })
}

fn create_comment_model(
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
  let created_at: chrono::DateTime<Utc> = Utc::now();
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
    inserted_at: Set(Some(created_at)),
    created_at: Set(Some(created_at)),
    updated_at: Set(Some(created_at)),
    ..Default::default()
  }
}

#[derive(Deserialize)]
struct CreateCommentQuery {
  lang: String,
}

#[derive(Deserialize, Clone)]
struct CreateCommentBody {
  comment: String,
  // or ""
  link: String,
  // or ""
  mail: String,
  // or ""
  nick: String,
  // user-agent
  ua: String,
  // path
  url: String,
  // Parent comment ID
  pid: Option<i32>,
  // span id
  rid: Option<i32>,
  //
  at: Option<String>,
}

/// create comment
/// No user is created if the user is anonymous
#[post("/api/comment")]
async fn create_comment(
  data: Data<AppState>,
  _query: Query<CreateCommentQuery>,
  body: Json<CreateCommentBody>,
) -> impl Responder {
  let Json(CreateCommentBody {
    comment,
    link,
    mail,
    nick,
    ua,
    url,
    pid,
    rid,
    at: _,
  }) = body;
  let conn = &data.conn;
  let user = WlUsers::find()
    .filter(wl_users::Column::Email.eq(mail.clone()))
    .filter(wl_users::Column::DisplayName.eq(nick.clone()))
    .one(conn)
    .await
    .unwrap();
  let html_output = render_markdown(&comment);
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

  match WlComment::insert(model).exec(conn).await {
    Ok(result) => {
      let comment = WlComment::find_by_id(result.last_insert_id)
        .one(conn)
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
          "avatar": ANONYMOUS_AVATAR,
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
        HttpResponse::Ok().json(ResponseModel {
          errno: 0,
          errmsg: "".to_string(),
          data,
        })
      } else {
        let user = WlUsers::find()
          .filter(wl_users::Column::DisplayName.eq(nick.clone()))
          .filter(wl_users::Column::Email.eq(mail.clone()))
          .one(conn)
          .await
          .unwrap()
          .unwrap();
        let avatar = if let Some(prefix) = extract_email_prefix(mail.clone()) {
          format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
        } else {
          ANONYMOUS_AVATAR.to_string()
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
        HttpResponse::Ok().json(ResponseModel {
          errno: 0,
          errmsg: "".to_string(),
          data,
        })
      }
    }
    Err(err) => HttpResponse::Ok().json(json! ({
      "errmsg": err.to_string(),
      "errno": 1000,
    })),
  }
}

#[derive(Debug, Deserialize)]
struct GetArticleQuery {
  path: String,
  r#type: String,
  lang: String,
}

#[get("/api/article")]
async fn get_article(data: Data<AppState>, query: Query<GetArticleQuery>) -> impl Responder {
  let conn = &data.conn;
  let Query(GetArticleQuery {
    path,
    r#type,
    lang: _,
  }) = query;

  if r#type == "time" {
    let mut data = vec![];
    for path in path.split(',') {
      let model = WlCounter::find()
        .filter(wl_counter::Column::Url.eq(path))
        .one(conn)
        .await
        .unwrap();
      if let Some(model) = model {
        data.push(json!({
          "time": model.time.unwrap(),
        }));
      } else {
        data.push(json!({ "time": 0 }))
      }
    }
    HttpResponse::Ok().json(ResponseModel {
      data,
      errmsg: "".to_string(),
      errno: 0,
    })
  } else {
    let mut data = vec![];
    let model = WlCounter::find()
      .filter(wl_counter::Column::Url.eq(path))
      .one(conn)
      .await
      .unwrap();

    if let Some(model) = model {
      data.push(json!({
        "reaction0": model.reaction0,
        "reaction1": model.reaction1,
        "reaction2": model.reaction2,
        "reaction3": model.reaction3,
        "reaction4": model.reaction4,
        "reaction5": model.reaction5,
      }));
    } else {
      data.push(json!({
        "reaction0": 0,
        "reaction1": 1,
        "reaction2": 2,
        "reaction3": 3,
        "reaction4": 4,
        "reaction5": 5,
      }));
    }
    HttpResponse::Ok().json(ResponseModel {
      data,
      errmsg: "".to_string(),
      errno: 0,
    })
  }
}

#[derive(Deserialize)]
struct ApiArticleBody {
  action: Option<String>,
  path: String,
  r#type: String,
}

#[derive(Debug, Deserialize)]
struct ApiArticleQuery {
  lang: String,
}

async fn has_counter(url: String, conn: &DatabaseConnection) -> bool {
  let res = WlCounter::find()
    .filter(wl_counter::Column::Url.eq(url))
    .one(conn)
    .await
    .unwrap();
  res.is_some()
}

#[post("/api/article")]
async fn update_article(
  data: Data<AppState>,
  _query: Query<ApiArticleQuery>,
  body: Json<ApiArticleBody>,
) -> impl Responder {
  let conn = &data.conn;
  let Json(ApiArticleBody {
    action,
    path,
    r#type,
  }) = body;
  let mut data: Vec<wl_counter::Model> = vec![];
  if r#type == "time" {
    if has_counter(path.clone(), conn).await {
      let model = WlCounter::find()
        .filter(wl_counter::Column::Url.eq(path.clone()))
        .one(conn)
        .await
        .unwrap()
        .unwrap();
      let model = wl_counter::ActiveModel {
        id: Set(model.id),
        time: Set(Some(model.time.unwrap() + 1)),
        ..Default::default()
      };
      data.push(WlCounter::update(model).exec(conn).await.unwrap());
    } else {
      let model = wl_counter::ActiveModel {
        url: Set(path.clone()),
        time: Set(Some(1)),
        ..Default::default()
      };
      WlCounter::insert(model).exec(conn).await.unwrap();
      data.push(
        WlCounter::find()
          .filter(wl_counter::Column::Url.eq(path.clone()))
          .one(conn)
          .await
          .unwrap()
          .unwrap(),
      )
    }
  } else {
    fn set_reaction_value(
      mut model: wl_counter::ActiveModel,
      counter: wl_counter::Model,
      reaction: &str,
      action: Option<String>,
    ) -> wl_counter::ActiveModel {
      match reaction {
        "reaction0" => {
          model.reaction0 = if counter.reaction0.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction0.unwrap() + 1))
          } else {
            Set(Some(counter.reaction0.unwrap() - 1))
          }
        }
        "reaction1" => {
          model.reaction1 = if counter.reaction1.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction1.unwrap() + 1))
          } else {
            Set(Some(counter.reaction1.unwrap() - 1))
          }
        }
        "reaction2" => {
          model.reaction2 = if counter.reaction2.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction2.unwrap() + 1))
          } else {
            Set(Some(counter.reaction2.unwrap() - 1))
          }
        }
        "reaction3" => {
          model.reaction3 = if counter.reaction3.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction3.unwrap() + 1))
          } else {
            Set(Some(counter.reaction3.unwrap() - 1))
          }
        }
        "reaction4" => {
          model.reaction4 = if counter.reaction4.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction4.unwrap() + 1))
          } else {
            Set(Some(counter.reaction4.unwrap() - 1))
          }
        }
        "reaction5" => {
          model.reaction5 = if counter.reaction5.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction5.unwrap() + 1))
          } else {
            Set(Some(counter.reaction5.unwrap() - 1))
          }
        }
        "reaction6" => {
          model.reaction6 = if counter.reaction6.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction6.unwrap() + 1))
          } else {
            Set(Some(counter.reaction6.unwrap() - 1))
          }
        }
        "reaction7" => {
          model.reaction7 = if counter.reaction7.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction7.unwrap() + 1))
          } else {
            Set(Some(counter.reaction7.unwrap() - 1))
          }
        }
        "reaction8" => {
          model.reaction8 = if counter.reaction8.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction8.unwrap() + 1))
          } else {
            Set(Some(counter.reaction8.unwrap() - 1))
          }
        }
        _ => {}
      }
      model
    }
    if has_counter(path.clone(), conn).await {
      let counter = WlCounter::find()
        .filter(wl_counter::Column::Url.eq(path.clone()))
        .one(conn)
        .await
        .unwrap()
        .unwrap();
      let mut model = wl_counter::ActiveModel {
        id: Set(counter.id),
        ..Default::default()
      };
      model = set_reaction_value(model, counter, &r#type, action);
      data.push(WlCounter::update(model).exec(conn).await.unwrap());
    } else {
      let mut model = wl_counter::ActiveModel {
        ..Default::default()
      };
      match r#type.as_str() {
        "reaction0" => model.reaction0 = Set(Some(0)),
        "reaction1" => model.reaction1 = Set(Some(0)),
        "reaction2" => model.reaction2 = Set(Some(0)),
        "reaction3" => model.reaction3 = Set(Some(0)),
        "reaction4" => model.reaction4 = Set(Some(0)),
        "reaction5" => model.reaction5 = Set(Some(0)),
        "reaction6" => model.reaction6 = Set(Some(0)),
        "reaction7" => model.reaction7 = Set(Some(0)),
        "reaction8" => model.reaction8 = Set(Some(0)),
        _ => {}
      }
      WlCounter::insert(model).exec(conn).await.unwrap();
      data.push(
        WlCounter::find()
          .filter(wl_counter::Column::Url.eq(path.clone()))
          .one(conn)
          .await
          .unwrap()
          .unwrap(),
      )
    }
  }

  HttpResponse::Ok().json(ResponseModel {
    data,
    errmsg: "".to_string(),
    errno: 0,
  })
}

/// delete comment
#[delete("/api/comment/{id}")]
async fn delete_comment(data: Data<AppState>, path: Path<u32>) -> impl Responder {
  let conn = &data.conn;
  let id = path.into_inner();
  let _ = WlComment::delete_by_id(id).exec(conn).await;
  HttpResponse::Ok().json(json! ({
    "data": "",
    "errmsg": "".to_string(),
    "errno": 0,
  }))
}

#[derive(Deserialize)]
struct UpdateCommentBody {
  status: Option<String>,
  like: Option<bool>,
  comment: Option<String>,
  link: Option<String>,
  mail: Option<String>,
  nick: Option<String>,
  ua: Option<String>,
  url: Option<String>,
}

/// update comment
#[put("/api/comment/{id}")]
async fn update_comment(
  data: Data<AppState>,
  path: Path<u32>,
  body: Json<UpdateCommentBody>,
) -> impl Responder {
  let conn = &data.conn;
  let actix_web::web::Json(UpdateCommentBody {
    status,
    like,
    comment,
    link: _,
    mail: _,
    nick: _,
    ua,
    url: _,
  }) = body;
  let updated_at: chrono::DateTime<Utc> = Utc::now();
  let id: u32 = path.into_inner();
  let new_comment;
  if let Some(like) = like {
    let comment = wl_comment::Entity::find_by_id(id)
      .one(conn)
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
    new_comment = WlComment::update(model).exec(conn).await.unwrap();
  } else if let Some(status) = status {
    let model = wl_comment::ActiveModel {
      id: Set(id),
      status: Set(status),
      updated_at: Set(Some(updated_at)),
      ..Default::default()
    };
    new_comment = WlComment::update(model).exec(conn).await.unwrap();
  } else {
    let model = wl_comment::ActiveModel {
      id: Set(id),
      comment: Set(comment),
      ua: Set(ua),
      ..Default::default()
    };
    new_comment = WlComment::update(model).exec(conn).await.unwrap();
  }

  let (browser, os) = ua::parse(new_comment.ua.unwrap());
  let like = new_comment.like.unwrap_or(0);
  let time = new_comment.created_at.unwrap().timestamp_millis();
  let pid = new_comment.pid;
  let rid = new_comment.rid;
  let html_output = render_markdown(new_comment.comment.clone().unwrap().as_str());

  if is_anonymous(id, conn).await {
    let data = json!({
      "addr":"",
      "avatar": ANONYMOUS_AVATAR.to_string(),
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
    HttpResponse::Ok().json(json! ({
      "data": data,
      "errmsg": "".to_string(),
      "errno": 0,
    }))
  } else {
    let user = get_user(UserQueryBy::Id(new_comment.user_id.unwrap() as u32), conn).await;
    let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
      format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
    } else {
      ANONYMOUS_AVATAR.to_string()
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
    HttpResponse::Ok().json(json! ({
      "data": data,
      "errmsg": "".to_string(),
      "errno": 0,
    }))
  }
}

async fn is_first_user(conn: &DatabaseConnection) -> bool {
  let users = WlUsers::find().all(conn).await.unwrap();
  users.is_empty()
}

async fn has_user(query_by: UserQueryBy, conn: &DatabaseConnection) -> bool {
  let mut query = WlUsers::find();
  match query_by {
    UserQueryBy::Id(id) => query = query.filter(wl_users::Column::Id.eq(id)),
    UserQueryBy::Email(email) => query = query.filter(wl_users::Column::Email.eq(email)),
  }
  let res = query.one(conn).await.unwrap();
  res.is_some()
}

#[derive(Deserialize)]
struct UserRegisterBody {
  display_name: String,
  email: String,
  password: String,
  url: String,
}

#[post("/api/user")]
async fn user_register(data: Data<AppState>, body: Json<UserRegisterBody>) -> impl Responder {
  let conn = &data.conn;
  let Json(UserRegisterBody {
    display_name,
    email,
    password,
    url,
  }) = body;
  if has_user(UserQueryBy::Email(email.clone()), conn).await {
    return HttpResponse::Ok().json(json!({
     "errmsg": "用户已存在".to_string(),
     "errno": 1000,
    }));
  }
  let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
  let mut model = wl_users::ActiveModel {
    display_name: Set(display_name),
    email: Set(email),
    url: Set(Some(url)),
    password: Set(hashed),
    ..Default::default()
  };
  if is_first_user(conn).await {
    model.r#type = Set("administrator".to_string());
  } else {
    model.r#type = Set("guest".to_string());
  }
  let _ = WlUsers::insert(model).exec(conn).await.unwrap();
  HttpResponse::Ok().json(json! ({
    "data": {
      "verify": true
    },
    "errmsg": "".to_string(),
    "errno": 0,
  }))
}

/// 获取用户列表
#[get("/api/user")]
async fn get_user_list(data: Data<AppState>, _body: Json<UserRegisterBody>) -> impl Responder {
  let _conn = &data.conn;
  HttpResponse::Ok().json(json! ({
    "data": json!({
      "data": [],
      "page": 1,
      "pageSize": 10,
      "totalPages": 1
    }),
    "errmsg": "".to_string(),
    "errno": 0,
  }))
}

#[derive(Debug, Deserialize)]
struct VerificationQuery {
  token: String,
  email: String,
}

/// 未实现
#[post("/verification")]
async fn verification(data: Data<AppState>, query: Query<VerificationQuery>) -> impl Responder {
  let conn = &data.conn;
  let email = &query.email;
  let user = WlUsers::find()
    .filter(wl_users::Column::Email.eq(email))
    .one(conn)
    .await
    .unwrap();

  if let Some(_) = user {
    // 用户已注册
    HttpResponse::Ok().json(json! ({
      "errmsg": "用户已注册".to_string(),
      "errno": 1000,
    }))
  } else {
    // 用户未注册
    HttpResponse::Ok().json(json! ({
      "errmsg": "用户已注册".to_string(),
      "errno": 1000,
    }))
  }
}

#[derive(Deserialize)]
struct ApiTokenBody {
  code: String,
  email: String,
  password: String,
}

#[post("/api/token")]
async fn user_login(data: Data<AppState>, body: Json<ApiTokenBody>) -> impl Responder {
  let conn = &data.conn;
  let user = WlUsers::find()
    .filter(wl_users::Column::Email.eq(body.email.clone()))
    .one(conn)
    .await
    .unwrap();
  match user {
    Some(user) => {
      let result = bcrypt::verify(body.password.clone(), user.password.as_str());
      match result {
        Ok(result) => {
          if !result {
            return HttpResponse::Ok().json(json! ({
              "errno": 1000,
            }));
          }
        }
        Err(_) => println!("验证错误"),
      }

      let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
        format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
      } else {
        ANONYMOUS_AVATAR.to_string()
      };

      let payload = token::Claims::new(user.email.clone(), 7);
      let token = token::sign(payload, "waline".to_string());
      let mailMd5 = format!("{:x}", md5::compute(user.email.clone()));
      let data = json!({
        "display_name": user.display_name,
        "email": user.email,
        "password": null,
        "type": user.r#type,
        "label": user.label,
        "url": user.url,
        "avatar": avatar,
        "github": user.github,
        "twitter": user.twitter,
        "facebook": user.facebook,
        "google": user.google,
        "weibo": user.weibo,
        "qq": user.qq,
        "2fa": user.two_factor_auth,
        "createdAt": user.created_at,
        "updatedAt": user.updated_at,
        "objectId": user.id,
        "mailMd5": mailMd5,
        "token": token
      });
      HttpResponse::Ok().json(json! ({
        "data": data,
        "errmsg": "".to_string(),
        "errno": 0,
      }))
    }
    None => HttpResponse::Ok().json(json! ({
      "errno": 1000,
    })),
  }
}

fn extract_token_from_header(header_value: &Option<&HeaderValue>) -> Option<String> {
  header_value.and_then(|value| {
    let value = value.to_str().ok()?;
    if value.starts_with("Bearer ") {
      Some(value.split(' ').nth(1)?.to_string())
    } else {
      None
    }
  })
}

/// 获取登录用户信息
#[get("/api/token")]
async fn get_login_user_info(req: HttpRequest, data: Data<AppState>) -> impl Responder {
  if let Some(token) = extract_token_from_header(&req.headers().get(AUTHORIZATION)) {
    println!(">>> token = {}", token)
    // 使用 token 进行后续的处理
  } else {
    // 如果 token 提取失败,则返回适当的错误响应
  }

  let header_value = req.headers().get(AUTHORIZATION);
  match header_value {
    Some(value) => {
      let mut value = value.to_str().unwrap();
      if value.starts_with("Bearer ") {
        value = value.split(' ').collect::<Vec<&str>>()[1];
      }
      let res = token::verify(value.to_string(), "waline".to_string());
      match res {
        Ok(value) => {
          println!(">>> {}", value);
          let conn = &data.conn;
          let user = WlUsers::find()
            .filter(wl_users::Column::Email.eq(value))
            .one(conn)
            .await
            .unwrap();
          match user {
            Some(user) => {
              let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
                format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
              } else {
                ANONYMOUS_AVATAR.to_string()
              };
              let mailMd5 = format!("{:x}", md5::compute(user.email.clone()));
              let data = json!({
                "display_name": user.display_name,
                "email": user.email,
                "type": user.r#type,
                "label": user.label,
                "url": user.url,
                "avatar": avatar,
                "github": user.github,
                "twitter": user.twitter,
                "facebook": user.facebook,
                "google": user.google,
                "weibo": user.weibo,
                "qq": user.qq,
                "2fa": user.two_factor_auth,
                "objectId": user.id,
                "mailMd5": mailMd5,
              });
              HttpResponse::Ok().json(json! ({
                "errno": 0,
                "errmsg": "",
                "data": data,
              }))
            }
            None => HttpResponse::Ok().json(json! ({
              "errno": 1000,
              "errmsg": "no this user",
            })),
          }
        }
        Err(err) => HttpResponse::Ok().json(json! ({
          "errno": 1000,
          "errmsg": err,
        })),
      }
    }
    None => HttpResponse::Ok().json(json! ({
      "errno":1000,
    })),
  }
}

#[delete("/api/token")]
async fn user_logout() -> impl Responder {
  HttpResponse::Ok().json(json! ({
    "errno": 0,
    "errmsg": "",
  }))
}

#[derive(Deserialize)]
struct SetUserProfileBody {
  display_name: Option<String>,
  label: Option<String>,
  url: Option<String>,
  password: Option<String>,
}

/// set user profile
#[put("/api/user")]
async fn set_user_profile(data: Data<AppState>, body: Json<SetUserProfileBody>) -> impl Responder {
  let conn = &data.conn;
  let Json(SetUserProfileBody {
    display_name,
    label,
    url,
    password: _,
  }) = body;
  // token::verify(value.to_string(), "waline".to_string());
  let model = wl_users::ActiveModel {
    display_name: Set(display_name.unwrap_or("".to_string())),
    label: Set(label),
    url: Set(url),
    ..Default::default()
  };
  let _ = WlUsers::update(model).exec(conn).await.unwrap();
  HttpResponse::Ok().json(json! ({
    "errno": 0,
    "errmsg": "",
  }))
}

#[derive(Deserialize)]
struct SetUserTypeBody {
  r#type: String,
}

/// 设置用户类型（未实现）
#[put("/api/token/{user_id}")]
async fn set_user_type(
  data: Data<AppState>,
  path: Path<i32>,
  body: Json<SetUserTypeBody>,
) -> impl Responder {
  let _conn = &data.conn;
  let _user_id = path.into_inner();
  let _type = &body.r#type;
  HttpResponse::Ok().json(json!({
    "errno": 0,
    "errmsg": "",
  }))
}

#[derive(Deserialize)]
struct Set2fa {
  code: String,
  secret: String,
}

/// 设置 2fa（未实现）
#[post("/api/token/2fa")]
async fn set_2fa(_data: Data<AppState>, _body: Json<Set2fa>) -> impl Responder {
  HttpResponse::Ok().json(json!({
    "errno": 1000,
    "errmsg": "二部验证失败"
  }))
}

#[derive(Deserialize)]
struct Get2faQuery {
  lang: String,
  email: Option<String>,
}

#[get("/api/token/2fa")]
async fn get_2fa(data: Data<AppState>, query: Query<Get2faQuery>) -> impl Responder {
  let conn = &data.conn;
  let email = &query.email;
  match email {
    Some(email) => {
      let res = WlUsers::find()
        .filter(wl_users::Column::Email.eq(email))
        .filter(wl_users::Column::TwoFactorAuth.is_not_null())
        .filter(wl_users::Column::TwoFactorAuth.ne(""))
        .one(conn)
        .await
        .unwrap();
      match res {
        Some(res) => {
          println!(">>> {:?}", res.two_factor_auth);
          HttpResponse::Ok().json(json!({
            "errno": 0,
            "errmsg": "",
            "data": {
              "enable": true
            }
          }))
        }
        None => HttpResponse::Ok().json(json!({
          "errno": 0,
          "errmsg": "",
          "data": {
            "enable": false
          }
        })),
      }
    }
    None => HttpResponse::Ok().json(json!({
      "errno": 0,
      "errmsg": "",
    })),
  }
}

struct AdminQuery {
  lng: Option<String>,
  token: Option<String>,
  redirect: Option<String>,
}

#[derive(Deserialize)]
struct UIProfilePageQuery {
  lng: Option<String>,
  token: Option<String>,
}

async fn ui_profile_page(_query: Query<UIProfilePageQuery>) -> HttpResponse {
  let SITE_URL = env::var("SITE_URL").ok().unwrap_or("''".to_string());
  let SITE_NAME = env::var("SITE_NAME").ok().unwrap_or("''".to_string());
  let recaptchaV3Key = env::var("recaptchaV3Key")
    .ok()
    .unwrap_or("undefined".to_string());
  let turnstileKey = env::var("turnstileKey")
    .ok()
    .unwrap_or("undefined".to_string());
  let serverURL = env::var("serverURL").ok().unwrap_or("".to_string());
  HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(format!(
      r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Waline Management System</title>
    <meta name="viewport" content="width=device-width,initial-scale=1">
  </head>
  <body>
    <script>
    window.SITE_URL = {SITE_URL};
    window.SITE_NAME = {SITE_NAME};
    window.recaptchaV3Key = {recaptchaV3Key};
    window.turnstileKey = {turnstileKey};
    window.serverURL = '{serverURL}/api/';
    </script>
    <script src="//unpkg.com/@waline/admin"></script>
  </body>
</html>
    "#
    ))
}

#[derive(Deserialize)]
struct UiLoginPageQeury {
  redirect: Option<String>,
}

async fn ui_login_page(query: Query<UiLoginPageQeury>) -> HttpResponse {
  match query.redirect.clone() {
    Some(_path) => {
      let SITE_URL = env::var("SITE_URL").ok().unwrap_or("''".to_string());
      let SITE_NAME = env::var("SITE_NAME").ok().unwrap_or("''".to_string());
      let recaptchaV3Key = env::var("recaptchaV3Key")
        .ok()
        .unwrap_or("undefined".to_string());
      let turnstileKey = env::var("turnstileKey")
        .ok()
        .unwrap_or("undefined".to_string());
      let serverURL = env::var("serverURL").ok().unwrap_or("".to_string());
      HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
          r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Waline Management System</title>
    <meta name="viewport" content="width=device-width,initial-scale=1">
  </head>
  <body>
    <script>
    window.SITE_URL = {SITE_URL};
    window.SITE_NAME = {SITE_NAME};
    window.recaptchaV3Key = {recaptchaV3Key};
    window.turnstileKey = {turnstileKey};
    window.serverURL = '{serverURL}/api/';
    </script>
    <script src="//unpkg.com/@waline/admin"></script>
  </body>
</html>
    "#
        ))
    }
    None => {
      let SITE_URL = env::var("SITE_URL").ok().unwrap_or("''".to_string());
      let SITE_NAME = env::var("SITE_NAME").ok().unwrap_or("''".to_string());
      let recaptchaV3Key = env::var("recaptchaV3Key")
        .ok()
        .unwrap_or("undefined".to_string());
      let turnstileKey = env::var("turnstileKey")
        .ok()
        .unwrap_or("undefined".to_string());
      let serverURL = env::var("serverURL").ok().unwrap_or("".to_string());
      HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
          r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Waline Management System</title>
    <meta name="viewport" content="width=device-width,initial-scale=1">
  </head>
  <body>
    <script>
    window.SITE_URL = {SITE_URL};
    window.SITE_NAME = {SITE_NAME};
    window.recaptchaV3Key = {recaptchaV3Key};
    window.turnstileKey = {turnstileKey};
    window.serverURL = '{serverURL}/api/';
    </script>
    <script src="//unpkg.com/@waline/admin"></script>
  </body>
</html>
    "#
        ))
    }
  }
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(get_comment);
  cfg.service(create_comment);
  cfg.service(delete_comment);
  cfg.service(update_comment);
  cfg.service(get_article);
  cfg.service(update_article);
  cfg.service(user_register);
  cfg.service(user_login);
  cfg.service(user_logout);
  cfg.service(get_login_user_info);
  cfg.service(get_user_list);
  cfg.service(get_2fa);
  cfg.service(set_2fa);
  cfg.service(set_user_profile);
  cfg.service(resource("/ui/profile").to(ui_profile_page));
  cfg.service(resource("/ui/login").to(ui_login_page));
}
