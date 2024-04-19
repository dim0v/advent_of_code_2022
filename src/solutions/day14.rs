use std::cmp::{max, min};
use std::ops::RangeInclusive;

use crate::solutions::common::{MyRange, RangeSet};
use crate::Stage;

type Map = Vec<RangeSet>;

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
        col.insert(target_h.into());
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

            let maybe_stop = get_next_stop(col, spawn_height);

            if let None = maybe_stop {
                // no ground below, time to stop
                break 'outer;
            }

            stop = maybe_stop.unwrap();
            spawn_height = stop;
            if is_empty(&map, spawn_col - 1, spawn_height) {
                spawn_col -= 1;
            } else if is_empty(&map, spawn_col + 1, spawn_height) {
                spawn_col += 1;
            } else {
                break;
            }
        }

        map[spawn_col as usize].insert((stop - 1).into());

        cnt += 1
    }

    fn is_empty(map: &Map, col: isize, height: isize) -> bool {
        if col < 0 || col >= map.len() as isize {
            return true;
        }
        let ranges = map[col as usize].ranges();
        for range in ranges {
            if range.from <= height {
                if range.to >= height {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }

    fn get_next_stop(col: &RangeSet, start: isize) -> Option<isize> {
        for r in col.ranges() {
            if r.from > start {
                return Some(r.from);
            }
        }

        None
    }

    cnt
}

fn load_base_map(input: &[&str], base_col_offset: isize, height_offset: isize) -> ProblemInput {
    let paths: Vec<Vec<MyRange>> = input
        .iter()
        .map(|row| {
            row.split(" -> ")
                .map(|c| {
                    let mut split = c.split(',').map(|s| s.parse().unwrap());
                    
                    MyRange::from((split.next().unwrap(), split.next().unwrap()))
                })
                .collect()
        })
        .collect();

    let max_h = paths.iter().flatten().map(|r| r.to).max().unwrap();
    let min_col = paths.iter().flatten().map(|r| r.from).min().unwrap();
    let max_col = paths.iter().flatten().map(|r| r.from).max().unwrap();

    let min_col = min(min_col, base_col_offset - (height_offset + max_h));
    let max_col = max(max_col, base_col_offset + (height_offset + max_h));

    let capacity = (max_col - min_col + 1) as usize;
    let mut map: Map = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        map.push(RangeSet::with_capacity(4));
    }

    for path in paths {
        for pair in path.windows(2) {
            let [mut start, mut end] = pair else { panic!() };
            start.from -= min_col;
            end.from -= min_col;

            let diff = (end.from.cmp(&start.from) as isize, end.to.cmp(&start.to) as isize);
            let mut curr = start;

            if diff.0 == 0 {
                let col: &mut _ = &mut map[curr.from as usize];
                col.insert((start.to, end.to).into());
            } else {
                loop {
                    let col: &mut _ = &mut map[curr.from as usize];
                    col.insert(curr.to.into());

                    if curr == end {
                        break;
                    }
                    curr.from += diff.0;
                    curr.to += diff.1;
                }
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
