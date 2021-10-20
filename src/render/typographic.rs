use fancy_regex::{escape, Regex};

fn sub_char(lines: &mut Vec<String>, regex: &str, sub: &str) {
    let regex = Regex::new(&escape(regex)).unwrap();

    for line in lines.iter_mut() {
        *line = regex.replace_all(line, sub.clone()).to_string();
    }
}

pub fn render(lines: &mut Vec<String>) {
    sub_char(lines, r"(c)", "©");
    sub_char(lines, r"(C)", "©");
    sub_char(lines, r"(r)", "®");
    sub_char(lines, r"(R)", "®");
    sub_char(lines, r"(tm)", "™");
    sub_char(lines, r"(TM)", "™");
    sub_char(lines, r"(p)", "§");
    sub_char(lines, r"(P)", "§");
    sub_char(lines, r"+-", "±");
}
