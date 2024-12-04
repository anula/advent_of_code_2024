//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
//use regex::Regex;
//use lazy_static::lazy_static;
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
    lines: Vec<Vec<char>>,
}

impl WordSearch {
    fn from_input<I>(input: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut lines = Vec::new();
        
        for l in input {
            let line = l.trim();
            lines.push(line.chars().collect());
        }
        WordSearch {
            lines,
        }
    }

    fn is_x_mas(&self, i: usize, j: usize) -> bool {
        if i > self.lines[0].len() - 3 { return false; }
        if j > self.lines.len() - 3 { return false; }

        let left_diag: String =
            vec![self.lines[i][j], self.lines[i+1][j+1], self.lines[i+2][j+2]].into_iter().collect();
        let right_diag: String =
            vec![self.lines[i+2][j], self.lines[i+1][j+1], self.lines[i][j+2]].into_iter().collect();

        (left_diag == "MAS" || left_diag == "SAM") &&
            (right_diag == "MAS" || right_diag == "SAM")
    }

    fn count_x_mas(&self) -> i64 {
        let mut res = 0;
        for i in 0..self.lines[0].len() {
            for j in 0..self.lines.len() {
                if self.is_x_mas(i, j) {
                    res += 1;
                }
            }
        }
        res
        
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let word_search = WordSearch::from_input(lines_it);

    dprintln!("words: {:?}", word_search);

    writeln!(output, "{}", word_search.count_x_mas()).unwrap();
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
            ".M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........",
            "9",
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
            "9",
        );
    }
}
