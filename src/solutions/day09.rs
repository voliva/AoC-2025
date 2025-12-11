use itertools::Itertools;
use num::integer::Roots;

use crate::coordinate::{Coordinate, Direction};

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

// 08:40 08:48

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Coordinate>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let (r, c) = line
                    .split(",")
                    .map(|v| v.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Coordinate::from_usize(r, c)
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut max = 0;

        for i in 0..input.len() {
            let coord_i = &input[i];
            for j in i + 1..input.len() {
                let coord_j = &input[j];

                let area = (coord_i.0 - coord_j.0 + 1).abs() * (coord_i.1 - coord_j.1 + 1).abs();
                max = max.max(area);
            }
        }

        Ok(max)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut prev_orientation = Direction::Down;
        let mut prev_direction = Direction::Right;

        let mut orientations = Vec::with_capacity(input.len());
        let mut shapes = Vec::with_capacity(input.len());
        for i in 0..input.len() {
            let coord = &input[i];
            let next = if i == input.len() - 1 {
                &input[0]
            } else {
                &input[i + 1]
            };

            let Coordinate(x_diff, y_diff) = next - coord;
            let direction = if x_diff > 0 {
                Direction::Right
            } else if x_diff < 0 {
                Direction::Left
            } else if y_diff > 0 {
                Direction::Down
            } else {
                Direction::Up
            };

            let orientation = if x_diff != 0 {
                match (&prev_orientation, &prev_direction) {
                    (Direction::Down | Direction::Up, _) => prev_orientation.clone(),
                    (Direction::Left, Direction::Down) | (Direction::Right, Direction::Up) => {
                        if x_diff < 0 {
                            Direction::Up
                        } else {
                            Direction::Down
                        }
                    }
                    (Direction::Left, Direction::Up) | (Direction::Right, Direction::Down) => {
                        if x_diff > 0 {
                            Direction::Up
                        } else {
                            Direction::Down
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                match (&prev_orientation, &prev_direction) {
                    (Direction::Left | Direction::Right, _) => prev_orientation.clone(),
                    (Direction::Down, Direction::Right) | (Direction::Up, Direction::Left) => {
                        if y_diff < 0 {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    }
                    (Direction::Down, Direction::Left) | (Direction::Up, Direction::Right) => {
                        if y_diff > 0 {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    }
                    _ => unreachable!(),
                }
            };

            let shape = match (&prev_orientation, &orientation, &prev_direction) {
                (a, b, _) if a == b => Shape::Flat,
                (_, a, b) if a == b => Shape::Convex,
                _ => Shape::Concave,
            };

            orientations.push(orientation.clone());
            shapes.push(shape);
            prev_orientation = orientation;
            prev_direction = direction;
        }

        let mut result = 0;

        let mut sorted = input.clone();
        for i in 0..input.len() {
            let target = &input[i];
            let target_shape = &shapes[i];
            let prev_orientation = if i == 0 {
                &orientations[orientations.len() - 1]
            } else {
                &orientations[i - 1]
            };
            let next_orientation = &orientations[i];

            sorted.sort_by(|a, b| {
                let d_a = a.distance_sq(target);
                let d_b = b.distance_sq(target);
                d_a.cmp(&d_b)
            });

            let mut quadrants_x_limits = vec![
                (isize::MAX / 2).sqrt(),
                (isize::MAX / 2).sqrt(),
                (isize::MAX / 2).sqrt(),
                (isize::MAX / 2).sqrt(),
            ];
            let mut quadrants_y_limits = vec![
                (isize::MAX / 2).sqrt(),
                (isize::MAX / 2).sqrt(),
                (isize::MAX / 2).sqrt(),
                (isize::MAX / 2).sqrt(),
            ];

            match target_shape {
                Shape::Flat => match prev_orientation {
                    Direction::Up => {
                        quadrants_y_limits[2] = 0;
                        quadrants_y_limits[3] = 0;
                    }
                    _ => {
                        quadrants_y_limits[0] = 0;
                        quadrants_y_limits[1] = 0;
                    }
                },
                Shape::Concave => {}
                _ => {}
            }

            // 0 must be self, skip that
            for j in 1..sorted.len() {
                let other = &sorted[j];

                let max_x_limit = quadrants_x_limits.iter().map(|v| v.abs()).max().unwrap();
                let max_y_limit = quadrants_y_limits.iter().map(|v| v.abs()).max().unwrap();
                if target.distance_sq(other) > max_x_limit * max_x_limit + max_y_limit * max_y_limit
                {
                    break;
                }

                let Coordinate(diff_x, diff_y) = other - target;
                let quadrant = if diff_y < 0 {
                    if diff_x < 0 { 0 } else { 1 }
                } else {
                    if diff_x < 0 { 3 } else { 2 }
                };

                if quadrants_x_limits[quadrant] < diff_x.abs()
                    || quadrants_y_limits[quadrant] < diff_y.abs()
                {
                    continue;
                }

                result = result.max((diff_x.abs() + 1) * (diff_y.abs() + 1));
            }
        }

        Ok(result)
    }
}

enum Shape {
    Concave,
    Convex,
    Flat,
}
