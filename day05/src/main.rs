#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type Out = N;

struct In {
    rules: Vec<[N; 2]>,
    updates: Vec<Vec<N>>,
}

fn parse(s: &'static str) -> In {
    let (a, b) = s.split_once("\n\n").unwrap();
    In {
        rules: a.lines().map(ints_n).collect(),
        updates: b.lines().map(ints).collect(),
    }
}

fn part1(n: &In) -> Out {
    let mut result = 0;
    for update in &n.updates {
        let mut seen = HashSet::new();
        let in_update = update.iter().collect::<HashSet<_>>();
        let mut bad = false;
        for page in update {
            for [left, right] in &n.rules {
                if page == right && !seen.contains(left) && in_update.contains(left) {
                    bad = true;
                    break;
                }
                if page == left && seen.contains(right) && in_update.contains(right) {
                    bad = true;
                    break;
                }
            }
            seen.insert(*page);
        }
        if !bad {
            result += update[update.len() / 2];
        }
    }
    result
}

fn part2(n: &In) -> Out {
    let mut result = 0;
    for update in &n.updates {
        let mut seen = HashSet::new();
        let in_update = update.iter().collect::<HashSet<_>>();
        let mut bad = false;
        for page in update {
            for [left, right] in &n.rules {
                if page == right && !seen.contains(left) && in_update.contains(left) {
                    bad = true;
                    break;
                }
                if page == left && seen.contains(right) && in_update.contains(right) {
                    bad = true;
                    break;
                }
            }
            seen.insert(*page);
        }
        if !bad {
            continue;
        }

        let mut update = update.clone();
        loop {
            let mut changed = false;
            for [l, r] in &n.rules {
                let Some(i) = update.iter().position(|v| v == l) else {
                    continue;
                };
                let Some(j) = update.iter().position(|v| v == r) else {
                    continue;
                };
                if j < i {
                    update.swap(i, j);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        result += update[update.len() / 2]
    }
    result
}

util::register!(parse, part1, part2, @alt);
