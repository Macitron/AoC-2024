use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = env::args().nth(1).expect("must specify input file");
    println!("Distance: {}", part1(&infile));
    println!("Similarity Score: {}", part2(&infile));
}

fn part1(infile: &str) -> u32 {
    let input = BufReader::new(File::open(infile).unwrap());
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

    col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>()
}

fn part2(infile: &str) -> u32 {
    let input = BufReader::new(File::open(infile).unwrap());
    let num_lines = 1000;

    let mut col1 = Vec::with_capacity(num_lines);
    let mut count_map = HashMap::with_capacity(num_lines);

    for line in input.lines() {
        let line = line.unwrap();
        let mut cols = line.split_whitespace();
        let (c1, c2) = (
            cols.next().unwrap().parse::<u32>().unwrap(),
            cols.next().unwrap().parse::<u32>().unwrap(),
        );
        col1.push(c1);
        *count_map.entry(c2).or_insert(0) += 1;
    }

    col1.iter()
        .map(|x| x * count_map.get(x).unwrap_or(&0))
        .sum::<u32>()
}
