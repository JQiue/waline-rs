use lettre::{
  message::header::ContentType, transport::smtp::authentication::Credentials, Message,
  SmtpTransport, Transport,
};
use tracing_subscriber::fmt::format;

use crate::config::Config;

/// Get the mail address prefix
pub fn extract_email_prefix(email: String) -> Option<String> {
  let mut res = email.split('@');
  res.next().map(|prefix| prefix.to_string())
}

pub struct CommentNotification {
  pub sender_name: String,
  pub sender_email: String,
  pub comment_id: u32,
  pub comment: String,
  pub url: String,
  pub notify_type: NotifyType,
}

pub enum NotifyType {
  NewComment,
  ReplyComment,
}

pub fn send_email_notification(notification: CommentNotification) {}

pub fn send_site_message(notification: CommentNotification) {
  let app_config = Config::from_env().unwrap();
  match notification.notify_type {
    NotifyType::NewComment => {
      let post_url = format!(
        "{}{}#{}",
        app_config.site_url, notification.url, notification.comment_id
      );
      let subject = format!("{} 上有新评论了", app_config.site_name);
      let body = format!("<div style='border-top:2px solid #12ADDB;box-shadow:0 1px 3px #AAAAAA;line-height:180%;padding:0 15px 12px;margin:50px auto;font-size:12px;'>
       <h2 style='border-bottom:1px solid #DDD;font-size:14px;font-weight:normal;padding:13px 0 10px 8px;'> 您在<a style='text-decoration:none;color: #12ADDB;' href='{}' target='_blank'>{}</a>上的文章有了新的评论 </h2> <p><strong>{}</strong>回复说：</p><div style='background-color: #f5f5f5;padding: 10px 15px;margin:18px 0;word-wrap:break-word;'>{}</div><p>您可以点击<a style='text-decoration:none; color:#12addb' href='{}' target='_blank'>查看回复的完整內容</a></p><br/> </div>", app_config.site_url, app_config.site_name, notification.sender_name, notification.comment, post_url);
      message(app_config.author_email, subject, body);
    }
    NotifyType::ReplyComment => {}
  }
}

pub fn message(reply_to: String, subject: String, body: String) {
  let app_config = Config::from_env().unwrap();
  let email = Message::builder()
    .from(
      format!("{} <{}>", app_config.site_name, app_config.smtp_user)
        .parse()
        .unwrap(),
    )
    .reply_to(reply_to.parse().unwrap())
    .to(app_config.smtp_user.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_HTML)
    .body(body)
    .unwrap();
  let creds = Credentials::new(app_config.smtp_user, app_config.smtp_pass);
  let mailer = SmtpTransport::relay("smtp.qq.com")
    .unwrap()
    .credentials(creds)
    .build();
  match mailer.send(&email) {
    Ok(v) => println!("{:#?}", v),
    Err(e) => panic!("Could not send email: {e:?}"),
  }
}
