use std::{env, fs};
use regex::Regex;

fn main() {
    let infile = env::args().nth(1).expect("missing input file");

    let haystack = fs::read_to_string(infile).unwrap();
    let mul_pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(mul_pattern).unwrap();

    let mut sum = 0;
    re.captures_iter(&haystack)
        .map(|c| c.extract())
        .for_each(|(_, [n1, n2])| {
            sum += n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap();
        });

    println!("Mul sum: {sum}");
}
