use std::fs;
use std::iter::zip;

fn main() {
    let mut sum: isize = 0;
    for line in fs::read_to_string("i9.txt").unwrap().lines() {
        let nums: Vec<isize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();   

        let mut rows = Vec::<Vec<isize>>::new();
        rows.push(nums);
        while !rows.last().unwrap().iter().all(|x| *x == 0) {
            rows.push(zip(rows.last().unwrap().iter(), rows.last().unwrap().iter().skip(1)).map(|(a, b)| b - a).collect());
        }
        sum += rows.iter().map(|row| *row.last().unwrap()).sum::<isize>();
    }
    println!("{sum}");
}
