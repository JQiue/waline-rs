use pulldown_cmark::{self, Event};

/// Render markdown to html
pub fn render_md_to_html(markdown: &str) -> String {
  let parser = pulldown_cmark::Parser::new(markdown);
  let parser = parser.map(|event| match event {
    Event::SoftBreak => Event::HardBreak,
    _ => event,
  });
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);
  html_output
}
