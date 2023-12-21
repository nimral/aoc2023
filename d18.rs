use std::fs;


fn main() {
    let plan: Vec<Vec<String>> = fs::read_to_string("i18.txt").unwrap().lines().map(
        |line| line.split_whitespace().map(String::from).collect()
    ).collect();

    let mut min_pos = [0, 0];
    let mut max_pos = [0, 0];
    let mut pos: [isize; 2] = [0, 0];
    for instruction in &plan {
        let step_size: isize = instruction[1].parse().unwrap();
        match instruction[0].as_str() {
            "U" => {pos[0] -= step_size},
            "D" => {pos[0] += step_size},
            "L" => {pos[1] -= step_size},
            "R" => {pos[1] += step_size},
            &_ => unreachable!()
        }
        min_pos[0] = min_pos[0].min(pos[0]);
        min_pos[1] = min_pos[1].min(pos[1]);
        max_pos[0] = max_pos[0].max(pos[0]);
        max_pos[1] = max_pos[1].max(pos[1]);
    }

    let mut field: Vec<Vec<char>> = vec![
        vec!['.'; (max_pos[1] - min_pos[1] + 1) as usize];
        (max_pos[0] - min_pos[0] + 1) as usize
    ];

    pos = [0, 0];
    for instruction in &plan {
        let step_size: usize = instruction[1].parse().unwrap();
        for _ in 0..step_size {
            match instruction[0].as_str() {
                "U" => {pos[0] -= 1},
                "D" => {pos[0] += 1},
                "L" => {pos[1] -= 1},
                "R" => {pos[1] += 1},
                &_ => unreachable!()
            }
            field[
                (pos[0] - min_pos[0]) as usize
            ][
                (pos[1] - min_pos[1]) as usize
            ] = '#';
        }
    }

    let mut num_inside = 0;
    let is_bent_up = |i: usize, j: usize| {
        i > 0 && i + 1 < field.len() &&
        field[i - 1][j] == '#' && field[i + 1][j] == '.'
    };
    for i in 0..field.len() {
        let mut last = '.';
        let mut before_last = '.';
        let mut last_bend_up = false;
        let mut inside = false;
        for j in 0..field[0].len() {
            if field[i][j] == '#' && last == '.' {
                last_bend_up = is_bent_up(i, j);
            }
            if field[i][j] == '.' && last == '#' {
                if !(before_last == '#') ||
                    (i > 0 && i + 1 < field.len() && is_bent_up(i, j - 1) != last_bend_up)
                {
                    inside = !inside;
                }
            }
            before_last = last;
            last = field[i][j];
            if field[i][j] == '#' || inside {
                num_inside += 1;
            }
        }
    }
    println!("{num_inside}");
}
