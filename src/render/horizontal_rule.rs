use fancy_regex::Regex;
use term_size::dimensions;

fn draw_line() -> String {
    if let Some((w, _h)) = dimensions() {
        (0..w).map(|_| "─").collect::<String>()
    } else {
        " ".to_string()
    }
}

pub fn render(lines: &mut Vec<String>) {
    let regex = Regex::new(r"^---").unwrap();
    let alt_regex = Regex::new(r"^___").unwrap();
    let alt_regex2 = Regex::new(r"^\*\*\*").unwrap();

    for line in lines.iter_mut() {
        *line = regex.replace_all(line, draw_line()).to_string();
        *line = alt_regex.replace_all(line, draw_line()).to_string();
        *line = alt_regex2.replace_all(line, draw_line()).to_string();
    }
}
