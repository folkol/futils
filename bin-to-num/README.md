# read binary numbers

## Compare performance to C version

Summing a few gig of LE uints. I haven't investiated much, but they do comparible number of syscalls.

    $ cat src/main.rs
    use std::io::{Read, stdin};

    fn main() {
        let mut sum: u64 = 0;
        let mut buffer = [0_u8; std::mem::size_of::<u32>()];
        while let Ok(_) = stdin().read_exact(&mut buffer) {
            sum += u32::from_le_bytes(buffer) as u64;
        }
        println!("{sum}");
    }
    $ cargo rustc --release -- -C target-cpu=native
    $ time ./target/release/bin-to-num <numbers.dat
    2147443220545536000
    real	0m27.768s
    user	0m25.968s
    sys	0m1.349s

    $ cat reference.c
    #include <stdio.h>
    #include <stdlib.h>

    #define BUF_SIZE 1024

    int main(void) {
        long sum = 0;
        size_t num_read;
        uint32_t buf[BUF_SIZE];
        while(num_read = fread(buf, sizeof (uint32_t), BUF_SIZE, stdin), num_read > 0) {
            for(int i = 0; i < num_read; i++) {
                sum += buf[i];
            }
        }
        printf("%ld\n", sum);
    }
    $ cc -O3 -march=native reference.c
    $ time ./a.out <numbers.dat
    2147443220545536000
    real	0m1.879s
    user	0m0.499s
    sys	0m1.184s
