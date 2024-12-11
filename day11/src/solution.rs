use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
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
    stones: Vec<i64>,
}

fn digits(mut num: i64) -> i64 {
    let mut res = 1;
    while num / 10 > 0 {
        res += 1;
        num /= 10;
    }
    res
}

fn ten_power(x: i64) -> i64 {
    let mut res = 10;
    for _ in 1..x {
        res *= 10;
    }
    res
}

fn split_number(num: i64) -> (i64, i64) {
    let len = digits(num);
    (num / ten_power(len/2), num % ten_power(len/2))
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let l = lines.next().unwrap();
        let line = l.trim();

        Solution {
            stones: line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect(),
        }
    }

    fn blink(&mut self) {
        let mut new_stones = Vec::new();
        for s in &self.stones {
            match s {
                0 => new_stones.push(1),
                e if digits(*e) % 2 == 0 =>  {
                    let (a, b) = split_number(*e);
                    new_stones.push(a);
                    new_stones.push(b);
                }
                o => new_stones.push(o * 2024),
            }
        }
        self.stones = new_stones;
    }

    fn solve(&mut self) -> i64 {
        for _ in 0..25 {
            self.blink();
        }
        return self.stones.len() as i64
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut solution = Solution::from_input(lines_it);

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
            "125 17",
            "55312",
        );
    }
}
