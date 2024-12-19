#![cfg_attr(test, feature(test))]

use std::cmp::Ordering;

use util::*;

type N = i32;
type P = Pos<N>;

type In = P;
type Out = String;

const SIZE: usize = if cfg!(test) { 7 } else { 71 };

fn parse(s: &'static str) -> In {
    let [x, y] = ints_n(s);
    P { x, y }
}

fn solve(bytes: &HashSet<P>) -> Option<N> {
    let start = P::new(0, 0);
    let mut distances = HashMap::from([(start, 0)]);
    let mut visited = HashSet::new();
    let mut to_visit = HashSet::from([start]);

    while !to_visit.is_empty() {
        let pos = *to_visit
            .iter()
            .min_by_key(|k| distances.get(k).unwrap_or(&N::MAX))
            .unwrap();

        let dist = distances[&pos];
        to_visit.remove(&pos);
        visited.insert(pos);
        for dir in udlr() {
            let neighbor = pos + dir;
            if bytes.contains(&neighbor) || !in_bounds(neighbor, (0, 0), (SIZE as N, SIZE as N)) {
                continue;
            }
            let entry = distances.entry(neighbor).or_insert(i32::MAX);
            *entry = (*entry).min(dist + 1);
            if !visited.contains(&neighbor) {
                to_visit.insert(neighbor);
            }
        }
    }

    let end = P::new(SIZE as N - 1, SIZE as N - 1);
    distances.get(&end).copied()
}

fn part1(input: &[In]) -> Out {
    let num_bytes = if cfg!(test) { 12 } else { 1024 };
    let rocks = input[..num_bytes].iter().copied().collect::<HashSet<_>>();
    solve(&rocks).unwrap().to_string()
}

fn part2(input: &[In]) -> Out {
    let f = |&n| {
        let bytes = input[..n].iter().copied().collect::<HashSet<_>>();
        if solve(&bytes).is_some() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    let counts = (0..input.len()).collect::<Vec<_>>();
    let index = counts.binary_search_by(f).unwrap_err() - 1;
    format!("{},{}", input[index].x, input[index].y)
}

util::register!(parse, part1, part2);
