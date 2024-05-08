#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{
  entities::{prelude::*, *},
  helpers::{token, ua},
  AppState,
};
use actix_web::{
  delete, get,
  http::header::{ContentType, AUTHORIZATION},
  post, put,
  web::{resource, Data, Json, Path, Query, ServiceConfig},
  HttpRequest, HttpResponse, Responder,
};
use chrono::Utc;
use pulldown_cmark::{self, Event};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Serialize, Deserialize)]
struct ResponseModel<T> {
  data: T,
  errmsg: String,
  errno: i8,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct GetCommentQuery {
  path: String,
  pageSize: i32,
  page: i32,
  lang: String,
  sortBy: String,
}

#[derive(Serialize, Debug)]
struct DataEntry {
  status: String,
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

/// get comment
#[get("/api/comment")]
async fn get_comment(query: Query<GetCommentQuery>, data: Data<AppState>) -> impl Responder {
  let conn = &data.conn;
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

  fn extract_email_prefix(email: String) -> Option<String> {
    let mut res = email.split('@');
    res.next().map(|prefix| prefix.to_string())
  }

  for model in parrent_comments {
    let status = model.status;
    let link = model.link;
    let mail = model.mail.clone();
    let nick = model.nick;
    let user_id = model.user_id;
    let objectId = model.id;
    let orig = model.comment.clone();
    let ip = model.ip;
    let pid = model.pid;
    let rid = model.rid;
    let markdown_input = model.comment.clone().unwrap().as_str().to_owned();
    let parser = pulldown_cmark::Parser::new(markdown_input.as_str());
    let parser = parser.map(|event| match event {
      Event::SoftBreak => Event::HardBreak,
      _ => event,
    });
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    let comment = Some(html_output);
    let avatar = if let Some(_) = model.user_id {
      let prefix = extract_email_prefix(mail.clone().unwrap()).unwrap();
      format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
    } else {
      "https://seccdn.libravatar.org/avatar/d41d8cd98f00b204e9800998ecf8427e".to_string()
    };
    let time = model.created_at.unwrap().timestamp_millis();
    // let time = DateTime::parse_from_str(model.created_at, "%Y-%m-%d %H:%M:%S");
    let (browser, os) = ua::parse(model.ua.unwrap());
    let browser = browser;
    let os = os;
    let user: wl_users::Model;
    let level = 0;
    let mut label = None;
    let mut r#type = None;
    match user_id {
      Some(user_id) => {
        user = WlUsers::find()
          .filter(wl_users::Column::Id.eq(user_id))
          .one(conn)
          .await
          .unwrap()
          .unwrap();
        label = user.label;
        r#type = Some(user.r#type);
      }
      None => {}
    }
    let mut children = vec![];
    let subcomments = wl_comment::Entity::find()
      .filter(wl_comment::Column::Url.contains(query.path.clone()))
      .filter(wl_comment::Column::Pid.eq(model.id))
      .order_by(wl_comment::Column::InsertedAt, Order::Asc)
      .all(conn)
      .await
      .unwrap();

    for subcomment in subcomments {
      count += 1;
      let status = subcomment.status;
      let link = subcomment.link;
      let mail = subcomment.mail.clone();
      let nick: Option<String> = subcomment.nick;
      let user_id = subcomment.user_id;
      let objectId = subcomment.id;
      let orig = subcomment.comment.clone();
      let ip = subcomment.ip;
      let pid = subcomment.pid;
      let rid = subcomment.rid;
      let markdown_input = model.comment.clone().unwrap().as_str().to_owned();
      let parser = pulldown_cmark::Parser::new(markdown_input.as_str());
      let parser = parser.map(|event| match event {
        Event::SoftBreak => Event::HardBreak,
        _ => event,
      });
      let mut html_output = String::new();
      pulldown_cmark::html::push_html(&mut html_output, parser);
      let comment = Some(html_output);
      let avatar = if let Some(_) = model.user_id {
        let prefix = extract_email_prefix(mail.clone().unwrap()).unwrap();
        format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
      } else {
        "https://seccdn.libravatar.org/avatar/d41d8cd98f00b204e9800998ecf8427e".to_string()
      };
      let time = subcomment.created_at.unwrap().timestamp_millis();
      // let time = DateTime::parse_from_str(model.created_at, "%Y-%m-%d %H:%M:%S");
      let (browser, os) = ua::parse(subcomment.ua.unwrap());
      let browser = browser;
      let os = os;
      let user: wl_users::Model;
      let r#type = "".to_string();
      let level = 0;
      let mut label = None;
      let mut r#type = None;
      match user_id {
        Some(user_id) => {
          user = WlUsers::find()
            .filter(wl_users::Column::Id.eq(user_id))
            .one(conn)
            .await
            .unwrap()
            .unwrap();
          label = user.label;
          r#type = Some(user.r#type);
        }
        None => {}
      }
      let subcomment_children = vec![];
      let data_entry = DataEntry {
        status,
        link,
        mail,
        nick,
        user_id,
        browser,
        os,
        r#type,
        objectId,
        ip,
        orig,
        time,
        pid,
        rid,
        comment,
        avatar,
        level,
        label,
        children: subcomment_children,
      };
      children.push(data_entry)
    }

    let data_entry = DataEntry {
      status,
      link,
      mail,
      nick,
      user_id,
      browser,
      os,
      r#type,
      objectId,
      ip,
      orig,
      time,
      pid,
      rid,
      comment,
      avatar,
      level,
      label,
      children,
    };
    comments.push(data_entry)
  }
  let data =
    json!({ "count": count, "data": comments, "page": 1, "pageSize": 10, "totalPages": 0 });
  HttpResponse::Ok().json(ResponseModel {
    errno: 0,
    errmsg: "".to_string(),
    data,
  })
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct CreateCommentQuery {
  lang: String,
}

#[allow(dead_code)]
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
  let conn = &data.conn;
  let user = WlUsers::find()
    .filter(wl_users::Column::Email.eq(body.mail.clone()))
    .filter(wl_users::Column::DisplayName.eq(body.nick.clone()))
    .one(conn)
    .await
    .unwrap();
  let model;
  let markdown_input = body.comment.clone().as_str().to_owned();
  let parser = pulldown_cmark::Parser::new(markdown_input.as_str());
  let parser = parser.map(|event| match event {
    Event::SoftBreak => Event::HardBreak,
    _ => event,
  });
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);

