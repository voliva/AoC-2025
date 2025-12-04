use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<isize>;
    type Output1 = usize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let v: isize = line[1..].parse().unwrap();
                return if line.chars().next().unwrap() == 'L' {
                    -v
                } else {
                    v
                };
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .into_iter()
            .scan(50, |a, b| {
                let result = (*a + *b).rem_euclid(100);
                *a = result;
                Some(result)
            })
            .filter(|v| *v == 0)
            .count())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .into_iter()
            .scan(50, |a, b| {
                let start_value = *a;
                let mut times = b.abs() / 100;
                let b = b % 100;

                let sum = *a + b;
                let has_crossed = start_value > 0 && sum < 0 || sum > 100;
                if has_crossed {
                    times += 1;
                }

                *a = if sum < 0 {
                    sum + 100
                } else if sum >= 100 {
                    sum - 100
                } else {
                    sum
                };

                if *a == 0 {
                    times += 1;
                }

                Some(times)
            })
            .reduce(|a, b| a + b)
            .unwrap())
    }
}
