use std::io::stdin;

fn main() {
    let mut sum: f64 = 0.;
    for line in stdin().lines().map_while(Result::ok) {
        let n: f64 = match line.parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Couldn't parse `{line}` as f64");
                continue;
            }
        };
        sum += n;
        println!("{sum}\t{n}")
    }
}
