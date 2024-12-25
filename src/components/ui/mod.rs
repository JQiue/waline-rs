mod handler;
mod model;
mod service;

use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(handler::ui_profile_page);
  cfg.service(handler::ui_login_page);
  cfg.service(handler::ui_migration_page);
}
