use std::env;
use std::io::{stdin, stdout, Write};
use std::io::ErrorKind::BrokenPipe;

fn main() {
    if env::args().skip(1).count() > 0 {
        for arg in env::args().skip(1) {
            println!("{}", arg.len());
        }
    } else {
        let mut out = stdout().lock();
        for line in stdin().lines().map_while(Result::ok) {
            match writeln!(out, "{}", line.len()) {
                Ok(_) => {}
                Err(e) if e.kind() == BrokenPipe => break,
                Err(e) => panic!("Unexpected error: {e}")
            }
        }
    }
}
