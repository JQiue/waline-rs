use serde::Deserialize;

#[derive(Deserialize)]
pub struct ImportQuery {
  table: String,
  lang: String,
}

#[derive(Deserialize)]
pub struct ImportDataBody {
  comment: Option<String>,
  ip: Option<String>,
  link: Option<String>,
  mail: Option<String>,
  nick: Option<String>,
  status: Option<String>,
  ua: Option<String>,
  url: Option<String>,
  #[serde(rename = "insertedAt")]
  inserted_at: Option<String>,
  #[serde(rename = "createdAt")]
  created_at: Option<String>,
  #[serde(rename = "updatedAt")]
  updated_at: Option<String>,
  #[serde(rename = "objectId")]
  object_id: u32,
  time: Option<u32>,
  reaction0: Option<u32>,
  reaction1: Option<u32>,
  reaction2: Option<u32>,
  reaction3: Option<u32>,
  reaction4: Option<u32>,
  reaction5: Option<u32>,
  reaction6: Option<u32>,
  reaction7: Option<u32>,
  reaction8: Option<u32>,
  #[serde(rename = "2fa")]
  two_factor_auth: Option<String>,
  display_name: Option<String>,
  email: Option<String>,
  label: Option<String>,
  password: Option<String>,
  r#type: Option<String>,
}
