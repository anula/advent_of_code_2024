use std::io::{BufRead, BufReader, Write};
use std::cmp::max;
use regex::Regex;
use lazy_static::lazy_static;
//use std::collections::HashSet;
use std::collections::HashMap;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
//	    #[cfg(test)]
            println!($($x), *);
        }
    };
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
    init_values: HashMap<String, u8>,
    z_num: usize,
    gates: Vec<Gate>,
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        lazy_static!{
            static ref VAL_RE: Regex = Regex::new(r"(\w{3}): (\d)").unwrap();
        }

        let mut init_values = HashMap::new();
        let mut gates = Vec::new();
        for l in lines.by_ref() {
            let line = l.trim();
            if line == "" { break; }
            let caps = VAL_RE.captures(line).unwrap();
            init_values.insert(caps[1].to_string(), caps[2].parse::<u8>().unwrap());
        }
        for l in lines {
            let line = l.trim();
            gates.push(Gate::from_line(line));
        }

        Solution {
            init_values,
            z_num: gates.iter().filter(|g| g.out.starts_with("z")).count(),
            gates,
        }
    }

    fn to_binary_rev(x: i64) -> Vec<u8> {
        let mut left = x;
        let mut res = Vec::new();
        while left > 0 {
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

    fn wrong_bits(vals: &HashMap<String, u8>, correct: i64) {
        let mut zs: Vec<&String>  = vals.keys().filter(|k| k.starts_with("z")).collect();
        zs.sort();

        let bin_correct = Self::to_binary_rev(correct);

        for i in 0..zs.len() {
            let val = *vals.get(zs[i]).unwrap();
            if val != bin_correct[i] {
                println!("[{}] was: {} but should be: {}", i, val, bin_correct[i]);
            }
        }

    }

    fn solve(&self) -> i64 {
        let mut curr_vals = self.init_values.clone();
        let mut z_num = 0;

        let mut its = 0;
        while z_num < self.z_num {
            its += 1;
            let mut only_z = curr_vals.iter().filter(|(k, _)| k.starts_with("z")).collect::<Vec<(&String, &u8)>>();
            only_z.sort_by_key(|(k, _)| k.to_string());
            dprintln!("only z: {:?}", only_z);
            if curr_vals.len() == 0 {
                panic!("No values!");
            }
            for g in &self.gates {
                let Some(out) = g.eval(&curr_vals) else {
                    continue;
                };

                curr_vals.insert(g.out.to_string(), out);
            }
            z_num = curr_vals.iter().filter(|(k, _)| k.starts_with("z")).count();
        }

        let x = Self::to_number("x", &curr_vals);
        let y = Self::to_number("y", &curr_vals);

        let expected = x + y;
        Self::wrong_bits(&curr_vals, expected);
        dprintln!("x: {}", Self::to_number("x", &curr_vals));
        dprintln!("y: {}", Self::to_number("y", &curr_vals));
        dprintln!("z: {}", Self::to_number("z", &curr_vals));
        dprintln!("its: {}", its);


        Self::to_number("z", &curr_vals)
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
            "x00: 1
            x01: 1
            x02: 1
            y00: 0
            y01: 1
            y02: 0

            x00 AND y00 -> z00
            x01 XOR y01 -> z01
            x02 OR y02 -> z02",
            "4",
        );
    }

    #[test]
    fn sample2() {
        test_ignore_whitespaces(
            "x00: 1
            x01: 0
            x02: 1
            x03: 1
            x04: 0
            y00: 1
            y01: 1
            y02: 1
            y03: 1
            y04: 1

            ntg XOR fgs -> mjb
            y02 OR x01 -> tnw
            kwq OR kpj -> z05
            x00 OR x03 -> fst
            tgd XOR rvg -> z01
            vdt OR tnw -> bfw
            bfw AND frj -> z10
            ffh OR nrd -> bqk
            y00 AND y03 -> djm
            y03 OR y00 -> psh
            bqk OR frj -> z08
            tnw OR fst -> frj
            gnj AND tgd -> z11
            bfw XOR mjb -> z00
            x03 OR x00 -> vdt
            gnj AND wpb -> z02
            x04 AND y00 -> kjc
            djm OR pbm -> qhw
            nrd AND vdt -> hwm
            kjc AND fst -> rvg
            y04 OR y02 -> fgs
            y01 AND x02 -> pbm
            ntg OR kjc -> kwq
            psh XOR fgs -> tgd
            qhw XOR tgd -> z09
            pbm OR djm -> kpj
            x03 XOR y03 -> ffh
            x00 XOR y04 -> ntg
            bfw OR bqk -> z06
            nrd XOR fgs -> wpb
            frj XOR qhw -> z04
            bqk OR frj -> z07
            y03 OR x01 -> nrd
            hwm AND bqk -> z03
            tgd XOR rvg -> z12
            tnw OR pbm -> gnj",
            "2024",
        );
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
