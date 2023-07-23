use std::io::stdin;

fn main() {
    let lines = stdin().lines().map_while(Result::ok);
    let numbers: Vec<f64> = lines
        .map(|line| line.parse())
        .filter_map(Result::ok)
        .collect();
    if numbers.is_empty() {
        eprintln!("Empty input");
        std::process::exit(1);
    }
    let (bins, min, max) = make_histogram(numbers);

    print_histogram(&bins, bins.len() / 2);
    println!("min={min}");
    println!("max={max}");
}

fn print_histogram(bins: &Vec<usize>, num_rows: usize) {
    let bin_max = bins.iter().max().unwrap();
    println!();
    for row in 1..num_rows {
        for bin in bins {
            let this_high =
                (*bin as f64 / *bin_max as f64) * (num_rows as f64) >= ((num_rows - row) as f64);
            print!("{}", if this_high { "X" } else { " " })
        }
        println!();
    }
    println!();
}

fn make_histogram(numbers: Vec<f64>) -> (Vec<usize>, f64, f64) {
    let fst: f64 = *numbers.first().unwrap();
    let (min, max) = numbers
        .iter()
        .fold((fst, fst), |(a, b), x| (a.min(*x), b.max(*x)));
    let num_bins: f64 = (numbers.len() as f64).log(1.5).ceil();
    let mut bins: Vec<usize> = vec![0; num_bins as usize];
    for number in numbers {
        let range = max - min;
        let bin = (num_bins * (number - 1. - min) / range).floor();
        bins[bin as usize] += 1;
    }
    (bins, min, max)
}
