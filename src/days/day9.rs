type Input = (Vec<Vec<usize>>, Vec<Vec<usize>>);

use itertools::{Either, Itertools};

fn parse_input(input: &str) -> Input {
    let mut p = 0;
    // Get every digit that is in the input (it is one line)
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .partition_map(|(i, n)| {
            let res = (p as usize..p+n as usize).collect::<Vec<usize>>();
            p += n as usize;
            if i % 2 == 0 {
                Either::Left(res)
            } else {
                Either::Right(res)
            }
        })
        .into()
}

pub fn solve_part1(input: &str) -> String {
    let (mut lhs, rhs) = parse_input(input);

    let mut rhs = rhs.iter().flatten().collect::<Vec<_>>();

    for y in (0..lhs.len()).rev() {
        for x in (0..lhs[y].len()).rev() {
            if lhs[y][x] >= *rhs[0] {
                lhs[y][x] = *rhs[0];
                rhs = rhs[1..].to_vec();
            }
        }
    }

    lhs.iter().enumerate().map(|(i, f)| f.iter().map(|j| j * i).sum::<usize>()).sum::<usize>().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (mut lhs, mut rhs) = parse_input(input);

    for y in (0..lhs.len()).rev() {
        for x in 0..rhs.len() {
            if rhs[x].len() >= lhs[y].len() && lhs[y][0] >= rhs[x][0] {
                lhs[y] = rhs[x][..lhs[y].len()].to_vec();
                rhs[x] = rhs[x][lhs[y].len()..].to_vec();
            }
        }
    }

    lhs.iter().enumerate().map(|(i, f)| f.iter().map(|j| j * i).sum::<usize>()).sum::<usize>().to_string()
}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
