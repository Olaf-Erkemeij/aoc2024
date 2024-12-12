type Input = Vec<Vec<i32>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Invalid number"))
                .collect()
        })
        .collect()
}

fn is_safe_row(row: &[i32]) -> bool {
    let diffs: Vec<i32> = row.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let all_positive = diffs.iter().all(|&x| x > 0 && x < 4);
    let all_negative = diffs.iter().all(|&x| x < 0 && x > -4);

    all_positive || all_negative
}

pub fn solve_part1(input: &str) -> String {
    let data: Input = parse_input(input);

    let safe = data.iter().filter(|row| is_safe_row(row)).count();

    safe.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let data: Input = parse_input(input);

    let safe = data
        .iter()
        .filter(|row| {
            if is_safe_row(row) {
                return true;
            }

            // Check if removing any single element makes the row safe
            (0..row.len()).any(|i| {
                let subset: Vec<_> = row
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &x)| x)
                    .collect();
                is_safe_row(&subset)
            })
        })
        .count();

    safe.to_string()
}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
