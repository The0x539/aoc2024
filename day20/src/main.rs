#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type P = Pos<N>;

type Out = usize;

#[derive(Default, Clone)]
struct In {
    track: HashSet<P>,
    start: P,
    end: P,
}

fn parse(s: &'static str) -> In {
    let mut map = In::default();
    for (l, y) in s.lines().zip(0..) {
        for (c, x) in l.chars().zip(0..) {
            let p = P { x, y };
            if matches!(c, '.' | 'S' | 'E') {
                map.track.insert(p);
            }
            match c {
                'S' => map.start = p,
                'E' => map.end = p,
                '#' | '.' => continue,
                _ => panic!(),
            }
        }
    }
    map
}

impl In {
    fn path(&self) -> Vec<P> {
        let mut path = vec![self.start];
        while path.last().unwrap() != &self.end {
            for dir in udlr() {
                let next = *path.last().unwrap() + dir;
                if self.track.contains(&next) && !path.contains(&next) {
                    path.push(next);
                    break;
                }
            }
        }
        path
    }
}

fn part1(input: &In) -> Out {
    let path = input.path();
    let positions = path.iter().copied().zip(0..).collect::<HashMap<_, _>>();

    let mut cheats = HashMap::<(P, P), N>::new();

    for step0 in path.iter().copied() {
        for d1 in udlr() {
            let step1 = step0 + d1;
            if positions.contains_key(&step1) {
                // not a cheat
                continue;
            }
            for d2 in udlr() {
                let step2 = step1 + d2;
                if step2 == step0 || !positions.contains_key(&step2) {
                    continue;
                }

                let gain = (positions[&step2] - positions[&step0]) - 2;
                if gain <= 0 {
                    continue;
                }
                let entry = cheats.entry((step1, step2)).or_insert(0);
                *entry = (*entry).max(gain);
            }
        }
    }

    cheats.values().filter(|n| **n >= 100).count()
}

fn distance(a: P, b: P) -> N {
    N::abs_diff(a.x, b.x) as N + N::abs_diff(a.y, b.y) as N
}

fn part2(input: &In) -> Out {
    let path = input.path();
    let positions = path.iter().copied().zip(0..).collect::<HashMap<_, _>>();

    let mut cheats = HashMap::<(P, P), N>::new();

    for step0 in path.iter().copied() {
        for step2 in path.iter().copied() {
            let cheat_len = distance(step0, step2);
            if cheat_len > 20 {
                continue;
            }

            let skip_len = positions[&step2] - positions[&step0];
            let gain = skip_len - cheat_len;
            if gain <= 0 {
                continue;
            }
            let entry = cheats.entry((step0, step2)).or_insert(0);
            *entry = (*entry).max(gain);
        }
    }

    cheats.values().filter(|n| **n >= 100).count()
}

util::register!(parse, part1, part2, @alt);
