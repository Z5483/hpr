use fancy_regex::Regex;

pub fn render(lines: &mut Vec<String>) {
    let link_re = Regex::new(r"!?\[(?P<name>[><?@+'`~^%&\*\{\}.!#|\\\$';,:;=/\(\),\-\w\s+]+)\]\((?P<link>[><?@+'`~^%&\*\[\]\{\}.!#|\\\$';,:;=/,\-\w\s+]+)\)").unwrap();

    for line in lines.iter_mut() {
        *line = link_re
            .replace_all(line, "[4m[38;5;33m$name[0m")
            .to_string();
    }
}
