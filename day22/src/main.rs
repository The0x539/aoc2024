#![cfg_attr(test, feature(test))]
#![feature(array_windows)]

use rayon::prelude::*;
use util::*;

type N = i64;

type In = N;
type Out = N;

const PRUNE: N = 16777216;

fn next(mut n: N) -> N {
    n ^= n * 64;
    n %= PRUNE;
    n ^= n / 32;
    n %= PRUNE;
    n ^= n * 2048;
    n %= PRUNE;
    n
}

fn parse(s: &'static str) -> In {
    p(s)
}

fn part1(input: &[In]) -> Out {
    let f = |initial| std::iter::successors(Some(initial), |n| Some(next(*n))).nth(2000);
    input.iter().copied().filter_map(f).sum()
}

fn deltas(prices: &[N]) -> impl Iterator<Item = N> + '_ {
    prices.array_windows().map(|[before, after]| after - before)
}

fn part2(input: &[In]) -> Out {
    let input = if cfg!(test) { &[1, 2, 3, 2024] } else { input };

    let mut price_sequences = vec![];
    let mut quartets = HashSet::<[N; 4]>::new();

    for &n in input {
        let secrets = std::iter::successors(Some(n), |a| Some(next(*a)));
        let prices = secrets.map(|s| s % 10).take(2000).collect::<Vec<_>>();
        let deltas = deltas(&prices).collect::<Vec<_>>();
        price_sequences.push(prices);
        quartets.extend(deltas.array_windows::<4>().copied());
    }

    let find_price = |quartet: [N; 4], sequence: &[N]| {
        sequence
            .array_windows::<5>()
            .find(|prices| deltas(&**prices).eq(quartet))
            .map_or(0, |prices| prices[4])
    };

    let total_profit = |q: &[N; 4]| price_sequences.par_iter().map(|s| find_price(*q, s)).sum();

    quartets.par_iter().map(total_profit).max().unwrap()
}

util::register!(parse, part1, part2);
