#![cfg_attr(test, feature(test))]

use util::*;

type N = i64;

type In = (N, Vec<N>);
type Out = N;

fn parse(s: &'static str) -> In {
    let mut v = ints(s);
    let n = v.remove(0);
    (n, v)
}

fn check((val, vals): &In, part2: bool) -> bool {
    let mut branches = HashSet::from([vals[0]]);

    for &v in &vals[1..] {
        let mut new_branches = HashSet::new();
        let mut push = |n| {
            if n <= *val {
                new_branches.insert(n);
            }
        };
        for b in branches.drain() {
            push(b + v);
            push(b * v);
            if part2 {
                push(b * 10_i64.pow(v.ilog10() + 1) + v);
            }
        }
        branches = new_branches;
    }

    branches.contains(&val)
}

fn part1(n: &[In]) -> Out {
    n.iter().filter(|nn| check(nn, false)).map(|nn| nn.0).sum()
}

fn part2(n: &[In]) -> Out {
    n.iter().filter(|nn| check(nn, true)).map(|nn| nn.0).sum()
}

util::register!(parse, part1, part2);
