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
struct Reports {
    reports: Vec<Vec<i64>>,
}

impl Reports {
    fn from_input<I>(lines: I) -> Reports
        where I: Iterator<Item = String>
    {
        let mut reports = Vec::new();
        for l in lines {
            let line = l.trim();
            let report = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();

            reports.push(report);
        }
        Reports {
            reports,
        }
    }

    fn is_safe(report: &[i64]) -> bool {
        if report.is_empty() { return true; }

        let mut last = report[0];
        let mut last_diff = 0;
        for x in report.iter().skip(1) {
            let diff = x - last;
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }
            if last_diff * diff < 0 {
                return false;
            }
            last_diff = diff;
            last = *x;
        }
        true
    }

    fn count_safe(&self) -> u64 {
        let mut safe = 0;
        for r in &self.reports {
            if Reports::is_safe(r) {
                safe += 1;
            }
        }
        safe
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let reports = Reports::from_input(lines_it);

    writeln!(output, "{}", reports.count_safe()).unwrap();
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
            "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9",
            "2",
        );
    }
}
