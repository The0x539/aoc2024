#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;

type In = [N; 2];
type Out = u32;

fn parse(s: &'static str) -> In {
    ints_n(s)
}

fn part1(n: &[In]) -> Out {
    let (mut left, mut right): (Vec<N>, Vec<N>) = n.iter().copied().map(|[a, b]| (a, b)).unzip();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| N::abs_diff(a, b))
        .sum()
}

fn part2(rows: &[In]) -> Out {
    let mut right = HashMap::<N, Out>::new();
    for &[_, b] in rows {
        *right.entry(b).or_default() += 1;
    }
    rows.iter()
        .map(|[a, _b]| *a as Out * right.get(a).copied().unwrap_or_default())
        .sum()
}

util::register!(parse, part1, part2);
