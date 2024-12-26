use std::io::{BufRead, BufReader, Write};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::collections::HashMap;
use rand::Rng;
use itertools::Itertools;
use std::collections::VecDeque;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
//	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum GtSt {
    BAD,
    GOOD,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Op {
    AND,
    OR,
    XOR
}

impl Op {
    fn from_str(op: &str) -> Self {
        match op {
            "AND" => Op::AND,
            "OR" => Op::OR,
            "XOR" => Op::XOR,
            _ => panic!("Don't know that Op: {:?}", op),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Op::AND => "AND",
            Op::OR  => "OR",
            Op::XOR => "XOR",
        }.to_string()
    }

    fn eval(&self, a: u8, b: u8) -> u8 {
        match self {
            Op::AND => a & b,
            Op::OR  => a | b,
            Op::XOR => a ^ b,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Gate {
    op: Op,
    ins: (String, String),
    out: String,
}

impl Gate {
    fn from_line(line: &str) -> Self {
        lazy_static!{
            static ref GATE_RE: Regex = Regex::new(r"(\w{3}) (AND|OR|XOR) (\w{3}) -> (\w{3})").unwrap();
        }
        let caps = GATE_RE.captures(line).unwrap();

        Self {
            op: Op::from_str(&caps[2]),
            ins: (caps[1].to_string(), caps[3].to_string()),
            out: caps[4].to_string(),
        }
    }

    fn eval(&self, values: &HashMap<String, u8>) -> Option<u8> {
        let Some(in1) = values.get(&self.ins.0) else {
            return None;
        };
        let Some(in2) = values.get(&self.ins.1) else {
            return None;
        };

        Some(self.op.eval(*in1, *in2))
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Solution {
    size: usize,
    gates: HashMap<String, Gate>,
}

impl Solution {
    const INT_ADDS: [(i64, i64); 14] = [
        (33990941402531, 22738689785549),
        (35184372088831, 35184372088831),
        (24546275348541, 31894036836153),
        (14676699754771, 32511573513569),
        (16533995937772, 25556078852050),
        (22084950160754, 25857126195856),
        (18447922774982, 30278130441237),
        (21661833849402, 28174804130689),
        (27260905902565, 25234030021653),
        (28988167108136, 21344516261901),
        (23540191997855, 31976529959358),
        (22801674810239, 21539533729933),
        (32000932786263, 18885865307825),
        (18120406095288, 22836369738318),

    ];

    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        lazy_static!{
            static ref VAL_RE: Regex = Regex::new(r"(\w{3}): (\d)").unwrap();
        }

        let mut gates = Vec::new();
        for l in lines.by_ref() {
            let line = l.trim();
            if line == "" { break; }
        }
        for l in lines {
            let line = l.trim();
            gates.push(Gate::from_line(line));
        }

        Solution {
            size: gates.iter().filter(|g| g.out.starts_with("z")).count(),
            gates: gates.into_iter().map(|g| (g.out.to_string(), g)).collect(),
        }
    }

    fn is_input_val(val: &String) -> bool {
        val.starts_with("x") || val.starts_with("y")
    }

    fn is_output_val(val: &String) -> bool {
        val.starts_with("z")
    }

    fn to_binary_rev(&self, x: i64) -> Vec<u8> {
        let mut left = x;
        let mut res = Vec::new();
        for i in 0..self.size {
            res.push((left % 2) as u8);
            left /= 2;
        }
        res
    }

    fn to_number(start_letter: &str, z_vals: &HashMap<String, u8>) -> i64 {
        let mut zs: Vec<&String>  = z_vals.keys().filter(|k| k.starts_with(start_letter)).collect();
        zs.sort();
        let mut out: i64 = 0;
        for i in 0..zs.len() {
            let val = *z_vals.get(zs[i]).unwrap();
            out += (val as i64) << i;
        }
        out
    }

    fn expression_for_out(&self, out: &String) -> String {
        if Self::is_input_val(out) {
            return out.to_string();
        }
        let g = self.gates.get(out).unwrap();
        format!("({}) {:?} ({})",
            self.expression_for_out(&g.ins.0), g.op, self.expression_for_out(&g.ins.1))
    }

    fn wrong_bits(&self, sol: &HashMap<String, u8>, correct: i64)  -> Vec<usize> {
        let mut zs: Vec<&String>  = sol.keys().filter(|k| k.starts_with("z")).collect();
        zs.sort();

        let bin_correct = self.to_binary_rev(correct);

        let mut res = vec![];
        for i in 0..zs.len() {
            let val = *sol.get(zs[i]).unwrap();
            if val != bin_correct[i] {
                res.push(i);
            }
        }
        res
    }

    fn find_wrong_bits(&self) -> Vec<usize> {
        let mut wrongs: HashSet<usize> = {
            let (a, b) = Self::INT_ADDS[0];
            let output = self.eval_as_adder(a, b);

            self.wrong_bits(&output, a + b).into_iter().collect()
        };

        for &(a, b) in Self::INT_ADDS.iter().skip(1) {
            let output = self.eval_as_adder(a, b);

            let other_wrongs = self.wrong_bits(&output, a + b);
            wrongs.extend(other_wrongs);
        }

        for _ in 0..1000 {
            let a = rand::thread_rng().gen_range((1<<44)..(1<<45));
            let b = rand::thread_rng().gen_range((1<<44)..(1<<45));
            let output = self.eval_as_adder(a, b);

            let other_wrongs: HashSet<usize> = self.wrong_bits(&output, a + b).into_iter().collect();
            let new_wrongs = other_wrongs.difference(&wrongs).cloned().collect::<HashSet<usize>>();
            if new_wrongs.len() > 0 {
                println!("Found new wrongs! (a,b): {:?}, wrongs: {:?}", (a, b), new_wrongs);
                wrongs.extend(new_wrongs);
            }
        }

        let mut vec_wrongs: Vec<usize> = wrongs.into_iter().collect();
        vec_wrongs.sort();
        println!("Wrongs: {:?}", vec_wrongs);
        vec_wrongs
    }


    fn eval(&self, init_values: &HashMap<String, u8>) -> HashMap<String, u8> {
        let mut curr_vals = init_values.clone();
        let mut z_num = 0;

        while z_num < self.size {
            let mut only_z = curr_vals.iter().filter(|(k, _)| k.starts_with("z")).collect::<Vec<(&String, &u8)>>();
            only_z.sort_by_key(|(k, _)| k.to_string());
            //dprintln!("only z: {:?}", only_z);

            for g in self.gates.values() {
                let Some(out) = g.eval(&curr_vals) else {
                    continue;
                };

                curr_vals.insert(g.out.to_string(), out);
            }
            z_num = curr_vals.iter().filter(|(k, _)| k.starts_with("z")).count();
        }

        curr_vals
    }

    fn bit_to_name(prefix: &str, bit_no: usize) -> String {
        format!("{}{:0>2}", prefix, bit_no)
    }

    fn to_set(&self, prefix: &str, num: i64) -> HashMap<String, u8> {
        self.to_binary_rev(num).iter().enumerate()
            .map(|(i, d)| (Self::bit_to_name(prefix, i), *d))
            .collect()
    }

    fn eval_as_adder(&self, x: i64, y: i64) -> HashMap<String, u8> {
        let mut input = self.to_set("x", x);
        input.extend(self.to_set("y", y));

        self.eval(&input)
    }

    fn as_adder(&self, x: i64, y: i64) -> i64 {
        let output = self.eval_as_adder(x, y);
        Self::to_number("z", &output)
    }

    fn is_correct_adder(&self) -> bool {
        for (a, b) in Self::INT_ADDS {
            let computed = self.as_adder(a, b);
            if computed != a + b {
                return false;
            }
        }
        true
    }

    fn mark_as(&self, gt: &String, st: GtSt, states: &mut HashMap<String, GtSt>) {
        states.insert(gt.to_string(), st);
        let Some(gate) = self.gates.get(gt) else {
            return;
        };
        self.mark_as(&gate.ins.0, st, states);
        self.mark_as(&gate.ins.1, st, states);
    }

    fn swap(&self, gt1: &String, gt2: &String) -> Self {
        let mut gates = self.gates.clone();
        let gt1_ptr = gates.get_mut(gt1).unwrap() as *mut Gate;
        let gt2_ptr = gates.get_mut(gt2).unwrap() as *mut Gate;
        unsafe {
            std::ptr::swap(gt1_ptr, gt2_ptr);
        }

        gates.get_mut(gt1).unwrap().out = gt1.to_string();
        gates.get_mut(gt2).unwrap().out = gt2.to_string();

        Self {
            size: self.size,
            gates,
        }
    }

    fn print_dot(&self) {
        println!("digraph G {{");
        let mut all_out = vec![];
        for i in 0..self.size {
            all_out.push(Self::bit_to_name("z", i));
        }

        let mut queue = VecDeque::new();
        for o in all_out {
            queue.push_back(o.to_string());
        }
        let mut visited = HashSet::new();

        while let Some(g_out) = queue.pop_front() {
            if !visited.insert(g_out.to_string()) { continue; }
            let Some(g) = self.gates.get(&g_out) else {
                continue;
            };
            let g_str = format!("{}_{}", g.op.to_str(), g_out);
            println!("  {} -> {}", g_str, g_out);
            println!("  {} -> {}", g.ins.0, g_str);
            println!("  {} -> {}", g.ins.1, g_str);

            queue.push_back(g.ins.0.to_string());
            queue.push_back(g.ins.1.to_string());
        }


        println!("}}");
    }

    fn analyze(&self) {
        let wrongs = self.find_wrong_bits();
        let mut gate_states = HashMap::new();

        for bit_no in &wrongs {
            self.mark_as(&Self::bit_to_name("z", *bit_no), GtSt::BAD, &mut gate_states);
        }

        for bit_no in 0..self.size {
            if wrongs.contains(&bit_no) { continue; }
            self.mark_as(&Self::bit_to_name("z", bit_no), GtSt::GOOD, &mut gate_states);
        }

        let bad_gates: Vec<String> = gate_states.iter()
            .filter(|(_, v)| **v == GtSt::BAD)
            .map(|(k, _)| k.to_string())
            .collect();

        if bad_gates.len() == 0 {
            println!("Hurray! No bads!");
        } else {
            println!("Bad gates: {:?}", bad_gates);
            println!("Bad gates num: {:?}", bad_gates.len());
        }
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut solution = Solution::from_input(lines_it);

    let mut swaps = vec![
        ("bjm", "z07"),
        ("hsw", "z13"),
        ("skf", "z18"),
        ("nvr", "wkr"),
    ];
    for (a, b) in &swaps {
        solution = solution.swap(&a.to_string(), &b.to_string());
    }

    //solution.print_dot();
    solution.analyze();

    let mut all_swaps: Vec<String> = swaps.iter().flat_map(|(a, b)| vec![a.to_string(), b.to_string()].into_iter()).collect();
    all_swaps.sort();

    println!("{}", all_swaps.join(","));
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn test_ignore_whitespaces(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    fn official_input() -> std::io::Lines<BufReader<File>> {
        let file = File::open("input").unwrap();
        BufReader::new(file).lines()
    }

    #[test]
    fn adding() {
        let sol = Solution::from_input(official_input().map(|l| l.unwrap()));
        assert_eq!(sol.as_adder(1, 2), 3);
        assert_eq!(sol.as_adder(1, 3), 4);
        assert_eq!(sol.as_adder(10, 10), 20);
    }

    #[test]
    fn ops() {
        assert_eq!(Op::AND.eval(1, 1), 1);
        assert_eq!(Op::AND.eval(1, 0), 0);

        assert_eq!(Op::OR.eval(1, 1), 1);
        assert_eq!(Op::OR.eval(0, 1), 1);

        assert_eq!(Op::XOR.eval(1, 1), 0);
        assert_eq!(Op::XOR.eval(0, 1), 1);
        assert_eq!(Op::XOR.eval(0, 0), 0);
    }

    #[test]
    fn read_gate() {
        let gate = Gate::from_line("ntg XOR fgs -> mjb");

        assert_eq!(gate.op, Op::XOR);
        assert_eq!(gate.ins, ("ntg".to_string(), "fgs".to_string()));
        assert_eq!(gate.out, "mjb".to_string());
    }

    #[test]
    fn z_conversion() {
        let z_vals: HashMap<String, u8> = HashMap::from([
            ("z01".to_string(), 1),
            ("z00".to_string(), 0),
            ("z04".to_string(), 1),
            ("z03".to_string(), 0),
        ]);
        assert_eq!(Solution::to_number("z", &z_vals), 10);
    }
}
