use itertools::Itertools;

use super::Solver;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

// 23:03 23:10 23:25

impl Solver for Problem {
    type Input = (Vec<Range>, Vec<usize>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let mut ranges = vec![];
        let mut ids = vec![];

        let mut i = 0;
        while lines[i] != "" {
            ranges.push(Range(
                lines[i]
                    .split("-")
                    .map(|v| v.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            ));
            i += 1;
        }

        i += 1;
        while i < lines.len() {
            ids.push(lines[i].parse().unwrap());
            i += 1;
        }

        (ranges, ids)
    }

    fn solve_first(&self, (ranges, ids): &Self::Input) -> Result<Self::Output1, String> {
        Ok(ids
            .into_iter()
            .filter(|v| ranges.iter().any(|r| r.includes(**v)))
            .count())
    }

    fn solve_second(&self, (ranges, _): &Self::Input) -> Result<Self::Output2, String> {
        let mut sorted_ranges = ranges.into_iter().cloned().collect_vec();
        sorted_ranges.sort_by(|a, b| {
            if a.0.0 < b.0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let mut res = 0;
        let mut current_range = sorted_ranges[0].clone();
        for i in 1..sorted_ranges.len() {
            if current_range.overlaps(&sorted_ranges[i]) {
                current_range = current_range.merge(&sorted_ranges[i]);
            } else {
                res += current_range.range();
                current_range = sorted_ranges[i].clone();
            }
        }
        res += current_range.range();

        Ok(res)
    }
}

#[derive(Clone)]
pub struct Range((usize, usize));

impl Range {
    fn includes(self: &Self, value: usize) -> bool {
        let (start, end) = self.0;
        start <= value && value <= end
    }

    fn overlaps(self: &Self, other: &Self) -> bool {
        let (self_start, self_end) = self.0;
        let (other_start, other_end) = other.0;

        (self_start <= other_start && self_end >= other_start)
            || (other_start <= self_start && other_end >= self_start)
    }

    fn merge(self: &Self, other: &Self) -> Self {
        let (self_start, self_end) = self.0;
        let (other_start, other_end) = other.0;

        Range((self_start.min(other_start), self_end.max(other_end)))
    }

    fn range(self: &Self) -> usize {
        let (start, end) = self.0;
        end - start + 1
    }
}
