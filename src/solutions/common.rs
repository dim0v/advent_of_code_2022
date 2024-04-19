use crate::solutions::common::RangeSetInsertPosition::{Insert, Replace};
use std::cmp::{max, min};
use std::ops::RangeInclusive;

#[derive(Ord, PartialOrd, Eq, PartialEq, Default, Debug, Hash, Clone, Copy)]
pub struct MyRange {
    pub(crate) from: isize,
    pub(crate) to: isize,
}

impl MyRange {
    pub(crate) fn new(from: isize, to: isize) -> Self {
        Self { from, to }
    }

    pub(crate) fn count(&self) -> usize {
        (self.to - self.from + 1) as usize
    }

    pub(crate) fn contains(&self, v: isize) -> bool {
        self.from <= v && self.to >= v
    }
}

impl From<(isize, isize)> for MyRange {
    fn from((from, to): (isize, isize)) -> Self {
        Self { from, to }
    }
}

impl From<isize> for MyRange {
    fn from(v: isize) -> Self {
        Self { from: v, to: v }
    }
}

pub struct RangeSet {
    ranges: Vec<MyRange>,
}

impl RangeSet {
    pub(crate) fn clear(&mut self) {
        self.ranges.clear()
    }

    pub fn with_capacity(capacity: usize) -> RangeSet {
        RangeSet {
            ranges: Vec::with_capacity(capacity),
        }
    }

    pub fn ranges(&self) -> &[MyRange] {
        &self.ranges
    }

    pub fn contains(&self, value: isize) -> bool {
        let pos = self.ranges.binary_search(&value.into());

        match pos {
            Ok(_) => true,
            Err(pos) => {
                pos > 0 && self.ranges[pos - 1].to >= value
                    || pos < self.ranges.len() && self.ranges[pos].from <= value
            }
        }
    }

    pub fn insert(&mut self, range: MyRange) {
        if range.from > range.to {
            self.insert(MyRange::new(range.to, range.from));
            return;
        }

        match self.get_insert_position(range, true) {
            Replace(replace_range) => {
                let upper_bound = match self.ranges.get(*replace_range.end()) {
                    None => range.to,
                    Some(last) => max(range.to, last.to),
                };

                let first = &mut self.ranges[*replace_range.start()];

                first.from = min(range.from, first.from);
                first.to = upper_bound;

                self.ranges
                    .drain((replace_range.start() + 1)..=*replace_range.end());
            }
            Insert(pos) => self.ranges.insert(pos, range),
        }
    }
}

impl RangeSet {
    fn get_insert_position(
        &self,
        range: MyRange,
        include_touching: bool,
    ) -> RangeSetInsertPosition {
        let range = if include_touching {
            MyRange::new(range.from - 1, range.to + 1)
        } else {
            range
        };

        let (mut idx_left, overlap_left, _) = get_index(self, range.from);
        let (mut idx_right, _, overlap_right) = get_index(self, range.to);

        if idx_left == idx_right && !overlap_left && !overlap_right {
            return Insert(idx_left);
        }

        if overlap_left {
            idx_left -= 1;
        }
        if !overlap_right {
            idx_right -= 1;
        }

        return Replace(idx_left..=idx_right);

        fn get_index(rs: &RangeSet, value: isize) -> (usize, bool, bool) {
            let idx_res = rs.ranges.binary_search(&value.into());
            match idx_res {
                Ok(pos) => (pos, false, true),
                Err(pos) => {
                    let overlap_prev = pos
                        .checked_sub(1)
                        .map(|pos| {
                            rs.ranges
                                .get(pos)
                                .map(|r| r.contains(value))
                                .unwrap_or(false)
                        })
                        .unwrap_or(false);
                    let overlap_next = rs
                        .ranges
                        .get(pos)
                        .map(|r| r.contains(value))
                        .unwrap_or(false);

                    (pos, overlap_prev, overlap_next)
                }
            }
        }
    }
}

