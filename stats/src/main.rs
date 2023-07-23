use std::io::stdin;

use incr_stats::incr::Stats;

fn main() {
    let mut stats = Stats::new();
    for line in stdin().lines().map_while(Result::ok) {
        let number: f64 = match line.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Couldn't parse '{line}', skipping.");
                continue;
            }
        };
        stats.update(number).unwrap()
    }
    if stats.count() == 0 {
        println!("Missing sample");
        std::process::exit(1);
    }
    println!("count: {}", stats.count());
    println!("  min: {}", stats.min().unwrap());
    println!("  max: {}", stats.max().unwrap());
    println!("  sum: {}", stats.sum().unwrap());
    println!(" mean: {}", stats.mean().unwrap());
    println!("sigma: {}", stats.population_standard_deviation().unwrap());
}