  match user {
    Some(user) => {
      let created_at: chrono::DateTime<Utc> = Utc::now();
      model = wl_comment::ActiveModel {
        user_id: Set(Some(user.id as i32)),
        comment: Set(Some(html_output)),
        link: Set(Some(body.link.clone())),
        mail: Set(Some(body.mail.clone())),
        nick: Set(Some(body.nick.clone())),
        ua: Set(Some(body.ua.clone())),
        url: Set(Some(body.url.clone())),
        status: Set("approved".to_string()),
        pid: Set(body.pid),
        rid: Set(body.rid),
        inserted_at: Set(Some(created_at)),
        created_at: Set(Some(created_at)),
        updated_at: Set(Some(created_at)),
        ..Default::default()
      };
    }
    None => {
      let created_at: chrono::DateTime<Utc> = Utc::now();
      model = wl_comment::ActiveModel {
        comment: Set(Some(html_output)),
        link: Set(Some(body.link.clone())),
        mail: Set(Some(body.mail.clone())),
        nick: Set(Some(body.nick.clone())),
        ua: Set(Some(body.ua.clone())),
        url: Set(Some(body.url.clone())),
        status: Set("approved".to_string()),
        pid: Set(body.pid),
        rid: Set(body.rid),
        inserted_at: Set(Some(created_at)),
        created_at: Set(Some(created_at)),
        updated_at: Set(Some(created_at)),
        ..Default::default()
      };
    }
  }

