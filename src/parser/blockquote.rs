use fancy_regex::Regex;

pub struct BlockQuote {
    pub position: usize,
}

impl BlockQuote {
    pub fn find_blockquotes(lines: Vec<String>) -> Vec<BlockQuote> {
        let bq_regex = Regex::new(r"^>\s").unwrap();

        let mut pos: Vec<usize> = Vec::new();

        for (index, line) in lines.iter().enumerate() {
            if bq_regex.is_match(line).unwrap() {
                pos.push(index);
            }
        }

        let mut blockquote: Vec<BlockQuote> = Vec::new();

        for index in 0..pos.len() {
            blockquote.push(BlockQuote {
                position: pos[index],
            });
        }

        blockquote
    }
}
