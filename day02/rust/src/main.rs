use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = env::args()
        .nth(1)
        .expect("You need to specify an input file");

    println!("Safe reports count: {}", safe_reports_count(&infile, is_safe));
    println!("Safe reports with Dampener: {}", safe_reports_count(&infile, is_safe_p2));
}

fn safe_reports_count<F>(infile: &str, safe_fn: F) -> usize
where
    F: Fn(&Vec<u8>) -> bool,
{
    let input = BufReader::new(File::open(infile).unwrap());

    input
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .filter(safe_fn)
        .count()
}

fn is_safe_p2(levels: &Vec<u8>) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut new_levels = levels.clone();
        new_levels.remove(i);
        if is_safe(&new_levels) {
            return true;
        }
    }

    false
}

fn is_safe(levels: &Vec<u8>) -> bool {
    let is_descending = match levels[1].cmp(&levels[0]) {
        Ordering::Less => true,
        Ordering::Equal => return false,
        Ordering::Greater => false,
    };

    for i in 1..levels.len() {
        match levels[i].cmp(&levels[i - 1]) {
            Ordering::Equal => return false,
            Ordering::Less => {
                if !is_descending {
                    return false;
                }
            }
            Ordering::Greater => {
                if is_descending {
                    return false;
                }
            }
        }

        if levels[i].abs_diff(levels[i - 1]) > 3 {
            return false;
        }
    }

    true
}
