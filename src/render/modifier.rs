use regex::Regex;

fn render_italic(lines: &mut Vec<String>) {
    let link_re = Regex::new(r"\*(?P<text>.+)\*").unwrap();

    for line in lines.iter_mut() {
        *line = line
            .split(" ")
            .map(|l| {
                let mut line = link_re.replace_all(l, "[3m$text[0m").to_string();
                line.push_str(" ");
                line
            })
            .collect();
    }
}

fn render_bold(lines: &mut Vec<String>) {
    let link_re = Regex::new(r"\*\*(?P<text>.+)\*\*").unwrap();

    for line in lines.iter_mut() {
        *line = line
            .split(" ")
            .map(|l| {
                let mut line = link_re.replace_all(l, "[4m$text[0m").to_string();
                line.push_str(" ");
                line
            })
            .collect();
    }
}

fn render_bold_italic(lines: &mut Vec<String>) {
    let link_re = Regex::new(r"\*\*(?P<text>.+)\*\*").unwrap();

    for line in lines.iter_mut() {
        *line = line
            .split(" ")
            .map(|l| {
                let mut line = link_re.replace_all(l, "[3m[4m$text[0m").to_string();
                line.push_str(" ");
                line
            })
            .collect();
    }
}

fn render_tick(lines: &mut Vec<String>) {
    let link_re = Regex::new(r"``?(?P<text>.+)``?").unwrap();

    for line in lines.iter_mut() {
        *line = line
            .split(" ")
            .map(|l| {
                let mut line = link_re
                    .replace_all(l, "[48;5;235m[38;5;220m$text[0m")
                    .to_string();
                line.push_str(" ");
                line
            })
            .collect();
    }
}

pub fn render(lines: &mut Vec<String>) {
    render_bold(lines);
    render_italic(lines);
    render_bold_italic(lines);
    render_tick(lines);
}
