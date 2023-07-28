use std::env::args;
use std::io::{stdin, stdout, Read, Write};

use base64::engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig};
use base64::{alphabet, Engine};

fn main() {
    let mut parts: Vec<String> = args().skip(1).collect();
    if parts.is_empty() {
        let mut data = String::new();
        parts = match stdin().read_to_string(&mut data) {
            Ok(0) => usage(),
            Ok(_) => vec![data],
            Err(e) => {
                eprintln!("failed to read stdin ({e})");
                usage();
            }
        }
    }
    for part in parts {
        process_part(&part);
    }
}

fn usage() -> ! {
    eprintln!("usage: base64url [DATA]... (reads from stdin if no args)");
    std::process::exit(1);
}

pub const URL_SAFE_INDIFFERENT: GeneralPurpose =
    GeneralPurpose::new(&alphabet::URL_SAFE, INDIFFERENT);
pub const INDIFFERENT: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);

fn process_part(part: &str) {
    let part_strip_newlines = part.split('\n').collect::<String>();
    match URL_SAFE_INDIFFERENT.decode(part_strip_newlines) {
        Ok(data) => {
            stdout().write_all(&data).unwrap();
            println!();
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to decode `{part}` ({e})")
        }
    }
}
