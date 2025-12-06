use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

// 22:30 22:38 22:44

impl Solver for Problem {
    type Input = Vec<Vec<usize>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                line.split("")
                    .filter(|v| v.len() > 0)
                    .map(|v| v.parse().unwrap())
                    .collect_vec()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .into_iter()
            .map(|bank| find_max_joltage(bank, 2))
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .into_iter()
            .map(|bank| find_max_joltage(bank, 12))
            .sum())
    }
}

fn find_max_joltage(values: &[usize], digits: usize) -> usize {
    let (max_idx, max_dec) = values[0..values.len() - digits + 1]
        .iter()
        .enumerate()
        .reduce(|max, v| if v.1 > max.1 { v } else { max })
        .unwrap();
    if digits == 1 {
        return *max_dec;
    }

    let rest = find_max_joltage(&values[max_idx + 1..], digits - 1);
    max_dec * (10usize).pow((digits as u32) - 1) + rest
}
