use fancy_regex::Regex;

pub fn render(lines: &mut Vec<String>) {
    let link_re = Regex::new(r"!?\[(?P<name>.+)\]\((?P<link>.+)\)").unwrap();

    for line in lines.iter_mut() {
        *line = line.split(" ").map(|l| {
            let mut line = link_re.replace_all(l, "[4m[38;5;33m$name[0m").to_string();
            line.push_str(" ");
            line
        }).collect();
    }
}
