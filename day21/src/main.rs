#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type P = Pos<N>;

type C = u8;

type In = [C; 4];
type Out = usize;

fn parse(s: &'static str) -> In {
    s.bytes().collect::<Vec<_>>().try_into().unwrap()
}

#[allow(dead_code)]
fn print(x: &[char]) {
    println!("{}", x.iter().map(|n| *n as char).collect::<String>());
}

const NUMPAD: [[C; 3]; 4] = [
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b' ', b'0', b'A'],
];

const DPAD: [[C; 3]; 2] = [
    [b' ', b'^', b'A'], //
    [b'<', b'v', b'>'],
];

fn find<const N: usize>(grid: &[[C; N]], c: C) -> P {
    for y in 0..grid.len() {
        for x in 0..N {
            if grid[y][x].eq_ignore_ascii_case(&c) {
                return P::new(x as N, y as N);
            }
        }
    }
    panic!()
}

fn sequence<const N: usize>(grid: &[[C; N]], seq: &[C]) -> Vec<C> {
    let mut pos = find(grid, b'A');
    let gap = find(grid, b' ');

    let mut inputs = vec![];
    for &c in seq {
        let dest = find(grid, c);
        let (dx, dy) = (dest.x - pos.x, dest.y - pos.y);

        let could_hit_gap = pos + (dx, 0) == gap || pos + (0, dy) == gap;

        let mut new_inputs = vec![];
        let mut hit_gap = false;

        let cx = if dx < 0 { b'<' } else { b'>' };
        for _ in 0..dx.abs() {
            new_inputs.push(cx);
            pos.x += dx.signum();
            if pos == gap {
                hit_gap = true;
            }
        }

        let cy = if dy < 0 { b'^' } else { b'v' };
        for _ in 0..dy.abs() {
            new_inputs.push(cy);
            pos.y += dy.signum();
            if pos == gap {
                hit_gap = true;
            }
        }

        if hit_gap {
            assert!(dx != 0 && dy != 0);
            new_inputs.reverse();
        }

        inputs.extend(new_inputs);
        inputs.push(if could_hit_gap { b'a' } else { b'A' });

        assert_eq!(pos, dest);
    }
    inputs
}

#[allow(dead_code)]
fn simulate<const N: usize>(grid: &[[C; N]], seq: &[C]) -> Vec<C> {
    let mut pos = find(grid, b'A');
    let mut out = vec![];
    for c in seq {
        match c {
            b'^' => pos.y -= 1,
            b'v' => pos.y += 1,
            b'<' => pos.x -= 1,
            b'>' => pos.x += 1,
            b'A' => out.push(grid[pos.y as usize][pos.x as usize]),
            _ => panic!(),
        }
    }
    out
}

fn does_not_hit_gap<const N: usize>(grid: &[[C; N]], seq: &[C]) -> bool {
    let mut pos = find(grid, b'A');
    let gap = find(grid, b' ');
    for c in seq {
        match c {
            b'^' => pos.y -= 1,
            b'v' => pos.y += 1,
            b'<' => pos.x -= 1,
            b'>' => pos.x += 1,
            b'A' => (),
            _ => panic!(),
        }
        if pos == gap {
            return false;
        }
    }
    true
}

fn shuffle(seq: Vec<C>) -> Vec<Vec<C>> {
    let mut big_orders = vec![vec![]];

    for motion in seq.split(|c| *c == b'A' || *c == b'a') {
        // let small_orders = motion
        // .iter()
        // .copied()
        // .permutations(motion.len())
        // .collect::<HashSet<_>>);
        let mut small_orders = vec![motion.to_vec(); 2];
        small_orders[1].reverse();
        if small_orders[0] == small_orders[1] {
            small_orders.pop();
        }

        big_orders = itertools::iproduct!(big_orders, &small_orders)
            .map(|(mut big, small)| {
                big.extend(small);
                big.push(b'A');
                big
            })
            .collect();
    }

    for o in &mut big_orders {
        o.pop();
    }

    big_orders
}

fn part1(input: &[In]) -> Out {
    let mut total = 0;

    for line in input {
        let complexity = std::iter::once(line)
            // stage 1
            .map(|s| sequence(&NUMPAD, s))
            .flat_map(shuffle)
            .filter(|s| does_not_hit_gap(&NUMPAD, s))
            // stage 2
            .map(|s| sequence(&DPAD, &s))
            .flat_map(shuffle)
            .filter(|s| does_not_hit_gap(&DPAD, s))
            // stage 3
            .map(|s| sequence(&DPAD, &s))
            // finalize
            .map(|s| s.len())
            .min()
            .unwrap();

        let code = p::<Out>(
            line.iter()
                .map(|n| *n as char)
                .collect::<String>()
                .trim_end_matches("A"),
        );

        total += complexity * code;
    }

    total
}

fn flip<T: Clone>(slice: &[T]) -> Vec<T> {
    let mut v = slice.to_vec();
    v.split_last_mut().unwrap().1.reverse();
    v
}

fn complexity(seq: &[C], steps: u8, memo: &mut HashMap<u8, HashMap<Vec<C>, usize>>) -> usize {
    if steps == 0 {
        return seq.len();
    }

    if let Some(memod) = memo.get(&steps).and_then(|m| m.get(seq)) {
        return *memod;
    }

    let mut counts = HashMap::<_, usize>::new();
    for chunk in seq.split_inclusive(|c| *c == b'A' || *c == b'a') {
        *counts.entry(chunk.to_owned()).or_default() += 1;
    }

    let mut total = 0;
    for (chunk, count) in &counts {
        let mut sub_result = complexity(&sequence(&DPAD, chunk), steps - 1, memo);

        if chunk.ends_with(&[b'A']) {
            let alt = complexity(&sequence(&DPAD, &flip(chunk)), steps - 1, memo);
            sub_result = sub_result.min(alt);
        }

        total += sub_result * count;
    }

    memo.entry(steps).or_default().insert(seq.to_owned(), total);
    total
}

fn part2(input: &[In]) -> Out {
    let mut total = 0;

    for line in input {
        let comp = complexity(&sequence(&NUMPAD, line), 25, &mut HashMap::new());
        let code = String::from_utf8_lossy(&line[..3]).parse::<Out>().unwrap();
        total += comp * code;
    }

    total
}

util::register!(parse, part1, part2);
