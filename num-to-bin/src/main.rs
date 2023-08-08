use std::io::ErrorKind::BrokenPipe;
use std::io::{stdin, stdout, BufWriter, Write};

use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};

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
    #[arg(long, short, default_value_t = ByteOrder::LittleEndian, value_enum)]
    byte_order: ByteOrder,
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());
    let args = Args::parse();
    for line in stdin().lines().map_while(Result::ok) {
        let num: u32 = match line.parse() {
            Ok(n) => n,
            _ => {
                eprintln!("failed to parse as u32: `{line}`");
                continue;
            }
        };
        // TODO: Warn if tty?
        let bytes = match args.byte_order {
            ByteOrder::LittleEndian => num.to_le_bytes(),
            ByteOrder::BigEndian => num.to_be_bytes(),
        };

        match out.write_all(&bytes[..]) {
            Ok(_) => {}
            Err(n) if n.kind() == BrokenPipe => break,
            Err(e) => {
                eprintln!("Something went wrong: ({e})")
            }
        }
    }
}
