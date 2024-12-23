#![cfg_attr(test, feature(test))]

use itertools::Itertools;
use util::*;

type In = (&'static str, &'static str);
type Out = String;

fn parse(s: &'static str) -> In {
    s.split_once('-').unwrap()
}

fn part1(input: &[In]) -> Out {
    let computers = input
        .iter()
        .flat_map(|(a, b)| [a, b])
        .collect::<HashSet<_>>();

    let connections = input
        .iter()
        .copied()
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .collect::<HashSet<_>>();

    computers
        .iter()
        .copied()
        .tuple_combinations()
        .filter(|(&a, &b, &c)| {
            [a, b, c].iter().any(|x| x.starts_with('t'))
                && [(a, b), (b, c), (a, c)]
                    .iter()
                    .all(|pair| connections.contains(&pair))
        })
        .count()
        .to_string()
}

fn part2(input: &[In]) -> Out {
    let computers = input
        .iter()
        .copied()
        .flat_map(|(a, b)| [a, b])
        .collect::<HashSet<_>>();

    let connections = input
        .iter()
        .copied()
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .collect::<HashSet<_>>();

    let mut edges = HashMap::<&str, Vec<&str>>::new();
    for (a, b) in &connections {
        edges.entry(a).or_default().push(b);
    }

    let is_complete = |a: &BTreeSet<&str>| {
        a.iter()
            .copied()
            .tuple_combinations()
            .all(|pair| connections.contains(&pair))
    };

    let mut groups = computers
        .iter()
        .copied()
        .combinations(3)
        .map(BTreeSet::from_iter)
        .filter(is_complete)
        .collect::<BTreeSet<_>>();

    while groups.len() > 1 {
        for mut group in std::mem::take(&mut groups) {
            for node in computers.iter().copied() {
                if group.contains(&node) {
                    continue;
                }

                group.insert(node);
                if is_complete(&group) {
                    groups.insert(group.clone());
                }
                group.remove(node);
            }
        }
    }

    groups.into_iter().next().unwrap().into_iter().join(",")
}

util::register!(parse, part1, part2);
