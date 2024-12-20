use lettre::{
  message::header::ContentType, transport::smtp::authentication::Credentials, Message,
  SmtpTransport, Transport,
};
use strfmt::strfmt;

use crate::{config::Config, locales::get_translation};

/// Get the mail address prefix
pub fn extract_email_prefix(email: String) -> Option<String> {
  let mut res = email.split('@');
  res.next().map(|prefix| prefix.to_string())
}

struct SmtpConfig {
  host: &'static str,
  port: u16,
}

enum SmtpService {
  Gmail,
  NetEase126,
  NetEase163,
  QQ,
}

impl SmtpService {
  fn config(&self) -> SmtpConfig {
    match self {
      &SmtpService::QQ => SmtpConfig {
        host: "smtp.qq.com",
        port: 465,
      },
      SmtpService::Gmail => SmtpConfig {
        host: "smtp.gmail.com",
        port: 587,
      },
      SmtpService::NetEase126 => SmtpConfig {
        host: "smtp.126.com",
        port: 25,
      },
      SmtpService::NetEase163 => SmtpConfig {
        host: "smtp.163.com",
        port: 25,
      },
    }
  }
}

pub struct CommentNotification {
  pub sender_name: String,
  pub sender_email: String,
  pub comment_id: u32,
  pub comment: String,
  pub url: String,
  pub notify_type: NotifyType,
  pub lang: Option<String>,
}

pub enum NotifyType {
  NewComment,
  ReplyComment,
}

pub fn send_email_notification(notification: CommentNotification) {
  let app_config = Config::from_env().unwrap();
  let subject;
  let body;
  let post_url = format!(
    "{}{}#{}",
    app_config.site_url, notification.url, notification.comment_id
  );
  match notification.notify_type {
    NotifyType::NewComment => {
      let subject_template = get_translation(
        &notification.lang.clone().unwrap_or("en".to_owned()),
        "MAIL_SUBJECT_ADMIN",
      );
      let body_template = get_translation(
        &notification.lang.unwrap_or("en".to_owned()),
        "MAIL_TEMPLATE_ADMIN",
      );
      subject = strfmt!(&subject_template, site_name => app_config.site_name.clone()).unwrap();
      body =
        strfmt!(&body_template, site_url=> app_config.site_url, site_name=>app_config.site_name, nick=>notification.sender_name, comment=>notification.comment, post_url=>post_url)
          .unwrap();
    }
    NotifyType::ReplyComment => {
      subject = "".to_owned();
      body = "".to_owned();
    }
  }
  if app_config.author_email.is_none() {
    return;
  }
  email(app_config.author_email.unwrap(), subject, body);
}

pub fn email(reply_to: String, subject: String, body: String) {
  let app_config = Config::from_env().unwrap();
  let host;
  let port;
  if app_config.smtp_user.is_none() || app_config.smtp_pass.is_none() {
    return;
  }
  if app_config.smtp_host.is_some() || app_config.smtp_port.is_some() {
    host = app_config.smtp_host.unwrap();
    port = app_config.smtp_port.unwrap();
  } else if app_config.smtp_service.is_some() {
    let smtp_service = match app_config.smtp_service.unwrap().as_str() {
      "QQ" => SmtpService::QQ,
      "Gmail" => SmtpService::Gmail,
      "126" => SmtpService::NetEase126,
      "163" => SmtpService::NetEase163,
      _ => {
        tracing::error!("Unsupported SMTP service");
        return;
      }
    };
    host = smtp_service.config().host.to_owned();
    port = smtp_service.config().port;
  } else {
    return;
  }
  let email = Message::builder()
    .from(
      format!(
        "{} <{}>",
        app_config.site_name,
        app_config.smtp_user.clone().unwrap()
      )
      .parse()
      .unwrap(),
    )
    .reply_to(reply_to.parse().unwrap())
    .to(app_config.smtp_user.clone().unwrap().parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_HTML)
    .body(body)
    .unwrap();
  let mailer = SmtpTransport::relay(&host)
    .unwrap()
    .credentials(Credentials::new(
      app_config.smtp_user.unwrap(),
      app_config.smtp_pass.unwrap(),
    ))
    .port(port)
    .build();
  match mailer.send(&email) {
    Ok(v) => println!("{:#?}", v),
    Err(e) => tracing::error!("Could not send email: {e:?}"),
  }
}
