use regex::{Regex, RegexSet};
use std::{env, fs};

fn main() {
    let infile = env::args().nth(1).expect("missing input file");

    let haystack = fs::read_to_string(infile).unwrap();
    println!("Mul sum: {}", part1(&haystack));
    println!("Enabled mul sum: {}", part2(&haystack));
}

fn part1(haystack: &str) -> u32 {
    let mul_pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(mul_pattern).unwrap();

    re.captures_iter(haystack)
        .map(|c| c.extract())
        .map(|(_, [n1, n2])| n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap())
        .sum()
}

fn part2(haystack: &str) -> u32 {
    let patterns = r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)";
    let re_all = Regex::new(patterns).unwrap();

    let ops = r"(\d+),(\d+)";
    let nums_re = Regex::new(ops).unwrap();

    let mut enabled = true;
    re_all
        .find_iter(haystack)
        .map(|m| {
            if m.as_str() == "do()" {
                enabled = true;
                0
            } else if m.as_str() == "don't()" {
                enabled = false;
                0
            } else if enabled {
                nums_re.captures(m.as_str()).unwrap()[1]
                    .parse::<u32>()
                    .unwrap()
                    * nums_re.captures(m.as_str()).unwrap()[2]
                        .parse::<u32>()
                        .unwrap()
            } else {
                0
            }
        })
        .sum()
}
