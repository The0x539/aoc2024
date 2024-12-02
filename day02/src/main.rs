#![cfg_attr(test, feature(test))]
#![feature(array_windows)]

use itertools::Itertools;
use util::*;

type N = i32;
type In = Vec<N>;
type Out = usize;

fn parse(s: &'static str) -> In {
    ints(s)
}

fn safe(ns: &[N]) -> bool {
    if ns
        .array_windows()
        .map(|[a, b]| a.cmp(b))
        .dedup()
        .exactly_one()
        .is_err()
    {
        return false;
    }

    for [a, b] in ns.array_windows() {
        if !(1..=3).contains(&a.abs_diff(*b)) {
            return false;
        }
    }

    true
}

fn part1(n: &[In]) -> Out {
    n.iter().filter(|s| safe(s)).count()
}

fn safe2(ns: &[N]) -> bool {
    if safe(ns) {
        return true;
    }

    for i in 0..ns.len() {
        let mut nss = ns.to_vec();
        nss.remove(i);
        if safe(&nss) {
            return true;
        }
    }

    false
}

fn part2(n: &[In]) -> Out {
    n.iter().filter(|s| safe2(s)).count()
}

util::register!(parse, part1, part2);
