use std::env::args;
use std::io::stdin;
use unicode_segmentation::UnicodeSegmentation;

use regex::Regex;

fn main() {
    let pattern: Regex = match args().nth(1) {
        None => panic!("usage: align PATTERN"),
        Some(pattern) => {
            Regex::new(&pattern).unwrap()
        }
    };
    println!("regex={:?}", pattern);
    let lines: Vec<(usize, String)> = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let i = match pattern.find(&line) {
                None => {
                    0
                }
                Some(m) => {
                    for (grapheme_offset, (i, _)) in line.grapheme_indices(true).enumerate() {
                        if i == m.start() {
                            return (grapheme_offset, line);
                        }
                    }
                    m.start()
                }
            };
            (i, line)
        })
        .collect();
    let max_indent = &lines.iter().map(|(x, _)| *x).max().unwrap_or(0);
    for (i, line) in lines {
        println!("{:<foo$}  {line}", "", foo = max_indent - i)
    }
}