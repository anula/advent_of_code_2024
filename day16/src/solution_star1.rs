use std::io::{BufRead, BufReader, Write};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
//use std::cmp::{max, min};
//use regex::Regex;
//use lazy_static::lazy_static;
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

    const fn turn_left(&self) -> Self {
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
        }
    }

    const fn turn_right(&self) -> Self {
        match self {
            UP => LEFT,
            RIGHT => UP,
            DOWN => RIGHT,
            LEFT => DOWN,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Node {
    Empty,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct QueueEntry {
    pos: XY,
    dir: Direction,
    dist: i64,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the order to get min queue.
        other.dist.cmp(&self.dist).then_with(|| other.pos.x.cmp(&self.pos.x))
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    nodes: Vec<Vec<Node>>,

    start: XY,
    end: XY,
}

#[allow(dead_code)]
impl Grid {
    const FORWARD_COST: i64 = 1;
    const TURN_COST: i64 = 1000;
    const START_TURN: Direction = RIGHT;

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
                let node = match c {
                    '.' => Node::Empty,
                    '#' => Node::Wall,
                    'S' => {
                        start = XY::unew(x, y);
                        Node::Empty
                    },
                    'E' => {
                        end = XY::unew(x, y);
                        Node::Empty
                    },
                    _ => panic!("bad input"),
                };
                nodes[y].push(node);
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

    fn neighbours(&self, pos: &XY, dir: &Direction, dist: i64) -> Vec<QueueEntry> {
        let mut neighs = Vec::new();
        let cheap = pos.step(dir);
        if *self.node_at(&cheap) != Node::Wall {
            neighs.push(QueueEntry {
                pos: cheap,
                dir: dir.clone(),
                dist: dist + Self::FORWARD_COST,
            });
        }

        let left_dir = dir.turn_left();
        let left = pos.step(&left_dir);
        if *self.node_at(&left) != Node::Wall {
            neighs.push(QueueEntry {
                pos: pos.clone(),
                dir: left_dir,
                dist: dist + Self::TURN_COST,
            });
        }

        let right_dir = dir.turn_right();
        let right = pos.step(&right_dir);
        if *self.node_at(&right) != Node::Wall {
            neighs.push(QueueEntry {
                pos: pos.clone(),
                dir: right_dir,
                dist: dist + Self::TURN_COST,
            });
        }
        neighs
    }

    fn shortest_path(&self) -> i64 {
        let mut queue = BinaryHeap::new();
        let mut dists = HashMap::<(XY, Direction), i64>::new();

        dists.insert((self.start, Self::START_TURN), 0);
        queue.push(QueueEntry {
            pos: self.start,
            dir: Self::START_TURN,
            dist: 0,
        });

        while let Some(QueueEntry { pos, dir, dist }) = queue.pop() {
            if let Some(ex_dist) = dists.get(&(pos, dir)) {
                if *ex_dist > dist {
                    continue;
                }
            }
            dists.insert((pos, dir), dist);
            for n in self.neighbours(&pos, &dir, dist) {
                if n.pos == self.end {
                    dprintln!("Distances: {:?}", dists);
                    return n.dist;
                }

                let already = dists.get(&(n.pos, n.dir));
                if already == None || *already.unwrap() > n.dist {
                    queue.push(n);
                }

            }
        }
        panic!("Found no path at all!");
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
    fn sample1() {
        test_ignore_whitespaces(
            "###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############",
            "7036",
        );
    }

    #[test]
    fn sample2() {
        test_ignore_whitespaces(
            "#################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################",
            "11048",
        );
    }
}
