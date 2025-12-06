use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

// 22:46 22:54 23:00

impl Solver for Problem {
    type Input = Vec<Vec<bool>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.chars().map(|v| v == '@').collect_vec())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut res = 0;

        for r in 0..input.len() {
            for c in 0..input[r].len() {
                if !input[r][c] {
                    continue;
                };

                if can_remove(input, (r, c)) {
                    res += 1;
                }
            }
        }

        Ok(res)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut res = 0;
        let mut removed = true;

        let mut field = input.clone();

        while removed {
            removed = false;

            for r in 0..field.len() {
                for c in 0..field[r].len() {
                    if !field[r][c] {
                        continue;
                    };

                    if can_remove(&field, (r, c)) {
                        res += 1;
                        field[r][c] = false;
                        removed = true;
                    }
                }
            }
        }

        Ok(res)
    }
}

fn can_remove(field: &Vec<Vec<bool>>, (r, c): (usize, usize)) -> bool {
    let mut count = 0;

    for tr in (r as isize) - 1..=(r as isize) + 1 {
        if tr < 0 || tr >= field.len() as isize {
            continue;
        }
        for tc in (c as isize) - 1..=(c as isize) + 1 {
            if tc < 0 || tc >= field[tr as usize].len() as isize {
                continue;
            }

            if field[tr as usize][tc as usize] {
                count += 1;
            }
        }
    }

    count <= 4
}
