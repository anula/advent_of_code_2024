use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
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

    fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            _ => { panic!("wrong direction input!"); },
        }
    }

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
    Wall,
    Boks,
    Robot,
    Empty,
}

#[allow(dead_code)]
impl Node {
    fn from_char(c: char) -> Node {
        match c {
            '.' => Node::Empty,
            '#' => Node::Wall,
            'O' => Node::Boks,
            '@' => Node::Robot,
            _ => { panic!("wrong input!"); },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    robot_pos: XY,
    nodes: Vec<Vec<Node>>,
    moves: Vec<Direction>,
}

#[allow(dead_code)]
impl Grid {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();
        let mut robot_pos = XY::new(-1, -1);

        for (y, l) in lines.by_ref().enumerate() {
            let line = l.trim();
            if line == "" { break; }

            nodes.push(Vec::new());

            for (x, c) in line.char_indices() {
                nodes[y].push(Node::from_char(c));
                if nodes[y][x] == Node::Robot {
                    robot_pos = XY::unew(x, y);
                }
            }
        }
        if robot_pos == XY::new(-1, -1) {
            panic!("Did not get robot position");
        }

        let mut moves = Vec::new();
        for l in lines {
            let line = l.trim();
            for (_, c) in line.char_indices() {
                moves.push(Direction::from_char(c));
            }
        }

        Grid {
            robot_pos,
            nodes,
            moves,
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

    fn single_move(&mut self, dir: &Direction) {
        let robot_pos = self.robot_pos.clone();
        let maybe_new_pos = self.robot_pos.step(dir);
        if *self.node_at(&maybe_new_pos) == Node::Wall {
            return;
        }
        if *self.node_at(&maybe_new_pos) == Node::Empty {
            let new_pos_node = self.mut_node_at(&maybe_new_pos);
            *new_pos_node = Node::Robot;
            let old_pos_node = self.mut_node_at(&robot_pos);
            *old_pos_node = Node::Empty;
            self.robot_pos = maybe_new_pos;
            return;
        }
        // So it has to be Boks.
        let mut box_line_end = maybe_new_pos.clone();
        while *self.node_at(&box_line_end) == Node::Boks {
            box_line_end = box_line_end.step(dir);
        }

        if *self.node_at(&box_line_end) == Node::Wall {
            return;
        }
        let new_pos_node = self.mut_node_at(&maybe_new_pos);
        *new_pos_node = Node::Robot;
        let old_pos_node = self.mut_node_at(&robot_pos);
        *old_pos_node = Node::Empty;
        let new_box_pos = self.mut_node_at(&box_line_end);
        *new_box_pos = Node::Boks;

        self.robot_pos = maybe_new_pos;
    }

    fn follow_moves(&mut self) {
        for i in 0..self.moves.len() {
            let dir = self.moves[i].clone();
            self.single_move(&dir);
        }
    }

    fn gps_sum(&self) -> i64 {
        let mut res = 0;
        for y in 0..self.nodes.len() {
            for x in 0..self.nodes[0].len() {
                let pos = XY::unew(x, y);
                if *self.node_at(&pos) == Node::Boks {
                    res += 100 * pos.y + pos.x;
                }
            }
        }
        res
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut grid = Grid::from_input(lines_it);
    dprintln!("grid: {:?}", grid);
    grid.follow_moves();
    dprintln!("grid after: {:?}", grid);

    writeln!(output, "{}", grid.gps_sum()).unwrap();
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
            "##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
            "10092",
        );
    }

    #[test]
    fn sample_smaller() {
        test_ignore_whitespaces(
            "########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<",
            "2028",
        );
    }
}
