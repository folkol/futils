use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader};

use terminal_size::{Height, Width};

const MARGIN: usize = 8;

fn main() {
    let mut input = BufReader::new(stdin().lock());
    let (min, max, numbers) = parse_input(&mut input);
    if numbers.is_empty() {
        eprintln!("Found no numbers");
        std::process::exit(1);
    }
    let (Width(w), Height(h)) = terminal_size::terminal_size().unwrap_or((Width(79), Height(30)));
    let w = w as usize - MARGIN;
    let h = h as usize - MARGIN;

    let points = get_scaled_points(&numbers, min, max, w, h);
    scatter_plot(points, min, max, w, h, numbers.len());
}

fn scatter_plot(
    points: HashSet<(usize, usize)>,
    min: f64,
    max: f64,
    w: usize,
    h: usize,
    num_items: usize,
) {
    let gutter = MARGIN - 2;
    for row in 0..=h {
        if row == 0 {
            print!("{:1$}^", ' ', gutter - 1);
        } else if row == 1 {
            print!("{:>1$} |", scale_num(max), gutter - 2);
        } else if row == h {
            print!("{:>1$} |", scale_num(min), gutter - 2);
        } else {
            print!("{:1$}|", ' ', gutter - 1);
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
    println!("{:indent$}+{}>", ' ', "-".repeat(w), indent = gutter - 1);
    println!(
        "{:gutter$}0{:>w$}",
        ' ',
        scale_num(num_items as f64),
        w = w - 1,
    );
}

fn get_scaled_points(
    numbers: &Vec<f64>,
    min: f64,
    max: f64,
    w: usize,
    h: usize,
) -> HashSet<(usize, usize)> {
    let num_items = numbers.len();
    let interval = max - min + 1.;
    let mut coords: Vec<_> = numbers
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x - min))
        .map(|(i, x)| (i * w / num_items, x * h as f64 / interval))
        .map(|(i, x)| (i, x as usize))
        .collect();
    coords.sort_by_key(|(i, _)| *i);
    HashSet::from_iter(coords)
}

fn scale_num(number: f64) -> String {
    let num_digits = number.log10();
    let precision = (2 - num_digits as usize).clamp(0, 2);
    if number > 9999. {
        format!("{:.e}", number)
    } else {
        format!("{:.precision$}", number)
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
