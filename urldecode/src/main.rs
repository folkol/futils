use std::env::args;
use std::io::stdin;

use urlencoding::decode;

fn main() {
    let num_args = args().skip(1).count();
    if num_args >= 1 {
        for arg in args().skip(1) {
            println!("{}", decode(&arg).unwrap());
        }
    } else {
        for line in stdin().lines().map_while(Result::ok) {
            println!("{}", decode(&line).unwrap())
        }
    }
}
