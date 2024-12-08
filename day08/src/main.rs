#![cfg_attr(test, feature(test))]

use itertools::Itertools;
use util::*;

type N = i32;
type P = Pos<N>;

type In = (HashMap<char, Vec<P>>, P);
type Out = usize;

fn parse(s: &'static str) -> In {
    let w = s.lines().next().unwrap().len() as _;
    let h = s.lines().count() as _;
    let mut m = HashMap::<_, Vec<_>>::new();
    for (row, y) in s.lines().zip(0..) {
        for (c, x) in row.chars().zip(0..) {
            if c != '.' {
                m.entry(c).or_default().push(P { x, y });
            }
        }
    }
    (m, P::new(w, h))
}

fn solve(nodes: &HashMap<char, Vec<P>>, mut add_antinodes: impl FnMut(P, (N, N))) {
    for ps in nodes.values() {
        for (&a, &b) in ps.iter().tuple_combinations() {
            add_antinodes(b, (b.x - a.x, b.y - a.y));
            add_antinodes(a, (a.x - b.x, a.y - b.y));
        }
    }
}

fn part1((nodes, dims): &In) -> Out {
    let mut locs = HashSet::new();
    solve(nodes, |mut point, delta| {
        point += delta;
        if in_bounds(point, (0, 0), *dims) {
            locs.insert(point);
        }
    });
    locs.len()
}

fn part2((nodes, dims): &In) -> Out {
    let mut locs = HashSet::new();
    solve(nodes, |mut point, delta| {
        while in_bounds(point, (0, 0), *dims) {
            locs.insert(point);
            point += delta;
        }
    });
    locs.len()
}

util::register!(parse, part1, part2, @alt);
