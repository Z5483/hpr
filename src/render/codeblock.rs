use std::fs;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

use fancy_regex::Regex;
use term_size::dimensions;

struct CodeBlock {
    header: String,
    start: usize,
    end: usize,
}

fn find_blocks(lines: Vec<String>) -> Vec<CodeBlock> {
    let cb_re = Regex::new(r"^```[(\s+)?(\w+)]?").unwrap();

    let mut cb_total: Vec<usize> = Vec::new();

    let mut cb_start: Vec<usize> = Vec::new();
    let mut cb_end: Vec<usize> = Vec::new();

    let mut header: Vec<String> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if cb_re.is_match(line).unwrap() {
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

pub fn render(lines: &mut Vec<String>) {
    let file = fs::read_to_string("/home/khue/.base16").unwrap();
    let scheme = file.split_whitespace().nth(0).unwrap();
    let theme = "/home/khue/code/hpr/theme/base16-".to_owned() + &scheme + ".tmTheme";

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::get_theme(theme).unwrap();

    let codeblock = find_blocks(lines.to_vec());

    for block in codeblock.iter() {
        let syntax = ps
            .find_syntax_by_token(&block.header)
            .unwrap_or_else(|| ps.find_syntax_by_token("txt").unwrap());
        let mut h = HighlightLines::new(syntax, &ts);
        for index in block.start..block.end {
            if let Some((w, _h)) = dimensions() {
                if lines[index].len() < w {
                    for _ in 0..(w - lines[index].len()) {
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
