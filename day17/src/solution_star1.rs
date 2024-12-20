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
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

impl Registers {
    fn from_slice(sli: &[i64]) -> Self {
        Self {
            a: sli[0],
            b: sli[1],
            c: sli[2],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct ComboOp(i64);

impl ComboOp {
    fn val(&self, regs: &Registers) -> i64 {
        match self.0 {
            v @ 0..=3 => v,
            4 => regs.a,
            5 => regs.b,
            6 => regs.c,
            e @ _ => panic!("Wrong ComboOp!: {}", e),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Ins {
    Adv(ComboOp),
    Blx(i64),
    Bst(ComboOp),
    Jnz(i64),
    Bxc(i64),
    Out(ComboOp),
    Bdv(ComboOp),
    Cdv(ComboOp),
}

impl Ins {
    fn new(opcode: i64, operand: i64) -> Ins {
        match opcode {
            0 => Ins::Adv(ComboOp(operand)),
            1 => Ins::Blx(operand),
            2 => Ins::Bst(ComboOp(operand)),
            3 => Ins::Jnz(operand),
            4 => Ins::Bxc(-1),
            5 => Ins::Out(ComboOp(operand)),
            6 => Ins::Bdv(ComboOp(operand)),
            7 => Ins::Cdv(ComboOp(operand)),
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Program {
    program: Vec<i64>,
    initial_regs: Registers,
}


impl Program {
    fn exec(&self) -> String {
        let mut out = Vec::new();
        let mut ins_ptr: usize = 0;
        let mut regs = self.initial_regs.clone();

        while ins_ptr < self.program.len() {
            let ins = Ins::new(self.program[ins_ptr], self.program[ins_ptr + 1]);
            dprintln!("ins_ptr: {}", ins_ptr);
            dprintln!("regs: {:?}", regs);
            dprintln!("ins: {:?}", ins);
            match ins {
                Ins::Adv(cop) => {
                    let val = cop.val(&regs);
                    let pow = 1 << val;
                    regs.a /= pow;
                    ins_ptr += 2;
                },
                Ins::Blx(op) => {
                    let res = op ^ regs.b;
                    regs.b = res;
                    ins_ptr += 2;
                },
                Ins::Bst(cop) => {
                    let val = cop.val(&regs);
                    let res = val % 8;
                    regs.b = res;
                    ins_ptr += 2;
                },
                Ins::Jnz(op) => {
                    if regs.a != 0 {
                        ins_ptr = op as usize;
                    } else {
                        ins_ptr += 2;
                    }
                },
                Ins::Bxc(_) => {
                    let res = regs.b ^ regs.c;
                    regs.b = res;
                    ins_ptr += 2;
                },
                Ins::Out(cop) => {
                    let val = cop.val(&regs);
                    out.push(val % 8);
                    ins_ptr += 2;
                },
                Ins::Bdv(cop) => {
                    let val = cop.val(&regs);
                    let pow = 1 << val;
                    let res = regs.a / pow;
                    regs.b = res;
                    ins_ptr += 2;
                },
                Ins::Cdv(cop) => {
                    let val = cop.val(&regs);
                    let pow = 1 << val;
                    let res = regs.a / pow;
                    regs.c = res;
                    ins_ptr += 2;
                },
            }
        }

        out.iter().map(|o| o.to_string()).collect::<Vec<String>>().join(",")
    }
}


#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    program: Program,
}

impl Solution {
    fn from_input<I>(mut lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut v_regs = vec![];
        for _ in 0..3 {
            let l = lines.next().unwrap();
            let line = l.trim();
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            v_regs.push(parts[2].parse::<i64>().unwrap());
        }
        let _ = lines.next(); // empty line

        let l = lines.next().unwrap();
        let line = l.trim();
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let s_prog = parts[1];

        Solution {
            program: Program {
                initial_regs: Registers::from_slice(&v_regs),
                program: s_prog.split(",").map(|s| s.parse::<i64>().unwrap()).collect(),
            },
        }
    }

    fn solve(&self) -> String {
        self.program.exec()
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let solution = Solution::from_input(lines_it);
    dprintln!("solution: {:?}", solution);

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
            "Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0",
            "4,6,3,5,6,3,5,2,1,0",
        );
    }
}