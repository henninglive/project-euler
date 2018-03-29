extern crate project_euler;

use project_euler as pe;
use std::collections::BTreeMap;

type Problem = fn() -> String;

fn problems() -> BTreeMap<usize, Problem> {
    let mut problems = BTreeMap::new();
    problems.insert(1,   pe::p001 as Problem);
    problems.insert(2,   pe::p002 as Problem);
    problems.insert(4,   pe::p004 as Problem);
    problems.insert(5,   pe::p005 as Problem);
    problems.insert(6,   pe::p006 as Problem);
    problems.insert(8,   pe::p008 as Problem);
    problems.insert(9,   pe::p009 as Problem);
    problems.insert(11,  pe::p011 as Problem);
    problems.insert(15,  pe::p015 as Problem);
    problems.insert(16,  pe::p016 as Problem);
    problems.insert(22,  pe::p022 as Problem);
    problems.insert(31,  pe::p031 as Problem);
    problems
}

fn main() {
    let problems = problems();
    match std::env::args().nth(1) {
        Some(ref s) if s == "--help" || s == "-h" => {
            print!("\
Project Euler is a [website](https://projecteuler.net/) dedicated to a series of\n\
[computational problems](https://projecteuler.net/archives) intended to be\n\
solved with computer programs. The problems are intended for adults and\n\
students interested in mathematics and computer programming.\n\
\n\
This program contains solutions to some of the problems solved with the\n\
[Rust programing language](https://www.rust-lang.org/en-US/).\n\
I am solving these recreationally in my spare time. It is unlikely that I will\n\
have the time or the knowledge to be able to solve all of them.\n\
\n\
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