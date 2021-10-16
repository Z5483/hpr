use crate::parser::Header;
use regex::Regex;

pub fn render(lines: &mut Vec<String>) {
    let header_pos = Header::find_headers(lines.to_vec());
    let header_char = Regex::new(r"#").unwrap();

    for heading in header_pos.iter() {
        lines[heading.position] = header_char
            .replace_all(&lines[heading.position], "█")
            .to_string();
        lines[heading.position] =
            "[38;5;122m".to_owned() + &lines[heading.position].clone().to_owned();
        lines[heading.position].push_str("[0m");
    }
}
