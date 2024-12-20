use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::HashSet;
use std::collections::HashMap;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
struct Solution {
    towels: HashSet<String>,
    designs: Vec<String>,
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let l_towels = lines.next().unwrap();
        let towels = l_towels.trim().split(",").map(|x| x.trim().to_string()).collect();
        let _ = lines.next();
        let mut designs = vec![];
        for l in lines {
            let line = l.trim();
            designs.push(line.to_string());
        }

        Solution {
            towels,
            designs,
        }
    }

    fn arrangements_num(&self, pattern: &str, cache: &mut HashMap<String, i64>, pp: &str) -> i64 {
        if let Some(res) = cache.get(pattern) {
            return *res;
        }
        let mut patt_options = 0;
        if self.towels.contains(pattern) {
            patt_options += 1;
        }
        let mut prefix_len = 1;
        dprintln!("{}pattern: {:?}", pp, pattern);
        while prefix_len < pattern.len() {
            let prefix = &pattern[..prefix_len];
            let suffix = &pattern[prefix_len..];
            prefix_len += 1;
            if !self.towels.contains(prefix) { continue; }

            let suffix_options = self.arrangements_num(suffix, cache, &(pp.to_string() + " "));
            dprintln!("{}prefix, suffix, options: {:?}", pp, (prefix, suffix, suffix_options));
            patt_options += suffix_options;
        }
        cache.insert(pattern.to_string(), patt_options);
        dprintln!("{}pattern: {:?}", pp, (pattern, patt_options));
        dprintln!("{}----", pp);
        return patt_options;
    }

    fn solve(&mut self) -> i64 {
        let designs = self.designs.clone();
        let mut cache = HashMap::new();
        designs.iter().map(|x| self.arrangements_num(x, &mut cache, "")).sum()
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut solution = Solution::from_input(lines_it);
    dprintln!("solution: {:?}", solution);

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
            "r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb",
            "16",
        );
    }
}
