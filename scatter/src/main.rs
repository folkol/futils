use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader};

use terminal_size::{Height, Width};

fn main() {
    let mut input = BufReader::new(stdin().lock());
    let (min, max, numbers) = parse_input(&mut input);
    if numbers.is_empty() {
        eprintln!("Found no numbers");
        std::process::exit(1);
    }
    let interval = max - min + 1.;
    let (Width(w), Height(h)) = terminal_size::terminal_size().unwrap_or((Width(79), Height(30)));
    let w = w - 8;
    let h = h - 8;

    let num_items = numbers.len();
    let mut coords: Vec<_> = numbers
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x - min))
        .map(|(i, x)| (i * w as usize / num_items, x * h as f64 / interval))
        .map(|(i, x)| (i as u16, x as u16))
        .collect();
    coords.sort_by_key(|(i, _)| *i);
    let points: HashSet<_> = HashSet::from_iter(coords);
    for row in 0..=h {
        if row == 0 {
            print!("     ^");
        } else if row == 1 {
            print!("{:>4} |", scale_num(max));
        } else if row == h {
            print!("{:>4} |", scale_num(min));
        } else {
            print!("     |");
        }
        for col in 0..=w {
            if points.contains(&(col, h - row)) {
                print!("x")
            } else {
                print!(" ")
            }
        }
        println!();
    }
    println!("     +{}>", "-".repeat(w as usize));
    println!(
        "      0{1:>0$}",
        w as usize - 1,
        scale_num(num_items as f64)
    );
}

fn scale_num(number: f64) -> String {
    if number > 9999. {
        format!("{:.e}", number)
    } else {
        number.to_string()
    }
}

fn parse_input(input: &mut dyn BufRead) -> (f64, f64, Vec<f64>) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    let mut numbers: Vec<f64> = Vec::new();
    for line in input.lines().map_while(Result::ok) {
        let number: f64 = match line.parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Couldn't parse `{line}` as f64 (`{e}`)");
                continue;
            }
        };
        lo = lo.min(number);
        hi = hi.max(number);
        numbers.push(number);
    }
    (lo, hi, numbers)
}
