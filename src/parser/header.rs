use regex::Regex;

pub struct Header {
    pub position: usize,
}

impl Header {
    pub fn find_headers(lines: Vec<String>) -> Vec<Header> {
        let header_re = Regex::new(r"^#").unwrap();

        let mut pos: Vec<usize> = Vec::new();

        for (index, line) in lines.iter().enumerate() {
            if header_re.is_match(line) {
                pos.push(index);
            }
        }

        let mut blockquote: Vec<Header> = Vec::new();

        for index in 0..pos.len() {
            blockquote.push(Header {
                position: pos[index],
            });
        }

        blockquote
    }
}
