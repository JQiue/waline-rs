use crate::entities::{
  wl_comment::Entity as Comment, wl_counter::Entity as Counter, wl_users::Entity as User,
};
use chrono::{Local, Utc};
use sea_orm::{prelude::DateTimeUtc, DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize)]
pub struct ExportQuery {
  pub lang: String,
}

#[derive(Deserialize)]
pub struct CreateDataQuery {
  pub table: String,
  pub lang: String,
}

#[derive(Deserialize)]
pub struct DeleteQuery {
  pub table: String,
  pub lang: String,
}

mod datetime_utc_format {
  use chrono::{DateTime, Local, NaiveDateTime, Utc};
  use serde::{self, Deserialize, Deserializer};

  use crate::prelude::LoggingResultErr;

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
      Some(s) => {
        let naive_dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
          .log_err()
          .map_err(serde::de::Error::custom)?;
        let dt = naive_dt.and_local_timezone(Local).unwrap().to_utc();
        Ok(Some(dt))
      }
      None => Ok(None),
    }
  }
}

#[derive(Deserialize)]
pub struct CreateDataBody {
  pub comment: Option<String>,
  pub ip: Option<String>,
  pub link: Option<String>,
  pub mail: Option<String>,
  pub nick: Option<String>,
  pub status: Option<String>,
  pub ua: Option<String>,
  pub url: Option<String>,
  #[serde(rename = "insertedAt", default, with = "datetime_utc_format")]
  pub inserted_at: Option<chrono::DateTime<Utc>>,
  #[serde(rename = "createdAt", default, with = "datetime_utc_format")]
  pub created_at: Option<chrono::DateTime<Utc>>,
  #[serde(rename = "updatedAt", default, with = "datetime_utc_format")]
  pub updated_at: Option<chrono::DateTime<Utc>>,
  #[serde(rename = "objectId")]
  pub object_id: Option<u32>,
  pub time: Option<i32>,
  pub reaction0: Option<i32>,
  pub reaction1: Option<i32>,
  pub reaction2: Option<i32>,
  pub reaction3: Option<i32>,
  pub reaction4: Option<i32>,
  pub reaction5: Option<i32>,
  pub reaction6: Option<i32>,
  pub reaction7: Option<i32>,
  pub reaction8: Option<i32>,
  #[serde(rename = "2fa")]
  pub two_factor_auth: Option<String>,
  pub display_name: Option<String>,
  pub email: Option<String>,
  pub label: Option<String>,
  pub password: Option<String>,
  pub r#type: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateDataQuery {
  #[serde(rename = "objectId")]
  pub object_id: u32,
  pub table: String,
  pub lang: String,
}

#[derive(Deserialize)]
pub struct UpdateDataBody {
  #[serde(rename = "objectId")]
  pub object_id: Option<u32>,
  #[serde(rename = "2fa")]
  pub two_factor_auth: Option<String>,
  pub display_name: Option<String>,
  pub email: Option<String>,
  pub label: Option<String>,
  pub password: Option<String>,
  pub r#type: Option<String>,
  pub url: Option<String>,
  #[serde(rename = "createdAt", default, with = "datetime_utc_format")]
  pub created_at: Option<chrono::DateTime<Utc>>,
  #[serde(rename = "updatedAt", default, with = "datetime_utc_format")]
  pub updated_at: Option<chrono::DateTime<Utc>>,
  pub pid: Option<i32>,
  pub rid: Option<i32>,
}

fn fmt_datetime<S>(datetime: &Option<DateTimeUtc>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  match datetime {
    Some(dt) => {
      let formatted = dt
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
      serializer.serialize_some(&formatted)
    }
    None => serializer.serialize_none(),
  }
}

#[derive(Debug, DerivePartialModel, FromQueryResult, Serialize)]
#[sea_orm(entity = "Comment")]
pub struct CommentData {
  #[serde(rename = "objectId")]
  pub id: u32,
  pub user_id: Option<i32>,
  pub comment: Option<String>,
  pub ip: Option<String>,
  pub link: Option<String>,
  pub mail: Option<String>,
  pub nick: Option<String>,
  pub pid: Option<i32>,
  pub rid: Option<i32>,
  pub sticky: Option<i8>,
  pub status: String,
  pub like: Option<i32>,
  pub ua: Option<String>,
  pub url: Option<String>,
  #[serde(serialize_with = "fmt_datetime")]
  #[sea_orm(from_col = "insertedAt")]
  pub inserted_at: Option<DateTimeUtc>,
  #[serde(serialize_with = "fmt_datetime")]
  #[sea_orm(from_col = "createdAt")]
  pub created_at: Option<DateTimeUtc>,
  #[serde(serialize_with = "fmt_datetime")]
  #[sea_orm(from_col = "updatedAt")]
  pub updated_at: Option<DateTimeUtc>,
}

#[derive(Debug, DerivePartialModel, FromQueryResult, Serialize)]
#[sea_orm(entity = "Counter")]
pub struct CounterData {
  #[serde(rename = "objectId")]
  pub id: u32,
  pub time: Option<i32>,
  pub reaction0: Option<i32>,
  pub reaction1: Option<i32>,
  pub reaction2: Option<i32>,
  pub reaction3: Option<i32>,
  pub reaction4: Option<i32>,
  pub reaction5: Option<i32>,
  pub reaction6: Option<i32>,
  pub reaction7: Option<i32>,
  pub reaction8: Option<i32>,
  pub url: String,
  #[serde(serialize_with = "fmt_datetime")]
  #[sea_orm(from_col = "createdAt")]
  pub created_at: Option<DateTimeUtc>,
  #[serde(serialize_with = "fmt_datetime")]
  #[sea_orm(from_col = "updatedAt")]
  pub updated_at: Option<DateTimeUtc>,
}
use crate::entities::wl_users;
use sea_orm::prelude::Expr;
#[derive(DerivePartialModel, FromQueryResult, Serialize)]
#[sea_orm(entity = "User")]
pub struct UserData {
  #[serde(rename = "objectId")]
  pub id: u32,
  pub display_name: String,
  pub email: String,
  pub password: String,
  pub label: Option<String>,
  pub url: Option<String>,
  pub avatar: Option<String>,
  pub github: Option<String>,
  pub twitter: Option<String>,
  pub facebook: Option<String>,
  pub google: Option<String>,
  pub weibo: Option<String>,
  pub qq: Option<String>,
  #[sea_orm(from_expr = "Expr::col(wl_users::Column::TwoFactorAuth)")]
  #[serde(rename = "2fa")]
  pub two_factor_auth: Option<String>,
  #[serde(serialize_with = "fmt_datetime")]
  #[sea_orm(from_col = "createdAt")]
  pub created_at: Option<DateTimeUtc>,
  #[serde(serialize_with = "fmt_datetime")]
  #[sea_orm(from_col = "updatedAt")]
  pub updated_at: Option<DateTimeUtc>,
}
