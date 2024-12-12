use std::io::{BufRead, BufReader, Write};
use std::cmp::{max, min};
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

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    nodes: Vec<Vec<char>>,
}

fn inter_diff(a: &[(usize, usize)], b: &[(usize, usize)]) -> i64 {
    let mut b_set = HashSet::new();
    for int_b in b {
        for i_b in int_b.0..int_b.1 {
            b_set.insert(i_b);
        }
    }

    let mut res = 0;
    for int_a in a {
        for i_a in int_a.0..int_a.1 {
            if !b_set.contains(&i_a) { res += 1 }
        }
    }
    res
}

fn inter_size(a: &[(usize, usize)]) -> i64 {
    a.iter().map(|x| (x.1 - x.0) as i64).sum()
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
                nodes[y].push(c);
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

    fn node_at(&self, at: &XY) -> char {
        if !self.is_within(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        self.nodes[at.y as usize][at.x as usize]
    }

    fn mut_node_at(&mut self, at: &XY) -> &mut char {
        if !self.is_within(at) {
            panic!("Getting node out of bounds: {:?}", at);
        }
        &mut self.nodes[at.y as usize][at.x as usize]
    }

    fn build_plots(&self) -> (Vec<Vec<i64>>, i64) {
        let mut plots = vec![vec![-1; self.nodes[0].len()]; self.nodes.len()];

        let mut plot_no = 0;
        for y in 0..self.nodes.len() {
            for x in 0..self.nodes[0].len() {
                if plots[y][x] != -1 {
                    continue;
                }
                let xy = XY::unew(x, y);
                let cha = self.node_at(&xy);
                let mut queue = VecDeque::new();
                queue.push_back(xy);
                while let Some(field) = queue.pop_front() {
                    dprintln!("field: {:?}", field);
                    let new_cha = self.node_at(&field);
                    if new_cha != cha { continue }
                    if plots[field.y as usize][field.x as usize] != -1 {continue;}
                    plots[field.y as usize][field.x as usize] = plot_no;
                    
                    for dir in Direction::ALL {
                        let neigh = field.step(&dir);
                        if !self.is_within(&neigh) { continue; }
                        if plots[neigh.y as usize][neigh.x as usize]  != -1 { continue; }
                        if self.node_at(&neigh) != cha { continue; }
                        queue.push_back(neigh);
                    }
                }
                plot_no += 1;
            }
        }

        (plots, plot_no)
    }

    fn solve(&self) -> i64 {
        let (plots, plots_num) = self.build_plots();

        // (size, perimeter)
        let mut plots_info: Vec<(i64, i64)> = vec![(-1, -1); plots_num as usize]; 
        let mut open_plots = HashMap::<i64, Vec<(usize, usize)>>::new();
        
        for y in 0..plots.len() {
            let mut new_open_plots = HashMap::<i64, Vec<(usize, usize)>>::new();
            let mut x = 0;
            dprintln!("new_open_plots: {:?}", new_open_plots);
            while x < plots[y].len() {
                let plot_no = plots[y][x];
                let start = x;
                while x < plots[y].len() && plots[y][x] == plot_no {
                    x += 1;
                }
                new_open_plots.entry(plot_no).or_insert(vec![]).push((start, x));
            }
            dprintln!("new_open_plots: {:?}", new_open_plots);

            for (plot_no, intervals) in &new_open_plots {
                if let Some(ex_intervals) = open_plots.get(plot_no) {
                    let diff = inter_diff(&intervals, &ex_intervals);
                    let pl_info = &mut plots_info[*plot_no as usize];
                    pl_info.0 += inter_size(intervals);
                    pl_info.1 += 2 * intervals.len() as i64 + 2 * diff;
                } else {
                    let mut size: i64 = 0;
                    let mut peri: i64 = 0;
                    for inter in intervals {
                        let i_size = (inter.1 - inter.0) as i64;
                        size += i_size;
                        peri += i_size * 2 + 2;
                    }
                    plots_info[*plot_no as usize] = (size, peri);
                }
            }
            open_plots = new_open_plots;
            dprintln!("open_plots at the end: {:?}", open_plots);
        }
        dprintln!("plots: {:?}", plots);
        dprintln!("plots_info: {:?}", plots_info);
        plots_info.iter().map(|(size, peri)| size * peri).sum()
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Grid::from_input(lines_it);

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
    fn sample1() {
        test_ignore_whitespaces(
            "AAAA
            BBCD
            BBCC
            EEEC",
            "140",
        );
    }

    #[test]
    fn sample2() {
        test_ignore_whitespaces(
            "OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO",
            "772",
        );
    }

    #[test]
    fn sample3() {
        test_ignore_whitespaces(
            "RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE",
            "1930",
        );
    }

    #[test]
    fn inter_diff_test() {
        let a = vec![(0, 5), (8, 10)];
        assert_eq!(inter_diff(&a, &vec![(6, 8)]), 7);
        assert_eq!(inter_diff(&a, &vec![(3, 8)]), 5);
        assert_eq!(inter_diff(&vec![(0, 10), (11, 16)], &vec![(3, 8)]), 10);
    }
}
