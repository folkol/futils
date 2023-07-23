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

fn make_histogram(numbers: Vec<f64>) -> (Vec<usize>, f64, f64) {
    let fst: f64 = *numbers.first().unwrap();
    let (min, max) = numbers
        .iter()
        .fold((fst, fst), |(a, b), x| (a.min(*x), b.max(*x)));
    let num_bins: f64 = 40.; // Maybe something like min(num_distinct, numbers.len()).sqrt()?
    let mut bins: Vec<usize> = vec![0; num_bins as usize];
    for number in numbers {
        let range = max - min;
        let placement = (number - min) / (max - min);
        let bin = (num_bins * placement) as usize;
        let bin = bin.clamp(0, bins.len() - 1);
        if bin as usize >= bins.len() {
            println!(
                "overflow: number={number} placement={placement} bin={bin} bin_usize={} bins.len={} range={range} max={max} min={min}",
                bin as usize,
                bins.len()
            )
        }
        bins[bin] += 1;
    }
    (bins, min, max)
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

#[cfg(test)]
mod test {
    use crate::make_histogram;

    #[test]
    fn uniform() {
        assert_eq!(
            make_histogram((1..=1000).map(f64::from).collect()),
            (vec![25; 40], 1., 1000.),
        )
    }

    #[test]
    fn uniform_large_evenly_distributed() {
        let (histogram, min, max) =
            make_histogram((1..=100).flat_map(|x| [x as f64; 1000]).collect());
        assert!(very_close(min, 1.), "Expected {min} to be very close to 1.");
        assert!(
            very_close(max, 100.),
            "Expected {max} to be very close to 1000."
        );
        dbg!(&histogram);
        let shortest_bin = *histogram.iter().min().unwrap() as f64;
        let tallest_bin = *histogram.iter().max().unwrap() as f64;
        let similarity = shortest_bin / tallest_bin;
        println!("{similarity}");
        assert!(similarity > 0.90)
    }

    #[test]
    fn uniform_narrow_interval() {
        let (histogram, min, max) =
            make_histogram((666..=1666).map(|x| x as f64 / 200000.).collect());
        assert!(very_close(min, 666. / 200000.));
        assert!(very_close(max, 1666. / 200000.));
        let shortest_bin = *histogram.iter().min().unwrap() as f64;
        let tallest_bin = *histogram.iter().max().unwrap() as f64;
        let similarity = shortest_bin / tallest_bin;
        println!("{similarity}");
        assert!(similarity > 0.90)
    }

    fn very_close(a: f64, b: f64) -> bool {
        (a - b).abs() <= f64::MIN_POSITIVE
    }
}
