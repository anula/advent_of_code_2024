use std::io::{BufRead, BufReader, Write};
use std::cmp::min;
use itertools::Itertools;
//use regex::Regex;
//use lazy_static::lazy_static;
//use std::collections::HashSet;
use std::collections::HashMap;
//use std::collections::VecDeque;

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

    const fn from_char(c: char) -> Self {
        match c {
            'v' => DOWN,
            '<' => LEFT,
            '^' => UP,
            '>' => RIGHT,
            _ => panic!("dont know that dir char"),
        }
    }

    const fn maybe_from_char(c: char) -> Option<Self> {
        match c {
            'v' => Some(DOWN),
            '<' => Some(LEFT),
            '^' => Some(UP),
            '>' => Some(RIGHT),
            _ => None,
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct Keypad {
    keys: HashMap<char, XY>,
    vals: HashMap<XY, char>,
    blank: XY,
}

impl Keypad {
    fn new(keymap: &[&str]) -> Self {
        let mut keys = HashMap::new();
        let mut vals = HashMap::new();
        let mut blank = XY::new(-1, -1);

        for (y, line) in keymap.iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c == ' ' { 
                    blank = XY::unew(x, y);
                    continue;
                }
                keys.insert(c, XY::unew(x, y));
                vals.insert(XY::unew(x, y), c);
            }
        }

        Keypad {
            keys,
            vals,
            blank,
        }
    }

    fn numkeypad() -> Self {
        Self::new(&vec![
            "789",
            "456",
            "123",
            " 0A",
        ])
    }

    fn dirkeypad() -> Self {
        Self::new(&vec![
            " ^A",
            "<v>",
        ])
    }

    fn is_within(&self, pos: &XY) -> bool {
        self.vals.contains_key(pos)
    }

    fn is_valid(&self, start_key: &XY, path: &[char]) -> bool {
        let mut curr_key = start_key.clone();
        for c in path {
            let dir = Direction::from_char(*c);
            curr_key = curr_key.step(&dir);
            if !self.is_within(&curr_key) { return false; }
        }
        true
    }

    fn really_all_dir_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        let start_key = self.keys.get(&start).unwrap();
        let end_key = self.keys.get(&end).unwrap();

        let mut vert = {
            let c = if start_key.y < end_key.y {
                'v'
            } else {
                '^'
            };

            vec![c; (end_key.y - start_key.y).abs() as usize]
        };

        let mut hori = {
            let c = if start_key.x < end_key.x {
                '>'
            } else {
                '<'
            };

            vec![c; (end_key.x - start_key.x).abs() as usize]
        };

        let mut all_dirs = Vec::new();
        all_dirs.append(&mut hori);
        all_dirs.append(&mut vert);
        let all_dirs_len = all_dirs.len();

        let mut res = vec![];
        for perm in all_dirs.into_iter().permutations(all_dirs_len) {
            if self.is_valid(&start_key, &perm) {
                res.push(perm);
            }
        }
        res
    }

    fn all_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        let mut res = self.really_all_dir_paths(start, end);
        for r in &mut res {
            r.push('A');
        }
        res
    }
}

#[derive(Debug, PartialEq, Eq)]
struct KeypadSeq {
    pads: Vec<Keypad>,
}

impl KeypadSeq {
    fn new(dir_pads: usize) -> Self {
        let mut pads = vec![Keypad::dirkeypad(); dir_pads];
        pads.push(Keypad::numkeypad());
        Self {
          pads
        }
    }

    fn shortest_path(&self, start: char, end: char, idx: usize, cache: &mut HashMap<(usize, char, char), usize>) -> usize {
        let _indent = vec![' '; self.pads.len() - idx].into_iter().collect::<String>();
        dprintln!("{}shortest path: {:?}, idx: {} {{", _indent, (start, end), idx);
        if let Some(res) = cache.get(&(idx, start, end)) {
            return *res;
        }
        let mut min_path = usize::MAX;
        for path in self.pads[idx].all_paths(start, end) {
            dprintln!("{}- checking path: {:?}", _indent, path);
            let len = if idx == 0 {
                path.len()
            } else {
                let mut ins_len = 0;
                let mut prev = 'A';
                for c in path {
                    ins_len += self.shortest_path(prev, c, idx - 1, cache);
                    prev = c;
                }
                ins_len
            };
            min_path = min(min_path, len);
        }

        if min_path == usize::MAX { panic!("Found no path at all!"); }
        dprintln!("{}}} shortest path: {:?}, idx: {} = {}", _indent, (start, end), idx, min_path);
        cache.insert((idx, start, end), min_path);
        min_path
    }

    fn shortest_code(&self, code: &str) -> usize {
        let mut res = 0;
        let mut prev = 'A';
        let mut cache = HashMap::new();
        for c in code.chars() {
            res += self.shortest_path(prev, c, self.pads.len() - 1, &mut cache);
            prev = c;
        }
        res
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    codes: Vec<String>,
}

impl Solution {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut codes = vec![];
        for l in lines {
            let line = l.trim();
            codes.push(line.to_string());
        }

        Solution {
            codes,
        }
    }

    fn num_code(code: &str) -> i64 {
        code[0..3].parse::<i64>().unwrap()
    }

    fn solve(&self) -> i64 {
        let pads = KeypadSeq::new(25);

        let mut res = 0;
        for code in &self.codes {
            res += (pads.shortest_code(code) as i64) * Self::num_code(code);
        }
        res
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Solution::from_input(lines_it);

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

    #[allow(dead_code)]
    fn test_ignore_whitespaces(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    #[test]
    fn small_keypads_sample() {
        let pads = KeypadSeq::new(2);
        assert_eq!(pads.shortest_code("029A"), 68);
        assert_eq!(pads.shortest_code("980A"), 60);
        assert_eq!(pads.shortest_code("179A"), 68);
        assert_eq!(pads.shortest_code("456A"), 64);
        assert_eq!(pads.shortest_code("379A"), 64);
    }

    #[test]
    fn small_keypads_broken_down() {
        let pads = KeypadSeq::new(2);
        assert_eq!(pads.shortest_code("0"), 18);
    }
}
