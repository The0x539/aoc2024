#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type P = Pos<N>;

#[derive(Debug, Clone)]
struct Robot {
    p: P,
    v: P,
}

const W: N = if cfg!(test) { 11 } else { 101 };
const H: N = if cfg!(test) { 7 } else { 103 };

impl Robot {
    fn step(&mut self) {
        self.p += (self.v.x, self.v.y);

        if self.p.x < 0 {
            self.p.x += W;
        } else if self.p.x >= W {
            self.p.x -= W;
        }
        if self.p.y < 0 {
            self.p.y += H;
        } else if self.p.y >= H {
            self.p.y -= H;
        }
    }
}

type In = Robot;
type Out = usize;

fn parse(s: &'static str) -> In {
    let [a, b, c, d] = ints_n(s);

    Robot {
        p: P::new(a, b),
        v: P::new(c, d),
    }
}

fn part1(n: &[In]) -> Out {
    let (w, h) = if cfg!(test) { (11, 7) } else { (101, 103) };

    let mut robots = n.to_vec();

    for _ in 0..100 {
        for r in &mut robots {
            r.step();
        }
    }

    let wm = w / 2;
    let hm = h / 2;

    let mut quadrants = HashMap::<_, usize>::new();

    for r in robots {
        if r.p.x == wm || r.p.y == hm {
            continue;
        }
        let q = (r.p.x.cmp(&wm), r.p.y.cmp(&hm));
        *quadrants.entry(q).or_default() += 1;
    }

    quadrants.values().product()
}

fn part2(n: &[In]) -> Out {
    if cfg!(test) {
        return 0;
    }

    let mut robots = n.to_vec();

    let mut dots = HashSet::new();

    for i in 1.. {
        for r in &mut robots {
            r.step();
        }

        dots.clear();
        dots.extend(robots.iter().map(|r| r.p));

        if !dots.iter().any(|p| {
            if p.x < 5 {
                return false;
            }
            (-3..=3).all(|dx| {
                let p2 = *p + (dx, 0);
                dots.contains(&p2)
            })
        }) {
            continue;
        }

        for (x, y) in iter_2d(0..W, 0..H) {
            if dots.contains(&P { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
            if x == W - 1 {
                println!();
            }
        }

        return i;
    }

    0
}

util::register!(parse, part1, part2);
