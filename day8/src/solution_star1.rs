use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::HashSet;
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

impl XY {
    const fn new(x: i64, y: i64) -> XY { XY {x, y} }

    const fn add(&self, other: &XY) -> XY { XY { x: self.x + other.x, y: self.y + other.y } }
    const fn sub(&self, other: &XY) -> XY { XY { x: self.x - other.x, y: self.y - other.y } }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
use Direction::{UP, RIGHT, DOWN, LEFT};

impl Direction {
    const fn as_direction(&self) -> XY {
        match self {
            UP => XY::new(0, -1),
            RIGHT => XY::new(1, 0),
            DOWN => XY::new(0, 1),
            LEFT => XY::new(-1, 0),
        }
    }
    
    const fn opposite(&self) -> Direction {
        match self {
            UP    => DOWN,
            RIGHT => LEFT,
            DOWN  => UP,
            LEFT  => RIGHT,
        }
    }

    const fn turn_right(&self) -> Direction {
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Type {
    Empty,
    Antenna(char),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Node {
    typ: Type,
}

impl Node {
    fn from_char(c: char) -> Node {
        let typ = match c {
            '.' => Type::Empty,
            c => Type::Antenna(c),
        };
        Node {
            typ,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    nodes: Vec<Vec<Node>>,
    antennas: HashMap<Type, Vec<XY>>,
}

impl Map {
    fn from_input<I>(lines: I) -> Map
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();
        let mut antennas = HashMap::new();

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for (x, c) in line.char_indices() {
                let curr = Node::from_char(c);
                nodes[y].push(curr);
                match curr.typ {
                    Type::Antenna(_) => {
                        antennas.entry(curr.typ).or_insert(vec![]).push(
                            XY::new(x as i64, y as i64)
                        );
                    }
                    _ => {},
                }
            }
        }

        Map {
            nodes,
            antennas,
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

    fn simple_step(&self, pos: &XY, dir: &Direction) -> XY {
        pos.add(&dir.as_direction())
    }

    fn antinodes_for(&self, a: &XY, b: &XY) -> Vec<XY> {
        let diff = b.sub(a);
        let mut antinodes = Vec::new();
        for n in vec![b.add(&diff), a.sub(&diff)] {
            if self.is_within(&n) {
                antinodes.push(n);
            }
        }
        antinodes
    }

    fn solve(&self) -> i64 {
        let mut antinodes = HashSet::new();
        for (_, locs) in &self.antennas {
            for i in 0..locs.len() {
                for j in (i+1)..locs.len() {
                    let new_antinodes = self.antinodes_for(&locs[i], &locs[j]);
                    antinodes.extend(new_antinodes.into_iter());
                }
            }
        }
        antinodes.len() as i64
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Map::from_input(lines_it);

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
            "............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............",
            "14",
        );
    }
}
