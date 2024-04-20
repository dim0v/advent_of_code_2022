use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::{Arc, mpsc};

use anyhow::anyhow;

use crate::solutions::common::RangeSet;
use crate::Stage;

pub fn solve(stage: Stage, input: &str) -> String {
    let (sensors, beacons_src): (Vec<_>, Vec<_>) = input.lines().map(parse_row).unzip();
    let mut beacons: Vec<Point2> = Vec::with_capacity(beacons_src.len());

    for b in beacons_src {
        if !beacons.contains(&b) {
            beacons.push(b);
        }
    }

    match stage {
        Stage::Easy => solve_easy(&sensors, &beacons),
        Stage::Hard => solve_hard(&sensors),
    }
    .to_string()
}

fn solve_easy(sensors: &[Sensor], beacons: &[Point2]) -> usize {
    let mut range_set = RangeSet::with_capacity(sensors.len());

    const TARGET_ROW: isize = 2000000;

    fill_range_set_for_row(&mut range_set, sensors, TARGET_ROW);

    let cnt: usize = range_set.ranges().iter().map(|r| r.count()).sum();
    let intersecting_beacons = beacons
        .iter()
        .filter(|b| b.y == TARGET_ROW && range_set.contains(b.x))
        .count();

    cnt - intersecting_beacons
}

fn solve_hard(sensors: &[Sensor]) -> usize {
    const POS_MAX: isize = 4000000;
    const X_MUL: isize = 4000000;
    let n_threads = thread::available_parallelism().unwrap().get();
    
    let (tx, rx) = mpsc::channel();
    let found = Arc::new(AtomicBool::new(false));

    let _threads: Vec<_> = (0..n_threads)
        .map(|i| {
            let sensors: Vec<_> = sensors.iter().map(|x| x.clone()).collect();
            let tx = tx.clone();
            let found = Arc::clone(&found);
            thread::spawn(move || {
                let mut range_set = RangeSet::with_capacity(sensors.len());

                for row in (0isize..=POS_MAX - i as isize).rev().step_by(n_threads) {
                    if found.load(Ordering::Acquire) { return }

                    fill_range_set_for_row(&mut range_set, &sensors, row);
                    for range in range_set.ranges() {
                        if range.to >= 0 && range.to < POS_MAX {
                            tx.send(X_MUL * (range.to + 1) + row).unwrap();
                            found.store(true, Ordering::Release)
                        }
                    }
                }
            })
        })
        .collect();

    rx.recv().unwrap() as usize
}

fn fill_range_set_for_row(range_set: &mut RangeSet, sensors: &[Sensor], row: isize) {
    range_set.clear();

    for sensor in sensors {
        let row_diff = (sensor.pos.y - row).abs();
        let row_range = sensor.range - row_diff;

        if row_range >= 0 {
            range_set.insert((sensor.pos.x - row_range, sensor.pos.x + row_range).into())
        }
    }
}

fn parse_row(row: &str) -> (Sensor, Point2) {
    let mut split = row.split(": ");
    let sensor_pos = split.next().unwrap()[10..].parse().unwrap();
    let beacon_pos = split.next().unwrap()[21..].parse().unwrap();

    (
        Sensor {
            pos: sensor_pos,
            range: sensor_pos.manhattan_dist(&beacon_pos),
        },
        beacon_pos,
    )
}

#[derive(Debug, Copy, Clone, Default)]
struct Sensor {
    pos: Point2,
    range: isize,
}

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point2 {
    x: isize,
    y: isize,
}

impl Point2 {
    fn manhattan_dist(&self, other: &Self) -> isize {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

impl FromStr for Point2 {
    type Err = anyhow::Error;

    // assumes "x=..., y=..." string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(", ").map(|s| s[2..].parse());

        Ok(Point2 {
            x: iter.next().ok_or(anyhow!("can't find x"))??,
            y: iter.next().ok_or(anyhow!("can't find y"))??,
        })
    }
}
