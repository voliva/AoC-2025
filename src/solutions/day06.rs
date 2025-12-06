use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

// 23:29 23:42 23:56

impl Solver for Problem {
    type Input = Vec<PProblem>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let mut transposed_lines: Vec<String> = vec![];
        for c in 0..lines[0].len() {
            transposed_lines.push(lines.iter().map(|l| &l[c..=c]).collect());
        }

        let mut problems = vec![];
        let mut i = 0;
        while i < transposed_lines.len() {
            let line = &transposed_lines[i];
            i += 1;

            if line.len() == 0 {
                continue;
            }
            if line.ends_with("+") || line.ends_with("*") {
                let mut problem = PProblem {
                    operation: if line.ends_with("+") {
                        Operand::Add
                    } else {
                        Operand::Mul
                    },
                    values: vec![line[0..line.len() - 1].trim().parse().unwrap()],
                };

                while i < transposed_lines.len() && transposed_lines[i].trim().len() != 0 {
                    problem
                        .values
                        .push(transposed_lines[i].trim().parse().unwrap());
                    i += 1;
                }

                problems.push(problem);
            }
        }

        problems
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.into_iter().map(|p| p.solve()).sum())
    }

    fn solve_second(&self, _input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}

#[derive(Clone, Copy)]
enum Operand {
    Add,
    Mul,
}

impl Operand {
    fn apply(self, a: usize, b: usize) -> usize {
        match self {
            Operand::Add => a + b,
            Operand::Mul => a * b,
        }
    }
}

#[derive(Clone)]
pub struct PProblem {
    values: Vec<usize>,
    operation: Operand,
}

impl PProblem {
    fn solve(&self) -> usize {
        self.values
            .iter()
            .cloned()
            .reduce(|a, b| self.operation.apply(a, b))
            .unwrap()
    }
}
