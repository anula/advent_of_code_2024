//use std::io::{BufRead, BufReader, Write};
//use std::collections::HashSet;


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

    const fn from_char(c: char) -> Self {
        match c {
            'v' => DOWN,
            '<' => LEFT,
            '^' => UP,
            '>' => RIGHT,
            _ => panic!("dont know that dir char"),
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
    Empty,
    Blocked,
    Special(char),
}

#[allow(dead_code)]
impl Node {
    fn from_char(c: char) -> Node {
        match c {
            '.' => Node::Empty,
            '#' => Node::Blocked,
            c => Node::Special(c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    nodes: Vec<Vec<Node>>,
}

#[allow(dead_code)]
impl Grid {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut nodes = Vec::new();

        for (y, l) in lines.enumerate() {
            let line = l.trim();

            nodes.push(Vec::new());

            for (_x, c) in line.char_indices() {
                nodes[y].push(Node::from_char(c));
            }
        }

        Grid {
            nodes,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xy_construction_correct() {
        let xy = XY::new(5, 8);

        assert_eq!(xy.x, 5);
        assert_eq!(xy.y, 8);
    }

    #[test]
    fn xy_adding() {
        let a = XY::new(5, 8);
        let b = XY::new(4, 3);

        assert_eq!(a.add(&b), XY::new(9, 11));
        assert_eq!(a.add(&b), b.add(&a));
    }

    #[test]
    fn xy_subtracting() {
        let a = XY::new(5, 8);
        let b = XY::new(4, 3);

        assert_eq!(a.sub(&b), XY::new(1, 5));
        assert_eq!(b.sub(&a), XY::new(-1, -5));
    }

    #[test]
    fn xy_multiplication() {
        let a = XY::new(5, 8);
        let b = XY::new(4, 3);

        assert_eq!(a.mul(&b), XY::new(20, 24));
        assert_eq!(a.mul(&b), b.mul(&a));
    }


    #[test]
    fn xy_step() {
        let s = XY::new(5, 0);

        assert_eq!(s.step(&DOWN), XY::new(5, 1));
        assert_eq!(s.step(&UP), XY::new(5, -1));
        assert_eq!(s.step(&LEFT), XY::new(4, 0));
        assert_eq!(s.step(&RIGHT), XY::new(6, 0));
    }

    #[test]
    fn dir_from_to() {
        assert_eq!(Direction::from_to(&XY::new(5, 4), &XY::new(5, 0)), UP);
        assert_eq!(Direction::from_to(&XY::new(5, 4), &XY::new(5, 15)), DOWN);
        assert_eq!(Direction::from_to(&XY::new(5, 4), &XY::new(15, 4)), RIGHT);
        assert_eq!(Direction::from_to(&XY::new(5, 4), &XY::new(-1, 4)), LEFT);
    }


    #[test]
    fn grid_build() {
        let input: Vec<String> = vec![
            "..#A#",
            ".....",
            "#.c.#",
        ].iter().map(|s| s.to_string()).collect();
        
        let mut grid = Grid::from_input(input.into_iter());
        assert_eq!(*grid.node_at(&XY::new(0, 0)), Node::Empty);
        assert_eq!(*grid.mut_node_at(&XY::new(0, 0)), Node::Empty);
        assert_eq!(*grid.node_at(&XY::new(2, 0)), Node::Blocked);
        assert_eq!(*grid.mut_node_at(&XY::new(2, 0)), Node::Blocked);
        assert_eq!(*grid.node_at(&XY::new(2, 2)), Node::Special('c'));
        assert_eq!(*grid.mut_node_at(&XY::new(2, 2)), Node::Special('c'));
    }

    #[test]
    #[should_panic(expected = "node out of bounds")]
    fn grid_outside_access() {
        let input: Vec<String> = vec![
            "..#A#",
            ".....",
            "#.c.#",
        ].iter().map(|s| s.to_string()).collect();
        
        let grid = Grid::from_input(input.into_iter());

        grid.node_at(&XY::new(5, 3));
    }

    #[test]
    #[should_panic(expected = "node out of bounds")]
    fn grid_mut_outside_access() {
        let input: Vec<String> = vec![
            "..#A#",
            ".....",
            "#.c.#",
        ].iter().map(|s| s.to_string()).collect();
        
        let mut grid = Grid::from_input(input.into_iter());

        grid.mut_node_at(&XY::new(5, 3));
    }

    #[test]
    fn grid_is_within() {
        let input: Vec<String> = vec![
            "..#A#",
            ".....",
            "#.c.#",
        ].iter().map(|s| s.to_string()).collect();
        
        let grid = Grid::from_input(input.into_iter());

        assert_eq!(grid.is_within(&XY::new(5, 3)), false);
        assert_eq!(grid.is_within(&XY::new(5, 0)), false);
        assert_eq!(grid.is_within(&XY::new(0, 3)), false);
        assert_eq!(grid.is_within(&XY::new(4, 2)), true);
        assert_eq!(grid.is_within(&XY::new(0, -1)), false);
        assert_eq!(grid.is_within(&XY::new(-1, 0)), false);
    }
}