  let mut data = json!({});
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
      if body.nick == "匿名" {
        data = json!({
          "addr":"",
          "avatar": "https://seccdn.libravatar.org/avatar/d41d8cd98f00b204e9800998ecf8427e",
          "browser": browser,
          "comment": comment.comment,
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
          .filter(wl_users::Column::DisplayName.eq(body.nick.clone()))
          .filter(wl_users::Column::Email.eq(body.mail.clone()))
          .one(conn)
          .await
          .unwrap()
          .unwrap();
        println!(">>>{:?}", user);
        fn extract_email_prefix(email: String) -> Option<String> {
          let mut res = email.split('@');
          res.next().map(|prefix| prefix.to_string())
        }
        let avatar = if let Some(prefix) = extract_email_prefix(body.mail.clone()) {
          format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
        } else {
          "https://seccdn.libravatar.org/avatar/d41d8cd98f00b204e9800998ecf8427e".to_string()
        };

        data = json!({
          "addr":"",
          "avatar": avatar,
          "browser": browser,
          "comment": comment.comment,
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
    Err(err) => panic!("{err}"),
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
  #[derive(Serialize)]
  struct DataEntry {
    time: i32,
  }
  let mut data: Vec<DataEntry> = vec![];
  for path in query.path.split(",") {
    let res = WlCounter::find()
      .filter(wl_counter::Column::Url.contains(path))
      .one(conn)
      .await
      .unwrap();
    let model = res.unwrap();
    data.push(DataEntry {
      time: model.time.unwrap(),
    });
  }
  HttpResponse::Ok().json(ResponseModel {
    data,
    errmsg: "".to_string(),
    errno: 0,
  })
}

#[derive(Debug, Deserialize)]
struct ApiArticleBody {
  action: String,
  path: String,
  r#type: String,
}

#[derive(Debug, Deserialize)]
struct ApiArticleQuery {
  lang: String,
}

#[post("/api/article")]
async fn update_article(
  data: Data<AppState>,
  _query: Query<ApiArticleQuery>,
  body: Json<ApiArticleBody>,
) -> impl Responder {
  let conn = &data.conn;

  let one = &WlCounter::find()
    .filter(wl_counter::Column::Url.contains(body.path.clone()))
    .all(conn)
    .await
    .unwrap()[0];

  let model = wl_counter::ActiveModel {
    id: Set(one.id),
    time: Set(Some(one.time.unwrap() + 1)),
    ..Default::default()
  };

  WlCounter::update(model).exec(conn).await.unwrap();

  let data = WlCounter::find()
    .filter(wl_counter::Column::Url.contains(body.path.clone()))
    .all(conn)
    .await
    .unwrap();

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
  WlComment::delete_by_id(id).exec(conn).await;
  HttpResponse::Ok().json(json! ({
    "data": "",
    "errmsg": "".to_string(),
    "errno": 0,
  }))
}

#[derive(Deserialize)]
struct UpdateCommentBody {
  status: Option<String>,
  comment: Option<String>,
  link: Option<String>,
  mail: Option<String>,
  nick: Option<String>,
  ua: Option<String>,
  url: Option<String>,
}

/// 更新评论（未实现）
#[put("/api/comment/{id}")]
async fn update_comment(
  _data: Data<AppState>,
  _path: Path<u32>,
  _body: Json<UpdateCommentBody>,
) -> impl Responder {
  HttpResponse::Ok().json(json! ({
    "data": "",
    "errmsg": "".to_string(),
    "errno": 0,
  }))
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct UserRegisterBody {
  display_name: String,
  email: String,
  password: String,
  url: String,
  lang: String,
}

#[post("/api/user")]
async fn user_register(data: Data<AppState>, _body: Json<UserRegisterBody>) -> impl Responder {
  let _conn = &data.conn;
  HttpResponse::Ok().json(json! ({
    "data": "",
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
      fn extract_email_prefix(email: String) -> Option<String> {
        let mut res = email.split('@');
        res.next().map(|prefix| prefix.to_string())
      }
      let avatar = if let Some(prefix) = extract_email_prefix(user.email.clone()) {
        format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
      } else {
        "https://seccdn.libravatar.org/avatar/d41d8cd98f00b204e9800998ecf8427e".to_string()
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
        "2fa": null,
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

/// 获取登录用户信息
#[get("/api/token")]
async fn get_login_user_info(req: HttpRequest, data: Data<AppState>) -> impl Responder {
  let header_value = req.headers().get(AUTHORIZATION);
  match header_value {
    Some(value) => {
      let mut value = value.to_str().unwrap();
      if value.starts_with("Bearer ") {
        value = value.split(" ").collect::<Vec<&str>>()[1];
      }
      let res = token::verify(value.to_string(), "waline".to_string());
      match res {
        Ok(value) => {
          println!("{}", value)
        }
        Err(_) => panic!(""),
      }
      let _conn = &data.conn;
      HttpResponse::Ok().json(json! ({
        "errno": 0,
        "errmsg": "",
        "data": [],
      }))
    }
    None => HttpResponse::Ok().json(json! ({
      "errno":1000,
    })),
  }

  // let user = WlUsers::find();
  //   .filter(wl_users::Column::Email.eq(body.email.clone()))
  //   .one(conn)
  //   .await
  //   .unwrap();

  // let data = json!({
  //   "display_name": user.display_name,
  //   "email": user.email,
  //   "password": null,
  //   "type": user.r#type,
  //   "label": user.label,
  //   "url": user.url,
  //   "avatar": avatar,
  //   "github": user.github,
  //   "twitter": user.twitter,
  //   "facebook": user.facebook,
  //   "google": user.google,
  //   "weibo": user.weibo,
  //   "qq": user.qq,
  //   "2fa": null,
  //   "createdAt": user.created_at,
  //   "updatedAt": user.updated_at,
  //   "objectId": user.id,
  //   "mailMd5": mailMd5,
  //   "token": token
  // });
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

/// 更新用户档案（未实现）
#[put("/api/user")]
async fn set_user_profile(
  _data: Data<AppState>,
  _bodyy: Json<SetUserProfileBody>,
) -> impl Responder {
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
        .one(conn)
        .await
        .unwrap();
      match res {
        Some(_) => HttpResponse::Ok().json(json!({
          "errno": 0,
          "errmsg": "",
          "data": {
            "enable": true
          }
        })),
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
    Some(path) => HttpResponse::Found()
      .insert_header(("Location", path))
      .finish(),
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
