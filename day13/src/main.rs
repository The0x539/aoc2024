#![cfg_attr(test, feature(test))]

use util::*;

type N = i64;
type P = Pos<N>;

type In = Vec<Machine>;
type Out = N;

#[derive(Debug, Clone)]
struct Machine {
    a: P,
    b: P,
    p: P,
}

fn parse(s: &'static str) -> In {
    s.split("\n\n")
        .map(|ss| {
            let [a, b, c, d, e, f] = ints_n(ss);
            Machine {
                a: P::new(a, b),
                b: P::new(c, d),
                p: P::new(e, f),
            }
        })
        .collect()
}

/*
    (ax)a + (bx)b = xp
    (ay)a + (by)b = yp

    (axby)a + (bxby)b = pxby // multiply by by
    (aybx)a + (bxby)b = pybx // multiply by bx

    (axby - bxay)a = pxby - pybx
    a = (pxby - pybx) / (axby - aybx)
    // to solve for b instead of a, swap a and b
*/
fn solve(a: P, b: P, p: P) -> Option<N> {
    let numer = p.x * b.y - p.y * b.x;
    let denom = a.x * b.y - a.y * b.x;
    (numer % denom == 0).then_some(numer / denom)
}

fn cost(m: &Machine) -> Option<(N, N)> {
    let a = solve(m.a, m.b, m.p)?;
    let b = solve(m.b, m.a, m.p)?;
    Some((a, b))
}

fn part1(n: &In) -> Out {
    n.iter().filter_map(cost).map(|(a, b)| a * 3 + b).sum()
}

fn part2(n: &In) -> Out {
    let mut n = n.to_vec();
    for m in &mut n {
        m.p += (10000000000000, 10000000000000);
    }

    part1(&n)
}

util::register!(parse, part1, part2, @alt);
