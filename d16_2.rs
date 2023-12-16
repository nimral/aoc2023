use std::fs;


fn dfs(
    pos: [isize; 2],
    dir: usize,
    input: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<Vec<bool>>>
) {
    if pos[0] < 0 ||
        pos[0] as usize >= input.len() ||
        pos[1] < 0 ||
        pos[1] as usize >= input[0].len()
    {
        return
    }
    if visited[pos[0] as usize][pos[1] as usize][dir] {
        return
    }
    visited[pos[0] as usize][pos[1] as usize][dir] = true;

    let dirs: Vec<[isize; 2]> = vec![[-1, 0], [0, 1], [1, 0], [0, -1]];

    let field = input[pos[0] as usize][pos[1] as usize];
    if (field == '.') ||
        (field == '|' && (dir == 0 || dir == 2)) ||
        (field == '-' && (dir == 1 || dir == 3))
    {
        dfs(
            [
                pos[0] + dirs[dir][0],
                pos[1] + dirs[dir][1],
            ],
            dir,
            input,
            visited
        );
    } else if field == '|' && (dir == 1 || dir == 3) {
        dfs([pos[0] - 1, pos[1]], 0, input, visited);
        dfs([pos[0] + 1, pos[1]], 2, input, visited);
    } else if field == '-' && (dir == 0 || dir == 2) {
        dfs([pos[0], pos[1] - 1], 3, input, visited);
        dfs([pos[0], pos[1] + 1], 1, input, visited);
    } else if (field == '/' && dir == 1) || (field == '\\' && dir == 3) {
        dfs([pos[0] - 1, pos[1]], 0, input, visited);
    } else if (field == '/' && dir == 2) || (field == '\\' && dir == 0) {
        dfs([pos[0], pos[1] - 1], 3, input, visited);
    } else if (field == '/' && dir == 3) || (field == '\\' && dir == 1) {
        dfs([pos[0] + 1, pos[1]], 2, input, visited);
    } else if (field == '/' && dir == 0) || (field == '\\' && dir == 2) {
        dfs([pos[0], pos[1] + 1], 1, input, visited);
    } else {
        unreachable!();
    }
}

fn main () {
    let input: Vec<Vec<char>> = fs::read_to_string("i16.txt").unwrap().lines().map(
        |line| line.chars().collect()
    ).collect();

    let get_num_visited = |pos: [isize; 2], dir: usize| {
        let mut visited = vec![vec![vec![false; 4]; input[0].len()]; input.len()];
        dfs(pos, dir, &input, &mut visited);
        visited.iter().map(
            |row| row.iter().map(
                |v| v.iter().any(|x| *x) as usize
            ).sum::<usize>()
        ).sum::<usize>()
    };

    let mut max = 0;
    for i in 0..input.len() {
        max = max.max(get_num_visited([i as isize, 0], 1));
        max = max.max(get_num_visited([i as isize, input[0].len() as isize - 1], 3));
    }
    for i in 0..input[0].len() {
        max = max.max(get_num_visited([0, i as isize], 2));
        max = max.max(get_num_visited([input.len() as isize - 1, i as isize], 0));
    }

    println!("{}", max);
}

