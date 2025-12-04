use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(u64, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .flat_map(|line| {
                line.split(',')
                    .map(|v| {
                        v.split('-')
                            .map(|v| v.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .into_iter()
            .cloned()
            .map(|(start, end)| get_count(start, end))
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}

fn get_len(mut value: u64) -> u32 {
    let mut r = 0;
    while value > 0 {
        value /= 10;
        r += 1;
    }
    r
}

fn get_count(start: u64, end: u64) -> u64 {
    let start_len = get_len(start);
    let end_len = get_len(end);

    (start_len..=end_len)
        .map(|len| {
            let range_start = if len == start_len {
                start
            } else {
                (10u64).pow(len - 1)
            };
            let range_end = if len == end_len {
                end
            } else {
                (10u64).pow(len) - 1
            };

            get_count_sl(range_start, range_end)
        })
        .sum()
}

fn get_count_sl(start: u64, end: u64) -> u64 {
    let len = get_len(start);
    if len % 2 == 1 {
        return 0;
    }
    let pow = (10u64).pow(len / 2);

    let start_h = start / pow;
    let start_l = start % pow;
    let start_v = start_h + if start_l > start_h { 1 } else { 0 };

    let end_h = end / pow;
    let end_l = end % pow;
    let end_v = end_h - if end_l < end_h { 1 } else { 0 };

    let res = if end_v >= start_v {
        let half_sum: u64 = (start_v..=end_v).sum();
        half_sum + half_sum * pow
    } else {
        0
    };

    res
}

// more than 19295890182
