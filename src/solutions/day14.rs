use std::collections::{BTreeSet, HashMap};

use ahash::RandomState;
use itertools::Itertools;

use crate::Stage;

type Map = HashMap<isize, BTreeSet<isize>, RandomState>;

pub fn solve(stage: Stage, input: &Vec<&str>) -> String {
    let mut map = load_base_map(input);
    let col_offset = 500;

    if stage.is_hard() {
        add_floor(&mut map, col_offset);
    }

    sim_sand_fall(&mut map, (col_offset, 0)).to_string()
}

fn add_floor(map: &mut Map, offset: isize) {
    let max_h = map.values().map(|col| col.last().unwrap()).max().unwrap();
    let target_h = max_h + 2;

    for i_col in (offset - target_h)..=(offset + target_h) {
        map.entry(i_col).or_default().insert(target_h);
    }
}

fn sim_sand_fall(map: &mut Map, spawn_point: (isize, isize)) -> usize {
    let mut cnt = 0;

    'outer: loop {
        let (mut spawn_col, mut spawn_height) = spawn_point;
        if !is_empty(map, spawn_col, spawn_height) {
            break;
        }

        let mut stop: isize;
        loop {
            let maybe_col = map.get_mut(&spawn_col);
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

        map.get_mut(&spawn_col).unwrap().insert(stop - 1);

        cnt += 1
    }

    fn is_empty(map: &Map, col: isize, height: isize) -> bool {
        if let Some(col) = map.get(&col) {
            return !col.contains(&height);
        }

        true
    }

    cnt
}

fn load_base_map(input: &[&str]) -> Map {
    let mut map: Map = HashMap::with_hasher(RandomState::new());

    for row in input {
        let path = row.split(" -> ").map(|c| {
            c.split(',')
                .map(|s| s.parse::<isize>().unwrap())
                .tuples::<(isize, isize)>()
                .next()
                .unwrap()
        });
        for (start, end) in path.tuple_windows() {
            let diff: (isize, isize) = (end.0.cmp(&start.0) as isize, end.1.cmp(&start.1) as isize);
            let mut curr = start;
            loop {
                let col: &mut _ = map.entry(curr.0).or_default();
                col.insert(curr.1);

                if curr == end {
                    break;
                }
                curr.0 += diff.0;
                curr.1 += diff.1;
            }
        }
    }

    map
}
