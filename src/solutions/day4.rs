use crate::Stage;
use std::error::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn solve(stage: Stage, input: &str) -> String {
    let pairs = input.lines().map(|x| x.parse().unwrap());

    let result = pairs.filter(|x: &JobPair| match stage {
        Stage::Easy => x.is_full_overlap(),
        Stage::Hard => x.is_any_overlap()
    }).count();

    result.to_string()
}

struct JobPair {
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>,
}

impl JobPair {
    fn is_full_overlap(&self) -> bool {
        self.first.contains(self.second.start()) && self.first.contains(self.second.end())
            || self.second.contains(self.first.start()) && self.second.contains(self.first.end())
    }

    fn is_any_overlap(&self) -> bool {
        self.first.contains(self.second.start())
            || self.first.contains(self.second.end())
            || self.second.contains(self.first.start())
            || self.second.contains(self.first.end())
    }
}

impl FromStr for JobPair {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        let parse_range = |s: &str| -> Result<RangeInclusive<i32>, Self::Err> {
            let parts: Vec<&str> = s.split('-').collect();
            Ok(RangeInclusive::new(parts[0].parse()?, parts[1].parse()?))
        };

        Ok(JobPair {
            first: parse_range(parts[0])?,
            second: parse_range(parts[1])?,
        })
    }
}
