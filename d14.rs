use std::fs;

fn main() {
    let mut input: Vec<Vec<char>> = fs::read_to_string("i14.txt").unwrap().lines().map(
        |line| line.chars().collect()
    ).collect();

    for x in 0..input[0].len() {
        let mut first_empty = 0;
        for y in 0..input.len() {
            if input[y][x] == '#' {
                first_empty = y + 1;
            } else if input[y][x] == 'O' {
                if y != first_empty {
                    input[first_empty][x] = 'O';
                    input[y][x] = '.';
                }
                first_empty += 1;
            }
        }
    }

    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'O' {
                sum += input.len() - y;
            }
        }
    }
    println!("{sum}");
}
