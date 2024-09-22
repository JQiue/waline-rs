use chrono::Utc;

pub fn get_current_utc_time() -> chrono::DateTime<Utc> {
  Utc::now()
}
