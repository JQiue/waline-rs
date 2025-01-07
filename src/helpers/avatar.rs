use helpers::time::utc_now;

pub fn get_avatar(plain: &str) -> String {
  let re = regex::Regex::new(r"^\d+@qq\.com$").unwrap();
  if re.is_match(plain) {
    let number = plain.split("@").next().unwrap();
    format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", number)
  } else {
    format!("https://api.multiavatar.com/{}.png", utc_now())
  }
}
