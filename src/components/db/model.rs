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
  use chrono::{DateTime, TimeZone, Utc};
  use serde::{self, Deserialize, Deserializer};

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
      Some(s) => {
        let dt = parse_datetime(&s).map_err(serde::de::Error::custom)?;
        // let fmt =
        //   DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map(|dt| dt.with_timezone(&Utc));
        Ok(Some(dt))
      }
      None => Ok(None),
    }
  }

  fn parse_datetime(s: &str) -> Result<DateTime<Utc>, String> {
    let formats = [
      // 2024-07-28T03:50:31Z
      |s: &str| DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc)),
      |s: &str| Utc.datetime_from_str(s, "%Y-%m-%d %H:%M:%S"),
      // 2024-12-21T14:08:39
      |s: &str| Utc.datetime_from_str(s, "%Y-%m-%dT%H:%M:%S"),
      // 2024-12-10T14:33:24.831981036
      |s: &str| Utc.datetime_from_str(s, "%Y-%m-%dT%H:%M:%S.%f"),
    ];

    for parse_attempt in formats.iter() {
      if let Ok(dt) = parse_attempt(s) {
        return Ok(dt);
      }
    }
    tracing::debug!("Unable to parse time format: {}", s);
    Err(format!("Unable to parse time format: {}", s))
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