enum RangeSetInsertPosition {
    Replace(RangeInclusive<usize>),
    Insert(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_set_insert_empty() {
        let mut rs = RangeSet::with_capacity(1);

        rs.insert(MyRange::new(0, 2));
        assert_eq!(rs.ranges.len(), 1);
        assert_eq!(rs.ranges[0], MyRange { from: 0, to: 2 });
    }

    #[test]
    fn range_set_insert_left() {
        let mut rs = RangeSet::with_capacity(2);

        rs.insert(MyRange::new(0, 2));
        rs.insert(MyRange::new(-5, -4));
        assert_eq!(rs.ranges.len(), 2);
        assert_eq!(rs.ranges[0], MyRange { from: -5, to: -4 });
        assert_eq!(rs.ranges[1], MyRange { from: 0, to: 2 });
    }

    #[test]
    fn range_set_insert_right() {
        let mut rs = RangeSet::with_capacity(2);

        rs.insert(MyRange::new(0, 2));
        rs.insert(MyRange::new(9, 10));
        assert_eq!(rs.ranges.len(), 2);
        assert_eq!(rs.ranges[0], MyRange { from: 0, to: 2 });
        assert_eq!(rs.ranges[1], MyRange { from: 9, to: 10 });
    }

    #[test]
    fn range_set_insert_between() {
        let mut rs = RangeSet::with_capacity(2);

        rs.insert(MyRange::new(0, 1));
        rs.insert(MyRange::new(9, 10));
        rs.insert(MyRange::new(3, 3));
        assert_eq!(rs.ranges.len(), 3);
        assert_eq!(rs.ranges[0], MyRange { from: 0, to: 1 });
        assert_eq!(rs.ranges[1], MyRange { from: 3, to: 3 });
        assert_eq!(rs.ranges[2], MyRange { from: 9, to: 10 });
    }

    #[test]
    fn range_set_extend_left() {
        let mut rs = RangeSet::with_capacity(2);

        rs.insert(MyRange::new(0, 1));
        rs.insert(MyRange::new(4, 5));
        rs.insert(MyRange::new(3, 3));
        assert_eq!(rs.ranges.len(), 2);
        assert_eq!(rs.ranges[0], MyRange { from: 0, to: 1 });
        assert_eq!(rs.ranges[1], MyRange { from: 3, to: 5 });
    }

    #[test]
    fn range_set_extend_right() {
        let mut rs = RangeSet::with_capacity(2);

        rs.insert(MyRange::new(0, 1));
        rs.insert(MyRange::new(4, 5));
        rs.insert(MyRange::new(2, 2));
        assert_eq!(rs.ranges.len(), 2);
        assert_eq!(rs.ranges[0], MyRange { from: 0, to: 2 });
        assert_eq!(rs.ranges[1], MyRange { from: 4, to: 5 });
    }

    #[test]
    fn range_set_extend_merge() {
        let mut rs = RangeSet::with_capacity(2);

        rs.insert(MyRange::new(0, 1));
        rs.insert(MyRange::new(4, 5));
        rs.insert(MyRange::new(2, 3));
        assert_eq!(rs.ranges.len(), 1);
        assert_eq!(rs.ranges[0], MyRange { from: 0, to: 5 });
    }

    #[test]
    fn range_set_include_left() {
        let mut rs = RangeSet::with_capacity(2);

        rs.insert(MyRange::new(0, 1));
        rs.insert(MyRange::new(9, 10));
        rs.insert(MyRange::new(5, 5));
        assert_eq!(rs.ranges.len(), 3);

        rs.insert(MyRange::new(-5, 8));
        assert_eq!(rs.ranges.len(), 1);
        assert_eq!(rs.ranges[0], MyRange { from: -5, to: 10 });
    }

    #[test]
    fn range_set_include_right() {
        let mut rs = RangeSet::with_capacity(2);

        rs.insert(MyRange::new(0, 3));
        rs.insert(MyRange::new(9, 10));
        rs.insert(MyRange::new(5, 5));
        assert_eq!(rs.ranges.len(), 3);

        rs.insert(MyRange::new(4, 18));
        assert_eq!(rs.ranges.len(), 1);
        assert_eq!(rs.ranges[0], MyRange { from: 0, to: 18 });
    }
}
