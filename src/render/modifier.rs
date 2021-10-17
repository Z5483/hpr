use fancy_regex::Regex;

fn render_italic(lines: &mut Vec<String>) {
    let link_re =
        Regex::new(r"\*(?P<text>[><?@+'`~^%&\[\]\{\}.!#|\\\$';,:;=/\(\),\-\w\s+]+)\*").unwrap();

    for line in lines.iter_mut() {
        *line = link_re.replace_all(line, "[3m$text[0m").to_string();
    }
}

fn render_bold(lines: &mut Vec<String>) {
    let link_re =
        Regex::new(r"\*\*(?P<text>[><?@+'`~^%&\[\]\{\}.!#|\\\$';,:;=/\(\),\-\w\s+]+)\*\*").unwrap();

    for line in lines.iter_mut() {
        *line = link_re.replace_all(line, "[1m$text[0m").to_string();
    }
}

fn render_bold_italic(lines: &mut Vec<String>) {
    let link_re =
        Regex::new(r"\*\*\*(?P<text>[><?@+'`~^%&\[\]\{\}.!#|\\\$';,:;=/\(\),\-\w\s+]+)\*\*\*")
            .unwrap();

    for line in lines.iter_mut() {
        *line = link_re.replace_all(line, "[3m[1m$text[0m").to_string();
    }
}

fn render_tick(lines: &mut Vec<String>) {
    let link_re =
        Regex::new(r"``?(?P<text>[><?@+'~^%&\*\{\}\[\].!#|\\\$';,:;=/\(\),\-\w\s+]+)``?").unwrap();

    for line in lines.iter_mut() {
        *line = link_re
            .replace_all(line, "[48;5;235m[38;5;220m$text[0m")
            .to_string();
    }
}

pub fn render(lines: &mut Vec<String>) {
    render_bold_italic(lines);
    render_bold(lines);
    render_italic(lines);
    render_tick(lines);
}
