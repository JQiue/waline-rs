use sea_orm::{ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set};
use serde_json::{json, Value};

use crate::{
  app::AppState,
  components::{
    comment::model::*,
    user::model::{get_user, UserQueryBy},
  },
  entities::wl_comment,
  error::AppError,
  helpers::{email::extract_email_prefix, markdown::render_md_to_html, ua},
  response::StatusCode,
};

pub async fn get_comment_info(
  state: &AppState,
  path: String,
  owner: Option<String>,
  page: i32,
  page_size: i32,
  sort_by: String,
) -> Result<Value, StatusCode> {
  match owner {
    Some(owner) => {
      let mut data = vec![];
      if owner == "mine" {
        let email = "";
        data = wl_comment::Entity::find()
          .filter(wl_comment::Column::Mail.eq(email))
          .all(&state.conn)
          .await
          .map_err(AppError::from)?;
      } else if owner == "all" {
        data = wl_comment::Entity::find()
          .all(&state.conn)
          .await
          .map_err(AppError::from)?;
      }
      Ok(json!({
        "data": data,
        "page": page,
        "pageSize": page_size,
        "spamCount": 0,
        "totalPages": 5,
        "waitingCount": 0,
      }))
    }
    None => {
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
      // 根据 path 获取根评论
      // let parrent_comments = wl_comment::Entity::find()
      //   .filter(wl_comment::Column::Url.contains(path.clone()))
      //   .filter(wl_comment::Column::Pid.is_null())
      //   .order_by(sort_col, sort_ord)
      //   .paginate(&state.conn, page_size as u64)
      //   .fetch_page((page - 1) as u64)
      //   .await
      //   .map_err(AppError::from)?;
      let paginator = wl_comment::Entity::find()
        .filter(wl_comment::Column::Url.contains(path.clone()))
        .filter(wl_comment::Column::Pid.is_null())
        .order_by(sort_col, sort_ord)
        .paginate(&state.conn, page_size as u64);
      let total_pages = paginator.num_pages().await.map_err(AppError::from)?;
      let parrent_comments = paginator
        .fetch_page((page - 1) as u64)
        .await
        .map_err(AppError::from)?;
      // Get comment count for articles
      let mut count = parrent_comments.len();
      let mut comments: Vec<DataEntry> = vec![];
      for parrent_comment in parrent_comments {
        let mut parrent_data_entry =
          build_data_entry(parrent_comment.clone(), state.anonymous_avatar.to_string());
        if let Some(user_id) = parrent_data_entry.user_id {
          let user = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await?;
          parrent_data_entry.label = user.label;
          parrent_data_entry.r#type = Some(user.r#type);
        }
        let subcomments = wl_comment::Entity::find()
          .filter(wl_comment::Column::Url.contains(path.clone()))
          .filter(wl_comment::Column::Pid.eq(parrent_comment.id))
          .order_by(wl_comment::Column::InsertedAt, Order::Asc)
          .all(&state.conn)
          .await
          .map_err(AppError::from)?;
        for subcomment in subcomments {
          count += 1;
          let mut subcomment_data_entry =
            build_data_entry(subcomment.clone(), state.anonymous_avatar.to_string());
          if let Some(user_id) = subcomment_data_entry.user_id {
            let user = get_user(UserQueryBy::Id(user_id as u32), &state.conn).await?;
            subcomment_data_entry.label = user.label;
            subcomment_data_entry.r#type = Some(user.r#type);
          }
          parrent_data_entry.children.push(subcomment_data_entry)
        }
        comments.push(parrent_data_entry)
      }
      Ok(
        json!({ "count": count, "data": comments, "page": 1, "pageSize": 10, "totalPages": total_pages }),
      )
    }
  }
}

/// create comment
/// No user is created if the user is anonymous
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
) -> Result<Value, StatusCode> {
  // 逻辑是先验证 jwt，如果 jwt 正确，则直接插入，否则需要验证评论是否重复，不重复则插入
  let html_output = render_md_to_html(&comment);
  let mut new_comment = create_comment_model(
    None,
    comment,
    link,
    email.clone(),
    nick.clone(),
    ua,
    url,
    pid,
    rid,
  );
  let user = get_user(UserQueryBy::Email(email.clone()), &state.conn).await;
  if let Ok(user) = user {
    new_comment.user_id = Set(Some(user.id as i32));
  }
  match wl_comment::Entity::insert(new_comment)
    .exec(&state.conn)
    .await
  {
    Ok(result) => {
      let comment = get_comment(CommentQueryBy::Id(result.last_insert_id), &state.conn).await?;
      let (browser, os) = ua::parse(comment.ua.unwrap_or("".to_owned()));
      let time = comment.created_at.unwrap().timestamp_millis();
      let pid = comment.pid;
      let rid = comment.rid;
      if nick == "匿名" {
        let mut data = json!({
          "addr":"",
          "avatar": state.anonymous_avatar.to_string(),
          "browser": browser,
          "comment": html_output,
          "like": comment.like.unwrap_or(0),
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
        let user = get_user(UserQueryBy::Email(email.clone()), &state.conn).await?;
        let avatar = if let Some(prefix) = extract_email_prefix(email.clone()) {
          format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", prefix)
        } else {
          state.anonymous_avatar.to_string()
        };
        let mut data = json!({
          "addr":"",
          "avatar": avatar,
          "browser": browser,
          "comment": html_output,
          "like": comment.like.unwrap_or(0),
          "ip": comment.ip,
          "label": user.label,
          "mail": user.email,
          "type": user.r#type,
          "user_id": user.id,
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
    Err(_) => Err(StatusCode::Error),
  }
}

pub async fn delete_comment(state: &AppState, id: u32) -> Result<bool, StatusCode> {
  match wl_comment::Entity::delete_by_id(id).exec(&state.conn).await {
    Ok(_) => Ok(true),
    Err(_) => Err(StatusCode::Error),
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
) -> Result<Value, StatusCode> {
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
    .await?;
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
