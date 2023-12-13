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


fn expand_vertically(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_v = Vec::<Vec<char>>::new();
    for row in v {
        new_v.push(row.clone());
        if row.iter().all(|x| *x == '.') {
            new_v.push(row.clone());
        }
    }
    return new_v;
}


fn main() {
    let mut sum = 0;
    let mut input = fs::read_to_string("i11.txt").unwrap().lines().map(
        |line| line.chars().collect()
    ).collect();

    input = expand_vertically(transpose(expand_vertically(input)));

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
        }
    }
    println!("{sum}");

}
