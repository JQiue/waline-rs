use instant_akismet::{AkismetClient, AkismetOptions, CheckResult, Comment};

use crate::{config::EnvConfig, error::AppError};

pub async fn check_comment(
  author: String,
  email: String,
  ip: String,
  content: String,
) -> Result<CheckResult, AppError> {
  let EnvConfig {
    site_url,
    akismet_key,
    ..
  } = EnvConfig::load_env()?;
  let akismet_client = AkismetClient::new(
    String::from(site_url),    // The URL for your blog
    akismet_key,               // Your Akismet API key
    reqwest::Client::new(),    // Reqwest client to use for requests
    AkismetOptions::default(), // AkismetOptions config
  );
  akismet_client.verify_key().await?;
  let comment = Comment::new(akismet_client.blog.as_ref(), &ip)
    .comment_author(&author)
    .comment_author_email(&email)
    .comment_content(&content);
  let result = akismet_client.check_comment(comment).await?;
  tracing::info!("Comment is {:#?}", result);
  Ok(result)
}
