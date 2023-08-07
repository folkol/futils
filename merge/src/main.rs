use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Lines, stdout, Write};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    filenames: Vec<String>,
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());
    // TODO: Perf analysis? Maybe stop allocating new Strings all the time.
    let args = Args::parse();
    let mut heads: Vec<(String, Lines<BufReader<File>>)> = Vec::new();
    for filename in args.filenames {
        let file = File::open(&filename).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        match lines.next() {
            Some(Ok(line)) => { heads.push((line, lines)) }
            _ => { eprintln!("file failed: {filename}"); }
        }
    }

    while !heads.is_empty() {
        let (i, _) = heads.iter().enumerate().min_by_key(|(_, (line, _))| line).unwrap();
        writeln!(out, "{}", heads[i].0).unwrap();
        let option = heads[i].1.next();
        match option {
            Some(Ok(line)) => {
                heads[i].0 = line;
            }
            _ => {
                // eprintln!("exhausted file: {i}");
                heads.remove(i);
            }
        }
    }
}
