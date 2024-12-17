#![cfg_attr(test, feature(test))]

use itertools::Itertools;
use util::*;

type N = u64;

type Out = String;

#[derive(Clone)]
struct Machine {
    a: N,
    b: N,
    c: N,
    program: Vec<N>,
}

fn parse(s: &'static str) -> Machine {
    let mut program = ints::<N>(s);
    Machine {
        a: program.remove(0),
        b: program.remove(0),
        c: program.remove(0),
        program,
    }
}

impl Machine {
    fn run(&self) -> Vec<N> {
        let (mut a, mut b, mut c) = (self.a, self.b, self.c);

        let mut ip = 0;
        let mut output = vec![];

        while let Some(&opcode) = self.program.get(ip) {
            let literal = self.program[ip + 1];

            let combo = match literal {
                0..=3 => literal,
                4 => a,
                5 => b,
                6 => c,
                _ => 0, // invalid, whatever
            };

            match opcode {
                0 => a >>= combo,
                1 => b ^= literal,
                2 => b = combo % 8,
                3 => {
                    if a != 0 {
                        ip = literal as usize;
                        continue;
                    }
                }
                4 => b ^= c,
                5 => output.push(combo % 8),
                6 => b = a >> combo,
                7 => c = a >> combo,

                _ => panic!(),
            }

            ip += 2;
        }

        output
    }
}

fn part1(n: &Machine) -> Out {
    n.run().iter().join(",")
}

fn part2(n: &Machine) -> Out {
    let mut n = if cfg!(test) {
        parse("Register A: 2024 Register B: 0 Register C: 0 Program: 0,3,5,4,3,0")
    } else {
        n.clone()
    };

    let len = n.program.len();

    let mut candidates = vec![vec![1; len]];

    for i in 0..len {
        for mut candidate in std::mem::take(&mut candidates) {
            for v in 0..=7 {
                if i == 0 && v == 0 {
                    continue;
                }

                candidate[i] = v;
                n.a = to_int(&candidate);

                let output = n.run();
                assert_eq!(output.len(), n.program.len());

                let j = len - i;
                if output[j..] == n.program[j..] {
                    candidates.push(candidate.clone());
                }
            }
        }
    }

    candidates.iter().map(to_int).min().unwrap().to_string()
}

fn to_int(octets: impl AsRef<[N]>) -> N {
    octets.as_ref().iter().fold(0, |a, v| (a << 3) | v)
}

util::register!(parse, part1, part2, @alt);
