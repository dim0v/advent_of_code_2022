use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::Stage;

type Map = Vec<BTreeSet<isize>>;

pub fn solve(stage: Stage, input: &Vec<&str>) -> String {
    let height_offset = 2;
    let ProblemInput {
        mut map,
        col_offset,
        row_range,
    } = load_base_map(input, 500, height_offset);

    if stage.is_hard() {
        add_floor(&mut map, row_range.end() + height_offset);
    }

    sim_sand_fall(&mut map, (col_offset, 0)).to_string()
}

fn add_floor(map: &mut Map, target_h: isize) {
    for col in map {
        col.insert(target_h);
    }
}

fn sim_sand_fall(map: &mut Map, spawn_point: (isize, isize)) -> isize {
    let mut cnt = 0;

    'outer: loop {
        let (mut spawn_col, mut spawn_height) = spawn_point;
        if !is_empty(map, spawn_col, spawn_height) {
            break;
        }

        let mut stop: isize;
        loop {
            let maybe_col = map.get_mut(spawn_col as usize);
            if let None = maybe_col {
                // empty column reached, time to stop
                break 'outer;
            }

            let col = maybe_col.unwrap();

            let maybe_stop = col.range(spawn_height..).next();

            if let None = maybe_stop {
                // no ground below, time to stop
                break 'outer;
            }

            stop = *maybe_stop.unwrap();
            spawn_height = stop;
            if is_empty(&map, spawn_col - 1, spawn_height) {
                spawn_col -= 1;
            } else if is_empty(&map, spawn_col + 1, spawn_height) {
                spawn_col += 1;
            } else {
                break;
            }
        }

        map[spawn_col as usize].insert(stop - 1);

        cnt += 1
    }

    fn is_empty(map: &Map, col: isize, height: isize) -> bool {
        return if col >= 0 && col < map.len() as isize {
            !map[col as usize].contains(&height)
        } else {
            true
        };
    }

    cnt
}

fn load_base_map(input: &[&str], base_col_offset: isize, height_offset: isize) -> ProblemInput {
    let paths: Vec<Vec<(isize, isize)>> = input
        .iter()
        .map(|row| {
            row.split(" -> ")
                .map(|c| {
                    c.split(',')
                        .map(|s| s.parse().unwrap())
                        .tuples()
                        .next()
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let max_h = *paths.iter().flatten().map(|(_, h)| h).max().unwrap();
    let min_col = paths.iter().flatten().map(|(c, _)| c).min().unwrap();
    let max_col = paths.iter().flatten().map(|(c, _)| c).max().unwrap();

    let min_col = min(*min_col, base_col_offset - (height_offset + max_h));
    let max_col = max(*max_col, base_col_offset + (height_offset + max_h));

    let mut map: Map = Vec::new();
    map.resize((max_col - min_col + 1) as usize, BTreeSet::default());

    for path in paths {
        for pair in path.windows(2) {
            let [mut start, mut end] = pair else { panic!() };
            start.0 -= min_col;
            end.0 -= min_col;

            let diff: (isize, isize) = (end.0.cmp(&start.0) as isize, end.1.cmp(&start.1) as isize);
            let mut curr = start;

            loop {
                let col: &mut _ = &mut map[curr.0 as usize];
                col.insert(curr.1);

                if curr == end {
                    break;
                }
                curr.0 += diff.0;
                curr.1 += diff.1;
            }
        }
    }

    ProblemInput {
        map,
        col_offset: base_col_offset - min_col,
        row_range: 0..=max_h,
    }
}

struct ProblemInput {
    map: Map,
    col_offset: isize,
    row_range: RangeInclusive<isize>,
}
