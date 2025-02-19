use actix_web::rt::spawn;
use helpers::{
  jwt,
  time::{self, utc_now},
};
use instant_akismet::CheckResult;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde_json::{json, Value};

use crate::{
  app::AppState,
  components::{
    comment::model::*,
    user::model::{get_user, is_admin_user, UserQueryBy},
  },
  entities::wl_comment,
  error::AppError,
  helpers::{
    avatar::get_avatar,
    email::{send_email_notification, CommentNotification, NotifyType},
    markdown::render_md_to_html,
    spam::check_comment,
    ua,
  },
  response::Code,
};

pub async fn get_comment_info(
  state: &AppState,
  path: String,
  page: i32,
  page_size: i32,
  sort_by: String,
  token: Result<String, AppError>,
) -> Result<Value, Code> {
  let (sort_col, sort_ord) = match sort_by.as_str() {
    "insertedAt_asc" => (wl_comment::Column::InsertedAt, Order::Asc),
    "like_desc" => (wl_comment::Column::Like, Order::Desc),
    _ => (wl_comment::Column::InsertedAt, Order::Desc),
  };
  let mut select = wl_comment::Entity::find()
    .filter(wl_comment::Column::Url.contains(&path))
    .filter(wl_comment::Column::Pid.is_null())
    .filter(wl_comment::Column::Status.is_not_in(["waiting", "spam"]));
  let mut is_admin = false;
  if let Ok(token) = token {
    if let Ok(email) = jwt::verify::<String>(&token, &state.jwt_token).map(|t| t.claims.data) {
      if is_admin_user(&email, &state.conn).await.unwrap_or(false) {
        is_admin = true;
        select = wl_comment::Entity::find()
          .filter(wl_comment::Column::Url.contains(&path))
          .filter(wl_comment::Column::Pid.is_null());
      }
    }
  }
  let paginator = select
    .order_by(sort_col, sort_ord)
    .paginate(&state.conn, page_size as u64);
  let total_pages = paginator.num_pages().await.map_err(AppError::from)?;
  let parrent_comments = paginator
    .fetch_page((page - 1) as u64)
    .await
    .map_err(AppError::from)?;
  // Get comment count for articles
  let mut count = paginator.num_items().await.map_err(AppError::from)?;
  let mut data = vec![];
  for parrent_comment in parrent_comments {
    let c = wl_comment::Entity::find()
      .filter(wl_comment::Column::Nick.eq(parrent_comment.clone().nick))
      .filter(wl_comment::Column::Mail.eq(parrent_comment.clone().mail))
      .count(&state.conn)
      .await
      .map_err(AppError::from)?;
    // let level;
    // if let Some(levels) = &state.levels {
    //   level = Some(get_level(c as usize, levels));
    // } else {
    //   level = None;
    // }

    let level = state
      .levels
      .as_ref()
      .map(|levels| get_level(c as usize, levels));

    let mut parrent_data = build_data_entry(parrent_comment.clone(), level);

    if let Some(user_id) = parrent_data.user_id {
      if let Ok(user) = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await {
        parrent_data.label = user.label;
        parrent_data.r#type = Some(user.user_type);
      }
    }

    if is_admin {
      parrent_data.mail = parrent_comment.mail.clone();
      parrent_data.ip = parrent_comment.ip.clone();
    }
    let mut subcomments = wl_comment::Entity::find()
      .filter(wl_comment::Column::Url.contains(path.clone()))
      .filter(wl_comment::Column::Pid.eq(parrent_comment.id))
      .filter(wl_comment::Column::Status.is_not_in(["waiting", "spam"]))
      .order_by(wl_comment::Column::InsertedAt, Order::Asc)
      .all(&state.conn)
      .await
      .map_err(AppError::from)?;
    if is_admin {
      subcomments = wl_comment::Entity::find()
        .filter(wl_comment::Column::Url.contains(path.clone()))
        .filter(wl_comment::Column::Pid.eq(parrent_comment.id))
        .order_by(wl_comment::Column::InsertedAt, Order::Asc)
        .all(&state.conn)
        .await
        .map_err(AppError::from)?;
    }
    count += subcomments.len() as u64;
    for subcomment in subcomments {
      let c = wl_comment::Entity::find()
        .filter(wl_comment::Column::Url.eq(parrent_comment.clone().url))
        .filter(wl_comment::Column::Nick.eq(parrent_comment.clone().nick))
        .filter(wl_comment::Column::Mail.eq(parrent_comment.clone().mail))
        .count(&state.conn)
        .await
        .map_err(AppError::from)?;
      // let level;
      // if let Some(levels) = &state.levels {
      //   level = Some(get_level(c as usize, levels));
      // } else {
      //   level = None;
      // }
      let level = state
        .levels
        .as_ref()
        .map(|levels| get_level(c as usize, levels));

      let mut subcomment_data = build_data_entry(subcomment.clone(), level);
      if let Some(user_id) = subcomment_data.user_id {
        let user = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await?;
        subcomment_data.label = user.label;
        subcomment_data.r#type = Some(user.user_type);
      }
      if is_admin {
        subcomment_data.mail = subcomment_data.mail.clone();
        subcomment_data.ip = subcomment_data.ip.clone();
      }
      subcomment_data.reply_user = Some(json!({
        "avatar": get_avatar(&parrent_comment.mail.clone().unwrap_or("default".to_owned())),
        "link": parrent_comment.link,
        "nick": parrent_comment.nick,
      }));
      parrent_data.children.push(subcomment_data)
    }
    data.push(parrent_data)
  }
  Ok(json!({
    "count": count,
    "data": data,
    "page": page,
    "pageSize": page_size,
    "totalPages": total_pages
  }))
}

