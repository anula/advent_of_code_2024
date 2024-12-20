use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::HashSet;
use std::collections::HashMap;
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

#[allow(dead_code)]
impl Node {
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    nodes: Vec<Vec<Node>>,
    start: XY,
    end: XY,
}

#[allow(dead_code)]
impl Grid {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();
        let mut start = XY::new(-1, -1);
        let mut end = XY::new(-1, -1);

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for (x, c) in line.char_indices() {
                nodes[y].push(
                    match c {
                        '.' => Node::Empty,
                        '#' => Node::Blocked,
                        'S' => {
                            start = XY::unew(x, y);
                            Node::Empty
                        }
                        'E' => {
                            end = XY::unew(x, y);
                            Node::Empty
                        }
                        _ => panic!("Unknown input!"),
                    }
                );
            }
        }

        Grid {
            nodes,
            start,
            end,
        }
    }

    fn is_within(&self, at: &XY) -> bool {
        at.x < self.nodes[0].len() as i64 && at.y < self.nodes.len() as i64 &&
            at.x >= 0 && at.y >= 0
    }

    fn node_at(&self, at: &XY) -> &Node {
        if !self.is_within(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        &self.nodes[at.y as usize][at.x as usize]
    }

    fn mut_node_at(&mut self, at: &XY) -> &mut Node {
        if !self.is_within(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        &mut self.nodes[at.y as usize][at.x as usize]
    }

    fn neighbours(&self, pos: &XY) -> Vec<XY> {
        let mut res = Vec::new();
        for dir in Direction::ALL {
            let cand = pos.step(&dir);
            if !self.is_within(&cand) { continue; }
            if self.node_at(&cand) == &Node::Empty {
                res.push(cand);
            }
        }
        res
    }

    fn all_neighbours(&self, pos: &XY) -> Vec<XY> {
        let mut res = Vec::new();
        for dir in Direction::ALL {
            let cand = pos.step(&dir);
            if !self.is_within(&cand) { continue; }
            res.push(cand);
        }
        res
    }

    fn all_backwards_shortcuts(&self, pos: &XY) -> Vec<(XY, XY)> {
        let mut res = Vec::new();
        for short_end in self.all_neighbours(pos) {
            for short_start in self.all_neighbours(&short_end) {
                if short_start == *pos { continue; }
                res.push((short_start, short_end));
            }
        }
        res
    }

    fn find_shortcuts(&self, at_least: i64) -> usize {
        let mut queue: VecDeque<(XY, i64)> = VecDeque::new();
        let mut edges_back = HashMap::new();
        let mut dists = HashMap::new();

        queue.push_back((self.start, 0));
        dists.insert(self.start, 0);

        while let Some((pos, dist)) = queue.pop_front() {
            for n in self.neighbours(&pos) {
                if dists.contains_key(&n) {
                    continue;
                }
                edges_back.insert(n, pos);
                dists.insert(n, dist + 1);
                queue.push_back((n, dist + 1));
                if n == self.end { 
                    break;
                }
            }
        }
        let edges_back = edges_back;
        let dists = dists;

        dprintln!("Shortest path found, edges_back: {:?}", edges_back);

        let mut path = HashSet::new();
        {
            let mut next_pos = Some(&self.end);
            while let Some(next) = next_pos {
                path.insert(next);
                next_pos = edges_back.get(&next);
            }
        }
        dprintln!("the nodes in path: {:?}", path);

        let path_dist = dists.get(&self.end).unwrap();
        let path = path;
        let mut shorts = HashSet::new();

        {
            let mut next_pos = Some(self.end);
            while let Some(next) = next_pos {
                let dist = dists.get(&next).unwrap();
                let dist_left = path_dist - dist;
                for shortcut in self.all_neighbours(&next) {
                    for from_path in self.all_neighbours(&shortcut) {
                        if from_path == next { continue; }
                        if !path.contains(&from_path) { continue; }
                        let cut_dist = dists.get(&from_path).unwrap();
                        let total_dist = cut_dist + 1 + dist_left;
                        let saved_dist = path_dist - total_dist;
                        dprintln!("Found a short {:?} with saved_dist: {}", shortcut, saved_dist);
                        if saved_dist >= at_least { shorts.insert(shortcut); }
                    }
                }
                next_pos = edges_back.get(&next).copied();
            }
        }
        dprintln!("shorts: {:?}", shorts);
        shorts.len()
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Solution {
    grid: Grid,
    at_least: i64,
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let l = lines.next().unwrap();
        let at_least = l.trim().parse::<i64>().unwrap();
        let grid = Grid::from_input(lines);
        Solution {
            grid,
            at_least,
        }
    }

    fn solve(&self) -> usize {
        self.grid.find_shortcuts(self.at_least)
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Solution::from_input(lines_it);
    dprintln!("solution: {:?}", solution);

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
            "64
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############",
            "1",
        );
    }
}
