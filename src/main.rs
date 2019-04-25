extern crate project_euler;

use project_euler as pe;
use std::collections::BTreeMap;

type Solution = fn() -> String;

fn problems() -> BTreeMap<usize, Solution> {
    let mut problems = BTreeMap::new();
    problems.insert(1,   pe::p001::solution as Solution);
    problems.insert(2,   pe::p002::solution as Solution);
    problems.insert(3,   pe::p003::solution as Solution);
    problems.insert(4,   pe::p004::solution as Solution);
    problems.insert(5,   pe::p005::solution as Solution);
    problems.insert(6,   pe::p006::solution as Solution);
    problems.insert(7,   pe::p007::solution as Solution);
    problems.insert(8,   pe::p008::solution as Solution);
    problems.insert(9,   pe::p009::solution as Solution);
    problems.insert(10,  pe::p010::solution as Solution);
    problems.insert(11,  pe::p011::solution as Solution);
    problems.insert(13,  pe::p013::solution as Solution);
    problems.insert(14,  pe::p014::solution as Solution);
    problems.insert(15,  pe::p015::solution as Solution);
    problems.insert(16,  pe::p016::solution as Solution);
    problems.insert(18,  pe::p018::solution as Solution);
    problems.insert(19,  pe::p019::solution as Solution);
    problems.insert(20,  pe::p020::solution as Solution);
    problems.insert(22,  pe::p022::solution as Solution);
    problems.insert(24,  pe::p024::solution as Solution);
    problems.insert(29,  pe::p029::solution as Solution);
    problems.insert(31,  pe::p031::solution as Solution);
    problems.insert(48,  pe::p048::solution as Solution);
    problems.insert(52,  pe::p052::solution as Solution);
    problems.insert(67,  pe::p067::solution as Solution);
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
