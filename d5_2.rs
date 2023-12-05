use std::fs;

// naive solution, 7 m 24 s on Core 2 Duo
fn main() {
    let mut map_groups: Vec<Vec<Vec<isize>>> = Vec::new();
    let mut ranges: Vec<isize> = Vec::new();
    for (i, group) in fs::read_to_string("i5.txt").unwrap().split("\n\n").enumerate() {
        if i == 0 {
            ranges = group.split_once(":").unwrap().1.split_whitespace().map(|x| x.parse().unwrap()).collect();
            continue;
        }

        let mut maps: Vec<Vec<isize>> = Vec::new();
        for line in group.split_once(":\n").unwrap().1.lines() {
            maps.push(line.split_whitespace().map(|x| x.parse().unwrap()).collect());
        }
        map_groups.push(maps);
    }
    let mut min = isize::MAX;
    for j in (0..ranges.len()).step_by(2) {
        for k in ranges[j]..ranges[j] + ranges[j+1] {
            let mut seed = k;
            for map_group in &map_groups {
                for map in map_group {
                    if (seed >= map[1]) && (seed < map[1] + map[2]) {
                        seed += map[0] - map[1];
                        break;
                    }
                }
            }
            min = min.min(seed);
        }
    }
    println!("{}", min);
}
