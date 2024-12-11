#![cfg_attr(test, feature(test))]

use util::*;

type N = num_bigint::BigUint;

type In = Vec<N>;
type Out = usize;

fn parse(s: &'static str) -> In {
    ints(s)
}

fn solve(input: &In, iterations: u8) -> Out {
    let mut rocks = HashMap::new();
    for v in input {
        *rocks.entry(v.clone()).or_default() += 1;
    }

    for _ in 0..iterations {
        let mut new_rocks = HashMap::new();
        for (value, count) in rocks {
            let mut add = |k: N| *new_rocks.entry(k).or_default() += count;

            if value == 0u8.into() {
                add(1u8.into());
                continue;
            }

            let s = value.to_string();
            if s.len() % 2 == 0 {
                let i = s.len() / 2;
                add(p(&s[..i]));
                add(p(&s[i..]));
            } else {
                add(value * 2024u32);
            }
        }
        rocks = new_rocks;
    }

    rocks.values().sum()
}

fn part1(n: &In) -> Out {
    solve(n, 25)
}

fn part2(n: &In) -> Out {
    solve(n, 75)
}

util::register!(parse, part1, part2, @alt);
