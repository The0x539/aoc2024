#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type P = Pos<N>;

type Out = N;

#[derive(Default, Debug, Clone)]
pub struct Map {
    walls: HashSet<P>,
    boxes: HashSet<P>,
    bot: P,
}

impl Map {
    pub fn print(&self, part2: bool) {
        let w = self.walls.iter().map(|p| p.x).max().unwrap();
        let h = self.walls.iter().map(|p| p.y).max().unwrap();

        let mut skip = false;

        for (x, y) in iter_2d(0..=w, 0..=h) {
            if skip {
                skip = false;
                continue;
            }

            let p = P { x, y };
            let s = if self.walls.contains(&p) {
                "#"
            } else if self.boxes.contains(&p) {
                if part2 {
                    skip = true;
                    "[]"
                } else {
                    "O"
                }
            } else if self.bot == p {
                "@"
            } else {
                "."
            };

            print!("{s}");
            if x == w {
                println!();
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

type In = (Map, Vec<Move>);

fn parse(s: &'static str) -> In {
    let mut map = Map::default();

    let (a, b) = s.split_once("\n\n").unwrap();
    for (line, y) in a.lines().zip(0..) {
        for (c, x) in line.chars().zip(0..) {
            match c {
                '#' => _ = map.walls.insert(P { x, y }),
                'O' => _ = map.boxes.insert(P { x, y }),
                '@' => map.bot = P { x, y },
                _ => {}
            }
        }
    }

    let moves = b
        .chars()
        .filter_map(|c| {
            Some(match c {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => return None,
            })
        })
        .collect();

    (map, moves)
}

fn part1((mapp, moves): &In) -> Out {
    let mut map = mapp.clone();

    for m in moves {
        let delta = match m {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        };

        let target = map.bot + delta;

        if map.walls.contains(&target) {
            continue;
        }

        if map.boxes.contains(&target) {
            let mut to_push = vec![target];
            let mut beyond = target + delta;
            let can_push = loop {
                if map.walls.contains(&beyond) {
                    break false;
                } else if map.boxes.contains(&beyond) {
                    to_push.push(beyond);
                    beyond += delta;
                } else {
                    break true;
                }
            };

            if !can_push {
                continue;
            }

            for b in &mut to_push {
                map.boxes.remove(b);
                *b += delta;
            }
            for b in &to_push {
                map.boxes.insert(*b);
            }
        }

        map.bot += delta;
    }

    map.boxes.into_iter().map(|b| b.x + 100 * b.y).sum()
}

fn part2((map_1x, moves): &In) -> Out {
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();

    for w in &map_1x.walls {
        walls.insert(P::new(w.x * 2, w.y));
        walls.insert(P::new(1 + w.x * 2, w.y));
    }

    for b in &map_1x.boxes {
        boxes.insert(P::new(b.x * 2, b.y));
    }

    let mut map = Map {
        boxes,
        walls,
        bot: P::new(map_1x.bot.x * 2, map_1x.bot.y),
    };

    'outer: for m in moves {
        let delta = match m {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        };

        let target = map.bot + delta;

        if map.walls.contains(&target) {
            continue;
        }

        let mut to_push = vec![];

        match m {
            Move::Left => {
                let mut box_left = target + delta;
                loop {
                    if map.boxes.contains(&box_left) {
                        to_push.push(box_left);
                    } else if map.walls.contains(&box_left) {
                        if map.walls.contains(&(box_left + (1, 0))) {
                            // truly hit a wall
                            continue 'outer;
                        } else {
                            // air gap
                            break;
                        }
                    } else {
                        break;
                    }

                    box_left += delta;
                    box_left += delta;
                }
            }
            Move::Right => {
                let mut box_left = target;
                loop {
                    if map.boxes.contains(&box_left) {
                        to_push.push(box_left);
                    } else if map.walls.contains(&box_left) {
                        continue 'outer;
                    } else {
                        break;
                    }

                    box_left += delta;
                    box_left += delta;
                }
            }
            Move::Up | Move::Down => {
                let mut layer = HashSet::new();
                let (l, r) = (target + (-1, 0), target);
                if map.boxes.contains(&l) {
                    layer.insert(l);
                }
                if map.boxes.contains(&r) {
                    layer.insert(r);
                }

                while !layer.is_empty() {
                    for b in std::mem::take(&mut layer) {
                        for dx in -1..=1 {
                            let b2 = b + delta + (dx, 0);
                            if dx >= 0 && map.walls.contains(&b2) {
                                continue 'outer;
                            }
                            if map.boxes.contains(&b2) {
                                layer.insert(b2);
                            }
                        }
                        to_push.push(b);
                    }
                }
            }
        }

        for b in &mut to_push {
            map.boxes.remove(b);
            *b += delta;
        }
        for b in &to_push {
            map.boxes.insert(*b);
        }

        map.bot = target;
    }

    map.boxes.into_iter().map(|b| b.x + 100 * b.y).sum()
}

util::register!(parse, part1, part2, @alt);
