fn main() {
    let w = match term_size::dimensions() {
        None => 79,
        Some((w, _)) => w
    };
    for line in 0..=3 {
        for n in 1..=w {
            let c = if n % 10 == 0 {
                format!("{}", n).chars().nth(line).unwrap_or(' ').to_string()
            } else if line == 0 && n % 5 == 0 {
                "|".to_owned()
            } else if line == 0 {
                ".".to_owned()
            } else {
                " ".to_owned()
            };
            print!("{c}");
        }
        println!();
    }
}
