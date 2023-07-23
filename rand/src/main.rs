use std::fmt::Display;
use std::io::ErrorKind::BrokenPipe;
use std::io::{stdout, BufWriter, Write};

use clap::Parser;
use rand::distributions::Distribution;
use rand_distr::{Normal, Uniform};

#[derive(Parser)]
struct Args {
    #[arg(long, group = "distribution", action)]
    uniform: bool,

    #[arg(long, group = "distribution", action)]
    normal: bool,
}

fn main() {
    let args = Args::parse();
    let rng = rand::thread_rng();
    if args.uniform {
        shoot(Uniform::new(0, 1000).sample_iter(rng))
    } else if args.normal {
        shoot(
            Normal::new(0., 1.)
                .unwrap()
                .sample_iter(rng)
                .map(|x| 500 + (x * 250.) as isize)
                .filter(|x| *x > 0 && *x < 1000),
        )
    } else {
        shoot(Uniform::new(0, 1000).sample_iter(rng))
    };
}

fn shoot<T, U>(t: T)
where
    T: Iterator<Item = U>,
    U: Display,
{
    let mut out = BufWriter::new(stdout().lock());
    for x in t {
        match writeln!(out, "{x}") {
            Ok(_) => {}
            Err(e) if e.kind() == BrokenPipe => break,
            Err(_) => panic!("Unexpected error"),
        }
    }
}
