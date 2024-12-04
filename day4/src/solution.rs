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
struct WordSearch {
    lines: Vec<String>,
    columns: Vec<String>,
    left_diags: Vec<String>,
    right_diags: Vec<String>,
}

fn count_xmas_in_coll(coll: &[String]) -> i64 {
    lazy_static!{
        static ref X_RE: Regex = Regex::new(r"XMAS").unwrap();
        static ref S_RE: Regex = Regex::new(r"SAMX").unwrap();
    }

    let mut res: i64 = 0;

    for l in coll {
        res += X_RE.find_iter(l).count() as i64;
        res += S_RE.find_iter(l).count() as i64;
    }
    res
}

impl WordSearch {
    fn from_input<I>(input: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut lines = Vec::<String>::new();
        
        for l in input {
            let line = l.trim();
            lines.push(line.to_owned());
        }

        let mut columns = vec![String::new(); lines[0].len()];
        for i in 0..lines[0].len() {
            for j in 0..lines.len() {
                columns[i].push_str(&lines[j][i..i+1]);
            }
        }
        

        let mut left_diags = vec![String::new(); lines[0].len() + lines.len() - 1];
        let mut idx_st = 0;
        for i in 0..lines[0].len() {
            let mut idx = idx_st;
            for j in (0..lines.len()).rev() {
                left_diags[idx].push_str(&lines[j][i..i+1]);
                idx += 1;
            }
            idx_st += 1;
        }

        let mut right_diags = vec![String::new(); lines[0].len() + lines.len() - 1];
        idx_st = 0;
        for i in (0..lines[0].len()).rev() {
            let mut idx = idx_st;
            for j in (0..lines.len()).rev() {
                right_diags[idx].push_str(&lines[j][i..i+1]);
                idx += 1;
            }
            idx_st += 1;
        }
        
        WordSearch {
            lines,
            columns,
            left_diags,
            right_diags,
        }
    }

    fn count_xmas(&self) -> i64 {

        count_xmas_in_coll(&self.lines) + count_xmas_in_coll(&self.columns) +
            count_xmas_in_coll(&self.left_diags) + count_xmas_in_coll(&self.right_diags)
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let word_search = WordSearch::from_input(lines_it);

    dprintln!("words: {:?}", word_search);

    writeln!(output, "{}", word_search.count_xmas()).unwrap();
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
            "....XXMAS.
            .SAMXMS...
            ...S..A...
            ..A.A.MS.X
            XMASAMX.MM
            X.....XA.A
            S.S.S.S.SS
            .A.A.A.A.A
            ..M.M.M.MM
            .X.X.XMASX",
            "18",
        );
        test_ignore_whitespaces(
            "MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX",
            "18",
        );
    }

    #[test]
    fn small() {
        test_ignore_whitespaces(
            "..X...
            .SAMX.
            .A..A.
            XMAS.S
            .X....",
            "4",
        );
    }

    #[test]
    fn line() {
        test_ignore_whitespaces(
            "XMASAMX.MM",
            "2",
        );
    }
}
