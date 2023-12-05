use std::fs;

fn main() {
    let mut seeds: Vec<isize> = Vec::new();
    for (i, group) in fs::read_to_string("i5.txt").unwrap().split("\n\n").enumerate() {
        if i == 0 {
            seeds = group.split_once(":").unwrap().1.split_whitespace().map(|x| x.parse().unwrap()).collect();
            continue;
        }
        let mut new_seeds = seeds.clone();
        for line in group.split_once(":\n").unwrap().1.lines() {
            let map: Vec<isize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            for (j, seed) in seeds.iter().enumerate() {
                if (*seed >= map[1]) && (*seed < map[1] + map[2]) {
                    new_seeds[j] += map[0] - map[1];
                }
            }
        }
        seeds = new_seeds;
    }
    println!("{}", seeds.iter().min().unwrap());
}
