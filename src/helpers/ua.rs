use woothee::parser::Parser;
pub fn parse(ua: String) -> (String, String) {
  let parser = Parser::new();
  match parser.parse(&ua) {
    Some(result) => (
      format!("{}{}", result.name, result.version),
      result.os.to_string(),
    ),
    None => ("".to_string(), "".to_string()),
  }
}
