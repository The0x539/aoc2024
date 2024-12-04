#![cfg_attr(test, feature(test))]

use util::*;

type N = char;

type In = Vec<N>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.chars().collect()
}

fn part1(n: &[In]) -> Out {
    let w = n[0].len() as isize;
    let h = n.len() as isize;

    let mut v = vec![];

    for y in 0..h {
        for x in 0..w {
            if n[y as usize][x as usize] == 'X' {
                for dx in -1..=1_isize {
                    for dy in -1..=1_isize {
                        if (dx, dy) == (0, 0) {
                            continue;
                        }

                        v.push((Pos { x, y }, Pos::new(dx, dy)))
                    }
                }
            }
        }
    }

    for c in ['M', 'A', 'S'] {
        v.retain_mut(|(p, dp)| {
            *p += (*dp).into();
            if !(0..h).contains(&p.x) || !(0..w).contains(&p.y) {
                return false;
            }
            n[p.y as usize][p.x as usize] == c
        });
    }

    v.len()
}

fn part2(n: &[In]) -> Out {
    let w = n[0].len();
    let h = n.len();

    let mut q = 0;

    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if n[y][x] != 'A' {
                continue;
            }

            let z = [
                n[y - 1][x - 1],
                n[y - 1][x + 1],
                n[y + 1][x + 1],
                n[y + 1][x - 1],
            ];

            if [
                ['M', 'M', 'S', 'S'],
                ['M', 'S', 'S', 'M'],
                ['S', 'S', 'M', 'M'],
                ['S', 'M', 'M', 'S'],
            ]
            .contains(&z)
            {
                q += 1;
            }
        }
    }

    q
}

util::register!(parse, part1, part2);
