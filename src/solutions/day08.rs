use itertools::Itertools;

use crate::many_to_many::ManyToMany;

use super::Solver;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

// 7:35 8:27 8:38 (hacks lol)

impl Solver for Problem {
    type Input = Vec<Coordinate>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| Coordinate {
                coord: line.split(",").map(|v| v.parse().unwrap()).collect_vec(),
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut connections: ManyToMany<&Coordinate, &Coordinate> = ManyToMany::new();
        let mut distances = vec![];

        for i in 0..input.len() {
            let coord1 = &input[i];
            for j in (i + 1)..input.len() {
                let coord2 = &input[j];
                let distance = coord1.distance(coord2);
                distances.push((distance, coord1, coord2));
            }
        }

        distances.sort_by(|a, b| a.0.cmp(&b.0));

        for i in 0..1000 {
            let (_, coord1, coord2) = distances[i];
            connections.insert(coord1, coord2);
            connections.insert(coord2, coord1);
        }

        let mut circuits = get_circuits(input, &connections);
        circuits.sort_by(|a, b| b.len().cmp(&a.len()));

        let res = circuits
            .iter()
            .take(3)
            .map(|v| v.len())
            .reduce(|a, b| a * b)
            .unwrap();

        Ok(res)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut connections: ManyToMany<&Coordinate, &Coordinate> = ManyToMany::new();
        let mut distances = vec![];

        for i in 0..input.len() {
            let coord1 = &input[i];
            for j in (i + 1)..input.len() {
                let coord2 = &input[j];
                let distance = coord1.distance(coord2);
                distances.push((distance, coord1, coord2));
            }
        }

        distances.sort_by(|a, b| a.0.cmp(&b.0));

        let mut next_i = 5205;
        for i in 0..next_i {
            let (_, coord1, coord2) = distances[i];
            connections.insert(coord1, coord2);
            connections.insert(coord2, coord1);
        }

        while get_circuits(input, &connections).len() > 1 {
            let (_, coord1, coord2) = distances[next_i];
            next_i += 1;
            connections.insert(coord1, coord2);
            connections.insert(coord2, coord1);
            println!("{next_i}");
        }

        let (_, coord1, coord2) = distances[next_i - 1];
        Ok(coord1.coord[0] * coord2.coord[0])
    }
}

fn get_circuits<'a>(
    input: &'a Vec<Coordinate>,
    connections: &ManyToMany<&'a Coordinate, &'a Coordinate>,
) -> Vec<HashSet<&'a Coordinate>> {
    let mut circuits = input.len();
    let mut coord_to_circuit: HashMap<&Coordinate, usize> = HashMap::new();
    for (i, coord) in input.iter().enumerate() {
        coord_to_circuit.insert(coord, i);
    }
    for coord in input {
        let mut connected = HashSet::new();
        let mut to_explore = VecDeque::new();
        to_explore.push_front(coord);
        while let Some(coord) = to_explore.pop_back() {
            if connected.contains(coord) {
                // println!("already added");
                continue;
            }
            connected.insert(coord);
            for other in connections.inner(&coord).unwrap_or(&HashSet::new()) {
                to_explore.push_front(&other);
            }
        }

        let existing_circuits: HashSet<_> = connected
            .iter()
            .filter_map(|v| coord_to_circuit.get(v))
            .cloned()
            .collect();
        if existing_circuits.len() == 0 {
            panic!("no existing circuits??");
        } else {
            let circuit = *existing_circuits.iter().next().unwrap();
            circuits = circuits - existing_circuits.len() + 1;
            for coord in connected {
                coord_to_circuit.insert(coord, circuit);
            }
        }
    }

    let mut circuits = ManyToMany::new();
    for (coord, circuit) in coord_to_circuit {
        circuits.insert(circuit, coord);
    }

    circuits.outer().values().into_iter().cloned().collect_vec()
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
pub struct Coordinate {
    coord: Vec<usize>,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> usize {
        self.coord
            .iter()
            .enumerate()
            .map(|(i, v)| {
                ((*v as isize) - (other.coord[i] as isize))
                    * ((*v as isize) - (other.coord[i] as isize))
            })
            .map(|v| v as usize)
            .sum()
    }
}
