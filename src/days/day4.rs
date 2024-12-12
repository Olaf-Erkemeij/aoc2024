type Input = Vec<Vec<char>>;

const DIRS: [(i32, i32); 8] = [
    (0, 1),  // Right
    (0, -1), // Left
    (1, 0),  // Down
    (-1, 0), // Up
    (1, 1),  // Down-right
    (-1, -1),// Up-left
    (1, -1), // Down-left
    (-1, 1), // Up-right
];


fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect()
}

pub fn solve_part1(input: &str) -> String {
    let data = parse_input(input);
    let word = "MAS".chars().collect::<Vec<char>>();

    let mut sum = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            // check if the current cell is an X
            if data[i][j] != 'X' {
                continue;
            }

            // check all directions
            for dir in DIRS.iter() {
                let mut x = i as i32 + dir.0;
                let mut y = j as i32 + dir.1;
                let mut count: usize = 0;

                while x >= 0 && x < data.len() as i32 && y >= 0 && y < data[i as usize].len() as i32 {
                    if data[x as usize][y as usize] != word[count] {
                        break;
                    }

                    count += 1;

                    if count == word.len() {
                        sum += 1;
                        break;
                    }

                    x += dir.0;
                    y += dir.1;
                }
            }
        }
    }



    sum.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let data = parse_input(input);
    let mut word = "MS".chars().collect::<Vec<char>>();

    word.sort();

    let mut sum = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            // check if the current cell is an X
            if data[i][j] != 'A' {
                continue;
            }

            let mut diag1: Vec<char> = Vec::new();
            let mut diag2: Vec<char> = Vec::new();

            for dir in DIRS.iter() {
                if dir.0 == 0 || dir.1 == 0 {
                    continue;
                }

                let x = i as i32 + dir.0;
                let y = j as i32 + dir.1;

                if x >= 0 && x < data.len() as i32 && y >= 0 && y < data[i as usize].len() as i32 {
                    if *dir == (1, 1) || *dir == (-1, -1) {
                        diag1.push(data[x as usize][y as usize]);
                    } else {
                        diag2.push(data[x as usize][y as usize]);
                    }
                }
            }

            diag1.sort();
            diag2.sort();

            if diag1 != word || diag2 != word {
                continue;
            }

            sum += 1;
        }
    }



    sum.to_string()
}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
