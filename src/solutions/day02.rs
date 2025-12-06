use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
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

    get_id_period(start, end, period, &mut HashSet::new())
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

    let mut seen_values = HashSet::new();
    for period in 1..=len / 2 {
        if len % period != 0 {
            continue;
        }
        res += get_id_period(start, end, period, &mut seen_values);
    }

    res
}

fn get_id_period(start: u64, end: u64, period: u32, seen_values: &mut HashSet<u64>) -> u64 {
    let pow = (10u64).pow(period);

    let (start_v, repeats) = simplify(start, period, true);
    let (end_v, _) = simplify(end, period, false);

    let mut res = 0;
    for v in start_v..=end_v {
        let value = repeat(v, pow, repeats);
        // println!("s={start} e={end} p={period} r={repeats} v={v}->{value}");
        if !seen_values.contains(&value) {
            seen_values.insert(value);
            res += value;
        }
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

fn repeat(value: u64, pow: u64, repeats: usize) -> u64 {
    let mut res = 0;
    for _ in 0..repeats {
        res = res * pow + value;
    }
    res
}

// larger than 31043429875
