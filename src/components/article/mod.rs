mod handler;
mod model;
mod service;

use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(handler::get_article);
  cfg.service(handler::update_article);
}
