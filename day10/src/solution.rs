use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::HashSet;
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

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    nodes: Vec<Vec<i64>>,
    trailheads: Vec<XY>,
}

#[allow(dead_code)]
impl Grid {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();
        let mut trailheads = Vec::new();

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for (x, c) in line.char_indices() {
                let h = c.to_digit(10).map_or(-1, |x| x as i64);
                if h == 0 {
                    trailheads.push(XY::unew(x, y));
                }
                nodes[y].push(h);
            }
        }

        Grid {
            nodes,
            trailheads,
        }
    }

    fn is_within(&self, at: &XY) -> bool {
        at.x < self.nodes[0].len() as i64 && at.y < self.nodes.len() as i64 &&
            at.x >= 0 && at.y >= 0
    }

    fn node_at(&self, at: &XY) -> i64 {
        if !self.is_within(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        self.nodes[at.y as usize][at.x as usize]
    }

    fn mut_node_at(&mut self, at: &XY) -> &mut i64 {
        if !self.is_within(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        &mut self.nodes[at.y as usize][at.x as usize]
    }

    fn neighbours(&self, x: &XY) -> Vec<XY> {
        if !self.is_within(x) {
            panic!("Getting node out of bounds: {:?}", x);
        }
        let val = self.node_at(x);

        let mut neighbours = Vec::new();
        for dir in Direction::ALL {
            let maybe_neigh = x.step(&dir);
            if self.is_within(&maybe_neigh) && self.node_at(&maybe_neigh) == val + 1 {
                neighbours.push(maybe_neigh);
            }
        }
        neighbours
    }

    fn dfs(&self, v: &XY, tops: &mut HashSet<XY>) {
        if self.node_at(v) == 9 {
            tops.insert(*v);
        }

        for n in &self.neighbours(v) {
            self.dfs(n, tops);
        }
    }

    fn solve(&self) -> i64 {
        let mut scores = 0;
        for head in &self.trailheads {
            let mut tops = HashSet::new();
            self.dfs(head, &mut tops);
            scores += tops.len() as i64;
            dprintln!("For: {:?}, score: {}, tops: {:?}", head, tops.len(), tops);
        }
        scores
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Grid::from_input(lines_it);

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
            "89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732",
            "36",
        );
    }

    #[test]
    fn sample_single_head() {
        test_ignore_whitespaces(
            "...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9",
            "2",
        );
    }

    #[test]
    fn sample_two_heads() {
        test_ignore_whitespaces(
            "10..9..
            2...8..
            3...7..
            4567654
            ...8..3
            ...9..2
            .....01",
            "3",
        );
    }
}
