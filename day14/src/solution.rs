use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
use regex::Regex;
use lazy_static::lazy_static;
//use std::collections::HashSet;
use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    fn print(&self) -> (String, bool) {
        let mid_width = self.wide/2;
        let mid_height = self.tall/2;

        let mut res = String::new();
        let mut robot_locations = HashMap::<XY, Vec<usize>>::new();
        for (i, robot) in self.robots.iter().enumerate() {
            robot_locations.entry(robot.pos).or_insert(vec![]).push(i);
        }
        let mut quadrant_tl = 0;
        let mut quadrant_tr = 0;
        let mut quadrant_bl = 0;
        let mut quadrant_br = 0;
        let mut interesting = true;
        //println!("hashmap: {:?}", robot_locations);
        for y in 0..self.tall {
            for x in 0..self.wide {
                let curr = XY::new(x, y);
                let cha = &robot_locations.get(&curr).map_or('.'.to_string(), |x| x.len().to_string());
                if y < 35 && x < 7 {
                    if cha != "." { interesting = false; }
                }
                res.push_str(cha);
                let num = robot_locations.get(&curr).map_or(0, |x| x.len() as i64);
                if curr.x < mid_width && curr.y < mid_height {
                    quadrant_tl += num;
                }
                if curr.x < mid_width && curr.y > mid_height {
                    quadrant_bl += num;
                }
                if curr.x > mid_width && curr.y < mid_height {
                    quadrant_tr += num;
                }
                if curr.x > mid_width && curr.y > mid_height {
                    quadrant_br += num;
                }
            }
            res.push_str("\n");
        }
        (res, quadrant_bl > quadrant_tl + 20 && quadrant_br > quadrant_tr + 20)
    }

    fn advance(&self, seconds: i64) -> Self {
        let mut new_robots = Vec::new();

        for robot in &self.robots {
            let mut new_pos = robot.pos.add(&robot.vel.mul(&XY::new(seconds, seconds)));
            new_pos.x %= self.wide;
            if new_pos.x < 0 { 
                new_pos.x += self.wide;
            }
            new_pos.y %= self.tall;
            if new_pos.y < 0 { new_pos.y += self.tall }
            new_robots.push(
                Robot {
                    pos: new_pos,
                    vel: robot.vel,
                }
            );
        }
        Solution {
            wide: self.wide,
            tall: self.tall,
            robots: new_robots,
        }
    }

}

fn solve<R: BufRead, W: Write>(input: R, mut _output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Solution::from_input(lines_it);
    dprintln!("solution: {:?}", solution);

    // The whole thing repeats after this many
    let total_offset = 733;
    let total_cycle = 10403;

    // for the middle heavy thing
    let offset = 35;
    let cycle = 101;

    let mut prev = String::new(); 
    let mut prev_sol = solution.clone();
    for i in 0..total_cycle {
        let secs = offset + i*cycle;
        if secs > total_cycle + total_offset { break }
        //let secs = i;
        let sol = solution.advance(secs);
        let (picture, interesting) = sol.print();
        if picture == prev {
            println!("same picture as prev");
        }
        if prev_sol == sol {
            println!("same sol as prev");
        }
        if interesting || true {
            println!("Seconds {}:", secs);
            println!("{}", picture);
            println!("\n\n");
        }
        prev = picture;
        prev_sol = sol;
    }

    //writeln!(output, "{}", solution.solve(100)).unwrap();
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
