use std::fs;
use std::collections::HashMap;

fn rotate_right(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_v = Vec::<Vec<char>>::new();

    for x in 0..v[0].len() {
        new_v.push(v.iter().rev().map(|row| row[x]).collect());
    }
    return new_v;
}


fn main() {
    let mut platform: Vec<Vec<char>> = fs::read_to_string("i14.txt").unwrap().lines().map(
        |line| line.chars().collect()
    ).collect();

    let mut positions2turn = HashMap::<Vec<[usize; 2]>, usize>::new();
    let max_turns = 1000000000;
    let mut turn = 0;
    let mut in_remainder = false;
    while turn < max_turns {
        if !in_remainder {
            let mut positions = Vec::<[usize; 2]>::new();
            for y in 0..platform.len() {
                for x in 0..platform[0].len() {
                    if platform[y][x] == 'O' {
                        positions.push([y, x]);
                    }
                }
            }

            if positions2turn.contains_key(&positions) {
                let cycle_start = positions2turn.get(&positions).unwrap();
                turn = 1 + max_turns -
                    (max_turns - cycle_start) % (turn - cycle_start) - 1;
                in_remainder = true;
            } else {
                positions2turn.insert(positions, turn);
            }
        }

        for _ in 0..4 {
            for x in 0..platform[0].len() {
                let mut first_empty = 0;
                for y in 0..platform.len() {
                    if platform[y][x] == '#' {
                        first_empty = y + 1;
                    } else if platform[y][x] == 'O' {
                        if y != first_empty {
                            platform[first_empty][x] = 'O';
                            platform[y][x] = '.';
                        }
                        first_empty += 1;
                    }
                }
            }
            platform = rotate_right(platform);
        }
        turn += 1;
    }
    let mut sum = 0;
    for y in 0..platform.len() {
        for x in 0..platform[0].len() {
            if platform[y][x] == 'O' {
                sum += platform.len() - y;
            }
        }
    }
    println!("{sum}");
}
