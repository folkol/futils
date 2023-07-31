use std::env::args;
use std::io::ErrorKind::BrokenPipe;
use std::io::{stdout, Write};

use rand::thread_rng;
use rand::Rng;

fn main() {
    let regex = match args().nth(1) {
        None => {
            eprintln!("usage: gen-strs SOME_REGEX");
            std::process::exit(1);
        }
        Some(pattern) => pattern,
    };
    let mut rng = thread_rng();
    let mut parser = regex_syntax::ParserBuilder::new()
        .unicode(false)
        .allow_invalid_utf8(true)
        .build();
    let hir = parser.parse(&regex).unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();

    for s in (&mut rng).sample_iter(&gen).filter_map(Result::ok) {
        match writeln!(stdout(), "{s}") {
            Ok(_) => {}
            Err(e) if e.kind() == BrokenPipe => break,
            Err(e) => panic!("{e}"),
        }
    }
}
