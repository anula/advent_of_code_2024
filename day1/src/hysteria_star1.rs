//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
//use std::collections::{HashMap};

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug)]
struct LocationLists {
    left_list: Vec<i64>,
    right_list: Vec<i64>,
}

impl LocationLists {
    fn from_input<I>(lines: I) -> LocationLists
        where I: Iterator<Item = String>
    {

        let mut left_list = Vec::new();
        let mut right_list = Vec::new();
        for l in lines {
            let line = l.trim();
            let locs = line.split_whitespace().collect::<Vec<&str>>();

            left_list.push(locs[0].parse::<i64>().unwrap());
            right_list.push(locs[1].parse::<i64>().unwrap());
        }
        LocationLists {
            left_list,
            right_list,
        }
    }

    fn sort_each(&mut self) {
        self.left_list.sort();
        self.right_list.sort();
    }

    fn sum_distances(&self) -> i64 {
        self.left_list.iter().zip(self.right_list.iter())
            .map(|(x, y)| (x - y).abs()).sum()
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut locs = LocationLists::from_input(lines_it);
    dprintln!("Locs: {:?}", locs);
    locs.sort_each();
    dprintln!("Sorted: {:?}", locs);

    writeln!(output, "{}", locs.sum_distances()).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_exact(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        assert_eq!(String::from_utf8(actual_out).unwrap(), output);
    }

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
            "3   4
            4   3
            2   5
            1   3
            3   9
            3   3",
            "11",
        );
    }
}
