use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::HashSet;
//use std::collections::HashMap;
use std::collections::VecDeque;

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
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Node {
    Empty,
    Blocked,
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    height: usize,
    width: usize,
    corruptions: Vec<XY>,
    corruptions_num: usize,
}

#[allow(dead_code)]
impl Grid {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut corruptions = Vec::new();
        let l_size = lines.next().unwrap();
        dprintln!("lsize: {:?}", l_size);
        let size = l_size.trim().parse::<usize>().unwrap();
        let l_corrs = lines.next().unwrap();
        let corruptions_num = l_corrs.trim().parse::<usize>().unwrap();
        for l in lines {
            let line = l.trim();
            let parts = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            corruptions.push(XY::new(parts[0], parts[1]));
        }

        Grid {
            height: size + 1,
            width: size + 1,
            corruptions,
            corruptions_num,
        }
    }

    fn is_within(&self, at: &XY) -> bool {
        at.x < self.width as i64 && at.y < self.height as i64 &&
            at.x >= 0 && at.y >= 0
    }

    fn node_at(&self, at: &XY) -> Node {
        if !self.is_within(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        if self.corruptions[0..self.corruptions_num].contains(at) {
            Node::Blocked
        } else {
            Node::Empty
        }
    }

    fn neighbours(&self, pos: &XY) -> Vec<XY> {
        let mut neighs = Vec::new();
        for dir in Direction::ALL {
            let potential = pos.step(&dir);
            if !self.is_within(&potential) { continue; }
            if self.node_at(&potential) == Node::Empty {
                neighs.push(potential);
            }
        }
        neighs
    }

    fn shortest_path(&self) -> i64 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let start = XY::new(0, 0);
        let end = XY::unew(self.width - 1, self.height - 1);

        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((pos, dist)) = queue.pop_front() {
            dprintln!("pos, dist: {:?}", (pos, dist));
            if pos == end { return dist; }
            for n in self.neighbours(&pos) {
                if pos == XY::new(5, 0) {
                    dprintln!("n: {:?}", n);
                }
                if visited.contains(&n) {
                    continue;
                }
                visited.insert(n);
                queue.push_back((n, dist+1));
            }

        }
        panic!("Found no path!")
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut solution = Grid::from_input(lines_it);

    writeln!(output, "{}", solution.shortest_path()).unwrap();
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
            "6
            12
            5,4
            4,2
            4,5
            3,0
            2,1
            6,3
            2,4
            1,5
            0,6
            3,3
            2,6
            5,1
            1,2
            5,5
            2,5
            6,5
            1,4
            0,4
            6,4
            1,1
            6,1
            1,0
            0,5
            1,6
            2,0",
            "22",
        );
    }
}
