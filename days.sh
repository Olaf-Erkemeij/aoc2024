#!/bin/bash

# Create Rust files for all 25 days
mkdir -p src/days

# Ensure mod.rs exists
mod_file="src/days/mod.rs"
if [[ ! -f $mod_file ]]; then
    echo "// Modules for each day" > "$mod_file"
fi

for day in {1..25}; do
    day_file="src/days/day${day}.rs"
    if [[ ! -f $day_file ]]; then
        echo "Creating template Rust file: $day_file"
        cat <<EOT > "$day_file"
pub fn solve_part1(input: &str) -> String {
    // Solve part 1
    String::from("Not implemented")
}

pub fn solve_part2(input: &str) -> String {
    // Solve part 2
    String::from("Not implemented")
}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
EOT
        echo "pub mod day${day};" >> "$mod_file"
    else
        echo "File already exists: $day_file"
    fi
done
