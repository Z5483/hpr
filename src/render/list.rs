use fancy_regex::Regex;

pub fn render(lines: &mut Vec<String>) {
    let regex = Regex::new(r"^(?P<space>\s+)?(?P<char>[\+\-\*])\s(?P<line>.+)").unwrap();

    for line in lines.iter_mut() {
        *line = regex.replace_all(line, "$space• $line").to_string();
    }
}
