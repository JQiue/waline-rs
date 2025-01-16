use actix_web::rt::spawn;
use helpers::{jwt, time::utc_now};
use instant_akismet::CheckResult;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde_json::{json, Value};

use crate::{
  app::AppState,
  components::{
    comment::model::*,
    user::model::{get_user, is_admin_user, is_first_admin_user, UserQueryBy},
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
  let sort_col;
  let sort_ord;
  if sort_by == "insertedAt_desc" {
    sort_col = wl_comment::Column::InsertedAt;
    sort_ord = Order::Desc;
  } else if sort_by == "insertedAt_asc" {
    sort_col = wl_comment::Column::InsertedAt;
    sort_ord = Order::Asc;
  } else if sort_by == "like_desc" {
    sort_col = wl_comment::Column::Like;
    sort_ord = Order::Desc;
  } else {
    sort_col = wl_comment::Column::InsertedAt;
    sort_ord = Order::Desc;
  }
  let mut select = wl_comment::Entity::find()
    .filter(wl_comment::Column::Url.contains(path.clone()))
    .filter(wl_comment::Column::Pid.is_null())
    .filter(wl_comment::Column::Status.is_not_in(["waiting", "spam"]));
  if token.is_ok() {
    let token = token.unwrap();
    if jwt::verify::<String>(token.clone(), state.clone().jwt_token).is_ok() {
      let email = jwt::verify::<String>(token, state.clone().jwt_token)
        .unwrap()
        .claims
        .data;
      if is_admin_user(email, &state.conn).await.unwrap() {
        select = wl_comment::Entity::find()
          .filter(wl_comment::Column::Url.contains(path.clone()))
          .filter(wl_comment::Column::Pid.is_null())
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
    let level;
    if let Some(levels) = &state.levels {
      level = Some(get_level(c as usize, levels));
    } else {
      level = None;
    }
    let mut parrent_data_entry = build_data_entry(parrent_comment.clone(), level);
    if let Some(user_id) = parrent_data_entry.user_id {
      if let Ok(user) = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await {
        parrent_data_entry.label = user.label;
        parrent_data_entry.r#type = Some(user.user_type);
      }
    }
    let subcomments = wl_comment::Entity::find()
      .filter(wl_comment::Column::Url.contains(path.clone()))
      .filter(wl_comment::Column::Pid.eq(parrent_comment.id))
      .filter(wl_comment::Column::Status.is_not_in(["waiting", "spam"]))
      .order_by(wl_comment::Column::InsertedAt, Order::Asc)
      .all(&state.conn)
      .await
      .map_err(AppError::from)?;
    count += subcomments.len() as u64;
    for subcomment in subcomments {
      let c = wl_comment::Entity::find()
        .filter(wl_comment::Column::Url.eq(parrent_comment.clone().url))
        .filter(wl_comment::Column::Nick.eq(parrent_comment.clone().nick))
        .filter(wl_comment::Column::Mail.eq(parrent_comment.clone().mail))
        .count(&state.conn)
        .await
        .map_err(AppError::from)?;
      let level;
      if let Some(levels) = &state.levels {
        level = Some(get_level(c as usize, levels));
      } else {
        level = None;
      }
      let mut subcomment_data_entry = build_data_entry(subcomment.clone(), level);
      if let Some(user_id) = subcomment_data_entry.user_id {
        let user = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await?;
        subcomment_data_entry.label = user.label;
        subcomment_data_entry.r#type = Some(user.user_type);
      }
      subcomment_data_entry.reply_user = Some(json!({
        "avatar": get_avatar(&parrent_comment.mail.clone().unwrap_or("default".to_owned())),
        "link": parrent_comment.link,
        "nick": parrent_comment.nick,
      }));
      parrent_data_entry.children.push(subcomment_data_entry)
    }
    data.push(parrent_data_entry)
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

pub async fn create_comment(
  state: &AppState,
  comment: String,
  link: String,
  email: String,
  nick: String,
  ua: String,
  url: String,
  pid: Option<i32>,
  rid: Option<i32>,
  _at: Option<String>,
  ip: String,
  lang: Option<String>,
) -> Result<Value, Code> {
  let html_output = render_md_to_html(&comment);
  let mut avatar = get_avatar("anonymous");
  let mut new_comment = create_comment_model(
    None,
    comment.clone(),
    link,
    email.clone(),
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
  let user = get_user(UserQueryBy::Email(email.clone()), &state.conn).await;
  let mut is_admin = false;
  if let Ok(user) = user {
    new_comment.user_id = Set(Some(user.id as i32));
    data["label"] = json!(user.label);
    data["mail"] = json!(user.email);
    data["type"] = json!(user.user_type);
    data["user_id"] = json!(user.id);
    avatar = get_avatar(&user.email);
    if user.user_type == "administrator" {
      is_admin = true;
    }
  }
  new_comment.status = Set(if state.comment_audit {
    "waiting".to_string()
  } else if is_admin
    || matches!(
      check_comment(nick, email, ip, comment).await?,
      CheckResult::Ham
    )
  {
    "approved".to_string()
  } else {
    "spam".to_string()
  });
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
      lang,
    });
  });
  Ok(data)
}

pub async fn delete_comment(state: &AppState, id: u32, email: String) -> Result<(), Code> {
  let user = get_user(UserQueryBy::Email(email.clone()), &state.conn).await?;
  let pass = if user.user_type == "administrator" {
    true
  } else {
    if wl_comment::Entity::find()
      .filter(wl_comment::Column::Id.eq(id))
      .filter(wl_comment::Column::UserId.eq(user.id))
      .one(&state.conn)
      .await
      .map_err(AppError::from)?
      .is_some()
    {
      true
    } else {
      false
    }
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
  id: u32,
  status: Option<String>,
  like: Option<bool>,
  comment: Option<String>,
  _link: Option<String>,
  _mail: Option<String>,
  _nick: Option<String>,
  ua: Option<String>,
  _url: Option<String>,
) -> Result<Value, Code> {
  let updated_at = helpers::time::utc_now();
  let new_comment;
  if let Some(like) = like {
    let comment = get_comment(CommentQueryBy::Id(id), &state.conn).await?;
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
    new_comment = wl_comment::Entity::update(model)
      .exec(&state.conn)
      .await
      .map_err(AppError::from)?;
  } else if let Some(status) = status {
    let model = wl_comment::ActiveModel {
      id: Set(id),
      status: Set(status),
      updated_at: Set(Some(updated_at)),
      ..Default::default()
    };
    new_comment = wl_comment::Entity::update(model)
      .exec(&state.conn)
      .await
      .map_err(AppError::from)?;
  } else {
    let model = wl_comment::ActiveModel {
      id: Set(id),
      comment: Set(comment),
      ua: Set(ua),
      ..Default::default()
    };
    new_comment = wl_comment::Entity::update(model)
      .exec(&state.conn)
      .await
      .map_err(AppError::from)?;
  }

  let (browser, os) = ua::parse(new_comment.ua.unwrap_or("".to_owned()));
  let like = new_comment.like.unwrap_or(0);
  let time = new_comment.created_at.unwrap().timestamp_millis();
  let pid = new_comment.pid;
  let rid = new_comment.rid;
  let html_output = render_md_to_html(new_comment.comment.clone().unwrap().as_str());

  if is_anonymous(id, &state.conn).await? {
    let data = json!({
      "addr":"",
      "avatar": get_avatar("anonymous"),
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
    .await?;
    let mut data = json!({
      "addr":"",
      "avatar": get_avatar(&user.email),
      "browser": browser,
      "comment": html_output,
      "ip": new_comment.ip,
      "label": user.label,
      "mail": user.email.clone(),
      "type": user.user_type,
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
