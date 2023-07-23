use std::collections::HashSet;
use std::io::{BufWriter, stdin, stdout, Write};
use std::io::ErrorKind::BrokenPipe;

fn main() {
    let mut out = BufWriter::new(stdout().lock());
    let mut seen: HashSet<String> = HashSet::new();
    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        if seen.contains(&line) {
            continue;
        }
        match writeln!(out, "{line}") {
            Ok(_) => {}
            Err(e) if e.kind() == BrokenPipe => {}
            Err(_) => panic!("Unexpected error"),
        }
        seen.insert(line);
    }
}
