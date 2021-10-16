pub fn render(lines: &mut Vec<String>) {
    super::codeblock::render(lines);
    super::blockquote::render(lines);
    super::link::render(lines);
    super::header::render(lines);
    super::modifier::render(lines);
}
