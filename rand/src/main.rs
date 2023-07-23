use std::fmt::Display;
use std::io::ErrorKind::BrokenPipe;
use std::io::{stdout, BufWriter, Write};

use clap::Parser;
use rand::distributions::Distribution;
use rand_distr::{Normal, Uniform};

#[derive(Parser)]
struct Args {
    /// parameters: low (0), hi (1000)
    #[arg(short, long, group = "distribution", action)]
    uniform: bool,

    /// parameters: mu (100), sigma (15)
    #[arg(short, long, group = "distribution", action)]
    normal: bool,

    /// Distribution-specific parameters with some defaults
    #[arg()]
    params: Vec<isize>,
}

fn main() {
    let args = Args::parse();
    let rng = rand::thread_rng();

    if args.normal {
        let mu: f64 = *args.params.first().unwrap_or(&100) as f64;
        let sigma: f64 = *args.params.get(1).unwrap_or(&15) as f64;
        shoot(
            Normal::new(mu as f64, sigma as f64)
                .unwrap()
                .sample_iter(rng)
                .filter(|x| *x > mu - 3. * sigma && *x < mu + 3. * sigma),
        );
    } else {
        let low: isize = *args.params.first().unwrap_or(&0);
        let hi: isize = *args.params.get(1).unwrap_or(&1000);
        shoot(Uniform::new(low, hi).sample_iter(rng))
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
