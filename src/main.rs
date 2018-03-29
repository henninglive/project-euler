extern crate project_euler;

use project_euler as pe;
use std::collections::BTreeMap;

type Problem = fn() -> String;

fn problems() -> BTreeMap<usize, Problem> {
    let mut problems = BTreeMap::new();
    problems.insert(1, pe::p1 as Problem);
    problems.insert(2, pe::p2 as Problem);
    problems.insert(4, pe::p4 as Problem);
    problems.insert(5, pe::p5 as Problem);
    problems.insert(6, pe::p6 as Problem);
    problems.insert(8, pe::p8 as Problem);
    problems.insert(9, pe::p9 as Problem);
    problems.insert(15, pe::p15 as Problem);
    problems.insert(22, pe::p22 as Problem);
    problems.insert(31, pe::p31 as Problem);
    problems
}

fn main() {
    let problems = problems();
    match std::env::args().nth(1) {
        Some(ref s) if s == "--help" || s == "-h" => {
            print!(
"Project Euler is a website(https://projecteuler.net) dedicated to a series of \n\
computational problems intended to be solved with computer programs.\n\
This program computes the solution to some of these problems.\n\n\
Usage: project-euler [problem number]\n")
        },
        Some(s) => {
            match s.parse::<usize>().ok() {
                Some(i) => {
                    match problems.get(&i) {
                        Some(p) => println!("Problem {}: {}", i, p()),
                        None => println!("No solution available for problem {}", i),
                    }
                },
                None => println!("Usage: project-euler [problem number]"),
            }
        },
        None => {
            for (i, p) in problems {
                println!("Problem {}: {}", i, p());
            }
        },
    }
}