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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Type {
    Lock,
    Key,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Item {
    typ: Type,
    heights: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    items: Vec<Item>,
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut items = vec![];
        loop {
            let mut columns = vec![vec![]; 5];
            let mut idx = 0;
            let typ;
            for l in lines.by_ref() {
                let line = l.trim();
                if line == "" {
                    break;
                }

                if idx == 0 {
                    typ = match line {
                        "#####" => Type::Lock,
                        "....." => Type::Key,
                        _ => panic!("Wrong type!"),
                    }
                }

                for (i, c) in line.char_indices() {
                    columns[i].push(c);
                }

                idx += 1;
            }
            if columns[0].len() == 0 {
                break;
            }
            //let mut heihts
            //items.push(Item {
            //    typ,

            //}
        }

        Solution {
        }
    }

    fn solve(&self) -> i64 {
        0
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
            "",
            "0",
        );
    }
}
