use std::io::{BufWriter, Read, stdin, stdout, Write};
use std::io::ErrorKind::BrokenPipe;

fn main() {
    let mut out = BufWriter::new(stdout().lock());
    let mut buffer = [0_u8; std::mem::size_of::<u32>()];
    while stdin().read_exact(&mut buffer).is_ok() {
        let n = u32::from_le_bytes(buffer);
        match writeln!(out, "{n}") {
            Ok(_) => {}
            Err(n) if n.kind() == BrokenPipe => break,
            Err(e) => { eprintln!("Something went wrong: ({e})") }
        }
    }
}
