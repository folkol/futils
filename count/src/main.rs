use std::io::ErrorKind::BrokenPipe;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(stdout().lock());
    let mut i = 0;
    loop {
        match writeln!(writer, "{i}") {
            Ok(_) => {}
            Err(x) if x.kind() == BrokenPipe => break,
            Err(_) => panic!("Unexpected error :("),
        };
        i += 1;
    }
}
