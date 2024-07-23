use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

mod jwt_numeric_date {
  //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
  use chrono::{DateTime, TimeZone, Utc};
  use serde::{self, Deserialize, Deserializer, Serializer};

  /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
  // pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
  // where
  //   S: Serializer,
  // {
  //   let timestamp = date.unix_timestamp();
  //   serializer.serialize_i64(timestamp)
  // }
  // utc

  pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_i64(date.timestamp())
  }

  /// Attempts to deserialize an i64 and use as a Unix timestamp
  // pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
  // where
  //   D: Deserializer<'de>,
  // {
  //   OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
  //     .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
  // }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let timestamp = i64::deserialize(deserializer)?;
    Utc
      .timestamp_opt(timestamp, 0)
      .single()
      .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
  email: String,
  #[serde(with = "jwt_numeric_date")]
  exp: DateTime<Utc>,
}

impl Claims {
  /// Creates a new [`Claims`].
  pub fn new(email: String, expire: i64) -> Self {
    // normalize the timestamps by stripping of microseconds
    let now = Utc::now();
    // let iat = now;
    let exp = now + Duration::seconds(expire);
    Self { email, exp }
  }
}

/// .
pub fn sign(payload: Claims, key: String) -> String {
  let header = Header::default();
  let key = EncodingKey::from_secret(key.as_ref());
  match encode(&header, &payload, &key) {
    Ok(token) => token,
    Err(error) => panic!("{error}"),
  }
}
/// .
pub fn verify(token: String, key: String) -> Result<String, String> {
  let key = DecodingKey::from_secret(key.as_ref());
  let validation = Validation::new(Algorithm::HS256);
  let result = match decode::<Claims>(&token, &key, &validation) {
    Ok(c) => Ok(c.claims.email),
    Err(err) => match *err.kind() {
      ErrorKind::InvalidToken => Err("Token is invalid".to_string()),
      ErrorKind::ExpiredSignature => Err("token expired".to_string()),
      _ => Err("Some other errors".to_string()),
    },
  };
  result
}
