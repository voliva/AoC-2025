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
        Ok(input
            .into_iter()
            .cloned()
            .map(|(start, end)| get_id(start, end))
            .sum())
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
    let period = len / 2;
    let pow = (10u64).pow(period);

    let (start_v, _) = simplify(start, period, true);
    let (end_v, _) = simplify(end, period, false);

    let res = {
        let half_sum: u64 = (start_v..=end_v).sum();
        half_sum + half_sum * pow
    };

    res
}

fn get_id(start: u64, end: u64) -> u64 {
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

            get_id_sl(range_start, range_end)
        })
        .sum()
}

fn get_id_sl(start: u64, end: u64) -> u64 {
    let len = get_len(start);

    let mut res = 0;

    for period in 1..=len / 2 {
        if len % period != 0 {
            continue;
        }
        let pow = (10u64).pow(period);

        let (start_v, repeats) = simplify(start, period, true);
        let (end_v, _) = simplify(end, period, false);

        res += {
            let partial_sum: u64 = (start_v..=end_v).sum();
            let repeated = (0..repeats)
                .map(|_| partial_sum)
                .reduce(|a, b| a * pow + b)
                .unwrap();
            repeated
        };
    }

    res
}

fn simplify(mut value: u64, period: u32, start: bool) -> (u64, usize) {
    let pow = (10u64).pow(period);
    let mut lead = 0u64;
    let mut min = u64::MAX;
    let mut max = 0u64;

    let mut repeats = 0usize;

    while value > 0 {
        lead = value % pow;
        min = lead.min(min);
        max = lead.max(max);

        value /= pow;
        repeats += 1;
    }

    if start && lead < max {
        lead += 1;
    } else if !start && lead > min {
        lead -= 1;
    }

    (lead, repeats)
}

// > 31043429875
