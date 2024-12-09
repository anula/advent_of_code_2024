use std::io::{BufRead, BufReader, Write};
use std::cmp::min;
//use std::cmp::max;
//use regex::Regex;
//use lazy_static::lazy_static;
//use std::collections::HashSet;
//use std::collections::HashMap;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    disk_map: Vec<i64>,
    total_blocks: i64,
}

fn sum_from_to(from: i64, to: i64) -> i64{
    (from + to) * (to - from + 1) / 2
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let l = lines.next().unwrap();
        let line = l.trim();

        let mut disk_map = Vec::new();
        let mut total_blocks = 0;

        for ch in line.chars() {
            let len = ch.to_digit(10).unwrap() as i64;
            disk_map.push(len);
            total_blocks += len;
        }

        Solution {
            disk_map,
            total_blocks,
        }
    }

    fn solve(&self) -> i64 {
        let mut i = 0 as i64;
        let mut j = (self.disk_map.len() as i64) - 1;
        if j % 2 == 1 {
            j -= 1;
        }

        let mut checksum = 0;
        let mut blocks_before = 0;
        let mut j_size_left = self.disk_map[j as usize];
        while i < j {
            dprintln!("i: {}", i);
            let block_size = self.disk_map[i as usize];
            let i_id = i / 2;
            if i % 2 == 0 {
                dprintln!(" -- is a file");
                dprintln!(" -- sum_from_to({}, {})", blocks_before, blocks_before + block_size - 1);
                let part_sum = sum_from_to(blocks_before, blocks_before + block_size - 1) * i_id;
                checksum += part_sum;
                dprintln!(" -- adding: {}", part_sum);
                blocks_before += block_size;
            } else {
                dprintln!(" -- is not a file");
                let mut size_filled = 0;
                while size_filled < block_size && j > i{
                    dprintln!(" :-- j: {}, j_size_left: {}", j, j_size_left);
                    dprintln!(" :-- bef -- size_filled: {}", size_filled);
                    let j_id = j/2;
                    let filling = min(block_size - size_filled, j_size_left);
                    let part_sum = sum_from_to(blocks_before, blocks_before + filling - 1) * j_id;
                    checksum += part_sum;
                    dprintln!(" :-- adding: {}", part_sum);
                    blocks_before += filling;
                    size_filled += filling;
                    j_size_left -= filling;
                    dprintln!(" :-- aft -- size_filled: {}", size_filled);

                    if j_size_left == 0 {
                        j -= 2;
                        if j > i {
                            j_size_left = self.disk_map[j as usize];
                        }
                    }
                }
            }
            i += 1;
        }
        if j_size_left > 0 {
            checksum += sum_from_to(blocks_before, blocks_before + j_size_left - 1) * j / 2;
        }
        checksum
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Solution::from_input(lines_it);

    writeln!(output, "{}", solution.solve()).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ignore_whitespaces(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    #[test]
    fn sample() {
        test_ignore_whitespaces(
            "2333133121414131402",
            "1928",
        );
    }

    #[test]
    fn simple() {
        test_ignore_whitespaces(
            "12345",
            "60",
        );
    }

    #[test]
    fn summing() {
        assert_eq!(sum_from_to(1, 2), 3);
        assert_eq!(sum_from_to(3, 5), 12);
        assert_eq!(sum_from_to(6, 8), 21);
    }
}
