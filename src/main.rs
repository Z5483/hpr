use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod parser;

fn main() {
    let path = &std::env::args().nth(1).expect("error: no path given");

    if Path::new(path).extension().and_then(OsStr::to_str) != Some("md") {
        eprintln!("error: filetype is not markdown");
        std::process::exit(1);
    }

    let file = File::open(path).expect("error: cannot read file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("error: cannot parse line"))
        .collect();
}
