#![cfg_attr(test, feature(test))]

use itertools::Itertools;
use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<char>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.chars().collect()
}

const NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

const DPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'], //
    ['<', 'v', '>'],
];

fn find<const N: usize>(grid: &[[char; N]], c: char) -> P {
    for (x, y) in iter_2d(0..N, 0..grid.len()) {
        if grid[y][x].eq_ignore_ascii_case(&c) {
            return P::new(x as N, y as N);
        }
    }
    panic!()
}

fn sequence<const N: usize>(grid: &[[char; N]], seq: &[char]) -> Vec<char> {
    let mut pos = find(grid, 'A');
    let gap = find(grid, ' ');

    let mut inputs = vec![];
    for &c in seq {
        let dest = find(grid, c);
        let (dx, dy) = (dest.x - pos.x, dest.y - pos.y);

        let could_hit_gap = pos + (dx, 0) == gap || pos + (0, dy) == gap;

        let mut new_inputs = vec![];
        let mut hit_gap = false;

        let cx = if dx < 0 { '<' } else { '>' };
        for _ in 0..dx.abs() {
            new_inputs.push(cx);
            pos.x += dx.signum();
            if pos == gap {
                hit_gap = true;
            }
        }

        let cy = if dy < 0 { '^' } else { 'v' };
        for _ in 0..dy.abs() {
            new_inputs.push(cy);
            pos.y += dy.signum();
            // it should only be possible to hit the gap in the first leg of the route,
            // as the gap will be in the "elbow"
            assert_ne!(pos, gap);
        }

        if hit_gap {
            assert!(could_hit_gap);
            assert!(dx != 0 && dy != 0);
            new_inputs.reverse();
        }

        assert_eq!(pos, dest);

        inputs.extend(new_inputs);
        // heinous hack: lowercase 'a' signifies "do not invert this segment or you will hit the gap"
        inputs.push(if could_hit_gap { 'a' } else { 'A' });
    }
    inputs
}

fn complexity(seq: &[char], steps: u8, memo: &mut HashMap<u8, HashMap<Vec<char>, usize>>) -> usize {
    if steps == 0 {
        return seq.len();
    }

    if let Some(known_result) = memo.get(&steps).and_then(|m| m.get(seq)) {
        return *known_result;
    }

    let counts = seq.split_inclusive(|&c| c == 'a' || c == 'A').counts();

    let mut total = 0;
    for (chunk, count) in counts {
        let mut sub_result = complexity(&sequence(&DPAD, chunk), steps - 1, memo);

        if chunk.last() != Some(&'a') {
            let mut flipped = chunk.to_owned();
            flipped.split_last_mut().unwrap().1.reverse();
            let alt = complexity(&sequence(&DPAD, &flipped), steps - 1, memo);
            sub_result = sub_result.min(alt);
        }

        total += sub_result * count;
    }

    memo.entry(steps).or_default().insert(seq.to_owned(), total);
    total
}

fn solve(input: &[In], steps: u8) -> Out {
    let mut total = 0;
    for line in input {
        let comp: Out = complexity(&sequence(&NUMPAD, line), steps, &mut HashMap::new());
        let code: Out = p(&String::from_iter(&line[..3]));
        total += comp * code;
    }
    total
}

fn part1(input: &[In]) -> Out {
    solve(input, 2)
}

fn part2(input: &[In]) -> Out {
    solve(input, 25)
}

util::register!(parse, part1, part2);
