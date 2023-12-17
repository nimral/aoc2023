use std::fs;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone)]
struct State {
    steps_in_row: usize,
    x: isize,
    y: isize,
    prev_x: isize,
    prev_y: isize,
}

fn main() {
    let heat_losses: Vec<Vec<usize>> = fs::read_to_string("i17.txt").unwrap().lines().map(
        |line| line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect()
    ).collect();

    let mut min_dists = HashMap::new();
    let mut heap = BinaryHeap::new();
    let init_state_x = State {
        steps_in_row: 0,
        y: 0,
        x: 0,
        prev_y: 0,
        prev_x: -1,
    };
    let init_state_y = State {
        steps_in_row: 0,
        y: 0,
        x: 0,
        prev_y: -1,
        prev_x: 0,
    };
    min_dists.insert(init_state_x.clone(), 0);
    min_dists.insert(init_state_y.clone(), 0);
    heap.push(Reverse((0, init_state_x)));
    heap.push(Reverse((0, init_state_y)));

    loop {
        let Reverse((dist, state)) = heap.pop().unwrap();
        if state.y as usize == heat_losses.len() - 1 &&
           state.x as usize == heat_losses[0].len() - 1 {
            println!("{}", dist);
            break;
        }

        match min_dists.get(&state) {
            Some(min_dist) => {
                if *min_dist < dist {
                    continue;
                }
            },
            None => {}
        }
        for dir in vec![[-1, 0], [0, 1], [1, 0], [0, -1]] {
            let new_y = state.y + dir[0];
            let new_x = state.x + dir[1];

            let mut new_steps_in_row = 1;
            if [state.y - state.prev_y, state.x - state.prev_x] == dir {
                new_steps_in_row += state.steps_in_row;
            } else if state.steps_in_row < 4 {
                continue;
            }
            if !(state.prev_y == new_y && state.prev_x == new_x) &&
                new_y >= 0 && (new_y as usize) < heat_losses.len() &&
                new_x >= 0 && (new_x as usize) < heat_losses[0].len() &&
                new_steps_in_row <= 10
            {
                let new_dist = dist + heat_losses[new_y as usize][new_x as usize];

                let new_state = State {
                    steps_in_row: new_steps_in_row,
                    y: new_y,
                    x: new_x,
                    prev_y: state.y,
                    prev_x: state.x
                };
                match min_dists.get(&new_state) {
                    Some(min_dist) => {
                        if *min_dist <= new_dist {
                            continue;
                        }
                    },
                    None => {}
                }
                min_dists.insert(new_state.clone(), new_dist);
                heap.push(Reverse((new_dist, new_state.clone())));
            }
        }
    }
}
