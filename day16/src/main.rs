#![cfg_attr(test, feature(test))]
#![feature(array_windows)]

use std::cmp::Ordering;

use util::*;

type N = i32;
type P = Pos<N>;

type Out = usize;

#[derive(Default, Clone, Debug)]
struct In {
    cells: HashSet<P>,
    start: P,
    end: P,
}

fn parse(s: &'static str) -> In {
    let mut input = In::default();

    for (line, y) in s.lines().zip(0..) {
        for (c, x) in line.chars().zip(0..) {
            let p = P { x, y };
            if matches!(c, '.' | 'S' | 'E') {
                input.cells.insert(p);
            }
            match c {
                '.' => _ = input.cells.insert(p),
                'S' => input.start = p,
                'E' => input.end = p,
                _ => {}
            }
        }
    }
    input
}

#[derive(Default, Clone)]
struct Path {
    canonical: Vec<P>,
    all: HashSet<P>,
}

impl Path {
    fn add(mut self, point: P) -> Self {
        self.canonical.push(point);
        self.all.insert(point);
        self
    }

    fn score(&self) -> Out {
        let path = &self.canonical;
        let mut steps = 0;
        let mut turns = 0;
        let mut facing = (1, 0);
        for [a, b] in path.array_windows() {
            steps += 1;
            let new_facing = (b.x - a.x, b.y - a.y);
            if new_facing != facing {
                facing = new_facing;
                turns += 1;
            }
        }
        1000 * turns + steps
    }

    fn key(&self) -> [P; 2] {
        *self.canonical.array_windows::<2>().next_back().unwrap()
    }

    fn compete(&mut self, new: Self) {
        assert_eq!(self.key(), new.key());
        match self.score().cmp(&new.score()) {
            Ordering::Less => {}
            Ordering::Equal => self.all.extend(new.all),
            Ordering::Greater => *self = new,
        }
    }
}

fn solve(n: &In) -> Vec<Path> {
    let mut paths = vec![Path::default().add(n.start)];

    let mut finished_paths = vec![];

    while !paths.is_empty() {
        for path in std::mem::take(&mut paths) {
            let end = *path.canonical.last().unwrap();
            if end == n.end {
                finished_paths.push(path);
                continue;
            }
            for dir in udlr() {
                let target = end + dir;
                if n.cells.contains(&target) && !path.canonical.contains(&target) {
                    paths.push(path.clone().add(target));
                }
            }
        }
        cull(&mut paths);
    }

    finished_paths
}

fn cull(paths: &mut Vec<Path>) {
    let mut grouped = HashMap::<_, Path>::new();

    for path in paths.drain(..) {
        // last_two covers current postion + facing direction
        let key = path.key();
        if let Some(existing) = grouped.get_mut(&key) {
            existing.compete(path);
        } else {
            grouped.insert(key, path);
        }
    }

    paths.extend(grouped.into_values());
}

fn part1(n: &In) -> Out {
    let finished_paths = solve(n);
    finished_paths.iter().map(Path::score).min().unwrap()
}

fn part2(n: &In) -> Out {
    let finished_paths = solve(n);
    finished_paths
        .iter()
        .min_by_key(|p| p.score())
        .unwrap()
        .all
        .len()
}

util::register!(parse, part1, part2, @alt);
