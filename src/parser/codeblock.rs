use regex::Regex;

pub struct CodeBlock {
    pub header: String,
    pub start: usize,
    pub end: usize,
}

impl CodeBlock {
    pub fn find_blocks(lines: Vec<String>) -> Vec<CodeBlock> {
        let cb_re = Regex::new(r"^```[(\s+)?(\w+)]?").unwrap();

        let mut cb_total: Vec<usize> = Vec::new();

        let mut cb_start: Vec<usize> = Vec::new();
        let mut cb_end: Vec<usize> = Vec::new();

        let mut header: Vec<String> = Vec::new();

        for (index, line) in lines.iter().enumerate() {
            if cb_re.is_match(line) {
                cb_total.push(index);
            }
        }

        for (index, pos) in cb_total.iter().enumerate() {
            if index % 2 == 0 {
                cb_start.push(*pos);
            } else if index % 2 == 1 {
                cb_end.push(*pos);
            }
        }

        for (index, line) in lines.iter().enumerate() {
            if cb_start.iter().any(|&i| i == index) {
                if line.chars().into_iter().any(char::is_alphanumeric) {
                    let mut token = line.clone();
                    token.retain(|c| !c.is_whitespace());
                    header.push(token.replace("`", "").to_string());
                } else {
                    header.push(" ".to_string());
                }
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
