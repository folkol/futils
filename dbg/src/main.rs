use env::args;
use std::env;
use std::io::{stdin, Write};

use anstream::stdout;
use chrono::{Duration, TimeZone, Utc};

use owo_colors::*;

fn main() {
    match args().nth(1) {
        Some(arg) if arg == "--help" || arg == "-h" => {
            println!("dbg: (pipeline debugger), prints samples of what's coming down the pipe");
            std::process::exit(0);
        }
        _ => {}
    }
    let mut when = Utc.timestamp_nanos(0);
    let mut scroller = ["|", "/", "-", "\\", "|", "/", "-"].iter().cycle();
    for (i, line) in stdin().lines().map_while(Result::ok).enumerate() {
        let now = Utc::now();
        if when < (Utc::now() - Duration::seconds(3)) {
            eprintln!("{}", format!("\r[#{}]: {}", (i + 1), line).dimmed());
            when = now;
        } else {
            eprint!("\r{}", scroller.next().unwrap());
            stdout().flush().unwrap();
        }
    }
}
