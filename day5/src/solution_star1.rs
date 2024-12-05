//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug)]
struct Solution {
    graph: HashMap<i64, Vec<i64>>,

    pages: Vec<Vec<i64>>,
}

impl Solution {
    fn from_input<I>(mut input: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut graph = HashMap::new();
        
        for l in input.by_ref() {
            let line = l.trim();
            if line.len() == 0 {
                break;
            }
            let nums: Vec<i64> = line.split("|").map(|x| x.parse::<i64>().unwrap()).collect();

            graph.entry(nums[0]).or_insert(vec![]).push(nums[1]);
        }

        let mut pages = Vec::new();

        for l in input {
            let line = l.trim();
            let ordering: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
            pages.push(ordering);
        }

        Solution {
            graph,
            pages,
        }
    }

    fn is_correct_then_mid(&self, order: &[i64]) -> Option<i64> {
        let mut before = HashSet::new();
        for p in order {
            if let Some(neighs) = self.graph.get(p) {
                for n in neighs {
                    if before.contains(n) {
                        return None;
                    }
                }
            }
            before.insert(p);
        }

        if order.len() % 2 != 1 {
            panic!("Order not odd!");
        }

        dprintln!("Good order: {:?}", order);
        let mid_idx = order.len() / 2;

        Some(order[mid_idx])
    }

    fn sum_correct_middles(&self) -> i64 {
        self.pages.iter()
            .filter_map(|p| self.is_correct_then_mid(&p))
            .sum()
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Solution::from_input(lines_it);

    writeln!(output, "{}", solution.sum_correct_middles()).unwrap();
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
            "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47",
            "143",
        );
    }
}
