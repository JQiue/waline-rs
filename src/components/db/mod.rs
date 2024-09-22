mod handler;
mod model;
mod service;

use actix_web::web::ServiceConfig;

pub struct DBComponent {}

impl DBComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(handler::export_data);
    cfg.service(handler::create_data);
    cfg.service(handler::delete_data);
    cfg.service(handler::update_data);
  }
}
