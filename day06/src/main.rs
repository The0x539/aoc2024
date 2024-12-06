#![cfg_attr(test, feature(test))]

use rayon::prelude::*;
use util::*;

type N = i32;
type P = Pos<N>;

#[derive(Clone)]
struct In {
    dims: P,
    walls: HashSet<P>,
    start: P,
}

type Out = usize;

fn parse(s: &'static str) -> In {
    let h = s.lines().count() as _;
    let w = s.lines().next().unwrap().len() as _;

    let mut start = P::new(0, 0);
    let mut walls = HashSet::new();
    for (line, y) in s.lines().zip(0..) {
        for (c, x) in line.chars().zip(0..) {
            if c == '#' {
                walls.insert(P { y, x });
            } else if c == '^' {
                start = P { y, x };
            }
        }
    }
    In {
        walls,
        start,
        dims: P::new(w, h),
    }
}

fn traverse(n: &In) -> Option<usize> {
    let mut visited = HashSet::<P>::new();
    let mut deja_vu = HashSet::<(P, (i32, i32))>::new();

    let mut pos = n.start;
    let mut dir = (0, -1);

    loop {
        if !deja_vu.insert((pos, dir)) {
            return None;
        }
        visited.insert(pos);

        let next_pos = pos + dir;
        if !(0..n.dims.x).contains(&next_pos.x) || !(0..n.dims.y).contains(&next_pos.y) {
            return Some(visited.len());
        } else if n.walls.contains(&next_pos) {
            dir = match dir {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => unreachable!(),
            };
        } else {
            pos = next_pos;
        }
    }
}

fn part1(n: &In) -> Out {
    traverse(n).unwrap()
}

fn part2(n: &In) -> Out {
    let mut coords = vec![];

    for y in 0..n.dims.y {
        for x in 0..n.dims.x {
            coords.push(P { x, y });
        }
    }

    coords
        .into_par_iter()
        .filter(|wall| *wall != n.start && !n.walls.contains(&wall))
        .map(|wall| {
            let mut state = n.clone();
            state.walls.insert(wall);
            state
        })
        .filter(|state| traverse(state).is_none())
        .count()
}

util::register!(parse, part1, part2, @alt);
