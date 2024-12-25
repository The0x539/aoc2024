#![cfg_attr(test, feature(test))]

struct Schematic {
    is_key: bool,
    heights: [usize; 5],
}

type In = Vec<Schematic>;
type Out = usize;

fn parse_lock(s: &str) -> Schematic {
    let mut rows = s
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    assert_eq!(rows.len(), 7);
    assert_eq!(rows[0].len(), 5);

    let is_key = rows[0] != ['#'; 5];
    if is_key {
        rows.reverse();
    }
    assert_eq!(rows[0], ['#'; 5]);

    let heights = std::array::from_fn(|x| rows.iter().position(|row| row[x] == '.').unwrap() - 1);

    Schematic { is_key, heights }
}

fn parse(s: &'static str) -> In {
    s.split("\n\n").map(parse_lock).collect()
}

fn part1(input: &In) -> Out {
    let (locks, keys): (Vec<_>, Vec<_>) = input.iter().partition(|s| s.is_key);

    itertools::iproduct!(locks, keys)
        .filter(|(lock, key)| {
            lock.heights
                .iter()
                .zip(key.heights)
                .all(|(a, b)| a + b <= 5)
        })
        .count()
}

util::register!(parse, part1, |_| 0, @alt);
