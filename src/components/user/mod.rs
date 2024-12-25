mod handler;
pub mod model;
mod service;

use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(handler::user_login);
  cfg.service(handler::user_logout);
  cfg.service(handler::user_register);
  cfg.service(handler::get_login_user_info);
  cfg.service(handler::set_user_type);
  cfg.service(handler::set_user_profile);
  cfg.service(handler::set_2fa);
  cfg.service(handler::get_2fa);
  cfg.service(handler::verification);
  cfg.service(handler::get_user_info);
}
