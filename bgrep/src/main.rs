use std::cmp::Ordering;
use std::fs::File;
use std::io::{Error, SeekFrom};
use std::io::Seek;

use clap::Parser;
use memmap::{Mmap, MmapOptions};

/// B as in Binary Search. Finds line in sorted file
#[derive(Parser)]
struct Args {
    /// matches at beginning of line (fixed string, case sensitive)
    pattern: String,
    filename: String,
}

fn main() {
    let Args { pattern, filename } = Args::parse();
    if let Err(e) = do_it(pattern, filename) {
        eprintln!("Something bad happened :( ({e:?})")
    }
}

fn do_it(pattern: String, filename: String) -> Result<(), Error> {
    let pattern = pattern.as_bytes();
    let k = pattern.len();

    let mut f = File::open(filename)?;
    let mmap = unsafe { MmapOptions::new().map(&f)? };

    let size = f.seek(SeekFrom::End(0))? as usize;
    let mut hi = size;
    let mut lo = 0;
    while lo < hi {
        let next = ((hi + lo) / 2) as i64;
        let next = find_beginning_of_line(&mmap, next);
        let here = &mmap[next..next + k];
        match here.cmp(pattern) {
            Ordering::Equal => {
                println!("Found it at {next}");
                return Ok(());
            }
            Ordering::Less => lo = find_next_line(&mmap, next, size),
            Ordering::Greater => hi = next - 1,
        }
    }

    println!("Not found");

    Ok(())
}

fn find_next_line(mmap: &Mmap, mut next: usize, size: usize) -> usize {
    while next < size {
        if mmap[next] == b'\n' {
            return next;
        }
        next += 1;
    }
    size
}

fn find_beginning_of_line(mmap: &Mmap, mut next: i64) -> usize {
    while next > 0 {
        if mmap[(next - 1) as usize] == b'\n' {
            return next as usize;
        }
        next -= 1;
    }
    0
}
