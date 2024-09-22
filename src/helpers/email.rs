/// Get the mail address prefix
pub fn extract_email_prefix(email: String) -> Option<String> {
  let mut res = email.split('@');
  res.next().map(|prefix| prefix.to_string())
}
