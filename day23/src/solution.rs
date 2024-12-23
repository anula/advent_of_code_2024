use std::io::{BufRead, BufReader, Write};
//use std::cmp::{max, min};
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

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    adjs: HashMap<String, Vec<String>>,
    verts: HashSet<String>,
}

impl Graph {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        let mut adjs = HashMap::new();
        let mut verts = HashSet::new();
        for l in lines {
            let line = l.trim();
            let parts = line.split('-').map(|s| s.to_string()).collect::<Vec<String>>();
            adjs.entry(parts[0].to_string()).or_insert(vec![]).push(parts[1].to_string());
            adjs.entry(parts[1].to_string()).or_insert(vec![]).push(parts[0].to_string());

            verts.insert(parts[0].to_string());
            verts.insert(parts[1].to_string());
        }

        Self {
            adjs,
            verts,
        }
    }

    fn is_neigh(&self, a: &String, b: &String) -> bool {
        self.adjs.get(a).unwrap().contains(b)
    }

    fn all_cliques_with_t(&self) -> usize {
        let mut cliques_with_t = HashSet::new();

        for v in &self.verts {
            if !v.starts_with('t') { continue; }
            let neighs = self.adjs.get(v).unwrap();
            for i in 0..neighs.len() {
                for j in (i+1)..neighs.len() {
                    if self.is_neigh(&neighs[i], &neighs[j]) {
                        let mut clique = vec![v.to_string(), neighs[i].to_string(), neighs[j].to_string()];
                        clique.sort();
                        dprintln!("Found clique: {:?}", clique);
                        cliques_with_t.insert(clique);
                    }
                }
            }
        }

        cliques_with_t.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Solution {
    graph: Graph,
}

impl Solution {
    fn from_input<I>(lines: I) -> Self
        where I: Iterator<Item = String>
    {
        Solution {
            graph: Graph::from_input(lines),
        }
    }

    fn solve(&self) -> usize {
        self.graph.all_cliques_with_t()
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
    fn sample() {
        test_ignore_whitespaces(
            "kh-tc
            qp-kh
            de-cg
            ka-co
            yn-aq
            qp-ub
            cg-tb
            vc-aq
            tb-ka
            wh-tc
            yn-cg
            kh-ub
            ta-co
            de-co
            tc-td
            tb-wq
            wh-td
            ta-ka
            td-qp
            aq-cg
            wq-ub
            ub-vc
            de-ta
            wq-aq
            wq-vc
            wh-yn
            ka-de
            kh-ta
            co-tc
            wh-qp
            tb-vc
            td-yn",
            "7",
        );
    }
}
