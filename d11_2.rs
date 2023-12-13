use std::fs;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}


fn main() {
    let mut sum = 0;
    let input: Vec<Vec<char>> = fs::read_to_string("i11.txt").unwrap().lines().map(
        |line| line.chars().collect()
    ).collect();

    let get_is_row_empty = |input: &Vec<Vec<char>>| {
        input.iter().map(|row| row.iter().all(|x| *x == '.')).collect()
    };

    let transposed = transpose(input.clone());
    let is_row_empty: Vec<bool> = get_is_row_empty(&input);
    let is_col_empty: Vec<bool> = get_is_row_empty(&transposed);

    let mut positions = Vec::<(usize, usize)>::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '#' {
                positions.push((i, j))
            }
        }
    }

    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            sum += positions[i].0.abs_diff(positions[j].0);
            sum += positions[i].1.abs_diff(positions[j].1);
            for y in positions[i].0.min(positions[j].0) + 1..positions[i].0.max(positions[j].0) {
                if is_row_empty[y] {
                    sum += 1000000 - 1
                }
            }
            for x in positions[i].1.min(positions[j].1) + 1..positions[i].1.max(positions[j].1) {
                if is_col_empty[x] {
                    sum += 1000000 - 1
                }
            }
        }
    }
    println!("{sum}");
}
