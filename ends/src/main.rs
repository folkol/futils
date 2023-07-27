use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let n: usize = 10;
    let mut i: usize = 0;
    let mut buffer: VecDeque<String> = VecDeque::new();
    for line in stdin().lines().map_while(Result::ok) {
        i += 1;
        if i <= n {
            println!("{line}");
        } else {
            if buffer.len() > n {
                buffer.pop_front();
            }
            buffer.push_back(line);
        }
    };
    if i > 20 {
        println!("... {} more", i - 2 * n);
    }
    for line in buffer {
        println!("{line}");
    }
}
