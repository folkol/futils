use std::io::{Read, stdin};

fn main() {
    let mut buffer = [0_u8; std::mem::size_of::<u32>()];
    while stdin().read_exact(&mut buffer).is_ok() {
        let n = u32::from_le_bytes(buffer);
        println!("{n}");
    }
}
