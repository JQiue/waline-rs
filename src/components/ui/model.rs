use serde::Deserialize;

#[derive(Deserialize)]
pub struct UIProfilePageQuery {
  pub lng: Option<String>,
  pub token: Option<String>,
}

#[derive(Deserialize)]
pub struct UiLoginPageQeury {
  pub redirect: Option<String>,
}
