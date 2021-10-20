use fancy_regex::Regex;

pub fn render(lines: &mut Vec<String>) {
    let regex = Regex::new(r"^#+(\s+)?(?P<text>.+)").unwrap();
    let hash_regex = Regex::new(r"#").unwrap();

    for line in lines {
        *line = regex
            .replace_all(line, {
                let count = hash_regex.find_iter(line).count();
                let mut text: String = String::new();

                text.push_str("[38;5;122m");

                for _ in 0..count {
                    text.push_str("█");
                }

                text.push_str(" $text[0m");

                text
            })
            .to_string();
    }
}
