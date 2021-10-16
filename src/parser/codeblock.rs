use regex::Regex;

pub struct CodeBlock {
    pub header: String,
    pub start: usize,
    pub end: usize,
}

impl CodeBlock {
    pub fn find_blocks(lines: Vec<String>) -> Vec<CodeBlock> {
        let cb_start_re = Regex::new(r"^```[\s+]?\w+").unwrap();
        let cb_end_re = Regex::new(r"^```").unwrap();

        let mut cb_start: Vec<usize> = Vec::new();
        let mut cb_end: Vec<usize> = Vec::new();
        let mut header: Vec<String> = Vec::new();

        for (index, line) in lines.iter().enumerate() {
            if cb_start_re.is_match(line) {
                cb_start.push(index);

                let mut token = line.clone();
                token.retain(|c| !c.is_whitespace());
                header.push(token.replace("`", "").to_string());
            } else if cb_end_re.is_match(line) {
                cb_end.push(index);
            }
        }

        let mut codeblock: Vec<CodeBlock> = Vec::new();

        for index in 0..cb_start.len() {
            codeblock.push(CodeBlock {
                header: header[index].clone(),
                start: cb_start[index],
                end: cb_end[index],
            });
        }

        codeblock
    }
}
