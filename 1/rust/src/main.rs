use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(File::open("../input").unwrap());
    let num_lines = 1000;

    let mut col1 = Vec::with_capacity(num_lines);
    let mut col2 = Vec::with_capacity(num_lines);

    for line in input.lines() {
        let line = line.unwrap();
        let mut cols = line.split_whitespace();
        col1.push(cols.next().unwrap().parse::<u32>().unwrap());
        col2.push(cols.next().unwrap().parse::<u32>().unwrap());
    }

    col1.sort();
    col2.sort();

    let res = col1
        .iter()
        .zip(col2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    println!("{res}");
}
