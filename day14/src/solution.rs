use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
use regex::Regex;
use lazy_static::lazy_static;
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
struct XY {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
impl XY {
    const AROUND: [XY; 4] = [
        XY::new(0, -1),
        XY::new(1, 0),
        XY::new(0, 1),
        XY::new(-1, 0),
    ];

    const fn new(x: i64, y: i64) -> XY { XY {x, y} }
    const fn unew(x: usize, y: usize) -> XY { XY {x: x as i64, y: y as i64} }

    const fn add(&self, other: &XY) -> XY { XY { x: self.x + other.x, y: self.y + other.y } }
    const fn sub(&self, other: &XY) -> XY { XY { x: self.x - other.x, y: self.y - other.y } }
    const fn mul(&self, other: &XY) -> XY { XY { x: self.x * other.x, y: self.y * other.y } }

    const fn ux(&self) -> usize { self.x as usize }
    const fn uy(&self) -> usize { self.y as usize }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Robot {
    pos: XY,
    vel: XY,
}

impl Robot {
    fn from_str(line: &str) -> Self {
        lazy_static!{
            static ref RO_RE: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        }
        let caps = RO_RE.captures(line).unwrap();
        Robot {
            pos: XY::new(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            vel: XY::new(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    wide: i64,
    tall: i64,
    robots: Vec<Robot>,
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let params_line = lines.next().unwrap();
        let params: Vec<i64> = params_line
            .split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
        let mut robots = Vec::new();
        for l in lines {
            let line = l.trim();
            robots.push(Robot::from_str(line));
        }

        Solution {
            wide: params[0],
            tall: params[1],
            robots,
        }
    }

    fn solve(&self, seconds: i64) -> i64 {
        let mid_width = self.wide/2;
        let mid_height = self.tall/2;

        let mut quadrant_tl = 0;
        let mut quadrant_tr = 0;
        let mut quadrant_bl = 0;
        let mut quadrant_br = 0;

        for robot in &self.robots {
            let mut new_pos = robot.pos.add(&robot.vel.mul(&XY::new(seconds, seconds)));
            dprintln!("robot: {:?}", robot);
            dprintln!(" -- new_pos: {:?}", new_pos);
            new_pos.x %= self.wide;
            if new_pos.x < 0 { 
                new_pos.x += self.wide;
            }
            new_pos.y %= self.tall;
            if new_pos.y < 0 { new_pos.y += self.tall }
            dprintln!(" -- new_pos mod: {:?}", new_pos);
            if new_pos.x < mid_width && new_pos.y < mid_height {
                dprintln!(" -- TL");
                quadrant_tl += 1;
            }
            if new_pos.x < mid_width && new_pos.y > mid_height {
                dprintln!(" -- BL");
                quadrant_bl += 1;
            }
            if new_pos.x > mid_width && new_pos.y < mid_height {
                dprintln!(" -- TR");
                quadrant_tr += 1;
            }
            if new_pos.x > mid_width && new_pos.y > mid_height {
                dprintln!(" -- BR");
                quadrant_br += 1;
            }
        }
        dprintln!("quadrants: {:?}", (quadrant_tl, quadrant_tr));
        dprintln!("quadrants: {:?}", (quadrant_bl, quadrant_br));
        quadrant_tl * quadrant_tr * quadrant_bl * quadrant_br
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut solution = Solution::from_input(lines_it);
    dprintln!("solution: {:?}", solution);

    writeln!(output, "{}", solution.solve(100)).unwrap();
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
            "11 7
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3",
            "12",
        );
    }
}