pub async fn get_comment_info_by_admin(
  state: &AppState,
  owner: String,
  email: String,
  keyword: String,
  status: String,
  page: i32,
) -> Result<Value, Code> {
  let mut comments = vec![];
  let mut total_pages = 0;
  if owner.clone() == "mine" {
    let paginator = wl_comment::Entity::find()
      .filter(wl_comment::Column::Mail.eq(email))
      .filter(wl_comment::Column::Status.eq(status))
      .filter(wl_comment::Column::Comment.contains(keyword))
      .paginate(&state.conn, 10);
    total_pages = paginator.num_pages().await.map_err(AppError::from)?;
    comments = paginator
      .fetch_page((page - 1) as u64)
      .await
      .map_err(AppError::from)?;
  } else if owner == "all" {
    let paginator = wl_comment::Entity::find()
      .filter(wl_comment::Column::Status.eq(status))
      .filter(wl_comment::Column::Comment.contains(keyword))
      .paginate(&state.conn, 10);
    total_pages = paginator.num_pages().await.map_err(AppError::from)?;
    comments = paginator
      .fetch_page((page - 1) as u64)
      .await
      .map_err(AppError::from)?;
  }
  let mut data = vec![];
  for comment in comments.iter() {
    let mut data_entry = build_data_entry(comment.clone(), None);
    if let Some(user_id) = data_entry.user_id {
      if let Ok(user) = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await {
        data_entry.label = user.label;
        data_entry.r#type = Some(user.user_type);
      }
    }
    data.push(data_entry);
  }
  Ok(json!({
    "data": data,
    "page": page,
    "pageSize": 10,
    "spamCount": 0,
    "totalPages": total_pages,
    "waitingCount": 0,
  }))
}

pub async fn create_comment<'a>(
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
  ip: String,
  user_type: UserType,
  lang: String,
) -> Result<Value, Code> {
  let html_output = render_md_to_html(&comment);
  let mut avatar = get_avatar("anonymous");
  let mut new_comment = create_comment_model(
    None,
    comment.clone(),
    link,
    mail.clone(),
    nick.clone(),
    ua.clone(),
    url.clone(),
    ip.clone(),
    pid,
    rid,
  );
  let (browser, os) = ua::parse(ua);
  let mut data = json!({
    "addr":"",
    "browser": browser,
    "os": os,
    "comment": html_output,
  });
  match user_type {
    UserType::Anonymous => {
      new_comment.status = Set(if state.comment_audit {
        "waiting".to_string()
      } else if has_forbidden_word(&comment, &state.forbidden_words) {
        "spam".to_string()
      } else {
        if matches!(
          check_comment(nick, mail, ip, comment).await?,
          CheckResult::Ham
        ) {
          "approved".to_string()
        } else {
          "spam".to_string()
        }
      });
    }
    UserType::Guest(email) => {
      let user = get_user(UserQueryBy::Email(email), &state.conn).await;
      if let Ok(user) = user {
        new_comment.user_id = Set(Some(user.id as i32));
        new_comment.status = Set(if state.comment_audit {
          "waiting".to_string()
        } else if has_forbidden_word(&comment, &state.forbidden_words) {
          "spam".to_string()
        } else {
          if matches!(
            check_comment(nick, mail, ip, comment).await?,
            CheckResult::Ham
          ) {
            "approved".to_string()
          } else {
            "spam".to_string()
          }
        });
        data["label"] = json!(user.label);
        data["mail"] = json!(user.email);
        data["type"] = json!(user.user_type);
        data["user_id"] = json!(user.id);
        avatar = get_avatar(&user.email);
      }
    }
    UserType::Administrator(email) => {
      let user = get_user(UserQueryBy::Email(email), &state.conn).await;
      if let Ok(user) = user {
        new_comment.user_id = Set(Some(user.id as i32));
        new_comment.status = Set("approved".to_string());
        data["label"] = json!(user.label);
        data["mail"] = json!(user.email);
        data["type"] = json!(user.user_type);
        data["user_id"] = json!(user.id);
        avatar = get_avatar(&user.email);
      }
    }
  }
  let comment = new_comment
    .insert(&state.conn)
    .await
    .map_err(AppError::from)?;
  data["avatar"] = json!(avatar);
  data["like"] = json!(comment.like);
  data["ip"] = json!(comment.ip);
  data["link"] = json!(comment.link);
  data["nick"] = json!(comment.nick);
  data["objectId"] = json!(comment.id);
  data["orig"] = json!(comment.comment);
  data["status"] = json!(comment.status);
  data["time"] = json!(comment.created_at.unwrap_or(utc_now()).timestamp_millis());
  data["url"] = json!(comment.url);
  if let Some(pid) = pid {
    data["pid"] = json!(pid);
  }
  if let Some(rid) = rid {
    data["rid"] = json!(rid);
  };
  spawn(async move {
    send_email_notification(CommentNotification {
      sender_name: comment.nick.unwrap(),
      sender_email: comment.mail.unwrap(),
      comment_id: comment.id,
      comment: comment.comment.unwrap(),
      url: comment.url.unwrap(),
      notify_type: NotifyType::NewComment,
      lang: Some(&lang),
    });
  });
  Ok(data)
}

