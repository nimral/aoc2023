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
    for pattern_str in fs::read_to_string("i13.txt").unwrap().split("\n\n") {
        // I don't need to change it, but I would like to pass it by reference?
        let mut pattern: Vec<Vec<char>> = pattern_str.lines().map(
            |line| line.chars().collect()
        ).collect();

        let get_num_left_cols = |pattern: &mut Vec<Vec<char>>| {
            let mut num_left_cols = 0;
            for mirror in 1..pattern[0].len() {
                let mut num_diffs = 0;
                let mut dist = 0;
                while mirror >= dist + 1 && mirror + dist < pattern[0].len() {
                    num_diffs += pattern.iter().map(
                        |row| (row[mirror - dist - 1] != row[mirror + dist]) as usize
                    ).sum::<usize>();
                    dist += 1;
                }
                if num_diffs == 1 {
                    num_left_cols = mirror;
                    break;
                }
            }
            num_left_cols
        };

        sum += get_num_left_cols(&mut pattern);
        sum += 100 * get_num_left_cols(&mut transpose(pattern));
    }
    println!("{sum}");
}


