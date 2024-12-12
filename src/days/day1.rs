type Input = (Vec<i32>, Vec<i32>);

pub fn solve_part1(input: &str) -> String {
    let mut data: Input = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>());

            match (nums.next(), nums.next()) {
                (Some(Ok(a)), Some(Ok(b))) => (a, b),
                _ => panic!("Invalid input"),
            }
        })
        .collect();

    // sort the data
    data.0.sort();
    data.1.sort();

    // Sum the pairwise absolute differences
    let sum: i32 = data.0.iter().zip(&data.1).map(|(a, b)| (a - b).abs()).sum();

    sum.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let data: Input = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>());

            match (nums.next(), nums.next()) {
                (Some(Ok(a)), Some(Ok(b))) => (a, b),
                _ => panic!("Invalid input"),
            }
        })
        .collect();

    let sum: i32 = data.0.iter().map(|a| a * data.1.iter().filter(|b| a == *b).count() as i32).sum();

    sum.to_string()
}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
