use std::fs;
use std::collections::HashMap;


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

    let mut previous = start;
    let mut current = neighbor;
    let mut length = 0;
    while current != start {
        length += 1;
        for dir in dirs.get(&(safe_input[current.0 as usize].as_bytes()[current.1 as usize] as char)).unwrap() {
            if (current.0 + dir.0, current.1 + dir.1) != previous {
                previous = current;
                current = (current.0 + dir.0, current.1 + dir.1);
                break;
            }
        }
    }

    println!("{}", length / 2 + length % 2);
}
