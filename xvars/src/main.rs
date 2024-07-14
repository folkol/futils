use std::error::Error;
use std::io::ErrorKind;

use clap::Parser;

/// E(x)tract variables (`foo=bar` things) from stdin
#[derive(Parser, Debug)]
struct Args {
    variable_names: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut patterns = Vec::new();
    for name in args.variable_names {
        let pattern = format!(r"{}=(\w+)", name);
        patterns.push(regex::Regex::new(&pattern)?);
    }
    for maybe_line in std::io::stdin().lines() {
        match maybe_line {
            Err(e) if e.kind() == ErrorKind::BrokenPipe => return Ok(()),
            Err(e) => {
                eprintln!("Unexpected error reading from stdin: '{}'", e);
                return Ok(());
            }
            Ok(line) => {
                let hits: Vec<_> = patterns
                    .iter()
                    .map(|p| match_or_default(line.as_str(), p))
                    .collect();
                println!("{}", hits.join("\t"));
            }
        }
    }
    Ok(())
}

fn match_or_default<'a>(line: &'a str, p: &regex::Regex) -> &'a str {
    match p.captures(line) {
        None => "",
        Some(captures) if captures.get(1).is_some() => captures.get(1).unwrap().as_str(),
        Some(captures) => {
            eprintln!("Unexpected capture ({:?}) in line ({:?})", captures, line);
            ""
        }
    }
}
