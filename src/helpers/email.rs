use lettre::{
  message::header::ContentType, transport::smtp::authentication::Credentials, Message,
  SmtpTransport, Transport,
};
use strfmt::strfmt;

use crate::{config::EnvConfig, locales::get_translation};

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

pub struct CommentNotification<'a> {
  pub sender_name: String,
  pub sender_email: String,
  pub comment_id: u32,
  pub comment: String,
  pub url: String,
  pub notify_type: NotifyType,
  pub lang: Option<&'a str>,
}

pub enum NotifyType {
  Notify,
  NewComment,
  _ReplyComment,
}

pub fn send_email_notification(notification: CommentNotification) {
  let EnvConfig {
    site_name,
    site_url,
    author_email,
    ..
  } = EnvConfig::load_env().unwrap();
  let to: &str;
  let reply_to;
  let subject;
  let body;
  let post_url = format!(
    "{}{}#{}",
    site_url, notification.url, notification.comment_id
  );
  if author_email.is_none() {
    return;
  }
  let author_email = author_email.unwrap();
  let lang = notification.lang.unwrap_or("en");
  match notification.notify_type {
    NotifyType::NewComment => {
      let subject_template = get_translation(lang, "MAIL_SUBJECT_ADMIN");
      let body_template = get_translation(lang, "MAIL_TEMPLATE_ADMIN");
      subject = strfmt!(&subject_template, site_name => site_name.clone()).unwrap();
      body =
        strfmt!(&body_template, site_url=> site_url, site_name=>site_name, nick=>notification.sender_name, comment=>notification.comment, post_url=>post_url)
          .unwrap();
      to = &author_email;
      reply_to = &author_email;
    }
    NotifyType::_ReplyComment => {
      subject = "".to_owned();
      body = "".to_owned();
      to = &notification.sender_email;
      reply_to = &author_email;
    }
    NotifyType::Notify => {
      let subject_template = get_translation(lang, "Registration Confirm Mail");
      let body_template = get_translation(lang, "confirm registration");
      subject = strfmt!(&subject_template, name => site_name.clone()).unwrap();
      body =
        strfmt!(&body_template, url=> notification.url.clone(), url=> notification.url).unwrap();
      tracing::debug!("Body: {:#?}", body);
      to = &notification.sender_email;
      reply_to = &author_email;
    }
  }
  mail(to, reply_to, &subject, body);
}

pub fn mail(to: &str, reply_to: &str, subject: &str, body: String) {
  let EnvConfig {
    site_name,
    smtp_service,
    smtp_host,
    smtp_port,
    smtp_user,
    smtp_pass,
    ..
  } = EnvConfig::load_env().unwrap();
  let host;
  let port;
  if smtp_user.is_none() || smtp_pass.is_none() {
    return;
  }
  if smtp_host.is_some() || smtp_port.is_some() {
    host = smtp_host.unwrap();
    port = smtp_port.unwrap();
  } else if smtp_service.is_some() {
    let smtp_service = match smtp_service.unwrap().as_str() {
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
  let msg = Message::builder()
    .from(
      format!("{} <{}>", site_name, smtp_user.clone().unwrap())
        .parse()
        .unwrap(),
    )
    .reply_to(reply_to.parse().unwrap())
    .to(to.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_HTML)
    .body(body)
    .unwrap();
  let mailer = SmtpTransport::relay(&host)
    .unwrap()
    .credentials(Credentials::new(smtp_user.unwrap(), smtp_pass.unwrap()))
    .port(port)
    .build();
  match mailer.send(&msg) {
    Ok(resp) => tracing::info!("{:#?}", resp),
    Err(e) => tracing::error!("Could not send email: {e:?}"),
  }
}
