use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::{json, Value};
use wl_counter::Model;

use crate::{
  components::article::model::*,
  entities::{prelude::*, *},
  AppState,
};

pub async fn get_article(
  state: &AppState,
  path: String,
  r#type: String,
  _lang: String,
) -> Result<Vec<Value>, String> {
  if r#type == "time" {
    let mut data = vec![];
    for path in path.split(',') {
      let model = WlCounter::find()
        .filter(wl_counter::Column::Url.eq(path))
        .one(&state.conn)
        .await
        .unwrap();
      if let Some(model) = model {
        data.push(json!({
          "time": model.time.unwrap(),
        }));
      } else {
        data.push(json!({ "time": 0 }))
      }
    }
    Ok(data)
  } else {
    let mut data = vec![];
    let model = WlCounter::find()
      .filter(wl_counter::Column::Url.eq(path))
      .one(&state.conn)
      .await
      .unwrap();

    if let Some(model) = model {
      data.push(json!({
        "reaction0": model.reaction0,
        "reaction1": model.reaction1,
        "reaction2": model.reaction2,
        "reaction3": model.reaction3,
        "reaction4": model.reaction4,
        "reaction5": model.reaction5,
      }));
    } else {
      data.push(json!({
        "reaction0": 0,
        "reaction1": 1,
        "reaction2": 2,
        "reaction3": 3,
        "reaction4": 4,
        "reaction5": 5,
      }));
    }
    Ok(data)
  }
}

pub async fn update_article(
  state: &AppState,
  action: Option<String>,
  path: String,
  r#type: String,
  _lang: String,
) -> Result<Vec<Model>, String> {
  let mut data = vec![];
  if r#type == "time" {
    if has_counter(path.clone(), &state.conn).await {
      let model = WlCounter::find()
        .filter(wl_counter::Column::Url.eq(path.clone()))
        .one(&state.conn)
        .await
        .unwrap()
        .unwrap();
      let model = wl_counter::ActiveModel {
        id: Set(model.id),
        time: Set(Some(model.time.unwrap() + 1)),
        ..Default::default()
      };
      data.push(WlCounter::update(model).exec(&state.conn).await.unwrap());
    } else {
      let model = wl_counter::ActiveModel {
        url: Set(path.clone()),
        time: Set(Some(1)),
        ..Default::default()
      };
      WlCounter::insert(model).exec(&state.conn).await.unwrap();
      data.push(
        WlCounter::find()
          .filter(wl_counter::Column::Url.eq(path.clone()))
          .one(&state.conn)
          .await
          .unwrap()
          .unwrap(),
      )
    }
  } else {
    fn set_reaction_value(
      mut model: wl_counter::ActiveModel,
      counter: wl_counter::Model,
      reaction: &str,
      action: Option<String>,
    ) -> wl_counter::ActiveModel {
      match reaction {
        "reaction0" => {
          model.reaction0 = if counter.reaction0.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction0.unwrap() + 1))
          } else {
            Set(Some(counter.reaction0.unwrap() - 1))
          }
        }
        "reaction1" => {
          model.reaction1 = if counter.reaction1.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction1.unwrap() + 1))
          } else {
            Set(Some(counter.reaction1.unwrap() - 1))
          }
        }
        "reaction2" => {
          model.reaction2 = if counter.reaction2.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction2.unwrap() + 1))
          } else {
            Set(Some(counter.reaction2.unwrap() - 1))
          }
        }
        "reaction3" => {
          model.reaction3 = if counter.reaction3.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction3.unwrap() + 1))
          } else {
            Set(Some(counter.reaction3.unwrap() - 1))
          }
        }
        "reaction4" => {
          model.reaction4 = if counter.reaction4.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction4.unwrap() + 1))
          } else {
            Set(Some(counter.reaction4.unwrap() - 1))
          }
        }
        "reaction5" => {
          model.reaction5 = if counter.reaction5.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction5.unwrap() + 1))
          } else {
            Set(Some(counter.reaction5.unwrap() - 1))
          }
        }
        "reaction6" => {
          model.reaction6 = if counter.reaction6.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction6.unwrap() + 1))
          } else {
            Set(Some(counter.reaction6.unwrap() - 1))
          }
        }
        "reaction7" => {
          model.reaction7 = if counter.reaction7.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction7.unwrap() + 1))
          } else {
            Set(Some(counter.reaction7.unwrap() - 1))
          }
        }
        "reaction8" => {
          model.reaction8 = if counter.reaction8.is_none() {
            Set(Some(1))
          } else if action.is_none() {
            Set(Some(counter.reaction8.unwrap() + 1))
          } else {
            Set(Some(counter.reaction8.unwrap() - 1))
          }
        }
        _ => {}
      }
      model
    }
    if has_counter(path.clone(), &state.conn).await {
      let counter = WlCounter::find()
        .filter(wl_counter::Column::Url.eq(path.clone()))
        .one(&state.conn)
        .await
        .unwrap()
        .unwrap();
      let mut model = wl_counter::ActiveModel {
        id: Set(counter.id),
        ..Default::default()
      };
      model = set_reaction_value(model, counter, &r#type, action);
      data.push(WlCounter::update(model).exec(&state.conn).await.unwrap());
    } else {
      let mut model = wl_counter::ActiveModel {
        ..Default::default()
      };
      match r#type.as_str() {
        "reaction0" => model.reaction0 = Set(Some(0)),
        "reaction1" => model.reaction1 = Set(Some(0)),
        "reaction2" => model.reaction2 = Set(Some(0)),
        "reaction3" => model.reaction3 = Set(Some(0)),
        "reaction4" => model.reaction4 = Set(Some(0)),
        "reaction5" => model.reaction5 = Set(Some(0)),
        "reaction6" => model.reaction6 = Set(Some(0)),
        "reaction7" => model.reaction7 = Set(Some(0)),
        "reaction8" => model.reaction8 = Set(Some(0)),
        _ => {}
      }
      WlCounter::insert(model).exec(&state.conn).await.unwrap();
      data.push(
        WlCounter::find()
          .filter(wl_counter::Column::Url.eq(path.clone()))
          .one(&state.conn)
          .await
          .unwrap()
          .unwrap(),
      )
    }
  }
  Ok(data)
}
