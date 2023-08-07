use std::io::{stdin, stdout, Write};

use clap::{Parser, ValueEnum};
use clap::builder::PossibleValue;

#[derive(Clone)]
enum ByteOrder {
    LittleEndian,
    BigEndian,
}

impl ValueEnum for ByteOrder {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::LittleEndian, Self::BigEndian]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::LittleEndian => PossibleValue::new("LE"),
            Self::BigEndian => PossibleValue::new("BE"),
        })
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, short, default_value_t = ByteOrder::BigEndian, value_enum)]
    byte_order: ByteOrder,
}

fn main() {
    let args = Args::parse();
    for line in stdin().lines().map_while(Result::ok) {
        let num: u32 = match line.parse() {
            Ok(n) => n,
            _ => {
                eprintln!("failed to parse as u32: `{line}`");
                continue;
            }
        };
        let x = num.to_be_bytes();
        // TODO: Warn if tty?
        stdout().write_all(&x[..]).unwrap();
    }
}
