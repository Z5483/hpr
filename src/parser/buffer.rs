use fancy_regex::Regex;

pub struct Buffer {
    pub contents: Vec<String>,
    pub num_of_page: usize,
    pub num_of_line: Vec<usize>,
}

impl Buffer {
    pub fn new(lines: Vec<String>) -> Buffer {
        let re = Regex::new(r"(^#)#+").unwrap();
        let num_of_page: usize = lines.iter().filter(|l| re.is_match(l).unwrap()).count();

        let mut header_pos: Vec<usize> = Vec::new();
        for (index, l) in lines.iter().enumerate() {
            if re.is_match(l).unwrap() {
                header_pos.push(index);
            }
        }

        let mut contents: Vec<String> = vec![String::new(); header_pos.len()];
        for (index, position) in header_pos.iter().enumerate() {
            let begin: usize = *position;
            let end: usize = if index + 1 < header_pos.len() {
                header_pos[index + 1]
            } else {
                lines.len()
            };

            for line_pos in begin..end {
                contents[index].push_str(&lines[line_pos]);
                contents[index].push_str("\n");
            }
        }

        let mut num_of_line: Vec<usize> = Vec::new();
        for page in contents.iter() {
            num_of_line.push(page.matches("\n").count());
        }

        Buffer {
            contents,
            num_of_page,
            num_of_line,
        }
    }
}
