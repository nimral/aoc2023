use std::fs;
use std::collections::HashMap;

fn dfs(
    pos: (isize, isize),
    parity: usize,
    safe_input: &Vec<String>,
    dirs: &HashMap<char, Vec<(isize, isize)>>,
    visited: &mut Vec<Vec<bool>>,
    in_dirs: &HashMap<(isize, isize), Vec<(isize, isize)>>,
    depth: usize,
    prev_pos: (isize, isize),
    parity_wrong: &mut bool,
    met_loop: &mut bool,
) -> usize {
    visited[pos.0 as usize][pos.1 as usize] = true;

    let mut num_inside = 0;
    if !in_dirs.contains_key(&pos) {
        num_inside = parity;
    }

    let mut next_parity;
    for (i, j) in vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1), (pos.0, pos.1 - 1), (pos.0, pos.1 + 1)] {
        if (i, j) == pos || i < 0 || j < 0 || i >= safe_input.len() as isize || j >= safe_input[0].len() as isize {
            continue;
        }
        if safe_input[i as usize].as_bytes()[j as usize] as char == 'S' {
            continue;
        }
        if visited[i as usize][j as usize] {
            continue;
        }
        next_parity = parity;
        if in_dirs.contains_key(&pos) {
            if !*met_loop {
                *met_loop = true;
                if in_dirs.get(&(pos)).unwrap().contains(&(prev_pos.0 - pos.0, prev_pos.1 - pos.1)) {
                    *parity_wrong = true;
                }
            }
            if in_dirs.get(&pos).unwrap().contains(&(i - pos.0, j - pos.1)) {
                next_parity = 1 - (*parity_wrong as isize) as usize;
            } else {
                next_parity = 0 + (*parity_wrong as isize) as usize;
            }
        }

        num_inside += dfs((i, j), next_parity, safe_input, dirs, visited, in_dirs, depth + 1, pos, parity_wrong, met_loop);
    }
    return num_inside;
}


fn main() {
    let binding = fs::read_to_string("i10.txt").unwrap();
    let input: Vec<&str> = binding.lines().collect();
    let width = input[0].as_bytes().len();
    let height = input.len();

    let safe_input = [
        &[".".repeat(width + 2)],
        &input.into_iter().map(
            |line| ".".to_owned() + &line + "."
        ).collect::<Vec<String>>()[..],
        &[".".repeat(width + 2)],
    ].concat();

    let mut start = (0, 0);
    for i in 1..height + 1 {
        for j in 1..width + 1 {
            if safe_input[i].as_bytes()[j] as char == 'S' {
                start = (i as isize, j as isize);
            }
        }
    }

    let dirs: HashMap<char, Vec<(isize, isize)>> = HashMap::from([
        ('|', vec![(-1, 0), (1, 0)]),
        ('-', vec![(0, -1), (0, 1)]),
        
        ('J', vec![(-1, 0), (0, -1)]),
        ('F', vec![(1, 0), (0, 1)]),

        ('7', vec![(0, -1), (1, 0)]),
        ('L', vec![(-1, 0), (0, 1)]),

        ('.', vec![]),
    ]);

    let mut neighbor = (0, 0);
    for i2 in start.0-1..start.0+2 {
        for j2 in start.1-1..start.1+2 {
            if (i2, j2) == start {
                continue;
            }
            for dir in dirs.get(&(safe_input[i2 as usize].as_bytes()[j2 as usize] as char)).unwrap() {
                if (i2 as isize + dir.0, j2 as isize + dir.1) == start {
                    neighbor = (i2, j2);
                }
            }
        }
    }

    let mut in_dirs: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    in_dirs.insert(start, vec![]);

    let mut previous = start;
    let mut current = neighbor;
    let mut next = start;
    let neighbor_dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    while current != start {
        for dir in dirs.get(&(safe_input[current.0 as usize].as_bytes()[current.1 as usize] as char)).unwrap() {
            if (current.0 + dir.0, current.1 + dir.1) != previous {
                next = (current.0 + dir.0, current.1 + dir.1);
            }
        }

        let mut started = false;
        for i in 0..8 {
            let nd = neighbor_dirs[i % 4];
            if (current.0 + nd.0, current.1 + nd.1) == previous {
                started = true;
            }
            if started {
                in_dirs.entry(current).or_default().push(nd);
                if (current.0 + nd.0, current.1 + nd.1) == next {
                    break;
                }
            }
        }
        previous = current;
        current = next;
    }

    let mut visited = vec![vec![false; width + 2]; height + 2];
    let mut parity_wrong = false;
    let mut met_loop = false;
    println!(
        "{}",
        dfs(
            (0, 0),
            0,
            &safe_input,
            &dirs,
            &mut visited,
            &in_dirs,
            0,
            (0, 0),
            &mut parity_wrong,
            &mut met_loop
        )
    );
}
