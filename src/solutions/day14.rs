use std::cmp::{max, min};
use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::Stage;

type Map = Vec<RangeSet>;
type MyRange = (isize, isize);

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

fn sim_sand_fall(map: &mut Map, spawn_point: MyRange) -> isize {
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

        map[spawn_col as usize].insert(stop - 1);

        cnt += 1
    }

    fn is_empty(map: &Map, col: isize, height: isize) -> bool {
        if col < 0 || col >= map.len() as isize {
            return true;
        }
        let ranges = &map[col as usize].ranges;
        for range in ranges {
            if range.0 <= height {
                if range.1 >= height {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }

    fn get_next_stop(col: &RangeSet, start: isize) -> Option<isize> {
        for (s, _) in &col.ranges {
            if *s > start {
                return Some(*s);
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

    let capacity = (max_col - min_col + 1) as usize;
    let mut map: Map = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        map.push(RangeSet::new());
    }

    for path in paths {
        for pair in path.windows(2) {
            let [mut start, mut end] = pair else { panic!() };
            start.0 -= min_col;
            end.0 -= min_col;

            let diff: MyRange = (end.0.cmp(&start.0) as isize, end.1.cmp(&start.1) as isize);
            let mut curr = start;

            if diff.0 == 0 {
                let col: &mut _ = &mut map[curr.0 as usize];
                col.insert_range((start.1, end.1));
            } else {
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

struct RangeSet {
    ranges: Vec<MyRange>,
}

impl RangeSet {
    fn new() -> RangeSet {
        RangeSet { ranges: Vec::new() }
    }

    fn insert(&mut self, v: isize) {
        self.insert_range((v, v));
    }

    fn get_overlaps_range(&self, range: MyRange) -> (Option<usize>, Option<usize>, usize) {
        let pos = self.ranges.binary_search(&range);
        match pos {
            Ok(pos) => (Some(pos), Some(pos), pos),
            Err(pos) => {
                let mut left: Option<usize> = None;
                let mut right: Option<usize> = None;

                if pos > 0 {
                    let prev_range = &self.ranges[pos - 1];
                    if prev_range.1 >= range.0 {
                        left = Some(pos - 1)
                    }
                }

                if pos < self.ranges.len() {
                    let next_range = &self.ranges[pos];
                    if next_range.0 <= range.1 {
                        right = Some(pos)
                    }
                }

                (left, right, pos)
            }
        }
    }

    fn insert_range(&mut self, range: MyRange) {
        if range.0 > range.1 {
            self.insert_range((range.1, range.0));
            return;
        }

        let (left, right, pos) = self.get_overlaps_range(range);

        if left.is_none() && right.is_none() {
            self.ranges.insert(pos, range);
            return;
        }
        if left.is_some() && right.is_some() {
            if left == right {
                return;
            }
            self.ranges[left.unwrap()].1 = self.ranges[right.unwrap()].1;
            self.ranges.remove(right.unwrap());
            return;
        }
        if left.is_some() {
            self.ranges[left.unwrap()].1 = max(range.1, self.ranges[left.unwrap()].1);
            return;
        }
        if right.is_some() {
            self.ranges[right.unwrap()].0 = min(range.0, self.ranges[right.unwrap()].0);
            return;
        }
    }
}
