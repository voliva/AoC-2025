use itertools::Itertools;

use super::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

// 08:26 08:36 08:43

impl Solver for Problem {
    type Input = (usize, Vec<HashSet<usize>>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let (start, _) = lines[0].chars().find_position(|v| *v == 'S').unwrap();
        let manifolds = lines[1..]
            .iter()
            .map(|line| {
                line.char_indices()
                    .filter(|(_, v)| *v == '^')
                    .map(|(p, _)| p)
                    .collect()
            })
            .collect_vec();

        (start, manifolds)
    }

    fn solve_first(&self, (start, manifold_groups): &Self::Input) -> Result<Self::Output1, String> {
        let mut beams = HashSet::new();
        let mut result = 0;
        beams.insert(*start);

        for manifolds in manifold_groups {
            if manifolds.len() == 0 {
                continue;
            }
            let mut new_beams = HashSet::new();

            for beam in beams {
                if manifolds.contains(&beam) {
                    result += 1;
                    new_beams.insert(beam - 1);
                    new_beams.insert(beam + 1);
                } else {
                    new_beams.insert(beam);
                }
            }
            beams = new_beams;
        }

        Ok(result)
    }

    fn solve_second(
        &self,
        (start, manifold_groups): &Self::Input,
    ) -> Result<Self::Output2, String> {
        let mut beams = HashMap::new();
        let mut result = 0;
        beams.insert(*start, 1);

        for manifolds in manifold_groups {
            if manifolds.len() == 0 {
                continue;
            }
            let mut new_beams = HashMap::new();
            let mut incr = |p: usize, amount: usize| {
                let value = new_beams.get(&p).map(|v| v + amount).unwrap_or(amount);
                new_beams.insert(p, value);
            };

            for (beam, amount) in beams {
                if manifolds.contains(&beam) {
                    result += 1;
                    incr(beam - 1, amount);
                    incr(beam + 1, amount);
                } else {
                    incr(beam, amount);
                }
            }
            beams = new_beams;
        }

        Ok(beams.values().sum())
    }
}
