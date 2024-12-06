//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
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
    Block,
    Start,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Node {
    typ: Type,
}

impl Node {
    fn from_char(c: char) -> Node {
        let typ = match c {
            '.' => Type::Empty,
            '#' => Type::Block,
            '^' => Type::Start,
            _ => panic!("Wrong char!"),
        };
        Node {
            typ,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Map {
    nodes: Vec<Vec<Node>>,
    start: XY,
}

impl Map {
    fn from_input<I>(lines: I) -> Map
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();
        let mut start = XY::new(-1, -1);

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for (x, c) in line.char_indices() {
                let curr = Node::from_char(c);
                nodes[y].push(Node {
                    typ: match curr.typ {
                        Type::Start => {
                            start = XY::new(x as i64, y as i64);
                            Type::Empty
                        },
                        x => x,
                    }
                });
            }
        }

        Map {
            nodes,
            start,
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

    fn next_step(&self, pos: &XY, dir: &Direction) -> (XY, Direction) {
        let maybe = pos.add(&dir.as_direction());
        if !self.is_within(&maybe) { return (maybe, dir.clone() ); }

        if self.node_at(&maybe).typ == Type::Block {
            (pos.clone(), dir.turn_right())
        } else {
            (maybe, dir.clone())
        }
    }

    fn simple_step(&self, pos: &XY, dir: &Direction) -> XY {
        pos.add(&dir.as_direction())
    }

    fn is_block(&self, pos: &XY) -> bool {
        self.is_within(pos) && self.node_at(pos).typ == Type::Block
    }

    //fn fill_the_line_of_visitation(
    //    &self, visited: &mut HashMap<XY, HashSet<Direction>>, pos: &XY, dir: &Direction) {
    //    let opp = dir.opposite();
    //    let block_check = opp.turn_right();
    //    let mut curr_pos = self.simple_step(pos, &opp);
    //    while self.is_within(&curr_pos) && self.node_at(&curr_pos).typ != Type::Block {
    //        let vi_dirs = visited.entry(curr_pos).or_insert(HashSet::new());
    //        if vi_dirs.contains(dir) {
    //            break;
    //        }
    //        dprintln!("visited add: {:?}", (curr_pos, dir));
    //        vi_dirs.insert(*dir);
    //        let maybe_block = self.simple_step(&curr_pos, &block_check);
    //        if self.is_block(&maybe_block) {
    //            dprintln!("{:?} is block", maybe_block);
    //            self.fill_the_line_of_visitation(visited, &curr_pos, &block_check);
    //        }
    //        curr_pos = self.simple_step(&curr_pos, &opp);
    //    }
    //}
    
    fn is_cycle(&self) -> bool {
        let mut curr_pos = self.start.clone();
        let mut curr_dir = UP;
        let mut visited = HashMap::<XY, HashSet<Direction>>::new();

        while self.is_within(&curr_pos) {
            if let Some(dirs) = visited.get(&curr_pos) {
                if dirs.contains(&curr_dir) { return true; }
            }
            visited.entry(curr_pos).or_insert(HashSet::new()).insert(curr_dir);
            let (next_pos, next_dir) = self.next_step(&curr_pos, &curr_dir);
            curr_pos = next_pos;
            curr_dir = next_dir;
        }
        false
    }

    fn walk_searching(&mut self) -> i64 {
        let mut curr_pos = self.start.clone();
        let mut curr_dir = UP;
        let mut new_obstacles = HashSet::new();

        while self.is_within(&curr_pos) {
            let potential_obstacle = self.simple_step(&curr_pos, &curr_dir);
            if self.is_within(&potential_obstacle)
                && !self.is_block(&potential_obstacle) && potential_obstacle != self.start {
                    self.mut_node_at(&potential_obstacle).typ = Type::Block;
                    if self.is_cycle() {
                        new_obstacles.insert(potential_obstacle);
                    }
                    self.mut_node_at(&potential_obstacle).typ = Type::Empty;
            }
            let (next_pos, next_dir) = self.next_step(&curr_pos, &curr_dir);
            curr_pos = next_pos;
            curr_dir = next_dir;
        }

        dprintln!("New obs: {:?}", new_obstacles);
        new_obstacles.len() as i64
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut solution = Map::from_input(lines_it);

    writeln!(output, "{}", solution.walk_searching()).unwrap();
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
            "....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...",
            "6",
        );
    }

    #[test]
    fn not_on_start() {
        test_ignore_whitespaces(
            "##...
             ....#
             .....
             ^....
             .#.#.",
            "0",
        );
    }

    #[test]
    fn along_path() {
        test_ignore_whitespaces(
            "##...
             ....#
             .....
             .....
             ^#.#.",
            "0",
        );
        test_ignore_whitespaces(
            "##...
             ....#
             ^....
             .....
             .#.#.",
            "1",
        );
    }

    #[test]
    fn path_blocked() {
        test_ignore_whitespaces(
            "##...
             ....#
             ...#.
             .....
             ^#.#.",
            "0",
        );
    }
    
    #[test]
    fn simple() {
        test_ignore_whitespaces(
            ".#..
             ...#
             ....
             ....
             .^#.",
            "1",
        );
        test_ignore_whitespaces(
            ".#..
             ...#
             .^..
             ....
             ..#.",
            "1",
        );
    }
}
