use std::{cmp::Ordering, collections::HashSet};

type Input = (HashSet<(usize, usize)>, Vec<Vec<usize>>);

fn parse_input(input: &str) -> Input {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let orders: HashSet<(usize, usize)> = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (orders, updates)
}

pub fn solve_part1(input: &str) -> String {
    let (orders, updates) = parse_input(input);

    updates
        .iter()
        .filter(|line| line.windows(2).all(|pair| !orders.contains(&(pair[1], pair[0]))))
        .map(|line| line[line.len() / 2])
        .sum::<usize>()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (orders, updates) = parse_input(input);

    let compare = |a: &usize, b: &usize| {
        match (orders.contains(&(*b, *a)), orders.contains(&(*a, *b))) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => Ordering::Equal,
        }
    };

    updates
        .iter()
        .filter(|line| line.windows(2).any(|pair| orders.contains(&(pair[1], pair[0]))))
        .map(|line| {
            let mut sorted_line = line.clone();
            sorted_line.sort_by(compare);
            sorted_line[sorted_line.len() / 2]
        })
        .sum::<usize>()
        .to_string()

}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
