use regex::Regex;

pub fn solve_part1(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum: i32 = re
        .captures_iter(input)
        .map(|caps| {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            num1 * num2
        })
        .sum();

    sum.to_string()
}

pub fn solve_part2(input: &str) -> String {

    let re = Regex::new(r"(?s)don't\(\).*?(?:do\(\)|$)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: i32 = re
        .captures_iter(input)
        .filter(|caps| caps.get(1).is_some())
        .map(|caps| {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            num1 * num2
        })
        .sum();

    sum.to_string()
}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
