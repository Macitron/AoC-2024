use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = env::args().nth(1).unwrap();
    let rows = BufReader::new(File::open(infile).unwrap())
        .lines()
        .map(|l| l.unwrap().bytes().collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    // println!("first two rows:\n{}\n{}", rows[0], rows[1]);
    // println!("first rows length: {}", rows[0].len()); <-- 140
    // println!("there are {} rows", rows.len());        <-- 140

    let mut xmas_count = 0;
    for row_idx in 0..rows.len() {
        for col_idx in 0..rows[row_idx].len() {
            if rows[row_idx][col_idx] == b'X' {
                for horiz in [Horizontal::Left, Horizontal::Right, Horizontal::None] {
                    for vert in [Vertical::Down, Vertical::Up, Vertical::None] {
                        xmas_count += check_mas(&rows, row_idx, col_idx, horiz, vert);
                    }
                }
            }
        }
    }

    println!("XMAS count: {xmas_count}");
}

#[derive(Copy, Clone)]
enum Horizontal {
    Left,
    Right,
    None,
}

#[derive(Copy, Clone)]
enum Vertical {
    Up,
    Down,
    None,
}

fn check_mas(
    haystack: &[Vec<u8>],
    row_idx: usize,
    col_idx: usize,
    horizontal: Horizontal,
    vertical: Vertical,
) -> u32 {
    const MAS: &[u8] = b"MAS";

    let mut row = row_idx;
    let mut col = col_idx;
    for c in MAS {
        row = match vertical {
            Vertical::Up => {
                if row == 0 {
                    return 0;
                }
                row - 1
            }
            Vertical::Down => {

                if row >= 139 {
                    return 0;
                }
                row + 1
            }
            _ => row,
        };

        col = match horizontal {
            Horizontal::Left => {
                if col == 0 {
                    return 0;
                }
                col - 1
            }
            Horizontal::Right => {
                if col >= 139 {
                    return 0;
                }
                col + 1
            }
            _ => col,
        };

        if haystack[row][col] != *c {
            return 0;
        }
    }

    1
}
