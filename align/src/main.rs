use std::env::args;
use std::io::stdin;
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

use regex::Regex;

fn main() {
    let pattern: Regex = match args().nth(1) {
        None => panic!("usage: align PATTERN"),
        Some(pattern) => {
            println!("pattern={pattern}");
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
                    println!("no match");
                    0
                }
                Some(m) => {
                    println!("match, i={}", m.start());
                    for (grapheme_offset, (i, _)) in line.grapheme_indices(true).enumerate() {
                        if i == m.start() {
                            println!("Found byte offset of grapheme cluster");
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
    println!("max_indent={max_indent}");
    for (i, line) in lines {
        println!("i={i} line={line}");
        println!("{:<foo$}  {line}", "", foo = max_indent - i)
    }
}