mod util;
mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;
mod p008;
mod p009;
mod p010;
mod p011;
mod p013;
mod p014;
mod p015;
mod p016;
mod p018;
mod p019;
mod p020;
mod p022;
mod p024;
mod p025;
mod p029;
mod p031;
mod p048;
mod p052;
mod p056;
mod p067;

macro_rules! solutions {
    ( $( $name:ident ),* ) => {
        fn problems() -> std::collections::BTreeMap<usize, fn() -> String> {
            let mut problems = std::collections::BTreeMap::new();
            let key = |name: &str| name.trim_start_matches('p').parse::<usize>().unwrap();
            $(
                problems.insert(key(stringify!($name)), $name::solution as fn() -> String);
            )*
            problems
        }
    };
}

solutions!(
    p001,
    p002,
    p003,
    p004,
    p005,
    p006,
    p007,
    p008,
    p009,
    p010,
    p011,
    p013,
    p014,
    p015,
    p016,
    p018,
    p019,
    p020,
    p022,
    p024,
    p025,
    p029,
    p031,
    p048,
    p052,
    p056,
    p067
);

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
        }
        Some(s) => {
            match s.parse::<usize>().ok() {
                Some(i) => {
                    match problems.get(&i) {
                        Some(p) => println!("Problem {}: {}", i, p()),
                        None => println!("No solution available for problem {}", i),
                    }
                }
                None => println!("Usage: project-euler [problem number]"),
            }
        }
        None => {
            for (i, p) in problems {
                println!("Problem {}: {}", i, p());
            }
        }
    }
}
