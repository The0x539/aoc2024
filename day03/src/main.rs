#![cfg_attr(test, feature(test))]

use util::*;

type In = &'static str;
type Out = usize;

use regex::Regex;

fn parse(s: &'static str) -> In {
    s
}

fn part1(n: &[In]) -> Out {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    n.iter()
        .flat_map(|s| re.captures_iter(s))
        .map(|m| p::<usize>(&m[1]) * p::<usize>(&m[2]))
        .sum()
}

fn part2(n: &[In]) -> Out {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0_usize;
    let mut should = true;
    for c in n.iter().flat_map(|s| re.captures_iter(s)) {
        if &c[0] == "do()" {
            should = true;
        } else if &c[0] == "don't()" {
            should = false;
        } else if should {
            sum += p::<usize>(&c[1]) * p::<usize>(&c[2]);
        }
    }
    sum
}

util::register!(parse, part1, part2);
