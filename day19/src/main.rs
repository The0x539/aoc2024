#![cfg_attr(test, feature(test))]

use std::collections::HashMap;

type Out = usize;

struct In {
    patterns: Vec<&'static str>,
    designs: Vec<&'static str>,
}

fn parse(s: &'static str) -> In {
    let (a, b) = s.split_once("\n\n").unwrap();
    let patterns = a.split(", ").collect();
    let designs = b.lines().collect();
    In { patterns, designs }
}

fn ways(design: &str, patterns: &[&str]) -> usize {
    let patterns = patterns
        .iter()
        .filter(|p| design.contains(**p))
        .collect::<Vec<_>>();

    let mut result = 0;

    let mut state = HashMap::from([(design, 1)]);

    while !state.is_empty() {
        for (rem, count) in std::mem::take(&mut state) {
            for p in &patterns {
                match rem.strip_prefix(*p) {
                    Some("") => result += count,
                    Some(s) => *state.entry(s).or_default() += count,
                    None => {}
                }
            }
        }
    }

    result
}

fn part1(input: &In) -> Out {
    let f = |design: &&&str| ways(design, &input.patterns) > 0;
    input.designs.iter().filter(f).count()
}

fn part2(input: &In) -> Out {
    let f = |design: &&str| ways(design, &input.patterns);
    input.designs.iter().map(f).sum()
}

util::register!(parse, part1, part2, @alt);
