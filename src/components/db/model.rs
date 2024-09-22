use chrono::Utc;
use serde::Deserialize;

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
  use chrono::{DateTime, NaiveDateTime, Utc};
  use serde::{self, Deserialize, Deserializer};

  const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
      Some(s) => {
        let naive = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)))
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
  pub object_id: Option<u32>,
  pub table: String,
  pub lang: String,
}

#[derive(Deserialize)]
pub struct UpdateDataBody {
  pub user_id: Option<u32>,
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
}
