use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
//use regex::Regex;
//use lazy_static::lazy_static;
//use std::collections::HashSet;
use std::collections::HashMap;

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

fn blink(stones: &[i64]) -> Vec<i64> {
    let mut new_stones = Vec::new();
    for s in stones {
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
    new_stones
}

fn many_blinks(stones: &[i64], num: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if num == 0 {
        return stones.len() as i64;
    }
    if stones.len() == 1 {
        if let Some(res) = cache.get(&(stones[0], num)) {
            return *res;
        }
        let new_stones = blink(stones);
        let res = many_blinks(&new_stones, num - 1, cache);
        cache.insert((stones[0], num), res);
        return res;
    }
    let mut res = 0;
    for i in 0..stones.len() {
        res += many_blinks(&stones[i..(i+1)], num, cache);
    }
    res
}

impl Solution {
    fn from_slice(sl: &[i64]) -> Self {
        Solution {
            stones: sl.to_vec(),
        }
    }

    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let l = lines.next().unwrap();
        let line = l.trim();

        let stones: Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        Solution::from_slice(&stones)
    }

    fn solve(&self, num: i64) -> i64 {
        let mut cache = HashMap::new();
        return many_blinks(&self.stones, num, &mut cache);
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Solution::from_input(lines_it);

    writeln!(output, "{}", solution.solve(75)).unwrap();
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
        let input = vec![125, 17];
        let sol = Solution::from_slice(&input);
        assert_eq!(sol.solve(25), 55312);
    }

    #[test]
    fn bigger() {
        let input = vec![20, 82084, 1650, 3, 346355, 363, 7975858, 0];
        let sol = Solution::from_slice(&input);
        assert_eq!(sol.solve(25), 172484);
    }
}