pub async fn delete_comment(state: &AppState, id: u32, email: String) -> Result<(), Code> {
  let user = get_user(UserQueryBy::Email(email.clone()), &state.conn).await?;
  let pass = if user.user_type == "administrator" {
    true
  } else {
    wl_comment::Entity::find()
      .filter(wl_comment::Column::Id.eq(id))
      .filter(wl_comment::Column::UserId.eq(user.id))
      .one(&state.conn)
      .await
      .map_err(AppError::from)?
      .is_some()
  };
  if !pass {
    return Err(Code::Forbidden);
  }
  match wl_comment::Entity::delete_by_id(id).exec(&state.conn).await {
    Ok(_) => Ok(()),
    Err(_) => Err(Code::Error),
  }
}

pub async fn update_comment(
  state: &AppState,
  email: String,
  id: u32,
  status: Option<String>,
  like: Option<bool>,
  comment: Option<String>,
  link: Option<String>,
  mail: Option<String>,
  nick: Option<String>,
  ua: Option<String>,
  url: Option<String>,
  sticky: Option<i8>,
) -> Result<Value, Code> {
  let mut active_comment = wl_comment::ActiveModel {
    id: Set(id),
    updated_at: Set(Some(time::utc_now())),
    ..Default::default()
  };
  let user = get_user(UserQueryBy::Email(email), &state.conn).await?;
  let comment_opt = wl_comment::Entity::find()
    .filter(wl_comment::Column::Id.eq(id))
    .filter(wl_comment::Column::UserId.eq(user.id))
    .one(&state.conn)
    .await
    .map_err(AppError::from)?;
  if comment_opt.is_none() {
    return Err(Code::Forbidden);
  }
  if let Some(like) = like {
    let comment = get_comment(CommentQueryBy::Id(id), &state.conn).await?;
    active_comment.like = Set(Some(comment.like.unwrap_or(0) + if like { 1 } else { -1 }));
  }
  if let Some(status) = status {
    active_comment.status = Set(status);
  }
  if let Some(sticky) = sticky {
    active_comment.sticky = Set(Some(sticky));
  }
  if let Some(comment) = comment {
    active_comment.comment = Set(Some(comment));
  }
  if let Some(ua) = ua {
    active_comment.ua = Set(Some(ua));
  }
  if let Some(nick) = nick {
    active_comment.nick = Set(Some(nick));
  }
  if let Some(link) = link {
    active_comment.link = Set(Some(link));
  }
  if let Some(mail) = mail {
    active_comment.mail = Set(Some(mail));
  }
  if let Some(url) = url {
    active_comment.url = Set(Some(url));
  }

  let updated_comment = active_comment
    .update(&state.conn)
    .await
    .map_err(AppError::from)?;
  let (browser, os) = ua::parse(updated_comment.ua.unwrap_or("".to_owned()));
  let like = updated_comment.like.unwrap_or(0);
  let time = updated_comment.created_at.unwrap().timestamp_millis();
  let pid = updated_comment.pid;
  let rid = updated_comment.rid;
  let html_output = render_md_to_html(updated_comment.comment.clone().unwrap().as_str());
  if is_anonymous(id, &state.conn).await? {
    let data = json!({
      "addr":"",
      "avatar": get_avatar("anonymous"),
      "browser": browser,
      "comment": html_output,
      "ip": updated_comment.ip,
      "mail": updated_comment.mail,
      "user_id": updated_comment.user_id,
      "like": like,
      "link": updated_comment.link,
      "nick": updated_comment.nick,
      "objectId": updated_comment.id,
      "orig": updated_comment.comment,
      "os": os,
      "status": updated_comment.status,
      "time": time,
      "url": updated_comment.url,
    });
    Ok(data)
  } else {
    let user = get_user(
      UserQueryBy::Id(updated_comment.user_id.unwrap() as u32),
      &state.conn,
    )
    .await?;
    let mut data = json!({
      "addr":"",
      "avatar": get_avatar(&user.email),
      "browser": browser,
      "comment": html_output,
      "ip": updated_comment.ip,
      "label": user.label,
      "mail": user.email.clone(),
      "type": user.user_type,
      "user_id": updated_comment.user_id,
      "like": like,
      "link": updated_comment.link,
      "nick": updated_comment.nick,
      "objectId": updated_comment.id,
      "orig": updated_comment.comment,
      "os": os,
      "status": updated_comment.status,
      "time": time,
      "url": updated_comment.url,
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
