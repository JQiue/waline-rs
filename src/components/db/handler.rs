use actix_web::{
  delete, get, post, put,
  web::{Data, Json, Query},
  HttpResponse,
};
use serde_json::json;

use crate::{
  components::db::{model::*, service},
  AppState,
};

#[get("/db")]
pub async fn export_data(state: Data<AppState>, query: Query<ExportQuery>) -> HttpResponse {
  let Query(ExportQuery { lang }) = query;
  match service::export_data(&state, lang).await {
    Ok(data) => HttpResponse::Ok().json(json!({
      "errno": 0,
      "errmsg": "",
      "data": data
    })),
    Err(err) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": err,
    })),
  }
}

#[post("/db")]
pub async fn create_data(
  state: Data<AppState>,
  query: Query<CreateDataQuery>,
  body: Json<CreateDataBody>,
) -> HttpResponse {
  let Query(CreateDataQuery { table, lang: _ }) = query;
  let Json(CreateDataBody {
    comment,
    ip,
    link,
    mail,
    nick,
    status,
    ua,
    url,
    inserted_at,
    created_at,
    updated_at,
    object_id,
    time,
    reaction0,
    reaction1,
    reaction2,
    reaction3,
    reaction4,
    reaction5,
    reaction6,
    reaction7,
    reaction8,
    two_factor_auth,
    display_name,
    email,
    label,
    password,
    r#type,
  }) = body;
  match table.as_str() {
    "Comment" => match service::create_comment_data(
      &state,
      comment,
      ip,
      link,
      mail,
      nick,
      status,
      ua,
      url,
      created_at,
      updated_at,
      inserted_at,
    )
    .await
    {
      Ok(data) => HttpResponse::Ok().json(json!({
        "data": data,
        "errno": 0,
        "errmsg": "",
      })),
      Err(_) => HttpResponse::Ok().json(json!({
        "errno": 1000,
        "errmsg": "",
      })),
    },
    "Counter" => match service::create_counter_data(
      &state, time, url, reaction0, reaction1, reaction2, reaction3, reaction4, reaction5,
      reaction6, reaction7, reaction8, created_at, updated_at,
    )
    .await
    {
      Ok(data) => HttpResponse::Ok().json(json!({
        "data": data,
        "errno": 0,
        "errmsg": "",
      })),
      Err(_) => HttpResponse::Ok().json(json!({
        "errno": 1000,
        "errmsg": "",
      })),
    },

    "Users" => match service::create_user_data(
      &state,
      object_id,
      display_name,
      password,
      email,
      r#type,
      label,
      url,
      two_factor_auth,
      created_at,
      updated_at,
    )
    .await
    {
      Ok(_) => HttpResponse::Ok().json(json!({
        "data": "",
        "errno": 0,
        "errmsg": "",
      })),
      Err(err) => HttpResponse::Ok().json(json!({
        "errno": 1000,
        "errmsg": err,
      })),
    },
    _ => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "",
    })),
  }
}

#[put("/db")]
pub async fn update_data(
  state: Data<AppState>,
  query: Query<UpdateDataQuery>,
  body: Json<UpdateDataBody>,
) -> HttpResponse {
  let Query(UpdateDataQuery {
    object_id: _,
    table,
    lang,
  }) = query;
  let Json(UpdateDataBody {
    user_id,
    object_id,
    two_factor_auth,
    display_name,
    email,
    label,
    password,
    r#type,
    url,
    created_at,
    updated_at,
  }) = body;
  match table.as_str() {
    "Comment" => match service::update_comment_data(&state, lang, object_id, user_id).await {
      Ok(_) => HttpResponse::Ok().json(json!({
        "data": "",
        "errno": 0,
        "errmsg": "",
      })),
      Err(_) => HttpResponse::Ok().json(json!({
        "errno": 1000,
        "errmsg": "",
      })),
    },
    "Users" => match service::update_user_data(
      &state,
      lang,
      object_id,
      display_name,
      password,
      email,
      url,
      label,
      r#type,
      two_factor_auth,
      created_at,
      updated_at,
    )
    .await
    {
      Ok(_) => HttpResponse::Ok().json(json!({
        "errno": 0,
        "errmsg": "",
      })),
      Err(err) => HttpResponse::Ok().json(json!({
        "errno": 1000,
        "errmsg": err,
      })),
    },

    _ => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "",
    })),
  }
}

#[delete("/db")]
pub async fn delete_data(state: Data<AppState>, query: Query<DeleteQuery>) -> HttpResponse {
  let Query(DeleteQuery { table, lang }) = query;
  match service::delete_data(&state, &table, lang).await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errno": 0,
      "errmsg": "",
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "todo",
    })),
  }
}
