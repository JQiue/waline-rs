use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};
use serde_json::{json, Value};

use crate::error::AppError;
use crate::{
  app::AppState, components::article::model::*, entities::wl_counter, response::StatusCode,
};

pub async fn get_article(
  state: &AppState,
  path: String,
  r#type: String,
) -> Result<Vec<Value>, StatusCode> {
  let mut data = vec![];
  if r#type == "time" {
    for path in path.split(',') {
      if has_counter(CounterQueryBy::Url(path.to_owned()), &state.conn).await? {
        let counter = get_counter(CounterQueryBy::Url(path.to_owned()), &state.conn).await?;
        data.push(json!({"time": counter.time}));
      } else {
        data.push(json!({"time": 0}));
      }
    }
  } else {
    let model = get_counter(CounterQueryBy::Url(path), &state.conn).await?;
    data.push(json!({
      "reaction0": model.reaction0,
      "reaction1": model.reaction1,
      "reaction2": model.reaction2,
      "reaction3": model.reaction3,
      "reaction4": model.reaction4,
      "reaction5": model.reaction5,
    }));
  }
  Ok(data)
}

pub async fn update_article(
  state: &AppState,
  action: Option<String>,
  path: String,
  r#type: String,
) -> Result<Vec<wl_counter::Model>, StatusCode> {
  let mut data = vec![];
  if r#type == "time" {
    let counter;
    if has_counter(CounterQueryBy::Url(path.to_owned()), &state.conn).await? {
      counter = get_counter(CounterQueryBy::Url(path.to_owned()), &state.conn).await?;
    } else {
      counter = create_counter(path, &state.conn).await?;
    }
    let mut active_counter = counter.into_active_model();
    active_counter.time = Set(Some(
      active_counter.time.take().unwrap_or(Some(0)).unwrap_or(0) + 1,
    ));
    data.push(
      active_counter
        .update(&state.conn)
        .await
        .map_err(AppError::from)?,
    )
  } else {
    fn set_reaction_value(
      mut counter: wl_counter::Model,
      reaction: &str,
      action: Option<String>,
    ) -> wl_counter::Model {
      match reaction {
        "reaction0" => {
          counter.reaction0 = if action.is_none() {
            Some(counter.reaction0.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction0.unwrap_or(1) - 1)
          }
        }
        "reaction1" => {
          counter.reaction1 = if action.is_none() {
            Some(counter.reaction1.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction1.unwrap_or(1) - 1)
          }
        }
        "reaction2" => {
          counter.reaction2 = if action.is_none() {
            Some(counter.reaction2.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction2.unwrap_or(1) - 1)
          }
        }
        "reaction3" => {
          counter.reaction3 = if action.is_none() {
            Some(counter.reaction3.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction3.unwrap_or(1) - 1)
          }
        }
        "reaction4" => {
          counter.reaction4 = if action.is_none() {
            Some(counter.reaction4.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction4.unwrap_or(1) - 1)
          }
        }
        "reaction5" => {
          counter.reaction5 = if action.is_none() {
            Some(counter.reaction5.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction5.unwrap_or(1) - 1)
          }
        }
        "reaction6" => {
          counter.reaction6 = if action.is_none() {
            Some(counter.reaction6.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction6.unwrap_or(1) - 1)
          }
        }
        "reaction7" => {
          counter.reaction7 = if action.is_none() {
            Some(counter.reaction7.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction7.unwrap_or(1) - 1)
          }
        }
        "reaction8" => {
          counter.reaction8 = if action.is_none() {
            Some(counter.reaction8.unwrap_or(0) + 1)
          } else {
            Some(counter.reaction8.unwrap_or(1) - 1)
          }
        }
        _ => {}
      }
      counter
    }
    let mut counter = get_counter(CounterQueryBy::Url(path), &state.conn).await?;
    counter = set_reaction_value(counter, &r#type, action);
    data.push(
      counter
        .into_active_model()
        .update(&state.conn)
        .await
        .map_err(AppError::from)?,
    );
  }
  Ok(data)
}
