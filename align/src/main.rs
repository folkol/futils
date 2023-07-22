use std::env::args;
use std::io::stdin;

use regex::Regex;

fn main() {
    let pattern: Regex = match args().nth(1) {
        None => panic!("usage: align PATTERN"),
        Some(pattern) => Regex::new(&pattern).unwrap(),
    };
    let lines: Vec<(usize, String)> = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            (
                match pattern.find(&line) {
                    None => 0,
                    Some(m) => m.start(),
                },
                line,
            )
        })
        .collect();
    let max_indent = &lines.iter().map(|(x, _)| *x).max().unwrap_or(0);
    for (i, line) in lines {
        println!("{:<foo$}  {line:>foo$}", "", foo = (max_indent - i))
    }
}
