pub trait LoggingResultErr<T, E> {
  fn log_err(self) -> Result<T, E>;
}

impl<T, E: std::fmt::Debug> LoggingResultErr<T, E> for Result<T, E> {
  fn log_err(self) -> Result<T, E> {
    if let Err(ref e) = self {
      tracing::error!("{:?}", e);
    }
    self
  }
}
