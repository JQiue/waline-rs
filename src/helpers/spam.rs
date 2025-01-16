use instant_akismet::{AkismetClient, AkismetOptions, CheckResult, Comment};

use crate::{config::Config, error::AppError};

pub async fn check_comment(
  author: String,
  email: String,
  ip: String,
  content: String,
) -> Result<CheckResult, AppError> {
  let app_config = Config::from_env()?;
  let akismet_client = AkismetClient::new(
    String::from(app_config.site_url), // The URL for your blog
    app_config.akismet_key,            // Your Akismet API key
    reqwest::Client::new(),            // Reqwest client to use for requests
    AkismetOptions::default(),         // AkismetOptions config
  );
  akismet_client.verify_key().await?;
  let comment = Comment::new(akismet_client.blog.as_ref(), &ip)
    .comment_author(&author)
    .comment_author_email(&email)
    .comment_content(&content);
  let result = akismet_client.check_comment(comment).await?;
  tracing::debug!("{:#?}", result);
  Ok(result)
}
