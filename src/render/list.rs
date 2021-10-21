use fancy_regex::Regex;

pub fn render(lines: &mut Vec<String>) {
    let regex = Regex::new(r"^(?P<space>\s+)?(?P<char>[\+\-\*])\s(?P<line>.+)").unwrap();

    for line in lines.iter_mut() {
        if regex.is_match(line).unwrap() {
            let indent: usize = {
                let ch = line.replace(" ", "").chars().nth(0).unwrap();
                match ch {
                    '+' => line.split('+').nth(0).unwrap().len(),
                    '-' => line.split('-').nth(0).unwrap().len(),
                    '*' => line.split('*').nth(0).unwrap().len(),
                    _ => 0,
                }
            };

            let indent_char: String = match indent / 2 {
                0 => String::from("•"),
                1 => String::from("○"),
                2 => String::from("■"),
                3 => String::from("□"),
                4 => String::from("◙"),
                5 => String::from("◘"),
                _ => String::from("□"),
            };

            *line = regex
                .replace_all(line, "$space".to_owned() + &indent_char + " $line")
                .to_string();
        }
    }
}
