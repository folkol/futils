use std::collections::HashMap;
use std::env::home_dir;
use std::fs::File;
use std::io::stdin;
use std::io::BufRead;

use clap::Parser;
use rand::prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    paragraphs: usize,
    #[arg(short, long, default_value_t = 5)]
    sentences: usize,
    #[arg(long, default_value_t = false)]
    rebuild: bool,
}

type Trigraph = (String, String, String);

fn main() {
    let args = Args::parse();
    if args.rebuild {
        let mut fst: String = "".to_owned();
        let mut snd: String = "".to_owned();
        let mut stats: HashMap<Trigraph, usize> = HashMap::new();

        let mut trigraphs: Vec<Trigraph> = Vec::new();
        for line in stdin().lines().map_while(Result::ok) {
            for word in line.split_whitespace() {
                let key = (fst.clone(), snd.clone(), word.to_owned());
                trigraphs.push(key.clone());
                stats.entry(key).or_insert(0);
                if "!.?".contains(word.chars().last().unwrap()) {
                    fst = "".to_owned();
                    snd = "".to_owned();
                } else {
                    fst = snd;
                    snd = word.into();
                }
            }
        }

        let home_dir = match home_dir() {
            None => {
                panic!("couldn't find homedir in order to write config file");
            }
            Some(e) => e,
        };
        let config_file = home_dir.join(".lorem_v2");
        let result = File::create(config_file);
        match result {
            Ok(f) => {
                serde_json::to_writer_pretty(f, &trigraphs).expect("couldn't write config file")
            }
            Err(e) => panic!("Failed opening config file {e}"),
        };
    } else {
        eprintln!("Reading trigraphs from file!");
        let home_dir = match home_dir() {
            None => {
                panic!("couldn't find homedir in order to read config file");
            }
            Some(e) => e,
        };
        let config_file = home_dir.join(".lorem_v2");
        let result = File::open(config_file);
        let trigraphs: Vec<Trigraph> = match result {
            Ok(f) => serde_json::from_reader(f).expect("couldn't write config file"),
            Err(_) => {
                eprintln!("[ERROR]: Couldn't find config file, try recreating it with --rebuild");
                std::process::exit(1);
            }
        };

        eprintln!("Read trigraph, let's go!");
        for _ in 1..=3 {
            for _ in 1..=4 {
                let mut rng = thread_rng();
                let mut fst: String = "".into();
                let mut snd: String = "".into();
                let mut word: String = "".into();
                let mut sentence: Vec<String> = Vec::new();
                loop {
                    if sentence.len() > 20 {
                        print!("{}. ", sentence.join(" "));
                        break;
                    } else if "!.?".contains(word.clone().chars().last().unwrap_or(' ')) {
                        print!("{} ", sentence.join(" "));
                        break;
                    };
                    fst = snd.clone();
                    snd = word.clone();
                    word = trigraphs
                        .iter()
                        .filter(|(a, b, _)| *a == fst && *b == snd)
                        .map(|(_, _, c)| c)
                        .choose(&mut rng)
                        .unwrap_or(&"wut".to_owned())
                        .clone();
                    sentence.push(word.clone());
                }
            }
            println!();
            println!();
        }
    }
}
