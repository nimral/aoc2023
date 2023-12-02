use std::fs;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("i2.txt").unwrap().lines() {
        let mut maxs = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);
        let mut game_str_it = line.split(":");
        game_str_it.next();
        for rgb in game_str_it.next().unwrap().split(";") {
            for num_color in rgb.split(",") {
                let mut num_color_it = num_color.split_whitespace();
                let num: u32 = num_color_it.next().unwrap().parse().unwrap();
                let color = num_color_it.next().unwrap();
                maxs.insert(color, cmp::max(*maxs.get(color).unwrap(), num));
            }
        }
        sum += maxs.get("red").unwrap() *
            maxs.get("green").unwrap() *
            maxs.get("blue").unwrap();
    }
    println!("{sum}");
}
