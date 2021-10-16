use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

use crate::parser::CodeBlock;
use term_size::dimensions;

pub fn render(lines: &mut Vec<String>) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::get_theme("theme/base16-monokai.tmTheme").unwrap();

    let codeblock = CodeBlock::find_blocks(lines.to_vec());

    for block in codeblock.iter() {
        let syntax = ps
            .find_syntax_by_token(&block.header)
            .unwrap_or_else(|| ps.find_syntax_by_token("txt").unwrap());
        let mut h = HighlightLines::new(syntax, &ts);
        for index in block.start..block.end {
            if let Some((w, _h)) = dimensions() {
                if lines[index].len() < w {
                    for _ in 0..(w - lines[index].len() - 4) {
                        lines[index].push_str(" ");
                    }
                }
            } else {
                println!("Unable to get term size :(")
            }

            let ranges: Vec<(Style, &str)> = h.highlight(&lines[index], &ps);
            lines[index] = as_24_bit_terminal_escaped(&ranges[..], true);
            lines[index].push_str("[0m");
        }
    }

    for block in codeblock.iter().rev() {
        lines.remove(block.end);
        lines.remove(block.start);
    }
}
