use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

// 23:29 23:42

impl Solver for Problem {
    type Input = Vec<PProblem>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap());

        let mut parsed_values: Vec<Vec<usize>> = vec![];

        let mut problems = vec![];

        for l in lines {
            if l.starts_with("+") || l.starts_with("*") {
                let operations = l
                    .split(" ")
                    .filter(|v| v.len() > 0)
                    .map(|v| if v == "+" { Operand::Add } else { Operand::Mul })
                    .collect_vec();

                for i in 0..operations.len() {
                    problems.push(PProblem {
                        values: parsed_values.iter().map(|v| v[i]).collect_vec(),
                        operation: operations[i].clone(),
                    });
                }
            } else {
                parsed_values.push(
                    l.split(" ")
                        .filter(|v| v.len() > 0)
                        .map(|v| v.parse().unwrap())
                        .collect_vec(),
                );
            }
        }

        problems
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.into_iter().map(|p| p.solve()).sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
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
