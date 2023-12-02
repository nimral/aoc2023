use std::fs;
use std::collections::HashMap;

fn main() {
    let limits = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut sum = 0;
    for line in fs::read_to_string("i2.txt").unwrap().lines() {
        let mut game_str_it = line.split(":");
        let mut game_id_str_it = game_str_it.next().unwrap().split_whitespace();
        game_id_str_it.next();
        let game_id: u32 = game_id_str_it.next().unwrap().parse().unwrap();
        let mut game_possible = true;
        for rgb in game_str_it.next().unwrap().split(";") {
            for num_color in rgb.split(",") {
                let mut num_color_it = num_color.split_whitespace();
                let num: u32 = num_color_it.next().unwrap().parse().unwrap();
                let color = num_color_it.next().unwrap();
                if *limits.get(color).unwrap() < num {
                    game_possible = false;
                    break;
                }
            }
        }
        if game_possible {
            sum += game_id;
        }
    }
    println!("{sum}");
}
