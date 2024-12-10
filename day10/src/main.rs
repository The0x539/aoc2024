#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<N>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.chars().map(|c| c.to_digit(10).unwrap() as N).collect()
}

fn solve<G: Default + Extend<P> + IntoIterator<Item = P>>(n: &[In]) -> Out {
    let mut heads = vec![];

    let w = n[0].len() as N;
    let h = n.len() as N;

    for x in 0..w {
        for y in 0..h {
            if n[y as usize][x as usize] == 0 {
                let mut group = G::default();
                group.extend([P { x, y }]);
                heads.push(group);
            }
        }
    }

    for height in 1..=9 {
        for octopus in &mut heads {
            for cursor in std::mem::take(octopus) {
                for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let pos = cursor + dir;
                    if in_bounds(pos, (0, 0), (w, h)) && n[pos.y as usize][pos.x as usize] == height
                    {
                        octopus.extend([pos]);
                    }
                }
            }
        }
    }

    heads.into_iter().map(|v| v.into_iter().count()).sum()
}

fn part1(n: &[In]) -> Out {
    solve::<HashSet<P>>(n)
}

fn part2(n: &[In]) -> Out {
    solve::<Vec<P>>(n)
}

util::register!(parse, part1, part2);
