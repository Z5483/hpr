use fancy_regex::Regex;

fn render_italic(lines: &mut Vec<String>) {
    let regex = Regex::new(r"\*(?P<text>(?!\*).+)\*").unwrap();
    let alt_regex = Regex::new(r"\_(?P<text>(?!\_).+)\_").unwrap();

    for line in lines.iter_mut() {
        *line = regex.replace_all(line, "[3m$text[0m").to_string();
        *line = alt_regex.replace_all(line, "[3m$text[0m").to_string();
    }
}

fn render_bold(lines: &mut Vec<String>) {
    let regex = Regex::new(r"\*\*(?P<text>(?!\*).+)\*\*").unwrap();
    let alt_regex = Regex::new(r"\_\_(?P<text>(?!\_).+)\_\_").unwrap();

    for line in lines.iter_mut() {
        *line = regex.replace_all(line, "[1m$text[0m").to_string();
        *line = alt_regex.replace_all(line, "[1m$text[0m").to_string();
    }
}

fn render_bold_italic(lines: &mut Vec<String>) {
    let regex = Regex::new(r"\*\*\*(?P<text>(?!\*).+)\*\*\*").unwrap();
    let alt_regex = Regex::new(r"\_\_\_(?P<text>(?!\_).+)\_\_\_").unwrap();

    for line in lines.iter_mut() {
        *line = regex.replace_all(line, "[3m[1m$text[0m").to_string();
        *line = alt_regex.replace_all(line, "[3m[1m$text[0m").to_string();
    }
}

fn render_inline(lines: &mut Vec<String>) {
    let regex = Regex::new(r"`(?P<text>(?!`).+)`").unwrap();

    for line in lines.iter_mut() {
        *line = regex
            .replace_all(line, "[48;5;235m[38;5;220m$text[0m")
            .to_string();
    }
}

pub fn render(lines: &mut Vec<String>) {
    render_bold_italic(lines);
    render_bold(lines);
    render_italic(lines);
    render_inline(lines);
}
