use std::io::ErrorKind::BrokenPipe;
use std::io::{stdin, stdout, Write};

use chrono::SecondsFormat;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Prefix, can contain placeholders (%t = datetime, %d = timedelta, %n = lineno)
    #[arg(default_value = "")]
    prefix: String,
}

fn main() {
    let args = Args::parse();
    let mut prev = chrono::Local::now();
    for (i, line) in stdin().lines().map_while(Result::ok).enumerate() {
        let now = chrono::Local::now();
        let prefix = args.prefix.replace("%n", &i.to_string());
        let prefix = prefix.replace("%t", &now.to_rfc3339_opts(SecondsFormat::Secs, false));
        let elapsed = (now - prev).num_microseconds().unwrap() as f64 / 1e6;
        let prefix = prefix.replace("%d", &elapsed.to_string());
        match writeln!(stdout(), "{}{}", prefix, line) {
            Ok(_) => {}
            Err(e) if e.kind() == BrokenPipe => break,
            Err(_) => panic!(),
        }
        prev = now;
    }
}
