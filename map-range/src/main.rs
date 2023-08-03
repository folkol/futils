use std::env::args;
use std::io::stdin;
use std::process;

fn main() {
    let args: Vec<_> = args().skip(1).collect();
    if args.len() == 2 {
        let image_lo: f64 = args[0].parse().expect("First argument must be a number");
        let image_hi: f64 = args[1].parse().expect("Second argument must be a number");
        process_batch(image_lo, image_hi);
    } else if args.len() == 4 {
        let domain_lo: f64 = args[0].parse().expect("First argument must be a number");
        let domain_hi: f64 = args[1].parse().expect("Second argument must be a number");
        let image_lo: f64 = args[2].parse().expect("Third argument must be a number");
        let image_hi: f64 = args[3].parse().expect("Fourth argument must be a number");
        process_stream(domain_lo, domain_hi, image_lo, image_hi);
    } else {
        eprintln!("usage: map-range image_from image_to ");
        eprintln!("   or: map-range domain_from domain_to image_from image_to ");
        process::exit(1);
    };
}

fn process_stream(domain_lo: f64, domain_hi: f64, image_lo: f64, image_hi: f64) {
    let image_range = image_lo - image_hi;
    let domain_range = domain_lo - domain_hi;

    stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<f64>())
        .map_while(Result::ok)
        .for_each(|number| {
            let x = (number - domain_lo) / domain_range;
            let y = image_lo + x * image_range;
            println!("{y}")
        });
}

fn process_batch(image_lo: f64, image_hi: f64) {
    let lines: Vec<_> = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<f64>())
        .map_while(Result::ok)
        .collect();

    if lines.is_empty() {
        return;
    }

    let domain_from = *lines.iter().min_by(|a, b| f64::total_cmp(a, b)).unwrap();
    let domain_to = *lines.iter().max_by(|a, b| f64::total_cmp(a, b)).unwrap();
    let image_range = image_lo - image_hi;
    let domain_range = domain_from - domain_to;
    for number in lines {
        let x = (number - domain_from) / domain_range;
        let y = image_lo + x * image_range;
        println!("{y}")
    }
}
