use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

use itertools::Itertools;

type Input = (HashMap<char, Vec<Point>>, Point);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn in_bound(&self, bound: &Point) -> bool {
        (0..bound.x).contains(&self.x) && (0..bound.y).contains(&self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn parse_input(input: &str) -> Input {
    let bounds = Point::new(
        input.lines().next().unwrap().len() as i64,
        input.lines().count() as i64,
    );

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                (c != '.').then(|| (c, Point::new(x as i64, y as i64)))
            })
        })
        .into_group_map();

    (antennas, bounds)
}

pub fn solve_part1(input: &str) -> String {
    // Solve part 1
    let (antennas, bounds) = parse_input(input);

    antennas
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .combinations(2)
                .flat_map(|pair| {
                    let antenna1 = *pair[0];
                    let antenna2 = *pair[1];
                    let diff = antenna2 - antenna1;

                    vec![
                        antenna2 + diff,
                        antenna1 - diff
                    ]
                        .into_iter()
                        .filter(|antinode| antinode.in_bound(&bounds))
                })
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (antennas, bounds) = parse_input(input);

    antennas
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .combinations(2)
                .flat_map(|pair| {
                    let antenna1 = *pair[0];
                    let antenna2 = *pair[1];
                    let diff = antenna2 - antenna1;

                    let mut antinodes = vec![];
                    let mut antinode = antenna2;

                    while antinode.in_bound(&bounds) {
                        antinodes.push(antinode);
                        antinode = antinode + diff;
                    }

                    let mut antinode = antenna1;

                    while antinode.in_bound(&bounds) {
                        antinodes.push(antinode);
                        antinode = antinode - diff;
                    }

                    antinodes
                })
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
