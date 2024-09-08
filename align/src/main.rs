use colored::{ColoredString, Colorize};
use std::env::args;
use std::io::stdin;
use std::io::IsTerminal;

use regex::{Regex, RegexBuilder};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let regex: Regex = match args().nth(1) {
        None => panic!("usage: align PATTERN"),
        Some(pattern) => RegexBuilder::new(&pattern)
            .unicode(true)
            .case_insensitive(true)
            .build()
            .unwrap(),
    };

    let mut max_indent = 0;
    let lines: Vec<(usize, usize, usize, String)> = stdin()
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            regex.find(&line).map(|m| {
                let my_line = line.clone();
                let (indent, begin, end) = 'block: {
                    let grapheme_indices =
                        line.grapheme_indices(true).enumerate().collect::<Vec<_>>();
                    for (indent, (i, _)) in grapheme_indices.into_iter() {
                        if i == m.start() {
                            max_indent = max_indent.max(indent);
                            break 'block (indent, m.start(), m.end());
                        }
                    }
                    (0, 0, 0)
                };
                (indent, begin, end, my_line)
            })
        })
        .collect();

    for (current_indent, begin, end, line) in lines {
        let alignment_match: ColoredString = if std::io::stdout().is_terminal() {
            line[begin..end].green().bold()
        } else {
            line[begin..end].into()
        };
        let indent = max_indent - current_indent;
        println!(
            "{:<indent$}  {}{}{}",
            "",
            &line[..begin],
            alignment_match,
            &line[end..],
        )
    }
}
