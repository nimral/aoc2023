use std::fs;
use std::collections::HashMap;

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("i12.txt").unwrap().lines() {
        let (springs_str, tiles_str) = line.split_once(" ").unwrap();
        let orig_tiles: Vec<usize> = tiles_str.split(",").map(|x| x.parse().unwrap()).collect();
        let orig_springs: Vec<char> = springs_str.chars().collect();
        let mut tiles = Vec::<usize>::new();
        let mut springs = Vec::<char>::new();
        for i in 0..5 {
            tiles.extend(orig_tiles.clone());
            springs.extend(orig_springs.clone());
            if i != 4 {
                springs.push('?');
            }
        }

        let mut states = vec![HashMap::<(usize, bool), usize>::new(); springs.len() + 1];
        states[0].insert((0, false), 1);
        for i in 0..springs.len() {
            // could I pls borrow... NO!!!
            let current_states = states[i].clone();
            for ((tile_i, previous_tiled), num) in current_states.iter() {
                if springs[i] == '#' || springs[i] == '?' {
                    if *tile_i < tiles.len() &&
                        i + tiles[*tile_i] <= springs.len() &&
                        springs[i..i + tiles[*tile_i]].iter().all(|x| *x == '#' || *x == '?') &&
                        !previous_tiled 
                    {
                        *states[i + tiles[*tile_i]].entry((tile_i + 1, true)).or_default() += num;
                    }
                }
                if springs[i] == '.' || springs[i] == '?' {
                    if i + 1 < states.len() {
                        *states[i + 1].entry((*tile_i, false)).or_default() += num;
                    }
                }
            }
        }
        for last_tiled in [false, true] {
            sum += match states.last().unwrap().get(&(tiles.len(), last_tiled)) {
                Some(x) => *x,
                None => 0
            }
        }
    }
    println!("{sum}");
}
