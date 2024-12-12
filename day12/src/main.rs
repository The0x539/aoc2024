#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<char>;
type Out = N;

fn parse(s: &'static str) -> In {
    s.chars().collect()
}

const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

fn flood_fill<T: PartialEq>(grid: &[Vec<T>], start: P) -> HashSet<P> {
    let value = &grid[start.y as usize][start.x as usize];
    let mut to_visit = vec![start];
    let mut visited = HashSet::from([start]);

    let (w, h) = (grid[0].len() as N, grid.len() as N);

    while let Some(xy) = to_visit.pop() {
        for delta in NEIGHBORS {
            let neighbor = xy + delta;
            if !in_bounds(neighbor, (0, 0), (w, h)) {
                continue;
            }

            let neighbor_value = &grid[neighbor.y as usize][neighbor.x as usize];

            if neighbor_value == value && !visited.contains(&neighbor) {
                to_visit.push(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    visited
}

fn cost(plot: &HashSet<P>) -> N {
    let area = plot.len() as N;
    let perimeter = plot
        .iter()
        .map(|cell| {
            let mut fence = 4;
            for delta in NEIGHBORS {
                if plot.contains(&(*cell + delta)) {
                    fence -= 1;
                }
            }
            fence
        })
        .sum::<N>();

    area * perimeter
}

fn prepare(input: &[In]) -> Vec<HashSet<P>> {
    let (w, h) = (input[0].len(), input.len());
    let mut grid = vec![vec![None::<usize>; w]; h];
    let mut plots = vec![];

    for (x, y) in iter_2d(0..w, 0..h) {
        if grid[y][x].is_some() {
            continue;
        }

        let plot = flood_fill(
            input,
            P {
                x: x as _,
                y: y as _,
            },
        );

        let plot_id = plots.len();
        for coord in &plot {
            grid[coord.y as usize][coord.x as usize] = Some(plot_id);
        }
        plots.push(plot);
    }

    plots
}

fn cost2(plot: &HashSet<P>) -> N {
    let area = plot.len() as N;

    type M = HashMap<N, Vec<N>>;

    let mut down_sides = M::new();
    let mut up_sides = M::new();
    let mut left_sides = M::new();
    let mut right_sides = M::new();

    for &cell in plot {
        if !plot.contains(&(cell + (0, 1))) {
            down_sides.entry(cell.y).or_default().push(cell.x);
        }
        if !plot.contains(&(cell + (0, -1))) {
            up_sides.entry(cell.y).or_default().push(cell.x);
        }
        if !plot.contains(&(cell + (1, 0))) {
            right_sides.entry(cell.x).or_default().push(cell.y);
        }
        if !plot.contains(&(cell + (-1, 0))) {
            left_sides.entry(cell.x).or_default().push(cell.y);
        }
    }

    let perimeter = [down_sides, up_sides, left_sides, right_sides]
        .into_iter()
        .flat_map(|sides| sides.into_values())
        .map(|mut group| {
            group.sort();
            for i in (0..group.len() - 1).rev() {
                let j = i + 1;
                if group[j] == group[i] + 1 {
                    group.remove(j);
                }
            }
            group.len() as N
        })
        .sum::<N>();

    perimeter * area
}

fn part1(input: &[In]) -> Out {
    prepare(input).iter().map(cost).sum()
}

fn part2(input: &[In]) -> Out {
    prepare(input).iter().map(cost2).sum()
}

util::register!(parse, part1, part2);
