#![feature(iter_intersperse)]

use std::io::stdin;

use clap::Parser;

use crate::ColExpr::Rest;

/// picks named columns form stdin
#[derive(Parser)]
struct Args {
    /// Which columns to pick.
    /// - starts at 0
    /// - negative indexes means from the end
    /// - '*' means 'all not otherwise picked'
    #[arg()]
    columns: Vec<String>,
}

#[derive(Debug)]
enum ColExpr {
    Col(isize),
    Rest,
}

fn main() {
    let args: Args = Args::parse();
    let requested_cols: Vec<ColExpr> = args
        .columns
        .iter()
        .filter_map(|x| {
            if *x == "*" {
                Some(Rest)
            } else {
                x.parse().map(ColExpr::Col).ok()
            }
        })
        .collect();
    if requested_cols.is_empty() {
        eprintln!("Pick what?");
        std::process::exit(1);
    }
    for line in stdin().lines().map_while(Result::ok) {
        let in_columns: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
        let numerical_columns: Vec<usize> = requested_cols
            .iter()
            .filter_map(|x| match x {
                ColExpr::Col(n) => Some(n),
                Rest => None,
            })
            .map(|n| to_actual_index(&in_columns, n))
            .collect();
        let glob_columns: Vec<usize> = in_columns
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .filter(|x| !numerical_columns.contains(x))
            .collect();
        let mut out_columns: Vec<String> = Vec::new();
        for col_expr in requested_cols.iter() {
            match col_expr {
                ColExpr::Col(n) => {
                    let n: usize = to_actual_index(&in_columns, n);
                    if let Some(item) = in_columns.get(n) {
                        out_columns.push(item.clone());
                    }
                }
                Rest => {
                    for glob_column in &glob_columns {
                        if let Some(item) = in_columns.get(*glob_column) {
                            out_columns.push(item.clone());
                        }
                    }
                }
            }
        }
        if !out_columns.is_empty() {
            println!("{}", out_columns.join("\t"));
        }
    }
}

fn to_actual_index(in_columns: &Vec<String>, n: &isize) -> usize {
    if *n < 0 {
        let n = *n;
        let m: isize = in_columns.len() as isize;
        let n = m + n;
        n.clamp(0, m - 1) as usize
    } else {
        *n as usize
    }
}
