//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use regex::Regex;
use lazy_static::lazy_static;
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
struct Memory {
    content: String,
}

impl Memory {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut content = String::new();
        for l in lines {
            let line = l.trim();
            content.push_str(line);
        }
        Memory {
            content,
        }
    }

    fn multiply(&self) -> i64 {
        lazy_static!{
            static ref MUL_RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        }

        let muls = MUL_RE.find_iter(&self.content).map(|m| m.as_str().to_owned()).collect::<Vec<String>>();
        let mut result = 0;
        for op in muls {
            let mat = MUL_RE.captures(&op).unwrap();
            let a = mat.get(1).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            let b = mat.get(2).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            result += a * b;
        }
        result
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let memory = Memory::from_input(lines_it);

    writeln!(output, "{}", memory.multiply()).unwrap();
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
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            "161",
        );
    }
    #[test]
    fn sample_enter() {
        test_ignore_whitespaces(
            "xmul(2,4)%&mul[3,7]!@^do_no
            t_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            "161",
        );
    }
}
