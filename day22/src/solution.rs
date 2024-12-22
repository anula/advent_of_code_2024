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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct SecretNum(i64);

impl SecretNum {
    fn mix(&self, value: i64) -> Self {
       SecretNum(self.0 ^ value)
    }

    fn prune(&self) -> Self {
        SecretNum(&self.0 % 16777216)
    }

    fn step(&self) -> Self {
        let mut num = self.mix(self.0 * 64);
        num = num.prune();
        num = num.mix(num.0 / 32);
        num = num.prune();
        num = num.mix(num.0 * 2048);
        num = num.prune();
        num
    }

    fn multi_step(&self, iterations: usize) -> Self {
        let mut res = self.clone();
        for _ in 0..iterations {
            res = res.step();
        }
        res
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    nums: Vec<SecretNum>,
}

impl Solution {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut nums = vec![];
        for l in lines {
            let line = l.trim();
            nums.push(SecretNum(line.parse::<i64>().unwrap()));
        }

        Solution {
            nums,
        }
    }

    fn solve(&self) -> i64 {
        self.nums.iter().map(|n| n.multi_step(2000).0).sum()
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

    #[allow(dead_code)]
    fn text_to_lines(input: &str) -> Vec<String>{
        input.split("\n").map(|s| s.to_string()).collect()
    }

    #[test]
    fn sample() {
        test_ignore_whitespaces(
            "1
            10
            100
            2024",
            "37327623",
        );
    }

    #[test]
    fn secret_num_step() {
        let sn = SecretNum(123);
        assert_eq!(sn.step(), SecretNum(15887950));
        assert_eq!(sn.step().step(), SecretNum(16495136));
        assert_eq!(sn.step().step().step(), SecretNum(527345));
    }

    #[test]
    fn secret_num_multistep() {
        let sn = SecretNum(123);
        assert_eq!(sn.multi_step(0), SecretNum(123));
        assert_eq!(sn.multi_step(1), SecretNum(15887950));
        assert_eq!(sn.multi_step(2), SecretNum(16495136));
        assert_eq!(sn.multi_step(3), SecretNum(527345));
        assert_eq!(sn.multi_step(4), SecretNum(704524));

        assert_eq!(SecretNum(1).multi_step(2000),    SecretNum(8685429));
        assert_eq!(SecretNum(10).multi_step(2000),   SecretNum(4700978));
        assert_eq!(SecretNum(100).multi_step(2000),  SecretNum(15273692));
        assert_eq!(SecretNum(2024).multi_step(2000), SecretNum(8667524));
    }
}
