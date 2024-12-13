use std::io::{BufRead, BufReader, Write};
use std::cmp::min;
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

    const fn step(&self, dir: &Direction) -> XY { self.add(&dir.as_coords()) }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
use Direction::{UP, RIGHT, DOWN, LEFT};

#[allow(dead_code)]
impl Direction {
    const ALL: [Direction; 4] = [
        UP,
        RIGHT,
        DOWN,
        LEFT,
    ];

    const fn as_coords(&self) -> XY {
        match self {
            UP => XY::new(0, -1),
            RIGHT => XY::new(1, 0),
            DOWN => XY::new(0, 1),
            LEFT => XY::new(-1, 0),
        }
    }

    const fn reverse(&self) -> Self {
        match self {
            UP => DOWN,
            RIGHT => LEFT,
            DOWN => UP,
            LEFT => RIGHT,
        }
    }

    const fn from_to(from: &XY, to: &XY) -> Self {
        let diff = to.sub(from);
        match diff {
            XY { x, y } if x < 0 && y == 0 => LEFT,
            XY { x, y } if x > 0 && y == 0 => RIGHT,
            XY { x, y } if x == 0 && y > 0 => DOWN,
            XY { x, y } if x == 0 && y < 0 => UP,
            _ => panic!("Diagonal!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    buttons: Vec<(XY, XY)>,
    prizes: Vec<XY>,
}

impl Solution {
    const A_COST: i64 = 3;
    const B_COST: i64 = 1;

    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        lazy_static!{
            static ref BUT_RE: Regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
            static ref PRI_RE: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        }

        let mut buttons = Vec::new();
        let mut prizes = Vec::new();
        
        loop {
            let l = if let Some(x) = lines.next() {
                x
            } else {
                break;
            };
            let first_line = l.trim();
            if first_line.len() == 0 { break; }
            let first_caps = BUT_RE.captures(first_line).unwrap();
            let button_a = XY::new(
                first_caps[1].parse::<i64>().unwrap(),
                first_caps[2].parse::<i64>().unwrap()
            );

            let l2 = lines.next().unwrap();
            let sec_line = l2.trim();
            dprintln!("sec_line: {:?}", sec_line);
            let sec_caps = BUT_RE.captures(sec_line).unwrap();
            let button_b = XY::new(
                sec_caps[1].parse::<i64>().unwrap(),
                sec_caps[2].parse::<i64>().unwrap()
            );

            let l3 = lines.next().unwrap();
            let tri_line = l3.trim();
            let tri_caps = PRI_RE.captures(tri_line).unwrap();
            let prize = XY::new(
                tri_caps[1].parse::<i64>().unwrap(),
                tri_caps[2].parse::<i64>().unwrap()
            );

            buttons.push((button_a, button_b));
            prizes.push(prize);

            let _ = lines.next();
        }

        Solution {
            buttons,
            prizes,
        }
    }

    fn cheapest_presses(&self, idx: usize) -> Option<i64> {
        let max_presses = 100;

        let button_a = self.buttons[idx].0;
        let button_b = self.buttons[idx].1;
        let prize = self.prizes[idx];
        let mut res = None;
        for a_press in 0..max_presses {
            let left_x = prize.x - button_a.x * a_press;
            if !(left_x > 0 && left_x % button_b.x == 0) {
                continue;
            }
            let b_press = left_x / button_b.x;
            if a_press * button_a.y + b_press * button_b.y != prize.y {
                continue;
            }

            let cost = a_press * Self::A_COST + b_press * Self::B_COST;
            res = if let Some(r) = res {
                Some(min(r, cost))
            } else {
                Some(cost)
            };
        }
        res
    }

    fn solve(&self) -> i64 {
        (0..self.buttons.len()).filter_map(|idx| self.cheapest_presses(idx)).sum()
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut solution = Solution::from_input(lines_it);
    dprintln!("sol: {:?}", solution);

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
            "Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279",
            "480",
        );
    }
}
