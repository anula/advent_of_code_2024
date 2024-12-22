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

    fn shortest_paths(&self, start: char, end: char) -> Vec<Vec<char>>{
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


        let mut res = vec![];
        if start_key.x == self.blank.x {
            res.append(&mut hori);
            res.append(&mut vert);
        } else {
            res.append(&mut vert);
            res.append(&mut hori);
        }

        res.into_iter().collect()
    }

    fn type_code(&self, code: &str) -> String {
        let mut res = String::new();
        dprintln!("Typing: {:?}", code);

        let mut prev = 'A';
        for c in code.chars() {
            //dprintln!("adding path: {:?}", (prev, c));
            res.push_str(&self.shortest_path(prev, c));
            res.push('A');
            //dprintln!("res: {:?}", res);
            prev = c;
        }

        dprintln!("Total res for {:?}: {:?}", code, res);
        res
    }

    fn eval_seq(&self, seq: &[char], start: char) -> Vec<char> {
        let mut pos = *self.keys.get(&start).unwrap();

        let mut res = vec![];
        for c in seq {
            if *c == 'A' {
                res.push(*self.vals.get(&pos).unwrap());
            } else {
                let dir = Direction::from_char(*c);
                pos = pos.step(&dir);
                if !self.is_within(&pos) {
                    return vec!['X'];
                }
            }
        }
        res
    }
    
    fn seq_to_presses(&self, seq: &[char], start: char) -> Option<Vec<XY>> {
        let mut pos = *self.keys.get(&start).unwrap();

        let mut res = vec![];
        for c in seq {
            if *c == 'A' {
                res.push(pos);
            } else {
                let dir = Direction::from_char(*c);
                pos = pos.step(&dir);
                if !self.is_within(&pos) {
                    return None;
                }
            }
        }
        Some(res)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct KeypadSeq {
    pads: Vec<Keypad>,
    last: Keypad,
}

impl KeypadSeq {
    fn new(pads: Vec<Keypad>) -> Self {
        Self {
            last: pads.last().unwrap().clone(),
            pads: pads[0..(pads.len()-1)].to_vec(),
        }
    }

    fn eval_seq(&self, seq: &[char], start: char) -> Vec<char> {
        //dprintln!("Eval seq: {:?}, start: {:?}", seq, start);
        let mut prev_seq = seq.to_vec();
        for pad in &self.pads {
            let new_seq = pad.eval_seq(&prev_seq, 'A');
            if new_seq.len() == 1 && new_seq[0] == 'X' {
                return new_seq;
            }
            prev_seq = new_seq;
        }
        let res = self.last.eval_seq(&prev_seq, start);
        //dprintln!(" -- res: {:?}", res);
        res
    }

    fn shortest_path(&self, start: char, end: char) -> Vec<char> {
        let mut queue = VecDeque::new();

        queue.push_back(vec![]);

        while let Some(seq) = queue.pop_front() {
            dprintln!("len: {}", seq.len());
            let last_char_dir = seq.last().map_or(None, |l| Direction::maybe_from_char(*l));
            //dprintln!("off the queue: {:?}", seq);
            for c in vec!['<', '^', '>', 'v', 'A'] {
                if let Some(d1) = last_char_dir {
                    if let Some(d2) = Direction::maybe_from_char(c) {
                        if d1 == d2.reverse() { continue; }
                    }
                }
                let mut new_seq = seq.clone();
                new_seq.push(c);
                //dprintln!(" >> new seq: {:?}", new_seq);
                if c != 'A' {
                    //dprintln!(" >> no a so no evaling");
                    queue.push_back(new_seq);
                    continue;
                }
                let eval = self.eval_seq(&new_seq, start);
                
                //dprintln!(" >> eval was: {:?}", eval);
                if eval.len() < 1 {
                    queue.push_back(new_seq);
                } else if eval[0] == end  {
                    return new_seq;
                }
            }
        }
        panic!("Not found for: {:?}", (start, end))
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
        0
    }

    fn solve1(&self) -> i64 {
        let robot1 = Keypad::numkeypad();
        let robot2 = Keypad::dirkeypad();
        let robot3 = Keypad::dirkeypad();
        let pads = KeypadSeq::new(vec![robot3, robot2, robot1]);

        let mut res = 0;
        for code in &self.codes {
            let mut p_char = 'A';
            let mut input = vec![];
            for c in code.chars() {
                let mut path = pads.shortest_path(p_char, c);
                input.append(&mut path);
                p_char = c;
            }
            res += Self::num_code(code) * (input.len() as i64);
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
