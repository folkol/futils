use std::io::stdin;
use std::thread;
use std::time::Duration;

use clap::Parser;

/// Shoves [frequency] lines per second from stdin to stdout
#[derive(Parser)]
struct Args {
    /// Max frequency of lines to shove
    #[arg(default_value_t = 10.)]
    frequency: f64,
}

fn main() {
    let args: Args = Args::parse();
    if args.frequency <= 0. {
        eprintln!("Frequency must be positive");
        std::process::exit(1);
    }
    for line in stdin().lines().map_while(Result::ok) {
        println!("{line}");
        thread::sleep(Duration::from_secs_f64(1. / args.frequency));
    }
}
