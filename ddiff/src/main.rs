use std::env::args;

use chrono::{NaiveDate, Utc};

fn main() {
    let num_args = args().skip(1).len();
    let incorrect_num_args = num_args == 0 || num_args > 2;
    if incorrect_num_args || args().any(|arg| arg == "--help" || arg == "-h") {
        usage(incorrect_num_args as i32);
    }
    let from = args().nth(1).unwrap();
    let from = to_date_or_die(&from);
    let until = args().nth(2).map(|to| {
        to_date_or_die(&to)
    });
    let until = until.unwrap_or(Utc::now().naive_local().date());
    let days_diff = (until - from).num_days().abs();
    println!("{days_diff}");
}

fn to_date_or_die(from: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(from, "%Y-%m-%d") {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Couldn't parse date from `{e}`");
            usage(1);
        }
    }
}

fn usage(exit: i32) -> ! {
    eprintln!("usage: ddiff SOME_DATE [SOME_DATE]");
    eprintln!("    - calculates number of days in between given dates");
    eprintln!("    - SOME_DATE defaults to TODAY");
    eprintln!("    - (format: %Y-%m-%d)");
    std::process::exit(exit);
}
