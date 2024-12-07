use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::HashSet;
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
enum Op {
    Add,
    Mul,
}

impl Op {
    fn eval(&self, a: i64, b: i64) -> i64{
        match &self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Equ {
    value: i64,
    numbers: Vec<i64>,
}

impl Equ {
    fn from_str(line: &str) -> Self {
        let vals = line.split(":").collect::<Vec<&str>>();
        let value = vals[0].parse::<i64>().unwrap();
        let numbers = vals[1].trim().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
        Equ {
            value,
            numbers,
        }
    }

    fn is_valid(&self) -> bool {
        let all_ops = vec![Op::Add, Op::Mul];

        let mut pos_results = HashSet::new();
        pos_results.insert(self.numbers[0]);
        for n in self.numbers.iter().skip(1) {
            let mut new_results = HashSet::new();
            for r in pos_results {
                for op in &all_ops {
                    new_results.insert(op.eval(r, *n));
                }
            }
            pos_results = new_results;
        }

        pos_results.contains(&self.value)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    equs: Vec<Equ>,
}

impl Solution {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        Solution {
            equs: lines.map(|l| Equ::from_str(l.trim())).collect(),
        }
    }

    fn solve(&self) -> i64 {
        let mut res = 0;
        for eq in &self.equs {
            if eq.is_valid() {
                res += eq.value;
            }
        }
        res
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
            "190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20",
            "3749",
        );
    }
}
