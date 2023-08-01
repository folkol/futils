extern crate core;

use std::env::args;
use std::error::Error;

fn main() {
    match args().nth(1) {
        None => {
            eprintln!("usage: hex2dec HEX_COLOR");
            std::process::exit(1);
        }
        Some(color) => {
            if let Err(e) = parse_and_print(color) {
                eprintln!("Something broke: {e}");
                std::process::exit(2);
            };
        }
    }
}

fn parse_and_print(color: String) -> Result<(), Box<dyn Error>> {
    let color = color.trim_start_matches('#');
    if color.len() != 6 {
        return Err("Input must be exactly 6 hex digits long".into());
    }
    print!("{} ", u8::from_str_radix(&color[0..2], 16)?);
    print!("{} ", u8::from_str_radix(&color[2..4], 16)?);
    println!("{}", u8::from_str_radix(&color[4..6], 16)?);
    Ok(())
}
