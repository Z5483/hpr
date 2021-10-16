use crate::parser::BlockQuote;

pub fn render(lines: &mut Vec<String>) {
    let blockquote_pos = BlockQuote::find_blockquotes(lines.to_vec());

    for block in blockquote_pos.iter() {
        lines[block.position] = "[38;5;242m".to_owned() + &lines[block.position].clone().to_owned();
        lines[block.position].push_str("[0m");
    }
}
