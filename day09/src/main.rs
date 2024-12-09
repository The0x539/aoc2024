#![cfg_attr(test, feature(test))]

use std::collections::VecDeque;

use util::*;

type N = usize;

type In = Vec<N>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as _)
        .collect()
}

fn part1(n: &In) -> Out {
    let mut initial = VecDeque::new();
    for (i, len) in n.iter().enumerate() {
        for _ in 0..*len {
            initial.push_back(if i % 2 == 0 { Some(i / 2) } else { None });
        }
    }

    let mut done = Vec::new();

    while let Some(front) = initial.pop_front() {
        if let Some(val) = front {
            done.push(val);
        } else {
            while let Some(back) = initial.pop_back() {
                if let Some(val) = back {
                    done.push(val);
                    break;
                }
            }
        }
    }

    done.into_iter().enumerate().map(|(a, b)| a * b).sum()
}

fn part2(n: &In) -> Out {
    let mut gaps = Vec::new();
    let mut locations = BTreeMap::new();
    let mut lengths = BTreeMap::new();

    let mut pos = 0;
    let mut max_id = 0;
    for (index, &len) in n.iter().enumerate() {
        if index % 2 == 0 {
            let file_id = index / 2;
            locations.insert(file_id, pos);
            lengths.insert(file_id, len);
            max_id = file_id;
        } else {
            gaps.push((pos, len));
        }
        pos += len;
    }

    for file_id in (0..=max_id).rev() {
        for gap_index in 0..gaps.len() {
            let old_file_pos = locations[&file_id];

            let (gap_pos, gap_len) = &mut gaps[gap_index];
            if old_file_pos <= *gap_pos {
                continue;
            }

            let file_len = lengths[&file_id];
            let Some(remaining_gap) = gap_len.checked_sub(file_len) else {
                continue;
            };

            locations.insert(file_id, *gap_pos);
            if remaining_gap == 0 {
                gaps.remove(gap_index);
            } else {
                *gap_pos += file_len;
                *gap_len = remaining_gap;
            }

            break;
        }
    }

    let mut disk = BTreeMap::new();
    for (id, base) in locations {
        for offset in 0..lengths[&id] {
            disk.insert(base + offset, id);
        }
    }

    disk.into_iter().map(|(a, b)| a * b).sum()
}

register!(parse, part1, part2, @alt);
