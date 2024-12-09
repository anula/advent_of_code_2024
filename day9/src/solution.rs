use std::io::{BufRead, BufReader, Write};
//use std::cmp::min;
//use std::cmp::max;
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Space {
    File{ id: i64, size: i64 },
    Empty{ size: i64 },
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    disk_map: Vec<Space>,
    total_blocks: i64,
}

fn sum_from_to(from: i64, to: i64) -> i64{
    (from + to) * (to - from + 1) / 2
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let l = lines.next().unwrap();
        let line = l.trim();

        let mut disk_map = Vec::new();
        let mut total_blocks = 0;

        let mut file = true;
        let mut file_id = 0;
        for ch in line.chars() {
            let len = ch.to_digit(10).unwrap() as i64;
            if file {
                disk_map.push(Space::File{ id: file_id, size: len});
                file_id += 1;
            } else {
                disk_map.push(Space::Empty{ size: len });
            }
            total_blocks += len;
            file = !file;
        }

        Solution {
            disk_map,
            total_blocks,
        }
    }

    fn checksum(disk_map: &[Space]) -> i64 {
        let mut blocks_before = 0;
        let mut checksum = 0;
        for s in disk_map {
            blocks_before += match s {
                Space::File { id, size } => {
                    checksum += sum_from_to(blocks_before, blocks_before + size - 1) * id;
                    size
                },
                Space::Empty { size } => size,
            }
        }
        checksum
    }

    fn solve(&self) -> i64 {
        let mut last_file = (self.disk_map.len() as i64) - 1;
        if last_file  % 2 == 1 {
            last_file -= 1;
        }
        let last_file = last_file;
        
        let mut new_disk_map = self.disk_map.clone();

        for j in (0..(last_file+1)).rev().step_by(2) {
            let (j_id, len) = match self.disk_map[j as usize] {
                Space::File { id, size } => (id, size),
                _ => panic!("j should be on File!"),
            };
            for idx in 0..new_disk_map.len() {
                if let Space::File { id, .. } = new_disk_map[idx] {
                    if id == j_id { break; }
                }
                match new_disk_map[idx] {
                    Space::File { .. } => continue,
                    Space::Empty { size } => {
                        if size >= len {
                            let file = self.disk_map[j as usize].clone();
                            let new_size = size - len;
                            new_disk_map.remove(idx);
                            if new_size > 0 {
                                new_disk_map.insert(idx, Space::Empty{ size: new_size });
                            }
                            new_disk_map.insert(idx, file);
                            for del_idx in (0..new_disk_map.len()).rev() {
                                match new_disk_map[del_idx] {
                                    Space::File { id, size } => {
                                        if id == j_id {
                                            new_disk_map.remove(del_idx);
                                            new_disk_map.insert(del_idx, Space::Empty{ size });
                                            break;
                                        }
                                    }
                                    _ => {},
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
        dprintln!("new map: {:?}", new_disk_map);

        Solution::checksum(&new_disk_map)
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Solution::from_input(lines_it);

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
            "2333133121414131402",
            "2858",
        );
    }

    #[test]
    fn simple() {
        test_ignore_whitespaces(
            "12345",
            "132",
        );
    }

    #[test]
    fn checksum() {
        let sol = Solution::from_input(vec!["12345".to_string()].into_iter());
        assert_eq!(Solution::checksum(&sol.disk_map), 132);
    }
}
