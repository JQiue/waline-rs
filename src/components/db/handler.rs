use actix_web::{
  delete, get, post,
  web::{Data, Json, Query},
  HttpResponse,
};
use serde_json::json;

use crate::{
  components::db::{model::*, service},
  AppState,
};

#[get("/db")]
pub async fn export_data() -> HttpResponse {
  match service::export_data().await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "未实现",
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "未实现",
    })),
  }
}

#[post("/db")]
pub async fn import_data(
  _data: Data<AppState>,
  _query: Query<ImportQuery>,
  _body: Json<ImportDataBody>,
) -> HttpResponse {
  match service::import_data().await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "未实现",
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "未实现",
    })),
  }
}

#[delete("/db")]
pub async fn delete_data() -> HttpResponse {
  match service::delete_data().await {
    Ok(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "未实现",
    })),
    Err(_) => HttpResponse::Ok().json(json!({
      "errno": 1000,
      "errmsg": "未实现",
    })),
  }
}
