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

    // Returns Option<new position, Option<output>>
    fn step(&self, start: char, step: char) -> Option<(char, Option<char>)> {
        match step {
            'A' => Some((start, Some(start))),
            '<' | '>' | '^' | 'v' => {
                let dir = Direction::from_char(step);
                let new_pos = self.keys.get(&start).unwrap().step(&dir);
                if !self.is_within(&new_pos) {
                    return None;
                }
                Some((*self.vals.get(&new_pos).unwrap(), None))
            },
            _ => panic!("unknown step: {}", step),
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
struct KeypadSeq {
    pads: Vec<Keypad>,
}

impl KeypadSeq {
    fn new(pads: Vec<Keypad>) -> Self {
        Self {
            pads,
        }
    }

    fn advance_state(&self, state: &[char], step: char) -> Option<(Vec<char>, Option<char>)> {
        if state.len() == 0 {
            return Some((vec![], Some(step)));
        }
        let curr_idx = self.pads.len() - state.len();
        let new_curr = self.pads[curr_idx].step(state[0], step);
        //dprintln!("curr_idx, new_curr: {:?}", (curr_idx, new_curr));
        let Some((curr_st, o_curr_out)) = new_curr else {
            return None;
        };
        match step {
            'A' => {
                if let Some(curr_out) = o_curr_out {
                    let mut out = vec![curr_st];
                    let Some((mut prt_st, out)) = self.advance_state(&state[1..], curr_out) else {
                        return None;
                    };
                    prt_st.insert(0, curr_st);
                    Some((prt_st, out))
                } else {
                    let mut st = state.to_vec();
                    st[0] = curr_st;
                    Some((st, None))
                }
            },
            '<' | '>' | '^' | 'v' => {
                let mut st = state.to_vec();
                st[0] = curr_st;
                Some((st, None))
            },
            _ => panic!("wrong char to advance: {}", step),
        }
    }

    fn neighbours(&self, state: &[char]) -> Vec<((char, char, char), Option<char>)> {
        let mut neighs = Vec::new();
        for c in vec!['<', '^', '>', 'v', 'A'] {
            let pot_state = self.advance_state(&state[0..3], c);
            if let Some((st, out)) = pot_state {
                let state = (st[0], st[1], st[2]);
                neighs.push((state, out));
            }
        }
        neighs
    }

    fn shortest_path(&self, start: char, end: char) -> i64 {
        let mut queue = VecDeque::new();
        let mut dists = HashMap::new();

        let start_state = ('A', 'A', start);
        queue.push_back(start_state);
        dists.insert(start_state, 0);

        while let Some(state) = queue.pop_front() {
            let curr_dist = *dists.get(&state).unwrap();
            let state_vec = vec![state.0, state.1, state.2];
            for (n_state, o_n_out) in self.neighbours(&state_vec) {
                if let Some(n_out) = o_n_out {
                    if n_out == end {
                        return curr_dist + 1;
                    }
                }
                if dists.contains_key(&n_state) { continue; }
                dists.insert(n_state, curr_dist + 1);
                queue.push_back(n_state);
            }
        }
        panic!("did not find it")
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
        let robot1 = Keypad::numkeypad();
        let robot2 = Keypad::dirkeypad();
        let robot3 = Keypad::dirkeypad();
        let pads = KeypadSeq::new(vec![robot3, robot2, robot1]);

        let mut res = 0;
        for code in &self.codes {
            let mut p_char = 'A';
            let mut in_len = 0;
            for c in code.chars() {
                in_len += pads.shortest_path(p_char, c);
                p_char = c;
            }
            res += Self::num_code(code) * in_len;
        }
        res
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut solution = Solution::from_input(lines_it);

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
            "029A
            980A
            179A
            456A
            379A",
            "126384",
        );
    }

    #[test]
    fn sample_short() {
        test_ignore_whitespaces(
            "029A",
            "1972",
        );
    }

    #[test]
    fn sample_bad() {
        test_ignore_whitespaces(
            "379A",
            "24256",
        );
    }
}
